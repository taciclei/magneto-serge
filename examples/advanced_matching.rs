//! Example: Using advanced matching strategies with Magneto-Serge
//!
//! This example demonstrates how to use flexible matching strategies to match
//! HTTP requests in different ways: regex URLs, ignoring query parameters,
//! partial body matching, and more.
//!
//! Run with: cargo run --example advanced_matching

use magneto_serge::{
    BodyMatchMode, HttpRequest, HttpResponse, MatchingStrategy, Player, Recorder, UrlMatchMode,
};
use std::collections::HashMap;
use tempfile::tempdir;

fn main() {
    println!("🎯 Magneto-Serge Advanced Matching Examples\n");

    // Example 1: Regex URL Matching
    example_regex_url_matching();

    // Example 2: Ignore Query Parameters
    example_ignore_query_params();

    // Example 3: JSON Path Body Matching
    example_json_path_matching();

    // Example 4: Lenient Matching Strategy
    example_lenient_matching();

    // Example 5: Header-Specific Matching
    example_header_matching();

    // Example 6: Body Size Only Matching
    example_body_size_matching();

    println!("\n✅ All examples completed successfully!");
}

fn example_regex_url_matching() {
    println!("1️⃣  Regex URL Matching");
    println!("   Match URLs using regex patterns (e.g., /users/:id)");

    // Record interaction with user ID 123
    let mut recorder = Recorder::new("regex-example".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/users/123".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"id\":123,\"name\":\"Alice\"}".to_vec()),
    };

    recorder.record_http(request, response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Create a regex matching strategy
    let strategy = MatchingStrategy::new().with_url_mode(UrlMatchMode::Regex {
        pattern: r"^https://api\.example\.com/users/\d+$".to_string(),
    });

    let mut player = Player::load(dir.path(), "regex-example")
        .unwrap()
        .with_matching_strategy(strategy);

    // Match with different user ID (456 instead of 123)
    let test_request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/users/456".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    match player.find_interaction_advanced(&test_request) {
        Ok(idx) => {
            let interaction = player.get_interaction(idx).unwrap();
            println!("   ✓ Matched user/456 with recorded user/123 using regex!");
            println!("   → Response: {:?}\n", interaction);
        }
        Err(e) => println!("   ✗ No match: {}\n", e),
    }
}

fn example_ignore_query_params() {
    println!("2️⃣  Ignore Query Parameters");
    println!("   Match URLs while ignoring specific query params (e.g., timestamp)");

    let mut recorder = Recorder::new("query-params-example".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/search?q=rust&page=1&timestamp=12345".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"results\":[{\"title\":\"Learning Rust\"}]}".to_vec()),
    };

    recorder.record_http(request, response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Ignore timestamp and page parameters
    let strategy = MatchingStrategy::new().with_url_mode(UrlMatchMode::IgnoreQueryParams {
        params: vec!["timestamp".to_string(), "page".to_string()],
    });

    let mut player = Player::load(dir.path(), "query-params-example")
        .unwrap()
        .with_matching_strategy(strategy);

    // Match with different timestamp and page
    let test_request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/search?q=rust&page=5&timestamp=99999".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    match player.find_interaction_advanced(&test_request) {
        Ok(_) => println!("   ✓ Matched despite different page and timestamp!\n"),
        Err(e) => println!("   ✗ No match: {}\n", e),
    }
}

fn example_json_path_matching() {
    println!("3️⃣  JSON Path Body Matching");
    println!("   Match only specific fields in JSON bodies (e.g., user.id)");

    let mut recorder = Recorder::new("json-path-example".to_string());

    let body1 = serde_json::json!({
        "user": {
            "id": 123,
            "name": "Alice",
            "email": "alice@example.com"
        },
        "timestamp": 1234567890,
        "nonce": "random123"
    });

    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/users".to_string(),
        headers: HashMap::new(),
        body: Some(serde_json::to_vec(&body1).unwrap()),
    };

    let response = HttpResponse {
        status: 201,
        headers: HashMap::new(),
        body: Some(b"{\"status\":\"created\"}".to_vec()),
    };

    recorder.record_http(request, response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Match only on user.id field
    let strategy = MatchingStrategy::new().with_body_mode(BodyMatchMode::JsonPath {
        path: "user.id".to_string(),
    });

    let mut player = Player::load(dir.path(), "json-path-example")
        .unwrap()
        .with_matching_strategy(strategy);

    // Match with same ID but different name, email, timestamp, and nonce
    let body2 = serde_json::json!({
        "user": {
            "id": 123,
            "name": "Bob",
            "email": "bob@example.com"
        },
        "timestamp": 9876543210i64,
        "nonce": "different456"
    });

    let test_request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/users".to_string(),
        headers: HashMap::new(),
        body: Some(serde_json::to_vec(&body2).unwrap()),
    };

    match player.find_interaction_advanced(&test_request) {
        Ok(_) => println!("   ✓ Matched based on user.id (123) only!\n"),
        Err(e) => println!("   ✗ No match: {}\n", e),
    }
}

