//! WebSocket Player
//!
//! Replays WebSocket interactions from cassettes.

use crate::cassette::{Cassette, CloseFrame, InteractionKind, WebSocketMessage};
use crate::error::{MatgtoError, Result};
use crate::player::LatencyMode;
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, info};

/// Replays WebSocket interactions from cassettes
pub struct WebSocketPlayer {
    /// Loaded cassette
    cassette: Option<Cassette>,
    /// Index of WebSocket interactions by URL
    ws_index: HashMap<String, Vec<usize>>,
    /// Current replay position for each URL
    replay_positions: HashMap<String, usize>,
    /// Total number of replays
    replay_count: usize,
    /// Latency simulation mode
    latency_mode: LatencyMode,
}

impl WebSocketPlayer {
    /// Create a new WebSocket player
    pub fn new() -> Self {
        Self {
            cassette: None,
            ws_index: HashMap::new(),
            replay_positions: HashMap::new(),
            replay_count: 0,
            latency_mode: LatencyMode::None,
        }
    }

    /// Set latency simulation mode
    pub fn with_latency(mut self, mode: LatencyMode) -> Self {
        self.latency_mode = mode;
        self
    }

    /// Get current latency mode
    pub fn latency_mode(&self) -> LatencyMode {
        self.latency_mode
    }

    /// Calculate delay for a message based on latency mode and timestamp
    ///
    /// For WebSocket messages, we use relative timestamps (timestamp_ms) to calculate delays.
    /// - LatencyMode::None: No delay, messages sent instantly
    /// - LatencyMode::Recorded: Use recorded timestamp_ms as delay
    /// - LatencyMode::Fixed(ms): Fixed delay for all messages
    /// - LatencyMode::Scaled(percentage): Scale recorded timestamps
    pub fn calculate_message_delay(&self, timestamp_ms: u64, base_timestamp: u64) -> Option<u64> {
        match self.latency_mode {
            LatencyMode::None => None,
            LatencyMode::Recorded => {
                // Use relative timestamp (current - base)
                Some(timestamp_ms.saturating_sub(base_timestamp))
            }
            LatencyMode::Fixed(ms) => Some(ms),
            LatencyMode::Scaled(percentage) => {
                // Scale the relative timestamp
                let relative = timestamp_ms.saturating_sub(base_timestamp);
                Some((relative * percentage) / 100)
            }
        }
    }

    /// Load a cassette from disk
    pub fn load(&mut self, cassette_dir: &Path, cassette_name: &str) -> Result<()> {
        info!("📼 Loading WebSocket cassette: {}", cassette_name);

        let cassette_path = cassette_dir.join(format!("{}.json", cassette_name));

        if !cassette_path.exists() {
            return Err(MatgtoError::CassetteNotFound {
                name: cassette_name.to_string(),
            });
        }

        let json = std::fs::read_to_string(&cassette_path).map_err(|e| {
            MatgtoError::CassetteLoadFailed {
                reason: format!("Failed to read cassette file: {}", e),
            }
        })?;

        let cassette: Cassette =
            serde_json::from_str(&json).map_err(|e| MatgtoError::CassetteLoadFailed {
                reason: format!("Failed to parse cassette JSON: {}", e),
            })?;

        info!(
            "✅ Cassette loaded: {} ({} interactions)",
            cassette_name,
            cassette.interactions.len()
        );

        // Build WebSocket index
        self.build_ws_index(&cassette);

        self.cassette = Some(cassette);
        Ok(())
    }

    /// Build index of WebSocket interactions by URL
    fn build_ws_index(&mut self, cassette: &Cassette) {
        self.ws_index.clear();
        self.replay_positions.clear();

        for (idx, interaction) in cassette.interactions.iter().enumerate() {
            if let InteractionKind::WebSocket { url, .. } = &interaction.kind {
                self.ws_index.entry(url.clone()).or_default().push(idx);

                // Initialize replay position
                self.replay_positions.entry(url.clone()).or_insert(0);

                debug!("📇 Indexed WebSocket interaction #{}: {}", idx, url);
            }
        }

        info!(
            "📇 Built WebSocket index: {} unique URLs",
            self.ws_index.len()
        );
    }

