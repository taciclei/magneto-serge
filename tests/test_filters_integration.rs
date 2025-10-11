//! Integration tests for recording filters

use magneto_serge::cassette::{HttpRequest, HttpResponse};
use magneto_serge::filters::{FilterPresets, RecordingFilters};
use magneto_serge::recorder::Recorder;
use std::collections::HashMap;
use tempfile::tempdir;

fn create_request_with_auth(url: &str) -> HttpRequest {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert(
        "Authorization".to_string(),
        "Bearer super-secret-token".to_string(),
    );
    headers.insert("X-API-Key".to_string(), "my-api-key-123".to_string());

    HttpRequest {
        method: "POST".to_string(),
        url: url.to_string(),
        headers,
        body: Some(b"{\"password\":\"secret123\"}".to_vec()),
    }
}

fn create_response(status: u16, content_type: &str) -> HttpResponse {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), content_type.to_string());
    headers.insert("Set-Cookie".to_string(), "session=abc123".to_string());

    HttpResponse {
        status,
        headers,
        body: Some(b"{\"token\":\"jwt-token-xyz\"}".to_vec()),
    }
}

#[test]
fn test_recorder_with_default_filters() {
    let filters = RecordingFilters::default();
    let mut recorder = Recorder::new_with_filters("test-default-filters".to_string(), filters);

    let request = create_request_with_auth("https://api.example.com/login");
    let response = create_response(200, "application/json");

    recorder.record_http(request, response);

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 1);

    // Check that sensitive headers are filtered
    if let magneto_serge::cassette::InteractionKind::Http { request, response } =
        &cassette.interactions[0].kind
    {
        assert_eq!(request.headers.get("Authorization"), Some(&"[FILTERED]".to_string()));
        assert_eq!(request.headers.get("X-API-Key"), Some(&"[FILTERED]".to_string()));
        assert_eq!(response.headers.get("Set-Cookie"), Some(&"[FILTERED]".to_string()));

        // Content-Type should not be filtered
        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    } else {
        panic!("Expected HTTP interaction");
    }
}

#[test]
fn test_recorder_with_body_filtering() {
    let filters = RecordingFilters::new()
        .filter_request_bodies(true)
        .filter_response_bodies(true);

    let mut recorder = Recorder::new_with_filters("test-body-filters".to_string(), filters);

    let request = create_request_with_auth("https://api.example.com/login");
    let response = create_response(200, "application/json");

    recorder.record_http(request, response);

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 1);

    // Check that bodies are filtered
    if let magneto_serge::cassette::InteractionKind::Http { request, response } =
        &cassette.interactions[0].kind
    {
        assert_eq!(request.body, Some(b"[FILTERED]".to_vec()));
        assert_eq!(response.body, Some(b"[FILTERED]".to_vec()));
    } else {
        panic!("Expected HTTP interaction");
    }
}

#[test]
fn test_recorder_with_url_filtering() {
    let filters = RecordingFilters::new()
        .ignore_url(r"analytics\.com")
        .unwrap()
        .ignore_url(r"/track")
        .unwrap();

    let mut recorder = Recorder::new_with_filters("test-url-filters".to_string(), filters);

    // This should be recorded
    let request1 = create_request_with_auth("https://api.example.com/users");
    let response1 = create_response(200, "application/json");
    recorder.record_http(request1, response1);

    // This should be filtered out (analytics.com)
    let request2 = create_request_with_auth("https://analytics.com/collect");
    let response2 = create_response(200, "application/json");
    recorder.record_http(request2, response2);

    // This should be filtered out (/track)
    let request3 = create_request_with_auth("https://api.example.com/track");
    let response3 = create_response(200, "application/json");
    recorder.record_http(request3, response3);

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 1);

    // Only the first interaction should be recorded
    if let magneto_serge::cassette::InteractionKind::Http { request, .. } =
        &cassette.interactions[0].kind
    {
        assert_eq!(request.url, "https://api.example.com/users");
    } else {
        panic!("Expected HTTP interaction");
    }
}

