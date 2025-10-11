//! Core proxy implementation

// Module declarations (must come first)
pub mod client;
pub mod http_handler;
pub mod server;

use crate::error::{MatgtoError, Result};
use crate::player::Player;
use crate::recorder::Recorder;
use crate::tls::CertificateAuthority;
use std::path::PathBuf;
use std::sync::{Arc, Mutex as StdMutex};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

// Import from submodules
use self::server::ProxyServer;
pub use client::HttpForwarder;
pub use http_handler::HttpHandler;
pub use server::MatgtoHttpHandler;

/// Proxy operation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyMode {
    /// Auto mode: Record if cassette doesn't exist, otherwise replay
    Auto,

    /// Always record (overwrites existing cassette)
    Record,

    /// Always replay (errors if cassette doesn't exist)
    Replay,

    /// Strict replay mode: Errors on missing cassette AND missing interactions
    /// Use this mode in CI/CD to ensure all network calls are captured
    ReplayStrict,

    /// Transparent proxy without record/replay
    Passthrough,
}

/// Internal mutable state for MagnetoProxy
struct ProxyState {
    /// Directory where cassettes are stored
    cassette_dir: PathBuf,

    /// Proxy listening port
    proxy_port: u16,

    /// Current operation mode
    mode: ProxyMode,

    /// Current cassette name (if recording/replaying)
    current_cassette: Option<String>,

    /// Current recorder (if in Record mode)
    recorder: Option<Arc<Mutex<Recorder>>>,

    /// Current player (if in Replay mode)
    player: Option<Arc<Mutex<Player>>>,
}

/// Main proxy struct - uses interior mutability for UniFFI compatibility
pub struct MagnetoProxy {
    /// Mutable state protected by Mutex
    state: Arc<StdMutex<ProxyState>>,

    /// Tokio runtime for async operations (immutable, shared)
    runtime: Arc<Runtime>,

    /// Certificate authority for MITM (immutable, shared)
    ca: Arc<CertificateAuthority>,
}

impl MagnetoProxy {
    /// Create a new proxy instance (internal version with Result)
    pub fn new_internal(cassette_dir: impl Into<PathBuf>) -> Result<Self> {
        let cassette_dir = cassette_dir.into();

        let runtime = Runtime::new().map_err(|e| MatgtoError::ProxyStartFailed {
            reason: format!("Failed to create Tokio runtime: {}", e),
        })?;

        // Create certificate authority
        let ca_dir = cassette_dir
            .parent()
            .unwrap_or(cassette_dir.as_ref())
            .join(".magneto/certs");

        let ca = Arc::new(CertificateAuthority::new(ca_dir)?);

        let state = ProxyState {
            cassette_dir,
            proxy_port: 8888,
            mode: ProxyMode::Auto,
            current_cassette: None,
            recorder: None,
            player: None,
        };

        Ok(Self {
            state: Arc::new(StdMutex::new(state)),
            runtime: Arc::new(runtime),
            ca,
        })
    }

    /// Create a new proxy instance (UniFFI compatible - panics on error)
    /// For non-UniFFI Rust code, use new_internal() instead
    pub fn new(cassette_dir: String) -> Self {
        use std::path::Path;
        Self::new_internal(Path::new(&cassette_dir)).expect("Failed to create MagnetoProxy")
    }

    /// Set the proxy port (builder style - returns clone for chaining)
    pub fn with_port(self, port: u16) -> Self {
        self.set_port(port);
        self
    }

    /// Set the proxy mode (builder style - returns clone for chaining)
    pub fn with_mode(self, mode: ProxyMode) -> Self {
        self.set_mode(mode);
        self
    }

    /// Set the proxy port (setter style for UniFFI)
    pub fn set_port(&self, port: u16) {
        let mut state = self.state.lock().unwrap();
        state.proxy_port = port;
    }

    /// Set the proxy mode (setter style for UniFFI)
    pub fn set_mode(&self, mode: ProxyMode) {
        let mut state = self.state.lock().unwrap();
        state.mode = mode;
    }

    /// Get the current proxy port
    pub fn port(&self) -> u16 {
        let state = self.state.lock().unwrap();
        state.proxy_port
    }

    /// Get the current proxy mode
    pub fn mode(&self) -> ProxyMode {
        let state = self.state.lock().unwrap();
        state.mode
    }

    /// Start recording a new cassette (internal version with Result)
    pub fn start_recording_internal(&self, cassette_name: String) -> Result<()> {
        let mut state = self.state.lock().unwrap();

        state.current_cassette = Some(cassette_name.clone());

        // Create recorder
        let recorder = Arc::new(Mutex::new(Recorder::new(cassette_name.clone())));
        state.recorder = Some(recorder.clone());

        // Create and start proxy server
        let server = ProxyServer::new(state.proxy_port, self.ca.clone(), ProxyMode::Record)?
            .with_recorder(recorder);

        tracing::info!("🎬 Starting recording for cassette: {}", cassette_name);

        // Start server in background (non-blocking for now)
        let runtime_handle = self.runtime.handle().clone();
        runtime_handle.spawn(async move {
            if let Err(e) = server.start().await {
                tracing::error!("Proxy server error: {}", e);
            }
        });

        Ok(())
    }