    /// Find and replay messages for a WebSocket URL
    pub fn replay_session(
        &mut self,
        url: &str,
    ) -> Result<(Vec<WebSocketMessage>, Option<CloseFrame>)> {
        if self.cassette.is_none() {
            return Err(MatgtoError::WebSocketError {
                reason: "No cassette loaded".to_string(),
            });
        }

        // Get interaction indices for this URL
        let indices = self
            .ws_index
            .get(url)
            .ok_or_else(|| MatgtoError::WebSocketError {
                reason: format!("No WebSocket session found for URL: {}", url),
            })?;

        // Get current replay position for this URL
        let position = self.replay_positions.get(url).copied().unwrap_or(0);

        if position >= indices.len() {
            return Err(MatgtoError::WebSocketError {
                reason: format!(
                    "All sessions for {} have been replayed ({} times)",
                    url, position
                ),
            });
        }

        // Get interaction
        let interaction_idx = indices[position];
        let cassette = self.cassette.as_ref().unwrap();
        let interaction = &cassette.interactions[interaction_idx];

        // Extract WebSocket data
        if let InteractionKind::WebSocket {
            messages,
            close_frame,
            ..
        } = &interaction.kind
        {
            // Increment replay position
            *self.replay_positions.entry(url.to_string()).or_insert(0) += 1;
            self.replay_count += 1;

            info!(
                "▶️  Replaying WebSocket session #{} for {}: {} messages",
                position + 1,
                url,
                messages.len()
            );

            Ok((messages.clone(), close_frame.clone()))
        } else {
            Err(MatgtoError::WebSocketError {
                reason: format!(
                    "Interaction #{} is not a WebSocket session",
                    interaction_idx
                ),
            })
        }
    }

    /// Get next message for a URL without consuming
    pub fn peek_next_message(&self, url: &str) -> Result<Option<WebSocketMessage>> {
        if self.cassette.is_none() {
            return Ok(None);
        }

        let indices = match self.ws_index.get(url) {
            Some(idx) => idx,
            None => return Ok(None),
        };

        let position = self.replay_positions.get(url).copied().unwrap_or(0);
        if position >= indices.len() {
            return Ok(None);
        }

        let interaction_idx = indices[position];
        let cassette = self.cassette.as_ref().unwrap();
        let interaction = &cassette.interactions[interaction_idx];

        if let InteractionKind::WebSocket { messages, .. } = &interaction.kind {
            Ok(messages.first().cloned())
        } else {
            Ok(None)
        }
    }

    /// Check if cassette is loaded
    pub fn has_cassette(&self) -> bool {
        self.cassette.is_some()
    }

    /// Get total replay count
    pub fn replay_count(&self) -> usize {
        self.replay_count
    }

    /// Get cassette reference
    pub fn cassette(&self) -> Option<&Cassette> {
        self.cassette.as_ref()
    }

    /// Reset replay positions
    pub fn reset(&mut self) {
        self.replay_positions.clear();
        self.replay_count = 0;
        info!("🔄 Reset replay positions");
    }
}

