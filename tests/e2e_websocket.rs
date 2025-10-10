//! End-to-end integration tests for WebSocket proxy
//!
//! These tests verify the complete WebSocket record/replay cycle.

use matgto_serge::{
    cassette::{CloseFrame, Direction, MessagePayload, WebSocketMessage},
    WebSocketInterceptor, WebSocketPlayer, WebSocketRecorder,
};
use std::path::Path;
use tempfile::TempDir;

#[tokio::test]
async fn test_websocket_recorder_basic() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("matgto_serge=debug")
        .try_init();

    let cassette_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_name = "ws-record-test";

    tracing::info!("📹 Testing WebSocket recorder");

    // Create recorder
    let mut recorder = WebSocketRecorder::new(cassette_name.to_string());
    assert!(!recorder.is_recording());

    // Start session
    recorder.start_session("ws://example.com/socket".to_string());
    assert!(recorder.is_recording());

    // Record messages
    recorder.record_message(WebSocketMessage {
        direction: Direction::Sent,
        timestamp_ms: 1000,
        payload: MessagePayload::Text {
            data: "Hello WebSocket".to_string(),
        },
    });

    recorder.record_message(WebSocketMessage {
        direction: Direction::Received,
        timestamp_ms: 1100,
        payload: MessagePayload::Text {
            data: "Hello Client".to_string(),
        },
    });

    recorder.record_message(WebSocketMessage {
        direction: Direction::Sent,
        timestamp_ms: 1200,
        payload: MessagePayload::Binary {
            data: vec![0x01, 0x02, 0x03, 0x04],
        },
    });

    // End session
    recorder.end_session(None);
    assert!(!recorder.is_recording());
    assert_eq!(recorder.interaction_count(), 1);

    // Save cassette
    recorder
        .save(cassette_dir.path())
        .expect("Failed to save cassette");

    // Verify cassette file exists
    let cassette_path = cassette_dir.path().join(format!("{}.json", cassette_name));
    assert!(cassette_path.exists());

    tracing::info!("✅ WebSocket recorder test passed");
}

#[tokio::test]
async fn test_websocket_player_basic() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("matgto_serge=debug")
        .try_init();

    let cassette_dir = TempDir::new().unwrap();
    let cassette_name = "ws-player-test";

    tracing::info!("▶️  Testing WebSocket player");

    // First, record a session
    let mut recorder = WebSocketRecorder::new(cassette_name.to_string());
    recorder.start_session("ws://test.com/chat".to_string());

    recorder.record_message(WebSocketMessage {
        direction: Direction::Sent,
        timestamp_ms: 2000,
        payload: MessagePayload::Text {
            data: "Message 1".to_string(),
        },
    });

    recorder.record_message(WebSocketMessage {
        direction: Direction::Received,
        timestamp_ms: 2100,
        payload: MessagePayload::Text {
            data: "Response 1".to_string(),
        },
    });

    recorder.end_session(None);
    recorder.save(cassette_dir.path()).unwrap();

    // Now, replay the session
    let mut player = WebSocketPlayer::new();
    assert!(!player.has_cassette());

    player
        .load(cassette_dir.path(), cassette_name)
        .expect("Failed to load cassette");
    assert!(player.has_cassette());

    // Replay session
    let result = player.replay_session("ws://test.com/chat");
    assert!(result.is_ok());

    let (messages, close_frame) = result.unwrap();
    assert_eq!(messages.len(), 2);
    assert!(close_frame.is_none());
    assert_eq!(player.replay_count(), 1);

    // Verify message content
    if let MessagePayload::Text { data } = &messages[0].payload {
        assert_eq!(data, "Message 1");
    } else {
        panic!("Expected text message");
    }

    if let MessagePayload::Text { data } = &messages[1].payload {
        assert_eq!(data, "Response 1");
    } else {
        panic!("Expected text message");
    }

    tracing::info!("✅ WebSocket player test passed");
}

