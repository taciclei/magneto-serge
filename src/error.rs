//! Error types for magneto-serge

use thiserror::Error;

/// Result type alias for magneto-serge operations
pub type Result<T> = std::result::Result<T, MatgtoError>;

/// Main error type for magneto-serge
#[derive(Error, Debug)]
pub enum MatgtoError {
    /// Cassette file not found
    #[error("Cassette not found: {name}")]
    CassetteNotFound { name: String },

    /// Failed to load cassette
    #[error("Failed to load cassette: {reason}")]
    CassetteLoadFailed { reason: String },

    /// No matching interaction found in cassette
    #[error("No matching interaction for {method} {url}")]
    NoMatchingInteraction { method: String, url: String },

    /// No interaction found (generic)
    #[error("No interaction found")]
    NoInteractionFound,

    /// Failed to record interaction
    #[error("Recording failed: {reason}")]
    RecordingFailed { reason: String },

    /// Failed to start proxy
    #[error("Proxy start failed: {reason}")]
    ProxyStartFailed { reason: String },

    /// I/O error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(String),

    /// WebSocket error
    #[error("WebSocket error: {reason}")]
    WebSocketError { reason: String },

    /// Serialization error (JSON)
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// MessagePack decode error
    #[cfg(feature = "msgpack")]
    #[error("MessagePack decode error: {0}")]
    MessagePackDecode(#[from] rmp_serde::decode::Error),

    /// MessagePack encode error
    #[cfg(feature = "msgpack")]
    #[error("MessagePack encode error: {0}")]
    MessagePackEncode(#[from] rmp_serde::encode::Error),

    /// Channel send error
    #[error("Channel send error")]
    ChannelSend,

    /// TLS/Certificate error
    #[error("TLS error: {0}")]
    Tls(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Regex compilation error
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// URL parsing error
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for MatgtoError {
    fn from(_: tokio::sync::mpsc::error::SendError<T>) -> Self {
        MatgtoError::ChannelSend
    }
}