impl Default for WebSocketPlayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cassette::{Direction, Interaction, MessagePayload};
    use chrono::Utc;
    use tempfile::TempDir;

    fn create_test_cassette(name: &str) -> Cassette {
        Cassette {
            version: "1.0".to_string(),
            name: name.to_string(),
            recorded_at: Utc::now(),
            interactions: vec![
                Interaction {
                    recorded_at: Utc::now(),
                    response_time_ms: None,
                    kind: InteractionKind::WebSocket {
                        url: "ws://example.com/socket".to_string(),
                        messages: vec![
                            WebSocketMessage {
                                direction: Direction::Sent,
                                timestamp_ms: 1000,
                                payload: MessagePayload::Text {
                                    data: "Hello".to_string(),
                                },
                            },
                            WebSocketMessage {
                                direction: Direction::Received,
                                timestamp_ms: 1100,
                                payload: MessagePayload::Text {
                                    data: "World".to_string(),
                                },
                            },
                        ],
                        close_frame: None,
                    },
                },
                Interaction {
                    recorded_at: Utc::now(),
                    response_time_ms: None,
                    kind: InteractionKind::WebSocket {
                        url: "ws://example.com/socket".to_string(),
                        messages: vec![WebSocketMessage {
                            direction: Direction::Sent,
                            timestamp_ms: 2000,
                            payload: MessagePayload::Text {
                                data: "Second session".to_string(),
                            },
                        }],
                        close_frame: None,
                    },
                },
            ],
        }
    }

    #[test]
    fn test_player_creation() {
        let player = WebSocketPlayer::new();
        assert!(!player.has_cassette());
        assert_eq!(player.replay_count(), 0);
    }

    #[test]
    fn test_load_cassette() {
        let temp_dir = TempDir::new().unwrap();
        let cassette = create_test_cassette("test-load");

        // Save cassette
        let cassette_path = temp_dir.path().join("test-load.json");
        let json = serde_json::to_string_pretty(&cassette).unwrap();
        std::fs::write(&cassette_path, json).unwrap();

        // Load cassette
        let mut player = WebSocketPlayer::new();
        let result = player.load(temp_dir.path(), "test-load");

        assert!(result.is_ok());
        assert!(player.has_cassette());
    }

    #[test]
    fn test_replay_session() {
        let temp_dir = TempDir::new().unwrap();
        let cassette = create_test_cassette("test-replay");

        // Save cassette
        let cassette_path = temp_dir.path().join("test-replay.json");
        let json = serde_json::to_string_pretty(&cassette).unwrap();
        std::fs::write(&cassette_path, json).unwrap();

        // Load and replay
        let mut player = WebSocketPlayer::new();
        player.load(temp_dir.path(), "test-replay").unwrap();

        // First replay
        let result = player.replay_session("ws://example.com/socket");
        assert!(result.is_ok());

        let (messages, close_frame) = result.unwrap();
        assert_eq!(messages.len(), 2);
        assert!(close_frame.is_none());
        assert_eq!(player.replay_count(), 1);

        // Second replay (different session)
        let result = player.replay_session("ws://example.com/socket");
        assert!(result.is_ok());

        let (messages, _) = result.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(player.replay_count(), 2);

        // Third replay should fail (no more sessions)
        let result = player.replay_session("ws://example.com/socket");
        assert!(result.is_err());
    }

    #[test]
    fn test_peek_next_message() {
        let temp_dir = TempDir::new().unwrap();
        let cassette = create_test_cassette("test-peek");

        let cassette_path = temp_dir.path().join("test-peek.json");
        let json = serde_json::to_string_pretty(&cassette).unwrap();
        std::fs::write(&cassette_path, json).unwrap();

        let mut player = WebSocketPlayer::new();
        player.load(temp_dir.path(), "test-peek").unwrap();

        // Peek should return first message
        let result = player.peek_next_message("ws://example.com/socket");
        assert!(result.is_ok());

        let msg = result.unwrap();
        assert!(msg.is_some());

        // Replay count should not change
        assert_eq!(player.replay_count(), 0);
    }

    #[test]
    fn test_reset() {
        let temp_dir = TempDir::new().unwrap();
        let cassette = create_test_cassette("test-reset");

        let cassette_path = temp_dir.path().join("test-reset.json");
        let json = serde_json::to_string_pretty(&cassette).unwrap();
        std::fs::write(&cassette_path, json).unwrap();

        let mut player = WebSocketPlayer::new();
        player.load(temp_dir.path(), "test-reset").unwrap();

        // Replay once
        player.replay_session("ws://example.com/socket").unwrap();
        assert_eq!(player.replay_count(), 1);

        // Reset
        player.reset();
        assert_eq!(player.replay_count(), 0);

        // Can replay again
        let result = player.replay_session("ws://example.com/socket");
        assert!(result.is_ok());
    }

    #[test]
    fn test_latency_mode_none() {
        let player = WebSocketPlayer::new();
        assert_eq!(player.latency_mode(), LatencyMode::None);

        // With None mode, no delay should be returned
        let delay = player.calculate_message_delay(5000, 1000);
        assert_eq!(delay, None);
    }

    #[test]
    fn test_latency_mode_recorded() {
        let player = WebSocketPlayer::new().with_latency(LatencyMode::Recorded);
        assert_eq!(player.latency_mode(), LatencyMode::Recorded);

        // Recorded mode uses relative timestamps
        let delay = player.calculate_message_delay(5000, 1000);
        assert_eq!(delay, Some(4000)); // 5000 - 1000 = 4000ms

        // First message (base timestamp)
        let delay = player.calculate_message_delay(1000, 1000);
        assert_eq!(delay, Some(0)); // No delay for first message
    }

    #[test]
    fn test_latency_mode_fixed() {
        let player = WebSocketPlayer::new().with_latency(LatencyMode::Fixed(100));
        assert_eq!(player.latency_mode(), LatencyMode::Fixed(100));

        // Fixed mode always returns same delay
        let delay1 = player.calculate_message_delay(1000, 1000);
        assert_eq!(delay1, Some(100));

        let delay2 = player.calculate_message_delay(5000, 1000);
        assert_eq!(delay2, Some(100));
    }

    #[test]
    fn test_latency_mode_scaled() {
        let player = WebSocketPlayer::new().with_latency(LatencyMode::Scaled(50));
        assert_eq!(player.latency_mode(), LatencyMode::Scaled(50));

        // Scaled mode scales relative timestamps
        // 50% = half speed (2x faster)
        let delay = player.calculate_message_delay(5000, 1000);
        assert_eq!(delay, Some(2000)); // (5000 - 1000) * 50 / 100 = 2000ms

        // 200% = double speed (2x slower)
        let player_slow = WebSocketPlayer::new().with_latency(LatencyMode::Scaled(200));
        let delay_slow = player_slow.calculate_message_delay(5000, 1000);
        assert_eq!(delay_slow, Some(8000)); // (5000 - 1000) * 200 / 100 = 8000ms
    }

    #[test]
    fn test_latency_mode_with_load() {
        let temp_dir = TempDir::new().unwrap();
        let cassette = create_test_cassette("test-latency-load");

        let cassette_path = temp_dir.path().join("test-latency-load.json");
        let json = serde_json::to_string_pretty(&cassette).unwrap();
        std::fs::write(&cassette_path, json).unwrap();

        // Create player with latency mode
        let mut player = WebSocketPlayer::new().with_latency(LatencyMode::Recorded);

        // Load cassette
        player.load(temp_dir.path(), "test-latency-load").unwrap();

        // Latency mode should be preserved after load
        assert_eq!(player.latency_mode(), LatencyMode::Recorded);
    }

    #[test]
    fn test_instant_mode_for_blockchain() {
        // Simulate blockchain test case from issue #5
        // Blocks arrive every 6000ms, but we want instant replay
        let player = WebSocketPlayer::new().with_latency(LatencyMode::None);

        // Message 1 at 0ms (connection start)
        let delay1 = player.calculate_message_delay(0, 0);
        assert_eq!(delay1, None); // Instant

        // Message 2 at 6000ms (first block)
        let delay2 = player.calculate_message_delay(6000, 0);
        assert_eq!(delay2, None); // Instant, no 6 second wait

        // Message 3 at 12000ms (second block)
        let delay3 = player.calculate_message_delay(12000, 0);
        assert_eq!(delay3, None); // Instant, no 12 second wait
    }
}
