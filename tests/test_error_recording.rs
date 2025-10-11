//! Integration tests for error recording and replay

use magneto_serge::cassette::{Cassette, HttpRequest, InteractionKind, NetworkError};
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_cassette_with_timeout_error() {
    let mut cassette = Cassette::new("test-timeout".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://slow-api.example.com/endpoint".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let error = NetworkError::timeout("Connection timed out after 5000ms", 5000);

    cassette.add_error(request, error);

    assert_eq!(cassette.interactions.len(), 1);

    // Verify it can be serialized/deserialized
    let dir = tempdir().unwrap();
    let path = dir.path().join("test-timeout.json");
    let file = std::fs::File::create(&path).unwrap();
    serde_json::to_writer_pretty(&file, &cassette).unwrap();

    // Read back
    let content = std::fs::read_to_string(&path).unwrap();
    let loaded: Cassette = serde_json::from_str(&content).unwrap();

    assert_eq!(loaded.interactions.len(), 1);
    match &loaded.interactions[0].kind {
        InteractionKind::HttpError { request, error } => {
            assert_eq!(request.method, "GET");
            assert_eq!(request.url, "https://slow-api.example.com/endpoint");
            match error {
                NetworkError::Timeout {
                    message,
                    timeout_ms,
                } => {
                    assert_eq!(*timeout_ms, 5000);
                    assert!(message.contains("timed out"));
                }
                _ => panic!("Expected Timeout error"),
            }
        }
        _ => panic!("Expected HttpError interaction"),
    }
}

#[test]
fn test_cassette_with_dns_error() {
    let mut cassette = Cassette::new("test-dns".to_string());

    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://nonexistent.invalid/api/endpoint".to_string(),
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: Some(b"{\"data\":\"test\"}".to_vec()),
    };

    let error = NetworkError::dns_failed("Failed to resolve domain: nonexistent.invalid");

    cassette.add_error(request, error);

    // Serialize and deserialize
    let dir = tempdir().unwrap();
    let path = dir.path().join("test-dns.json");
    let file = std::fs::File::create(&path).unwrap();
    serde_json::to_writer_pretty(&file, &cassette).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let loaded: Cassette = serde_json::from_str(&content).unwrap();

    assert_eq!(loaded.interactions.len(), 1);
    match &loaded.interactions[0].kind {
        InteractionKind::HttpError { request, error } => {
            assert_eq!(request.method, "POST");
            assert_eq!(request.url, "https://nonexistent.invalid/api/endpoint");
            assert!(matches!(error, NetworkError::DnsResolutionFailed { .. }));
        }
        _ => panic!("Expected HttpError interaction"),
    }
}

