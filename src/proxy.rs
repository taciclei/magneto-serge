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

    /// Hybrid mode: Replay from cassette if interaction exists, otherwise record new
    /// Perfect for evolving APIs where you want to keep old interactions but record new ones
    Hybrid,

    /// Once mode: Record only if cassette doesn't exist, then always replay
    /// Protects against accidental overwrites - cassette is immutable once created
    Once,

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

    /// Get the current cassette name (if any)
    pub fn current_cassette_name(&self) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.current_cassette.clone()
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

        eprintln!("🎬 Starting recording for cassette: {}", cassette_name);
        tracing::info!("🎬 Starting recording for cassette: {}", cassette_name);

        // Start server in dedicated thread with its own Tokio runtime
        // This ensures the async server task actually runs
        std::thread::spawn(move || {
            eprintln!("🧵 Proxy server thread started");
            // Create a new runtime for this thread
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            eprintln!("✅ Tokio runtime created in thread");

            // Block on the server start - this will run until shutdown
            if let Err(e) = rt.block_on(server.start()) {
                eprintln!("❌ Proxy server error: {}", e);
                tracing::error!("Proxy server error: {}", e);
            }
            eprintln!("🧵 Proxy server thread ending");
        });

        // Give the proxy server a moment to start listening
        // The server starts in a background thread, so we need to wait briefly
        eprintln!("⏳ Waiting for proxy to start listening...");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        eprintln!("✅ Wait complete, proxy should be ready");

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
                // Drop the lock before saving
                drop(state);

                // Try to lock the recorder (should succeed immediately since we own the Arc)
                // Use try_lock to avoid async context issues
                let can_lock_immediately = recorder.try_lock().is_ok();

                if can_lock_immediately {
                    // Lock again (we know it will succeed)
                    let recorder_guard = recorder.try_lock().unwrap();
                    recorder_guard.save(&cassette_dir)?;
                    tracing::info!("✅ Cassette saved");
                } else {
                    // If try_lock fails, fall back to spawning a thread
                    let save_result = std::thread::spawn(move || {
                        let rt =
                            tokio::runtime::Runtime::new().expect("Failed to create save runtime");
                        let result = rt.block_on(async move {
                            let recorder_lock = recorder.lock().await;
                            recorder_lock.save(&cassette_dir)
                        });
                        // Forget the runtime to avoid "drop in async context" error
                        std::mem::forget(rt);
                        result
                    })
                    .join()
                    .map_err(|e| MatgtoError::RecordingFailed {
                        reason: format!("Save thread panicked: {:?}", e),
                    })?;
                    save_result?;
                    tracing::info!("✅ Cassette saved (via thread)");
                }
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

    /// Start hybrid mode: replay existing interactions, record new ones (internal version with Result)
    /// This mode is perfect for:
    /// - Evolving APIs: Keep old interactions, record new endpoints
    /// - Incremental testing: Gradually build up cassettes
    /// - API exploration: Capture only new interactions during development
    pub fn hybrid_internal(&self, cassette_name: String) -> Result<()> {
        let mut state = self.state.lock().unwrap();

        state.current_cassette = Some(cassette_name.clone());

        let cassette_dir = state.cassette_dir.clone();

        // Try to load existing cassette, or create new one
        let (player, recorder) =
            match Player::load(&cassette_dir, &cassette_name) {
                Ok(player) => {
                    tracing::info!(
                        "📼 Loaded existing cassette '{}' for hybrid mode",
                        cassette_name
                    );
                    tracing::info!(
                        "   Existing interactions will be replayed, new ones will be recorded"
                    );

                    // Load existing cassette into recorder to append to it
                    let cassette = player.cassette().cloned().ok_or_else(|| {
                        MatgtoError::CassetteNotFound {
                            name: cassette_name.clone(),
                        }
                    })?;

                    let mut recorder = Recorder::new(cassette_name.clone());
                    // Copy existing interactions
                    recorder.cassette_mut().interactions = cassette.interactions.clone();

                    (Some(player), recorder)
                }
                Err(_) => {
                    tracing::info!("📹 No existing cassette found, starting fresh in hybrid mode");
                    tracing::info!(
                        "   All interactions will be recorded to '{}'",
                        cassette_name
                    );

                    (None, Recorder::new(cassette_name.clone()))
                }
            };

        let recorder_arc = Arc::new(Mutex::new(recorder));
        state.recorder = Some(recorder_arc.clone());

        let player_arc = player.map(|p| Arc::new(Mutex::new(p)));
        state.player = player_arc.clone();

        // Create and start proxy server in hybrid mode
        let mut server = ProxyServer::new(state.proxy_port, self.ca.clone(), ProxyMode::Hybrid)?
            .with_recorder(recorder_arc);

        if let Some(player) = player_arc {
            server = server.with_player(player);
        }

        tracing::info!("🔀 Starting HYBRID mode for cassette: {}", cassette_name);

        // Start server in background
        let runtime_handle = self.runtime.handle().clone();
        runtime_handle.spawn(async move {
            if let Err(e) = server.start().await {
                tracing::error!("Proxy server error: {}", e);
            }
        });

        Ok(())
    }

    /// Start hybrid mode: replay existing interactions, record new ones (UniFFI compatible - returns bool)
    pub fn hybrid(&self, cassette_name: String) -> bool {
        self.hybrid_internal(cassette_name).is_ok()
    }

    /// Stop hybrid mode and save the cassette with new interactions (internal version with Result)
    pub fn stop_hybrid_internal(&self) -> Result<()> {
        // Same as stop_recording since we need to save the updated cassette
        self.stop_recording_internal()
    }

    /// Stop hybrid mode and save the cassette (UniFFI compatible - returns bool)
    pub fn stop_hybrid(&self) -> bool {
        self.stop_hybrid_internal().is_ok()
    }

    /// Start once mode: record if cassette doesn't exist, otherwise replay (internal version with Result)
    /// This mode protects against accidental overwrites - cassette becomes immutable once created
    /// Perfect for:
    /// - Production tests: Never overwrite recorded cassettes
    /// - Safety: Prevent accidental re-recording
    /// - CI/CD: Ensure cassettes are preserved
    pub fn once_internal(&self, cassette_name: String) -> Result<()> {
        let mut state = self.state.lock().unwrap();

        state.current_cassette = Some(cassette_name.clone());

        let cassette_dir = state.cassette_dir.clone();

        // Try to load existing cassette
        let cassette_exists = cassette_dir
            .join(format!("{}.json", cassette_name))
            .exists()
            || cassette_dir
                .join(format!("{}.json.gz", cassette_name))
                .exists()
            || cassette_dir
                .join(format!("{}.msgpack", cassette_name))
                .exists()
            || cassette_dir
                .join(format!("{}.msgpack.gz", cassette_name))
                .exists();

        if cassette_exists {
            // Cassette exists, switch to replay mode (read-only)
            tracing::info!(
                "🔒 Once mode: Cassette '{}' exists, using replay (read-only)",
                cassette_name
            );

            let player = Player::load(&cassette_dir, &cassette_name)?;
            let player_arc = Arc::new(Mutex::new(player));
            state.player = Some(player_arc.clone());

            // Create and start proxy server in once mode (will replay)
            let server = ProxyServer::new(state.proxy_port, self.ca.clone(), ProxyMode::Once)?
                .with_player(player_arc);

            // Start server in background
            let runtime_handle = self.runtime.handle().clone();
            runtime_handle.spawn(async move {
                if let Err(e) = server.start().await {
                    tracing::error!("Proxy server error: {}", e);
                }
            });
        } else {
            // Cassette doesn't exist, record it
            tracing::info!(
                "📹 Once mode: Cassette '{}' doesn't exist, recording (first time only)",
                cassette_name
            );

            let recorder = Arc::new(Mutex::new(Recorder::new(cassette_name.clone())));
            state.recorder = Some(recorder.clone());

            // Create and start proxy server in once mode (will record)
            let server = ProxyServer::new(state.proxy_port, self.ca.clone(), ProxyMode::Once)?
                .with_recorder(recorder);

            // Start server in background
            let runtime_handle = self.runtime.handle().clone();
            runtime_handle.spawn(async move {
                if let Err(e) = server.start().await {
                    tracing::error!("Proxy server error: {}", e);
                }
            });
        }

        Ok(())
    }

    /// Start once mode: record if cassette doesn't exist, otherwise replay (UniFFI compatible - returns bool)
    pub fn once(&self, cassette_name: String) -> bool {
        self.once_internal(cassette_name).is_ok()
    }

    /// Stop once mode and save the cassette if it was recording (internal version with Result)
    pub fn stop_once_internal(&self) -> Result<()> {
        // If recording, save the cassette
        // If replaying, just clean up
        let state = self.state.lock().unwrap();

        if state.recorder.is_some() {
            // Was recording, save the cassette
            drop(state);
            self.stop_recording_internal()
        } else {
            // Was replaying, just clean up
            Ok(())
        }
    }

    /// Stop once mode (UniFFI compatible - returns bool)
    pub fn stop_once(&self) -> bool {
        self.stop_once_internal().is_ok()
    }

    /// Start the proxy and wait (blocks until Ctrl+C)
    pub async fn start(&self) -> Result<()> {
        // The proxy should already be started via start_recording, replay, etc.
        // This just waits for Ctrl+C
        tracing::info!("Proxy running... Press Ctrl+C to stop");
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("Shutting down...");
        Ok(())
    }

    /// Start in auto mode: replay if cassette exists, record if not
    pub fn auto(&self, cassette_name: &str) {
        let cassette_dir = {
            let state = self.state.lock().unwrap();
            state.cassette_dir.clone()
        };

        // Check if cassette exists
        let cassette_exists = cassette_dir
            .join(format!("{}.json", cassette_name))
            .exists()
            || cassette_dir
                .join(format!("{}.json.gz", cassette_name))
                .exists()
            || cassette_dir
                .join(format!("{}.msgpack", cassette_name))
                .exists()
            || cassette_dir
                .join(format!("{}.msgpack.gz", cassette_name))
                .exists();

        if cassette_exists {
            tracing::info!("🔄 Auto mode: Cassette exists, replaying");
            let _ = self.replay_internal(cassette_name.to_string());
        } else {
            tracing::info!("🔄 Auto mode: Cassette doesn't exist, recording");
            let _ = self.start_recording_internal(cassette_name.to_string());
        }
    }

    /// Start in passthrough mode (no recording/replaying)
    pub fn passthrough(&self) {
        let mut state = self.state.lock().unwrap();
        state.mode = ProxyMode::Passthrough;

        let server =
            ProxyServer::new(state.proxy_port, self.ca.clone(), ProxyMode::Passthrough).unwrap();

        tracing::info!("🔀 Starting passthrough mode");

        let runtime_handle = self.runtime.handle().clone();
        runtime_handle.spawn(async move {
            if let Err(e) = server.start().await {
                tracing::error!("Proxy server error: {}", e);
            }
        });
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
