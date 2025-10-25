//! # magneto-serge
//!
//! Multi-language HTTP/WebSocket proxy library for testing with record/replay capabilities.
//!
//! This library provides a MITM proxy that intercepts HTTP/HTTPS and WebSocket traffic,
//! records interactions into "cassettes", and can replay them deterministically.

// Allow clippy warnings from UniFFI generated code
#![allow(clippy::empty_line_after_doc_comments)]

// Core modules (always available)
pub mod cassette;
pub mod cookies;
pub mod error;
pub mod filters;
pub mod hooks;
pub mod matching;
pub mod player;
pub mod proxy;
pub mod recorder;
pub mod test_helpers;
pub mod tls;
pub mod websocket;

// Optional API module (requires 'api' feature)
#[cfg(feature = "api")]
pub mod api;

// Core exports (always available)
pub use error::{MatgtoError, Result};
pub use filters::{FilterPresets, RecordingFilters};
pub use hooks::{RecordHook, RecordHooks, ReplayHook, ReplayHooks};
pub use matching::{
    BodyMatchMode, CustomMatcher, MatchingStrategy, RequestSignature, UrlMatchMode,
};
pub use player::{LatencyMode, Player};
pub use proxy::{MagnetoProxy, ProxyMode};
pub use recorder::Recorder;
pub use tls::CertificateAuthority;
pub use websocket::{WebSocketInterceptor, WebSocketPlayer, WebSocketRecorder};

// Re-export cassette types
pub use cassette::{Cassette, HttpRequest, HttpResponse, Interaction, WebSocketMessage};

// Re-export cookie types
pub use cookies::{Cookie, CookieJar, SameSite};

// API exports (only when 'api' feature is enabled)
#[cfg(feature = "api")]
pub use api::{
    ApiConfig, ApiResponse, ApiServer, ProxyStatus, StartProxyRequest, StopProxyRequest,
};

// UniFFI factory function
/// Create a new MagnetoProxy instance (returns None on error)
///
/// This is a convenience function for language bindings
pub fn create_proxy(cassette_dir: String) -> Option<std::sync::Arc<MagnetoProxy>> {
    use proxy::MagnetoProxy;
    let path: &std::path::Path = cassette_dir.as_ref();
    MagnetoProxy::new_internal(path)
        .ok()
        .map(std::sync::Arc::new)
}

/// Get library version
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Include UniFFI scaffolding
uniffi::include_scaffolding!("magneto_serge");

#[cfg(test)]
mod tests {
    #[test]
    fn test_library_loads() {
        // Basic smoke test - verify version is set
        assert!(!crate::version().is_empty());
    }
}
