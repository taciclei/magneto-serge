//! Integration tests for STRICT replay mode

use magneto_serge::cassette::{HttpRequest, HttpResponse};
use magneto_serge::player::Player;
use magneto_serge::proxy::{MagnetoProxy, ProxyMode};
use magneto_serge::recorder::Recorder;
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_strict_mode_load() {
    // Create a cassette with one interaction
    let mut recorder = Recorder::new("strict-test".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/users".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"users\":[]}".to_vec()),
    };

    recorder.record_http(request, response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load in strict mode
    let player = Player::load_strict(dir.path(), "strict-test").unwrap();

    assert!(player.is_strict());
    assert!(player.has_cassette());
}

#[test]
fn test_strict_mode_error_on_missing_cassette() {
    let dir = tempdir().unwrap();

    // Try to load non-existent cassette in strict mode
    let result = Player::load_strict(dir.path(), "nonexistent");

    assert!(result.is_err());
}

#[test]
fn test_strict_mode_error_on_missing_interaction() {
    // Create a cassette with one interaction
    let mut recorder = Recorder::new("strict-missing".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/users".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"users\":[]}".to_vec()),
    };

    recorder.record_http(request, response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load in strict mode
    let mut player = Player::load_strict(dir.path(), "strict-missing").unwrap();

    // Try to find an interaction that doesn't exist
    let missing_signature = magneto_serge::player::RequestSignature {
        method: "POST".to_string(),
        url: "https://api.example.com/posts".to_string(),
        body_hash: None,
    };

    let result = player.find_interaction(&missing_signature);
    assert!(result.is_err());
}

#[test]
fn test_proxy_strict_mode_creation() {
    let dir = tempdir().unwrap();

    // Create a cassette first
    let mut recorder = Recorder::new("proxy-strict".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/test".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{}".to_vec()),
    };

    recorder.record_http(request, response);
    recorder.save(dir.path()).unwrap();

    // Create proxy and try to replay in strict mode
    let proxy = MagnetoProxy::new_internal(dir.path()).unwrap();

    let result = proxy.replay_strict_internal("proxy-strict".to_string());
    assert!(result.is_ok());

    assert_eq!(proxy.mode(), ProxyMode::Auto); // Mode doesn't change, just the operation
}

#[test]
fn test_strict_mode_vs_normal_mode() {
    // Create a cassette
    let mut recorder = Recorder::new("mode-comparison".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/data".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{}".to_vec()),
    };

    recorder.record_http(request, response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load in normal mode
    let player_normal = Player::load(dir.path(), "mode-comparison").unwrap();
    assert!(!player_normal.is_strict());

    // Load in strict mode
    let player_strict = Player::load_strict(dir.path(), "mode-comparison").unwrap();
    assert!(player_strict.is_strict());

    // Both should have the same cassette content
    assert_eq!(
        player_normal.cassette().unwrap().interactions.len(),
        player_strict.cassette().unwrap().interactions.len()
    );
}

#[test]
fn test_strict_mode_multiple_interactions() {
    // Create a cassette with multiple interactions
    let mut recorder = Recorder::new("strict-multi".to_string());

    for i in 0..5 {
        let request = HttpRequest {
            method: "GET".to_string(),
            url: format!("https://api.example.com/resource/{}", i),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(format!("{{\"id\":{}}}", i).into_bytes()),
        };

        recorder.record_http(request, response);
    }

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load in strict mode
    let mut player = Player::load_strict(dir.path(), "strict-multi").unwrap();

    // All 5 interactions should be findable
    for i in 0..5 {
        let signature = magneto_serge::player::RequestSignature {
            method: "GET".to_string(),
            url: format!("https://api.example.com/resource/{}", i),
            body_hash: None,
        };

        let result = player.find_interaction(&signature);
        assert!(result.is_ok());
    }

    // 6th interaction should not exist
    let missing_signature = magneto_serge::player::RequestSignature {
        method: "GET".to_string(),
        url: "https://api.example.com/resource/5".to_string(),
        body_hash: None,
    };

    let result = player.find_interaction(&missing_signature);
    assert!(result.is_err());
}

#[test]
fn test_strict_mode_with_body_hash() {
    // Create a cassette with POST request (has body hash)
    let mut recorder = Recorder::new("strict-body".to_string());

    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/create".to_string(),
        headers: HashMap::new(),
        body: Some(b"{\"name\":\"test\"}".to_vec()),
    };

    let response = HttpResponse {
        status: 201,
        headers: HashMap::new(),
        body: Some(b"{\"id\":1}".to_vec()),
    };

    recorder.record_http(request.clone(), response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load in strict mode
    let mut player = Player::load_strict(dir.path(), "strict-body").unwrap();

    // Find with correct body
    let signature_correct = magneto_serge::player::RequestSignature::from(request);
    let result = player.find_interaction(&signature_correct);
    assert!(result.is_ok());

    // Try with different body (different hash)
    let different_body = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/create".to_string(),
        headers: HashMap::new(),
        body: Some(b"{\"name\":\"different\"}".to_vec()),
    };

    let signature_different = magneto_serge::player::RequestSignature::from(different_body);
    let result = player.find_interaction(&signature_different);
    assert!(result.is_err()); // Different body = different hash = no match
}