#[tokio::test]
async fn test_websocket_full_cycle() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("matgto_serge=debug")
        .try_init();

    let cassette_dir = TempDir::new().unwrap();
    let cassette_name = "ws-full-cycle";

    tracing::info!("🔄 Testing full WebSocket record/replay cycle");

    // ========== PHASE 1: RECORD ==========
    tracing::info!("📹 Phase 1: Recording");
    {
        let mut recorder = WebSocketRecorder::new(cassette_name.to_string());

        // Session 1: Chat messages
        recorder.start_session("ws://chat.example.com/room1".to_string());
        recorder.record_message(WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: 1000,
            payload: MessagePayload::Text {
                data: "{\"action\":\"join\",\"room\":\"room1\"}".to_string(),
            },
        });
        recorder.record_message(WebSocketMessage {
            direction: Direction::Received,
            timestamp_ms: 1050,
            payload: MessagePayload::Text {
                data: "{\"status\":\"joined\",\"room\":\"room1\"}".to_string(),
            },
        });
        recorder.record_message(WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: 2000,
            payload: MessagePayload::Text {
                data: "{\"action\":\"message\",\"text\":\"Hello everyone!\"}".to_string(),
            },
        });
        recorder.record_message(WebSocketMessage {
            direction: Direction::Received,
            timestamp_ms: 2100,
            payload: MessagePayload::Text {
                data: "{\"from\":\"user1\",\"text\":\"Hello!\"}".to_string(),
            },
        });
        recorder.end_session(Some(CloseFrame {
            code: 1000,
            reason: "Normal closure".to_string(),
        }));

        // Session 2: Binary data transfer
        recorder.start_session("ws://data.example.com/stream".to_string());
        recorder.record_message(WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: 3000,
            payload: MessagePayload::Binary {
                data: vec![0xFF, 0xFE, 0xFD, 0xFC],
            },
        });
        recorder.record_message(WebSocketMessage {
            direction: Direction::Received,
            timestamp_ms: 3100,
            payload: MessagePayload::Binary {
                data: vec![0x00, 0x01, 0x02, 0x03],
            },
        });
        recorder.end_session(None);

        // Session 3: Ping/Pong
        recorder.start_session("ws://heartbeat.example.com".to_string());
        recorder.record_message(WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: 4000,
            payload: MessagePayload::Ping { data: vec![] },
        });
        recorder.record_message(WebSocketMessage {
            direction: Direction::Received,
            timestamp_ms: 4050,
            payload: MessagePayload::Pong { data: vec![] },
        });
        recorder.end_session(None);

        assert_eq!(recorder.interaction_count(), 3);
        recorder.save(cassette_dir.path()).unwrap();

        tracing::info!("✅ Recorded 3 WebSocket sessions");
    }

    // ========== PHASE 2: REPLAY ==========
    tracing::info!("▶️  Phase 2: Replaying");
    {
        let mut player = WebSocketPlayer::new();
        player.load(cassette_dir.path(), cassette_name).unwrap();

        // Replay session 1
        let (messages, close_frame) = player
            .replay_session("ws://chat.example.com/room1")
            .unwrap();
        assert_eq!(messages.len(), 4);
        assert!(close_frame.is_some());
        assert_eq!(close_frame.unwrap().code, 1000);

        // Replay session 2
        let (messages, _) = player
            .replay_session("ws://data.example.com/stream")
            .unwrap();
        assert_eq!(messages.len(), 2);

        // Verify binary data
        if let MessagePayload::Binary { data } = &messages[0].payload {
            assert_eq!(data, &vec![0xFF, 0xFE, 0xFD, 0xFC]);
        }

        // Replay session 3
        let (messages, _) = player.replay_session("ws://heartbeat.example.com").unwrap();
        assert_eq!(messages.len(), 2);

        // Verify ping/pong
        assert!(matches!(messages[0].payload, MessagePayload::Ping { .. }));
        assert!(matches!(messages[1].payload, MessagePayload::Pong { .. }));

        assert_eq!(player.replay_count(), 3);

        tracing::info!("✅ Replayed all 3 sessions successfully");
    }

    // ========== PHASE 3: VERIFY CASSETTE ==========
    tracing::info!("📊 Phase 3: Verifying cassette structure");
    {
        let cassette_path = cassette_dir.path().join(format!("{}.json", cassette_name));
        let json = std::fs::read_to_string(&cassette_path).unwrap();

        tracing::info!("Cassette size: {} bytes", json.len());

        // Parse and verify
        let cassette: matgto_serge::Cassette = serde_json::from_str(&json).unwrap();
        assert_eq!(cassette.name, cassette_name);
        assert_eq!(cassette.version, "1.0");
        assert_eq!(cassette.interactions.len(), 3);

        tracing::info!("✅ Cassette structure valid");
    }

    tracing::info!("🎉 Full WebSocket cycle test completed!");
}