#[test]
fn test_recorder_with_status_filtering() {
    let filters = RecordingFilters::new()
        .skip_status_code(404)
        .skip_status_code(500);

    let mut recorder = Recorder::new_with_filters("test-status-filters".to_string(), filters);

    // This should be recorded (200)
    let request1 = create_request_with_auth("https://api.example.com/users");
    let response1 = create_response(200, "application/json");
    recorder.record_http(request1, response1);

    // This should be filtered out (404)
    let request2 = create_request_with_auth("https://api.example.com/not-found");
    let response2 = create_response(404, "application/json");
    recorder.record_http(request2, response2);

    // This should be filtered out (500)
    let request3 = create_request_with_auth("https://api.example.com/error");
    let response3 = create_response(500, "application/json");
    recorder.record_http(request3, response3);

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 1);
}

#[test]
fn test_recorder_with_content_type_filtering() {
    let filters = RecordingFilters::new()
        .skip_content_type("image/".to_string())
        .skip_content_type("video/".to_string());

    let mut recorder = Recorder::new_with_filters("test-content-type-filters".to_string(), filters);

    // This should be recorded (JSON)
    let request1 = create_request_with_auth("https://api.example.com/data");
    let response1 = create_response(200, "application/json");
    recorder.record_http(request1, response1);

    // This should be filtered out (image)
    let request2 = create_request_with_auth("https://cdn.example.com/logo.png");
    let response2 = create_response(200, "image/png");
    recorder.record_http(request2, response2);

    // This should be filtered out (video)
    let request3 = create_request_with_auth("https://cdn.example.com/video.mp4");
    let response3 = create_response(200, "video/mp4");
    recorder.record_http(request3, response3);

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 1);
}

#[test]
fn test_recorder_with_max_body_size() {
    let filters = RecordingFilters::new().max_body_size(20);

    let mut recorder = Recorder::new_with_filters("test-max-body-size".to_string(), filters);

    let mut request = create_request_with_auth("https://api.example.com/upload");
    request.body = Some(vec![0u8; 100]); // 100 bytes

    let mut response = create_response(200, "application/json");
    response.body = Some(vec![1u8; 150]); // 150 bytes

    recorder.record_http(request, response);

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 1);

    // Check that bodies are truncated
    if let magneto_serge::cassette::InteractionKind::Http { request, response } =
        &cassette.interactions[0].kind
    {
        assert_eq!(request.body.as_ref().unwrap().len(), 20);
        assert_eq!(response.body.as_ref().unwrap().len(), 20);
    } else {
        panic!("Expected HTTP interaction");
    }
}

#[test]
fn test_recorder_preset_security() {
    let filters = FilterPresets::security();
    let mut recorder = Recorder::new_with_filters("test-preset-security".to_string(), filters);

    let request = create_request_with_auth("https://api.example.com/secure");
    let response = create_response(200, "application/json");

    recorder.record_http(request, response);

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 1);

    // Check that headers are filtered
    if let magneto_serge::cassette::InteractionKind::Http { request, response } =
        &cassette.interactions[0].kind
    {
        assert_eq!(request.headers.get("Authorization"), Some(&"[FILTERED]".to_string()));
        assert_eq!(response.headers.get("Set-Cookie"), Some(&"[FILTERED]".to_string()));

        // Bodies should NOT be filtered (security preset only filters headers)
        assert!(request.body.is_some());
        assert_ne!(request.body, Some(b"[FILTERED]".to_vec()));
    } else {
        panic!("Expected HTTP interaction");
    }
}

#[test]
fn test_recorder_preset_no_analytics() {
    let filters = FilterPresets::no_analytics().unwrap();
    let mut recorder = Recorder::new_with_filters("test-preset-no-analytics".to_string(), filters);

    // This should be recorded
    let request1 = create_request_with_auth("https://api.example.com/users");
    let response1 = create_response(200, "application/json");
    recorder.record_http(request1, response1);

    // These should be filtered out
    let analytics_urls = vec![
        "https://google-analytics.com/collect",
        "https://www.googletagmanager.com/gtag/js",
        "https://doubleclick.net/ads",
        "https://www.facebook.com/tr",
        "https://cdn.segment.com/analytics.js",
    ];

    for url in analytics_urls {
        let request = create_request_with_auth(url);
        let response = create_response(200, "application/javascript");
        recorder.record_http(request, response);
    }

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 1);
}

