//! # magneto-serge
//!
//! Multi-language HTTP/WebSocket proxy library for testing with record/replay capabilities.
//!
//! This library provides a MITM proxy that intercepts HTTP/HTTPS and WebSocket traffic,
//! records interactions into "cassettes", and can replay them deterministically.

// Allow clippy warnings from UniFFI generated code
#![allow(clippy::empty_line_after_doc_comments)]

pub mod cassette;
pub mod error;
pub mod player;
pub mod proxy;
pub mod recorder;
pub mod tls;
pub mod websocket;

pub use error::{MatgtoError, Result};
pub use proxy::{MagnetoProxy, ProxyMode};
pub use tls::CertificateAuthority;
pub use websocket::{WebSocketInterceptor, WebSocketPlayer, WebSocketRecorder};

// Re-export common types
pub use cassette::{Cassette, HttpRequest, HttpResponse, Interaction, WebSocketMessage};

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
