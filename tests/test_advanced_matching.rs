//! Integration tests for advanced matching strategies

use magneto_serge::{BodyMatchMode, MatchingStrategy, Player, Recorder, UrlMatchMode};
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_regex_url_matching() {
    // Create cassette with user ID 123
    let mut recorder = Recorder::new("test-regex-url".to_string());

    let request = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/users/123".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = magneto_serge::HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"id\":123,\"name\":\"Alice\"}".to_vec()),
    };

    recorder.record_http(request.clone(), response);

    // Save cassette
    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load with regex matching strategy
    let strategy = MatchingStrategy::new().with_url_mode(UrlMatchMode::Regex {
        pattern: r"^https://api\.example\.com/users/\d+$".to_string(),
    });

    let mut player = Player::load(dir.path(), "test-regex-url")
        .unwrap()
        .with_matching_strategy(strategy);

    // Should match user ID 456 (different from recorded 123)
    let request_456 = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/users/456".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let idx = player.find_interaction_advanced(&request_456).unwrap();
    assert_eq!(idx, 0);

    // Should not match /posts endpoint
    let request_posts = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/posts/123".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    assert!(player.find_interaction_advanced(&request_posts).is_err());
}

#[test]
fn test_ignore_query_params_matching() {
    // Create cassette with query params
    let mut recorder = Recorder::new("test-ignore-query".to_string());

    let request = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/search?q=rust&page=1&timestamp=12345".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = magneto_serge::HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"results\":[]}".to_vec()),
    };

    recorder.record_http(request, response);

    // Save cassette
    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load with strategy that ignores timestamp and page
    let strategy = MatchingStrategy::new().with_url_mode(UrlMatchMode::IgnoreQueryParams {
        params: vec!["timestamp".to_string(), "page".to_string()],
    });

    let mut player = Player::load(dir.path(), "test-ignore-query")
        .unwrap()
        .with_matching_strategy(strategy);

    // Should match with different page and timestamp
    let request_diff = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/search?q=rust&page=2&timestamp=67890".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let idx = player.find_interaction_advanced(&request_diff).unwrap();
    assert_eq!(idx, 0);

    // Should not match with different q parameter
    let request_diff_q = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/search?q=python&page=1&timestamp=12345".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    assert!(player.find_interaction_advanced(&request_diff_q).is_err());
}

#[test]
fn test_json_path_body_matching() {
    // Create cassette with JSON body
    let mut recorder = Recorder::new("test-json-path".to_string());

    let body1 = serde_json::json!({
        "user": {
            "id": 123,
            "name": "Alice",
            "email": "alice@example.com"
        },
        "timestamp": 1234567890
    });

    let request = magneto_serge::HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/users".to_string(),
        headers: HashMap::new(),
        body: Some(serde_json::to_vec(&body1).unwrap()),
    };

    let response = magneto_serge::HttpResponse {
        status: 201,
        headers: HashMap::new(),
        body: Some(b"{\"status\":\"created\"}".to_vec()),
    };

    recorder.record_http(request, response);

    // Save cassette
    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load with JSON path matching (only match on user.id)
    let strategy = MatchingStrategy::new().with_body_mode(BodyMatchMode::JsonPath {
        path: "user.id".to_string(),
    });

    let mut player = Player::load(dir.path(), "test-json-path")
        .unwrap()
        .with_matching_strategy(strategy);

    // Should match with same ID but different name and timestamp
    let body2 = serde_json::json!({
        "user": {
            "id": 123,
            "name": "Bob",
            "email": "bob@example.com"
        },
        "timestamp": 9999999999i64
    });

    let request_diff = magneto_serge::HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/users".to_string(),
        headers: HashMap::new(),
        body: Some(serde_json::to_vec(&body2).unwrap()),
    };

    let idx = player.find_interaction_advanced(&request_diff).unwrap();
    assert_eq!(idx, 0);

    // Should not match with different ID
    let body3 = serde_json::json!({
        "user": {
            "id": 456,
            "name": "Alice",
            "email": "alice@example.com"
        },
        "timestamp": 1234567890
    });

    let request_diff_id = magneto_serge::HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/users".to_string(),
        headers: HashMap::new(),
        body: Some(serde_json::to_vec(&body3).unwrap()),
    };

    assert!(player.find_interaction_advanced(&request_diff_id).is_err());
}