fn example_lenient_matching() {
    println!("4️⃣  Lenient Matching Strategy");
    println!("   Ignore query params and body for maximum flexibility");

    let mut recorder = Recorder::new("lenient-example".to_string());

    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/events?token=abc&session=xyz".to_string(),
        headers: HashMap::new(),
        body: Some(b"event data here".to_vec()),
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"status\":\"ok\"}".to_vec()),
    };

    recorder.record_http(request, response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Use lenient strategy (ignores query params and body)
    let strategy = MatchingStrategy::lenient();

    let mut player = Player::load(dir.path(), "lenient-example")
        .unwrap()
        .with_matching_strategy(strategy);

    // Match with completely different query params and body
    let test_request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/events?token=123&session=456".to_string(),
        headers: HashMap::new(),
        body: Some(b"different event data".to_vec()),
    };

    match player.find_interaction_advanced(&test_request) {
        Ok(_) => println!("   ✓ Matched despite different query params and body!\n"),
        Err(e) => println!("   ✗ No match: {}\n", e),
    }
}

fn example_header_matching() {
    println!("5️⃣  Header-Specific Matching");
    println!("   Match on specific headers (e.g., Authorization)");

    let mut recorder = Recorder::new("header-example".to_string());

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer secret123".to_string());
    headers.insert("User-Agent".to_string(), "MyApp/1.0".to_string());
    headers.insert("X-Request-ID".to_string(), "req-001".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/protected".to_string(),
        headers,
        body: None,
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"data\":\"secret stuff\"}".to_vec()),
    };

    recorder.record_http(request, response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Match only on Authorization header
    let strategy = MatchingStrategy::new().match_header("Authorization".to_string());

    let mut player = Player::load(dir.path(), "header-example")
        .unwrap()
        .with_matching_strategy(strategy);

    // Match with same Authorization but different User-Agent and X-Request-ID
    let mut test_headers = HashMap::new();
    test_headers.insert("Authorization".to_string(), "Bearer secret123".to_string());
    test_headers.insert("User-Agent".to_string(), "DifferentApp/2.0".to_string());
    test_headers.insert("X-Request-ID".to_string(), "req-999".to_string());

    let test_request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/protected".to_string(),
        headers: test_headers,
        body: None,
    };

    match player.find_interaction_advanced(&test_request) {
        Ok(_) => println!("   ✓ Matched based on Authorization header only!\n"),
        Err(e) => println!("   ✗ No match: {}\n", e),
    }
}

fn example_body_size_matching() {
    println!("6️⃣  Body Size Only Matching");
    println!("   Match based on body size (useful for binary uploads)");

    let mut recorder = Recorder::new("body-size-example".to_string());

    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/upload".to_string(),
        headers: HashMap::new(),
        body: Some(vec![0u8; 1024]), // 1KB of zeros
    };

    let response = HttpResponse {
        status: 200,
        headers: HashMap::new(),
        body: Some(b"{\"status\":\"uploaded\"}".to_vec()),
    };

    recorder.record_http(request, response);

    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Match only on body size
    let strategy = MatchingStrategy::new().with_body_mode(BodyMatchMode::SizeOnly);

    let mut player = Player::load(dir.path(), "body-size-example")
        .unwrap()
        .with_matching_strategy(strategy);

    // Match with same size but different content
    let test_request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/upload".to_string(),
        headers: HashMap::new(),
        body: Some(vec![255u8; 1024]), // 1KB of 0xFF
    };

    match player.find_interaction_advanced(&test_request) {
        Ok(_) => println!("   ✓ Matched based on body size (1024 bytes) only!\n"),
        Err(e) => println!("   ✗ No match: {}\n", e),
    }
}