#[tokio::test]
async fn test_websocket_multiple_replays() {
    let cassette_dir = TempDir::new().unwrap();
    let cassette_name = "ws-multi-replay";

    // Record same URL multiple times
    let mut recorder = WebSocketRecorder::new(cassette_name.to_string());

    for i in 1..=3 {
        recorder.start_session("ws://api.example.com/events".to_string());
        recorder.record_message(WebSocketMessage {
            direction: Direction::Sent,
            timestamp_ms: (i * 1000) as u64,
            payload: MessagePayload::Text {
                data: format!("Request {}", i),
            },
        });
        recorder.record_message(WebSocketMessage {
            direction: Direction::Received,
            timestamp_ms: (i * 1000 + 100) as u64,
            payload: MessagePayload::Text {
                data: format!("Response {}", i),
            },
        });
        recorder.end_session(None);
    }

    recorder.save(cassette_dir.path()).unwrap();

    // Replay all sessions in order
    let mut player = WebSocketPlayer::new();
    player.load(cassette_dir.path(), cassette_name).unwrap();

    for i in 1..=3 {
        let (messages, _) = player
            .replay_session("ws://api.example.com/events")
            .unwrap();
        assert_eq!(messages.len(), 2);

        if let MessagePayload::Text { data } = &messages[0].payload {
            assert_eq!(data, &format!("Request {}", i));
        }

        if let MessagePayload::Text { data } = &messages[1].payload {
            assert_eq!(data, &format!("Response {}", i));
        }
    }

    assert_eq!(player.replay_count(), 3);

    // Fourth replay should fail
    let result = player.replay_session("ws://api.example.com/events");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_websocket_reset() {
    let cassette_dir = TempDir::new().unwrap();
    let cassette_name = "ws-reset-test";

    // Record session
    let mut recorder = WebSocketRecorder::new(cassette_name.to_string());
    recorder.start_session("ws://test.com".to_string());
    recorder.record_message(WebSocketMessage {
        direction: Direction::Sent,
        timestamp_ms: 1000,
        payload: MessagePayload::Text {
            data: "Test".to_string(),
        },
    });
    recorder.end_session(None);
    recorder.save(cassette_dir.path()).unwrap();

    // Load and replay
    let mut player = WebSocketPlayer::new();
    player.load(cassette_dir.path(), cassette_name).unwrap();

    // First replay
    let result = player.replay_session("ws://test.com");
    assert!(result.is_ok());
    assert_eq!(player.replay_count(), 1);

    // Reset
    player.reset();
    assert_eq!(player.replay_count(), 0);

    // Can replay again after reset
    let result = player.replay_session("ws://test.com");
    assert!(result.is_ok());
    assert_eq!(player.replay_count(), 1);
}

#[tokio::test]
#[ignore] // Requires real WebSocket server
async fn test_websocket_interceptor_live() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("matgto_serge=debug")
        .try_init();

    tracing::info!("🔌 Testing live WebSocket interceptor");

    // This would connect to a real WebSocket server
    // Example: wss://echo.websocket.org
    let interceptor = WebSocketInterceptor::new("wss://echo.websocket.org".to_string());

    // Start intercepting
    let result = interceptor.start().await;

    #[cfg(not(feature = "offline-tests"))]
    {
        if let Ok(()) = result {
            tracing::info!("✅ Connected to WebSocket server");
            assert!(interceptor.is_active().await);

            // Send test message
            let test_msg = WebSocketMessage {
                direction: Direction::Sent,
                timestamp_ms: chrono::Utc::now().timestamp_millis() as u64,
                payload: MessagePayload::Text {
                    data: "Hello WebSocket!".to_string(),
                },
            };

            interceptor.send(test_msg).await.ok();

            // Wait for response
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            let messages = interceptor.messages().await;
            tracing::info!("Captured {} messages", messages.len());

            interceptor.close().await.ok();
        } else {
            tracing::warn!("Network request failed (expected in offline environments)");
        }
    }
}