#[test]
fn test_lenient_matching_strategy() {
    // Create cassette
    let mut recorder = Recorder::new("test-lenient".to_string());

    let request = magneto_serge::HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/users?token=abc&timestamp=12345".to_string(),
        headers: HashMap::new(),
        body: Some(b"some body content".to_vec()),
    };

    let response = magneto_serge::HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"status\":\"ok\"}".to_vec()),
    };

    recorder.record_http(request, response);

    // Save cassette
    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load with lenient strategy (ignores query params and body)
    let strategy = MatchingStrategy::lenient();

    let mut player = Player::load(dir.path(), "test-lenient")
        .unwrap()
        .with_matching_strategy(strategy);

    // Should match with completely different query params and body
    let request_diff = magneto_serge::HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/users?token=xyz&timestamp=99999".to_string(),
        headers: HashMap::new(),
        body: Some(b"completely different body".to_vec()),
    };

    let idx = player.find_interaction_advanced(&request_diff).unwrap();
    assert_eq!(idx, 0);
}

#[test]
fn test_path_only_matching() {
    // Create cassette
    let mut recorder = Recorder::new("test-path-only".to_string());

    let request = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com:443/v1/users".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = magneto_serge::HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"[]".to_vec()),
    };

    recorder.record_http(request, response);

    // Save cassette
    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load with path-only matching
    let strategy = MatchingStrategy::new().with_url_mode(UrlMatchMode::PathOnly);

    let mut player = Player::load(dir.path(), "test-path-only")
        .unwrap()
        .with_matching_strategy(strategy);

    // Should match with different host and scheme
    let request_diff = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "http://localhost:8080/v1/users".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let idx = player.find_interaction_advanced(&request_diff).unwrap();
    assert_eq!(idx, 0);
}

#[test]
fn test_header_specific_matching() {
    // Create cassette
    let mut recorder = Recorder::new("test-header-match".to_string());

    let mut headers1 = HashMap::new();
    headers1.insert("Authorization".to_string(), "Bearer token123".to_string());
    headers1.insert("User-Agent".to_string(), "MyApp/1.0".to_string());

    let request = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/protected".to_string(),
        headers: headers1,
        body: None,
    };

    let response = magneto_serge::HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"data\":\"secret\"}".to_vec()),
    };

    recorder.record_http(request, response);

    // Save cassette
    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load with strategy that matches on Authorization header
    let strategy = MatchingStrategy::new().match_header("Authorization".to_string());

    let mut player = Player::load(dir.path(), "test-header-match")
        .unwrap()
        .with_matching_strategy(strategy);

    // Should match with same Authorization but different User-Agent
    let mut headers2 = HashMap::new();
    headers2.insert("Authorization".to_string(), "Bearer token123".to_string());
    headers2.insert("User-Agent".to_string(), "DifferentApp/2.0".to_string());

    let request_same_auth = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/protected".to_string(),
        headers: headers2,
        body: None,
    };

    let idx = player
        .find_interaction_advanced(&request_same_auth)
        .unwrap();
    assert_eq!(idx, 0);

    // Should not match with different Authorization
    let mut headers3 = HashMap::new();
    headers3.insert("Authorization".to_string(), "Bearer token456".to_string());
    headers3.insert("User-Agent".to_string(), "MyApp/1.0".to_string());

    let request_diff_auth = magneto_serge::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/protected".to_string(),
        headers: headers3,
        body: None,
    };

    assert!(player
        .find_interaction_advanced(&request_diff_auth)
        .is_err());
}

#[test]
fn test_body_size_only_matching() {
    // Create cassette
    let mut recorder = Recorder::new("test-body-size".to_string());

    let request = magneto_serge::HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/upload".to_string(),
        headers: HashMap::new(),
        body: Some(vec![0u8; 1024]), // 1KB of zeros
    };

    let response = magneto_serge::HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"status\":\"ok\"}".to_vec()),
    };

    recorder.record_http(request, response);

    // Save cassette
    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Load with size-only matching
    let strategy = MatchingStrategy::new().with_body_mode(BodyMatchMode::SizeOnly);

    let mut player = Player::load(dir.path(), "test-body-size")
        .unwrap()
        .with_matching_strategy(strategy);

    // Should match with same size but different content
    let request_same_size = magneto_serge::HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/upload".to_string(),
        headers: HashMap::new(),
        body: Some(vec![1u8; 1024]), // 1KB of ones
    };

    let idx = player
        .find_interaction_advanced(&request_same_size)
        .unwrap();
    assert_eq!(idx, 0);

    // Should not match with different size
    let request_diff_size = magneto_serge::HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/upload".to_string(),
        headers: HashMap::new(),
        body: Some(vec![0u8; 2048]), // 2KB
    };

    assert!(player
        .find_interaction_advanced(&request_diff_size)
        .is_err());
}