#[test]
fn test_recorder_preset_no_media() {
    let filters = FilterPresets::no_media();
    let mut recorder = Recorder::new_with_filters("test-preset-no-media".to_string(), filters);

    // This should be recorded
    let request1 = create_request_with_auth("https://api.example.com/data");
    let response1 = create_response(200, "application/json");
    recorder.record_http(request1, response1);

    // These should be filtered out
    let media_types = vec![
        "image/png",
        "image/jpeg",
        "video/mp4",
        "audio/mpeg",
        "font/woff2",
    ];

    for content_type in media_types {
        let request = create_request_with_auth("https://cdn.example.com/file");
        let response = create_response(200, content_type);
        recorder.record_http(request, response);
    }

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 1);
}

#[test]
fn test_recorder_preset_success_only() {
    let filters = FilterPresets::success_only();
    let mut recorder = Recorder::new_with_filters("test-preset-success-only".to_string(), filters);

    // These should be recorded (2xx and 3xx)
    for status in [200, 201, 204, 301, 302, 304] {
        let request = create_request_with_auth("https://api.example.com/endpoint");
        let response = create_response(status, "application/json");
        recorder.record_http(request, response);
    }

    // These should be filtered out (4xx and 5xx)
    for status in [400, 401, 403, 404, 500, 502, 503] {
        let request = create_request_with_auth("https://api.example.com/error");
        let response = create_response(status, "application/json");
        recorder.record_http(request, response);
    }

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 6);
}

#[test]
fn test_recorder_save_with_filters() {
    let filters = RecordingFilters::default();
    let mut recorder = Recorder::new_with_filters("test-save-filters".to_string(), filters);

    let request = create_request_with_auth("https://api.example.com/endpoint");
    let response = create_response(200, "application/json");

    recorder.record_http(request, response);

    // Save and reload
    let dir = tempdir().unwrap();
    recorder.save(dir.path()).unwrap();

    // Verify file was created
    let cassette_path = dir.path().join("test-save-filters.json");
    assert!(cassette_path.exists());

    // Load and verify content
    let cassette_json = std::fs::read_to_string(cassette_path).unwrap();
    assert!(cassette_json.contains("[FILTERED]"));
    assert!(!cassette_json.contains("super-secret-token"));
    assert!(!cassette_json.contains("my-api-key-123"));
}

#[test]
fn test_recorder_dynamic_filter_change() {
    let mut recorder = Recorder::new("test-dynamic-filters".to_string());

    // Record without filters
    let request1 = create_request_with_auth("https://api.example.com/unfiltered");
    let response1 = create_response(200, "application/json");
    recorder.record_http(request1, response1);

    // Add filters
    let filters = RecordingFilters::default();
    recorder.set_filters(filters);

    // Record with filters
    let request2 = create_request_with_auth("https://api.example.com/filtered");
    let response2 = create_response(200, "application/json");
    recorder.record_http(request2, response2);

    let cassette = recorder.cassette();
    assert_eq!(cassette.interactions.len(), 2);

    // First interaction should have unfiltered auth
    if let magneto_serge::cassette::InteractionKind::Http { request, .. } =
        &cassette.interactions[0].kind
    {
        assert_eq!(
            request.headers.get("Authorization"),
            Some(&"Bearer super-secret-token".to_string())
        );
    }

    // Second interaction should have filtered auth
    if let magneto_serge::cassette::InteractionKind::Http { request, .. } =
        &cassette.interactions[1].kind
    {
        assert_eq!(request.headers.get("Authorization"), Some(&"[FILTERED]".to_string()));
    }
}
