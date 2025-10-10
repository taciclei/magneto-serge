//! WebSocket Recorder
//!
//! Records WebSocket messages into cassettes for later replay.

use crate::cassette::{Cassette, CloseFrame, Interaction, InteractionKind, WebSocketMessage};
use crate::error::{MatgtoError, Result};
use chrono::Utc;
use std::path::Path;
use tracing::{debug, info};

/// Records WebSocket interactions to cassettes
pub struct WebSocketRecorder {
    /// Name of the cassette being recorded
    cassette_name: String,
    /// Cassette being recorded
    cassette: Cassette,
    /// Current WebSocket session URL
    current_url: Option<String>,
    /// Messages in current session
    current_messages: Vec<WebSocketMessage>,
}

impl WebSocketRecorder {
    /// Create a new WebSocket recorder
    pub fn new(cassette_name: String) -> Self {
        Self {
            cassette_name: cassette_name.clone(),
            cassette: Cassette {
                version: "1.0".to_string(),
                name: cassette_name,
                recorded_at: Utc::now(),
                interactions: Vec::new(),
            },
            current_url: None,
            current_messages: Vec::new(),
        }
    }

    /// Start recording a WebSocket session
    pub fn start_session(&mut self, url: String) {
        info!("📹 Starting WebSocket recording session: {}", url);
        self.current_url = Some(url);
        self.current_messages.clear();
    }

    /// Record a WebSocket message
    pub fn record_message(&mut self, message: WebSocketMessage) {
        debug!("📝 Recording WebSocket message: {:?}", message);
        self.current_messages.push(message);
    }

    /// End the current WebSocket session
    pub fn end_session(&mut self, close_frame: Option<CloseFrame>) {
        if let Some(url) = self.current_url.take() {
            info!(
                "✅ Ending WebSocket recording session: {} ({} messages)",
                url,
                self.current_messages.len()
            );

            // Create interaction
            let interaction = Interaction {
                recorded_at: Utc::now(),
                kind: InteractionKind::WebSocket {
                    url,
                    messages: self.current_messages.drain(..).collect(),
                    close_frame,
                },
            };

            self.cassette.interactions.push(interaction);
        }
    }

    /// Save the cassette to disk
    pub fn save(&self, cassette_dir: &Path) -> Result<()> {
        info!(
            "💾 Saving WebSocket cassette: {} ({} interactions)",
            self.cassette_name,
            self.cassette.interactions.len()
        );

        // Ensure directory exists
        std::fs::create_dir_all(cassette_dir).map_err(|e| MatgtoError::CassetteLoadFailed {
            reason: format!("Failed to create cassette directory: {}", e),
        })?;

        // Save cassette
        let cassette_path = cassette_dir.join(format!("{}.json", self.cassette_name));
        let json = serde_json::to_string_pretty(&self.cassette).map_err(|e| {
            MatgtoError::CassetteLoadFailed {
                reason: format!("Failed to serialize cassette: {}", e),
            }
        })?;

        std::fs::write(&cassette_path, json).map_err(|e| MatgtoError::CassetteLoadFailed {
            reason: format!("Failed to write cassette file: {}", e),
        })?;

        info!("✅ Cassette saved: {:?}", cassette_path);
        Ok(())
    }

    /// Get the current cassette
    pub fn cassette(&self) -> &Cassette {
        &self.cassette
    }

    /// Get number of recorded interactions
    pub fn interaction_count(&self) -> usize {
        self.cassette.interactions.len()
    }

    /// Check if currently recording a session
    pub fn is_recording(&self) -> bool {
        self.current_url.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cassette::{Direction, MessagePayload};
    use tempfile::TempDir;

    #[test]
    fn test_recorder_creation() {
        let recorder = WebSocketRecorder::new("test-cassette".to_string());
        assert_eq!(recorder.cassette_name, "test-cassette");
        assert_eq!(recorder.interaction_count(), 0);
        assert!(!recorder.is_recording());
    }

    #[test]
    fn test_record_websocket_session() {
        let mut recorder = WebSocketRecorder::new("ws-session".to_string());

        // Start session
        recorder.start_session("ws://example.com/socket".to_string());
        assert!(recorder.is_recording());

        // Record messages
        recorder.record_message(WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: 1000,
            payload: MessagePayload::Text {
                data: "Hello".to_string(),
            },
        });

        recorder.record_message(WebSocketMessage {
            direction: Direction::Received,
            timestamp_ms: 1100,
            payload: MessagePayload::Text {
                data: "World".to_string(),
            },
        });

        // End session
        recorder.end_session(None);
        assert!(!recorder.is_recording());
        assert_eq!(recorder.interaction_count(), 1);

        // Verify interaction
        let cassette = recorder.cassette();
        assert_eq!(cassette.interactions.len(), 1);

        if let InteractionKind::WebSocket { url, messages, .. } = &cassette.interactions[0].kind {
            assert_eq!(url, "ws://example.com/socket");
            assert_eq!(messages.len(), 2);
        } else {
            panic!("Expected WebSocket interaction");
        }
    }

    #[test]
    fn test_save_cassette() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut recorder = WebSocketRecorder::new("save-test".to_string());

        // Record a session
        recorder.start_session("ws://test.com".to_string());
        recorder.record_message(WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: 2000,
            payload: MessagePayload::Text {
                data: "Test".to_string(),
            },
        });
        recorder.end_session(None);

        // Save
        let result = recorder.save(temp_dir.path());
        assert!(result.is_ok());

        // Verify file exists
        let cassette_path = temp_dir.path().join("save-test.json");
        assert!(cassette_path.exists());

        // Verify content
        let content = std::fs::read_to_string(&cassette_path).unwrap();
        assert!(content.contains("ws://test.com"));
        assert!(content.contains("Test"));
    }

    #[test]
    fn test_multiple_sessions() {
        let mut recorder = WebSocketRecorder::new("multi-session".to_string());

        // Session 1
        recorder.start_session("ws://example.com/1".to_string());
        recorder.record_message(WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: 1000,
            payload: MessagePayload::Text {
                data: "Message 1".to_string(),
            },
        });
        recorder.end_session(None);

        // Session 2
        recorder.start_session("ws://example.com/2".to_string());
        recorder.record_message(WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: 2000,
            payload: MessagePayload::Text {
                data: "Message 2".to_string(),
            },
        });
        recorder.end_session(None);

        // Verify both sessions recorded
        assert_eq!(recorder.interaction_count(), 2);
    }
}