    /// Start recording a new cassette (UniFFI compatible - returns bool)
    pub fn start_recording(&self, cassette_name: String) -> bool {
        self.start_recording_internal(cassette_name).is_ok()
    }

    /// Stop recording and save the cassette (internal version with Result)
    pub fn stop_recording_internal(&self) -> Result<()> {
        let mut state = self.state.lock().unwrap();

        if let Some(cassette_name) = state.current_cassette.take() {
            tracing::info!("💾 Stopping recording for cassette: {}", cassette_name);

            // Save the cassette
            if let Some(recorder) = state.recorder.take() {
                let cassette_dir = state.cassette_dir.clone();
                let runtime = self.runtime.clone();
                // Drop the lock before blocking
                drop(state);

                runtime.block_on(async move {
                    let recorder_lock = recorder.lock().await;
                    recorder_lock.save(&cassette_dir)
                })?;

                tracing::info!("✅ Cassette saved");
            }

            Ok(())
        } else {
            Err(MatgtoError::RecordingFailed {
                reason: "No recording in progress".to_string(),
            })
        }
    }

    /// Stop recording and save the cassette (UniFFI compatible - returns bool)
    pub fn stop_recording(&self) -> bool {
        self.stop_recording_internal().is_ok()
    }

    /// Replay an existing cassette (internal version with Result)
    pub fn replay_internal(&self, cassette_name: String) -> Result<()> {
        let mut state = self.state.lock().unwrap();

        state.current_cassette = Some(cassette_name.clone());

        // Load cassette
        let cassette_dir = state.cassette_dir.clone();
        let player = Player::load(&cassette_dir, &cassette_name)?;

        let player_arc = Arc::new(Mutex::new(player));
        state.player = Some(player_arc.clone());

        // Create and start proxy server in replay mode
        let server = ProxyServer::new(state.proxy_port, self.ca.clone(), ProxyMode::Replay)?
            .with_player(player_arc);

        tracing::info!("▶️  Starting replay for cassette: {}", cassette_name);

        // Start server in background
        let runtime_handle = self.runtime.handle().clone();
        runtime_handle.spawn(async move {
            if let Err(e) = server.start().await {
                tracing::error!("Proxy server error: {}", e);
            }
        });

        Ok(())
    }

    /// Replay an existing cassette (UniFFI compatible - returns bool)
    pub fn replay(&self, cassette_name: String) -> bool {
        self.replay_internal(cassette_name).is_ok()
    }

    /// Replay an existing cassette in STRICT mode (internal version with Result)
    /// In strict mode, any request not found in the cassette will cause an error
    pub fn replay_strict_internal(&self, cassette_name: String) -> Result<()> {
        let mut state = self.state.lock().unwrap();

        state.current_cassette = Some(cassette_name.clone());

        // Load cassette in strict mode
        let cassette_dir = state.cassette_dir.clone();
        let player = Player::load_strict(&cassette_dir, &cassette_name)?;

        let player_arc = Arc::new(Mutex::new(player));
        state.player = Some(player_arc.clone());

        // Create and start proxy server in strict replay mode
        let server = ProxyServer::new(state.proxy_port, self.ca.clone(), ProxyMode::ReplayStrict)?
            .with_player(player_arc);

        tracing::info!("🔒 Starting STRICT replay for cassette: {}", cassette_name);
        tracing::info!("⚠️  Any missing interaction will cause an error");

        // Start server in background
        let runtime_handle = self.runtime.handle().clone();
        runtime_handle.spawn(async move {
            if let Err(e) = server.start().await {
                tracing::error!("Proxy server error: {}", e);
            }
        });

        Ok(())
    }

    /// Replay an existing cassette in STRICT mode (UniFFI compatible - returns bool)
    pub fn replay_strict(&self, cassette_name: String) -> bool {
        self.replay_strict_internal(cassette_name).is_ok()
    }

    /// Shutdown the proxy (internal version with Result)
    pub fn shutdown_internal(&self) -> Result<()> {
        let mut state = self.state.lock().unwrap();

        // TODO: Stop the proxy server
        tracing::info!("Shutting down proxy");

        state.current_cassette = None;
        Ok(())
    }

    /// Shutdown the proxy (UniFFI compatible)
    pub fn shutdown(&self) {
        let _ = self.shutdown_internal();
    }
}

impl Drop for MagnetoProxy {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_creation() {
        let proxy = MagnetoProxy::new("./cassettes".to_string());
        assert_eq!(proxy.port(), 8888);
        assert_eq!(proxy.mode(), ProxyMode::Auto);
    }

    #[test]
    fn test_proxy_with_custom_port() {
        let proxy = MagnetoProxy::new("./cassettes".to_string());
        proxy.set_port(9999);
        assert_eq!(proxy.port(), 9999);
    }

    #[test]
    fn test_proxy_with_mode() {
        let proxy = MagnetoProxy::new("./cassettes".to_string());
        proxy.set_mode(ProxyMode::Record);
        assert_eq!(proxy.mode(), ProxyMode::Record);
    }
}
