//! Cassette format definitions and types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        });
    }
}
