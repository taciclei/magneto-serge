//! Tests for latency simulation during replay

use magneto_serge::cassette::{Cassette, HttpRequest, HttpResponse, InteractionKind};
use magneto_serge::player::{LatencyMode, Player};
use std::collections::HashMap;
use std::time::Instant;
use tempfile::tempdir;

/// Helper to create a test cassette with response times
fn create_cassette_with_timing(name: &str, response_time_ms: u64) -> Cassette {
    let mut cassette = Cassette::new(name.to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/test".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"test response".to_vec()),
    };

    cassette.add_interaction_with_timing(
        InteractionKind::Http { request, response },
        response_time_ms,
    );

    cassette
}

#[test]
fn test_latency_mode_none() {
    let cassette = create_cassette_with_timing("test-no-latency", 500);
    let interaction = &cassette.interactions[0];

    let player = Player::new();
    assert_eq!(player.latency_mode(), LatencyMode::None);

    let delay = player.calculate_delay(interaction);
    assert_eq!(delay, None);
}

#[test]
fn test_latency_mode_recorded() {
    let cassette = create_cassette_with_timing("test-recorded-latency", 500);
    let interaction = &cassette.interactions[0];

    let player = Player::new().with_latency(LatencyMode::Recorded);
    assert_eq!(player.latency_mode(), LatencyMode::Recorded);

    let delay = player.calculate_delay(interaction);
    assert_eq!(delay, Some(500));
}

#[test]
fn test_latency_mode_fixed() {
    let cassette = create_cassette_with_timing("test-fixed-latency", 500);
    let interaction = &cassette.interactions[0];

    let player = Player::new().with_latency(LatencyMode::Fixed(100));
    assert_eq!(player.latency_mode(), LatencyMode::Fixed(100));

    let delay = player.calculate_delay(interaction);
    assert_eq!(delay, Some(100)); // Fixed 100ms regardless of recorded time
}

#[test]
fn test_latency_mode_scaled() {
    let cassette = create_cassette_with_timing("test-scaled-latency", 500);
    let interaction = &cassette.interactions[0];

    // 50% speed (half the recorded time)
    let player = Player::new().with_latency(LatencyMode::Scaled(50));
    let delay = player.calculate_delay(interaction);
    assert_eq!(delay, Some(250)); // 500ms * 50% = 250ms

    // 200% speed (double the recorded time)
    let player = Player::new().with_latency(LatencyMode::Scaled(200));
    let delay = player.calculate_delay(interaction);
    assert_eq!(delay, Some(1000)); // 500ms * 200% = 1000ms
}

#[test]
fn test_latency_mode_scaled_without_recorded_time() {
    // Create interaction without response time
    let mut cassette = Cassette::new("test-no-time".to_string());
    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/test".to_string(),
        headers: HashMap::new(),
        body: None,
    };
    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"test".to_vec()),
    };
    cassette.add_interaction(InteractionKind::Http { request, response });

    let interaction = &cassette.interactions[0];

    let player = Player::new().with_latency(LatencyMode::Scaled(50));
    let delay = player.calculate_delay(interaction);
    assert_eq!(delay, None); // No recorded time, so no delay
}

#[test]
fn test_interaction_with_timing() {
    let cassette = create_cassette_with_timing("test-with-timing", 123);

    assert_eq!(cassette.interactions.len(), 1);
    assert_eq!(cassette.interactions[0].response_time_ms, Some(123));
}

#[test]
fn test_interaction_without_timing() {
    let mut cassette = Cassette::new("test-without-timing".to_string());
    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/test".to_string(),
        headers: HashMap::new(),
        body: None,
    };
    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"test".to_vec()),
    };
    cassette.add_interaction(InteractionKind::Http { request, response });

    assert_eq!(cassette.interactions.len(), 1);
    assert_eq!(cassette.interactions[0].response_time_ms, None);
}

#[test]
fn test_latency_simulation_realistic_timing() {
    // Test that actual delays match expected latency
    let cassette = create_cassette_with_timing("test-timing", 100);
    let interaction = &cassette.interactions[0];

    let player = Player::new().with_latency(LatencyMode::Recorded);
    let delay_ms = player.calculate_delay(interaction).unwrap();

    assert_eq!(delay_ms, 100);

    // Simulate applying the delay
    let start = Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
    let elapsed = start.elapsed().as_millis() as u64;

    // Allow some tolerance for timing (±20ms)
    assert!(
        (90..=120).contains(&elapsed),
        "Expected delay ~100ms, got {}ms",
        elapsed
    );
}

#[test]
fn test_load_player_with_latency() {
    let cassette = create_cassette_with_timing("test-load-latency", 200);

    let dir = tempdir().unwrap();
    let path = dir.path().join("test-load-latency.json");
    let file = std::fs::File::create(&path).unwrap();
    serde_json::to_writer_pretty(file, &cassette).unwrap();

    // Load player and set latency mode
    let player = Player::load(dir.path(), "test-load-latency")
        .unwrap()
        .with_latency(LatencyMode::Recorded);

    assert!(player.has_cassette());
    assert_eq!(player.latency_mode(), LatencyMode::Recorded);

    // Verify we can calculate delay from loaded interaction
    let interaction = player.get_interaction(0).unwrap();
    let delay = player.calculate_delay(interaction);
    assert_eq!(delay, Some(200));
}

#[test]
fn test_multiple_latency_modes() {
    let cassette = create_cassette_with_timing("test-multi-mode", 400);
    let interaction = &cassette.interactions[0];

    // Test None
    let player = Player::new();
    assert_eq!(player.calculate_delay(interaction), None);

    // Test Recorded
    let player = player.with_latency(LatencyMode::Recorded);
    assert_eq!(player.calculate_delay(interaction), Some(400));

    // Test Fixed
    let player = player.with_latency(LatencyMode::Fixed(50));
    assert_eq!(player.calculate_delay(interaction), Some(50));

    // Test Scaled
    let player = player.with_latency(LatencyMode::Scaled(150)); // 150% = 1.5x
    assert_eq!(player.calculate_delay(interaction), Some(600)); // 400ms * 1.5 = 600ms
}
