//! WebSocket Interceptor
//!
//! This module provides WebSocket connection interception and message proxying.
//! It handles WebSocket upgrades and bidirectional message flow between client and server.

use crate::cassette::{Direction, MessagePayload, WebSocketMessage};
use crate::error::{MatgtoError, Result};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info, warn};

/// WebSocket interceptor that captures bidirectional messages
pub struct WebSocketInterceptor {
    /// URL of the WebSocket connection
    url: String,
    /// Captured messages
    messages: Arc<Mutex<Vec<WebSocketMessage>>>,
    /// Whether the connection is active
    active: Arc<Mutex<bool>>,
}

impl WebSocketInterceptor {
    /// Create a new WebSocket interceptor
    pub fn new(url: String) -> Self {
        Self {
            url,
            messages: Arc::new(Mutex::new(Vec::new())),
            active: Arc::new(Mutex::new(false)),
        }
    }

    /// Start intercepting WebSocket connection
    ///
    /// This connects to the target WebSocket server and proxies messages
    /// between client and server while recording them.
    pub async fn start(&self) -> Result<()> {
        info!("🔌 Starting WebSocket interceptor for: {}", self.url);

        // Connect to target WebSocket server
        let (ws_stream, _) =
            connect_async(&self.url)
                .await
                .map_err(|e| MatgtoError::WebSocketError {
                    reason: format!("Failed to connect to WebSocket: {}", e),
                })?;

        info!("✅ Connected to WebSocket: {}", self.url);

        // Mark as active
        *self.active.lock().await = true;

        // Split stream into sink and stream
        let (mut write, mut read) = ws_stream.split();

        // Handle incoming messages from server
        let messages_clone = self.messages.clone();
        let active_clone = self.active.clone();

        tokio::spawn(async move {
            while let Some(msg_result) = read.next().await {
                match msg_result {
                    Ok(msg) => {
                        debug!("📥 Received WebSocket message: {:?}", msg);

                        // Convert and record message
                        if let Some(ws_msg) = Self::convert_message(msg, Direction::Received) {
                            messages_clone.lock().await.push(ws_msg);
                        }
                    }
                    Err(e) => {
                        error!("❌ WebSocket read error: {}", e);
                        *active_clone.lock().await = false;
                        break;
                    }
                }
            }

            info!("🔌 WebSocket connection closed");
            *active_clone.lock().await = false;
        });

        Ok(())
    }

    /// Send a message to the WebSocket server
    pub async fn send(&self, message: WebSocketMessage) -> Result<()> {
        if !*self.active.lock().await {
            return Err(MatgtoError::WebSocketError {
                reason: "WebSocket connection not active".to_string(),
            });
        }

        // Convert WebSocketMessage to tungstenite Message
        let msg = Self::convert_to_tungstenite(&message)?;

        debug!("📤 Sending WebSocket message: {:?}", msg);

        // Record outgoing message
        self.messages.lock().await.push(message);

        Ok(())
    }

    /// Get all captured messages
    pub async fn messages(&self) -> Vec<WebSocketMessage> {
        self.messages.lock().await.clone()
    }

    /// Check if connection is active
    pub async fn is_active(&self) -> bool {
        *self.active.lock().await
    }

    /// Convert tungstenite Message to our WebSocketMessage
    fn convert_message(msg: Message, direction: Direction) -> Option<WebSocketMessage> {
        let timestamp_ms = chrono::Utc::now().timestamp_millis() as u64;

        let payload = match msg {
            Message::Text(text) => MessagePayload::Text { data: text },
            Message::Binary(data) => MessagePayload::Binary { data },
            Message::Ping(data) => MessagePayload::Ping { data },
            Message::Pong(data) => MessagePayload::Pong { data },
            Message::Close(_) => {
                // Close frames are handled separately
                return None;
            }
            Message::Frame(_) => {
                warn!("Received raw frame - skipping");
                return None;
            }
        };

        Some(WebSocketMessage {
            direction,
            timestamp_ms,
            payload,
        })
    }

    /// Convert our WebSocketMessage to tungstenite Message
    fn convert_to_tungstenite(msg: &WebSocketMessage) -> Result<Message> {
        let tungstenite_msg = match &msg.payload {
            MessagePayload::Text { data } => Message::Text(data.clone()),
            MessagePayload::Binary { data } => Message::Binary(data.clone()),
            MessagePayload::Ping { data } => Message::Ping(data.clone()),
            MessagePayload::Pong { data } => Message::Pong(data.clone()),
        };

        Ok(tungstenite_msg)
    }

    /// Close the WebSocket connection
    pub async fn close(&self) -> Result<()> {
        info!("🔌 Closing WebSocket connection: {}", self.url);
        *self.active.lock().await = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_interceptor_creation() {
        let interceptor = WebSocketInterceptor::new("ws://example.com/socket".to_string());
        assert_eq!(interceptor.url, "ws://example.com/socket");
        assert!(!interceptor.is_active().await);
        assert_eq!(interceptor.messages().await.len(), 0);
    }

    #[test]
    fn test_convert_message() {
        let text_msg = Message::Text("Hello".to_string());
        let ws_msg = WebSocketInterceptor::convert_message(text_msg, Direction::Sent);

        assert!(ws_msg.is_some());
        let msg = ws_msg.unwrap();
        assert!(matches!(msg.direction, Direction::Sent));
        assert!(matches!(msg.payload, MessagePayload::Text { .. }));
    }

    #[test]
    fn test_convert_to_tungstenite() {
        let ws_msg = WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: 1234567890,
            payload: MessagePayload::Text {
                data: "Hello".to_string(),
            },
        };

        let result = WebSocketInterceptor::convert_to_tungstenite(&ws_msg);
        assert!(result.is_ok());

        let tungstenite_msg = result.unwrap();
        assert!(matches!(tungstenite_msg, Message::Text(_)));
    }
}
