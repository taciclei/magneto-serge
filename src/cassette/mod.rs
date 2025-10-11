//! Cassette format definitions and types

pub mod storage;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export storage types
pub use storage::{detect_format, AsyncCassetteStorage, BufferedCassetteWriter, CassetteFormat};

/// A cassette containing recorded HTTP/WebSocket interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cassette {
    /// Cassette format version
    pub version: String,

    /// Cassette name
    pub name: String,

    /// Recording timestamp
    pub recorded_at: DateTime<Utc>,

    /// List of recorded interactions
    pub interactions: Vec<Interaction>,
}

/// A single recorded interaction (HTTP or WebSocket)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// Type of interaction
    #[serde(flatten)]
    pub kind: InteractionKind,

    /// When this interaction was recorded
    pub recorded_at: DateTime<Utc>,

    /// Response time in milliseconds (for latency simulation)
    /// None if not recorded or unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<u64>,
}

/// Network error types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "error_type")]
pub enum NetworkError {
    /// DNS resolution failed
    DnsResolutionFailed { message: String },

    /// Connection refused by server
    ConnectionRefused { message: String },

    /// Connection timed out
    Timeout { message: String, timeout_ms: u64 },

    /// TLS/SSL error
    TlsError { message: String },

    /// Connection reset by peer
    ConnectionReset { message: String },

    /// Too many redirects
    TooManyRedirects {
        message: String,
        redirect_count: usize,
    },

    /// Other network error
    Other { message: String },
}

/// Type of interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InteractionKind {
    /// HTTP request/response pair
    Http {
        request: HttpRequest,
        response: HttpResponse,
    },

    /// HTTP request that resulted in an error (timeout, DNS failure, connection refused, etc.)
    HttpError {
        request: HttpRequest,
        error: NetworkError,
    },

    /// WebSocket connection with messages
    WebSocket {
        url: String,
        messages: Vec<WebSocketMessage>,
        close_frame: Option<CloseFrame>,
    },
}

/// HTTP request data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    /// HTTP method (GET, POST, etc.)
    pub method: String,

    /// Request URL
    pub url: String,

    /// Request headers
    pub headers: HashMap<String, String>,

    /// Request body (None if empty)
    pub body: Option<Vec<u8>>,
}

/// HTTP response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    /// HTTP status code
    pub status: u16,

    /// Response headers
    pub headers: HashMap<String, String>,

    /// Response body (None if empty)
    pub body: Option<Vec<u8>>,
}

/// WebSocket message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    /// Message direction
    pub direction: Direction,

    /// Timestamp in milliseconds (relative to connection start)
    pub timestamp_ms: u64,

    /// Message payload
    #[serde(flatten)]
    pub payload: MessagePayload,
}

/// Message direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Direction {
    /// Client → Server
    Sent,

    /// Server → Client
    Received,
}

/// WebSocket message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "msg_type")]
pub enum MessagePayload {
    /// Text message
    Text { data: String },

    /// Binary message
    Binary { data: Vec<u8> },

    /// Ping frame
    Ping { data: Vec<u8> },

    /// Pong frame
    Pong { data: Vec<u8> },
}

/// WebSocket close frame
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseFrame {
    /// Close code
    pub code: u16,

    /// Close reason
    pub reason: String,
}

impl Cassette {
    /// Create a new empty cassette
    pub fn new(name: String) -> Self {
        Self {
            version: "1.0".to_string(),
            name,
            recorded_at: Utc::now(),
            interactions: Vec::new(),
        }
    }

    /// Add an interaction to the cassette
    pub fn add_interaction(&mut self, kind: InteractionKind) {
        self.interactions.push(Interaction {
            kind,
            recorded_at: Utc::now(),
            response_time_ms: None,
        });
    }

    /// Add an interaction with response time
    pub fn add_interaction_with_timing(&mut self, kind: InteractionKind, response_time_ms: u64) {
        self.interactions.push(Interaction {
            kind,
            recorded_at: Utc::now(),
            response_time_ms: Some(response_time_ms),
        });
    }

    /// Add a network error interaction
    pub fn add_error(&mut self, request: HttpRequest, error: NetworkError) {
        self.interactions.push(Interaction {
            kind: InteractionKind::HttpError { request, error },
            recorded_at: Utc::now(),
            response_time_ms: None,
        });
    }
}

impl NetworkError {
    /// Create a DNS resolution error
    pub fn dns_failed(message: impl Into<String>) -> Self {
        Self::DnsResolutionFailed {
            message: message.into(),
        }
    }

    /// Create a connection refused error
    pub fn connection_refused(message: impl Into<String>) -> Self {
        Self::ConnectionRefused {
            message: message.into(),
        }
    }

    /// Create a timeout error
    pub fn timeout(message: impl Into<String>, timeout_ms: u64) -> Self {
        Self::Timeout {
            message: message.into(),
            timeout_ms,
        }
    }

    /// Create a TLS error
    pub fn tls_error(message: impl Into<String>) -> Self {
        Self::TlsError {
            message: message.into(),
        }
    }

    /// Create a connection reset error
    pub fn connection_reset(message: impl Into<String>) -> Self {
        Self::ConnectionReset {
            message: message.into(),
        }
    }

    /// Create a too many redirects error
    pub fn too_many_redirects(message: impl Into<String>, redirect_count: usize) -> Self {
        Self::TooManyRedirects {
            message: message.into(),
            redirect_count,
        }
    }

    /// Create a generic network error
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other {
            message: message.into(),
        }
    }
}