#[test]
fn test_cassette_with_connection_refused() {
    let mut cassette = Cassette::new("test-connection-refused".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "http://localhost:9999/health".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let error = NetworkError::connection_refused("Connection refused on port 9999");

    cassette.add_error(request, error);

    // Serialize and deserialize
    let dir = tempdir().unwrap();
    let path = dir.path().join("test-connection-refused.json");
    let file = std::fs::File::create(&path).unwrap();
    serde_json::to_writer_pretty(&file, &cassette).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let loaded: Cassette = serde_json::from_str(&content).unwrap();

    assert_eq!(loaded.interactions.len(), 1);
    match &loaded.interactions[0].kind {
        InteractionKind::HttpError { request, error } => {
            assert_eq!(request.method, "GET");
            assert_eq!(request.url, "http://localhost:9999/health");
            assert!(matches!(error, NetworkError::ConnectionRefused { .. }));
        }
        _ => panic!("Expected HttpError interaction"),
    }
}

#[test]
fn test_cassette_with_tls_error() {
    let mut cassette = Cassette::new("test-tls".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://expired-cert.badssl.com/".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let error = NetworkError::tls_error("TLS certificate validation failed: certificate expired");

    cassette.add_error(request, error);

    // Serialize and deserialize
    let dir = tempdir().unwrap();
    let path = dir.path().join("test-tls.json");
    let file = std::fs::File::create(&path).unwrap();
    serde_json::to_writer_pretty(&file, &cassette).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let loaded: Cassette = serde_json::from_str(&content).unwrap();

    assert_eq!(loaded.interactions.len(), 1);
    match &loaded.interactions[0].kind {
        InteractionKind::HttpError { error, .. } => {
            assert!(matches!(error, NetworkError::TlsError { .. }));
        }
        _ => panic!("Expected HttpError interaction"),
    }
}

#[test]
fn test_cassette_with_connection_reset() {
    let mut cassette = Cassette::new("test-reset".to_string());

    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/upload".to_string(),
        headers: HashMap::from([(
            "Content-Type".to_string(),
            "multipart/form-data".to_string(),
        )]),
        body: Some(vec![0u8; 1024]), // 1KB of data
    };

    let error = NetworkError::connection_reset("Connection reset by peer during upload");

    cassette.add_error(request, error);

    // Serialize and deserialize
    let dir = tempdir().unwrap();
    let path = dir.path().join("test-reset.json");
    let file = std::fs::File::create(&path).unwrap();
    serde_json::to_writer_pretty(&file, &cassette).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let loaded: Cassette = serde_json::from_str(&content).unwrap();

    assert_eq!(loaded.interactions.len(), 1);
    match &loaded.interactions[0].kind {
        InteractionKind::HttpError { error, .. } => {
            assert!(matches!(error, NetworkError::ConnectionReset { .. }));
        }
        _ => panic!("Expected HttpError interaction"),
    }
}

#[test]
fn test_cassette_with_too_many_redirects() {
    let mut cassette = Cassette::new("test-redirects".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/redirect-loop".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let error = NetworkError::too_many_redirects("Too many redirects (10)", 10);

    cassette.add_error(request, error);

    // Serialize and deserialize
    let dir = tempdir().unwrap();
    let path = dir.path().join("test-redirects.json");
    let file = std::fs::File::create(&path).unwrap();
    serde_json::to_writer_pretty(&file, &cassette).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let loaded: Cassette = serde_json::from_str(&content).unwrap();

    assert_eq!(loaded.interactions.len(), 1);
    match &loaded.interactions[0].kind {
        InteractionKind::HttpError { error, .. } => match error {
            NetworkError::TooManyRedirects { redirect_count, .. } => {
                assert_eq!(*redirect_count, 10);
            }
            _ => panic!("Expected TooManyRedirects error"),
        },
        _ => panic!("Expected HttpError interaction"),
    }
}

#[test]
fn test_cassette_with_other_error() {
    let mut cassette = Cassette::new("test-other".to_string());

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/unknown".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    let error = NetworkError::other("Unknown network error occurred");

    cassette.add_error(request, error);

    // Serialize and deserialize
    let dir = tempdir().unwrap();
    let path = dir.path().join("test-other.json");
    let file = std::fs::File::create(&path).unwrap();
    serde_json::to_writer_pretty(&file, &cassette).unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let loaded: Cassette = serde_json::from_str(&content).unwrap();

    assert_eq!(loaded.interactions.len(), 1);
    match &loaded.interactions[0].kind {
        InteractionKind::HttpError { error, .. } => {
            assert!(matches!(error, NetworkError::Other { .. }));
        }
        _ => panic!("Expected HttpError interaction"),
    }
}

#[test]
fn test_network_error_equality() {
    let error1 = NetworkError::timeout("timeout", 5000);
    let error2 = NetworkError::timeout("timeout", 5000);
    let error3 = NetworkError::timeout("timeout", 3000);

    assert_eq!(error1, error2);
    assert_ne!(error1, error3);

    let dns1 = NetworkError::dns_failed("dns error");
    let dns2 = NetworkError::dns_failed("dns error");
    assert_eq!(dns1, dns2);
}
