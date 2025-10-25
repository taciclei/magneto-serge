//! Integration tests for REST API

#![cfg(feature = "api")]

use magneto_serge::{ApiConfig, ApiServer};
use std::time::Duration;

#[tokio::test]
async fn test_api_root() {
    // Create API config
    let config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 8890, // Use different port to avoid conflicts
        proxy_port: 8888,
        cassette_dir: "./cassettes".to_string(),
        auth_enabled: false,
        api_key: None,
    };

    // Start API server in background
    let server = ApiServer::new(config.clone());
    tokio::spawn(async move {
        let _ = server.start().await;
    });

    // Wait for server to start
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Test GET /
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}:{}/", config.host, config.port))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");

    // Verify JSON-LD context
    assert_eq!(json["@context"], "https://www.w3.org/ns/hydra/core");
    assert_eq!(json["@type"], "hydra:Resource");
    assert_eq!(json["success"], true);

    // Verify API info
    assert_eq!(json["data"]["title"], "Magneto-Serge API");
    assert!(json["data"]["version"].is_string());

    // Verify Hydra links exist
    assert!(json["hydra:link"].is_array());
    let links = json["hydra:link"].as_array().unwrap();
    assert!(!links.is_empty());
}

#[tokio::test]
async fn test_health_endpoint() {
    let config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 8891,
        proxy_port: 8888,
        cassette_dir: "./cassettes".to_string(),
        auth_enabled: false,
        api_key: None,
    };

    let server = ApiServer::new(config.clone());
    tokio::spawn(async move {
        let _ = server.start().await;
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}:{}/health", config.host, config.port))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["status"], "healthy");
    assert!(json["data"]["uptime_seconds"].is_number());
}

#[tokio::test]
async fn test_openapi_endpoint() {
    let config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 8892,
        proxy_port: 8888,
        cassette_dir: "./cassettes".to_string(),
        auth_enabled: false,
        api_key: None,
    };

    let server = ApiServer::new(config.clone());
    tokio::spawn(async move {
        let _ = server.start().await;
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "http://{}:{}/openapi.json",
            config.host, config.port
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");

    // Verify OpenAPI structure
    assert_eq!(json["openapi"], "3.0.3");
    assert_eq!(json["info"]["title"], "Magneto-Serge API");
    assert!(json["paths"].is_object());
    assert!(json["components"].is_object());

    // Verify key paths exist
    let paths = json["paths"].as_object().unwrap();
    assert!(paths.contains_key("/"));
    assert!(paths.contains_key("/health"));
    assert!(paths.contains_key("/proxy/start"));
    assert!(paths.contains_key("/proxy/stop"));
    assert!(paths.contains_key("/proxy/status"));
}

#[tokio::test]
async fn test_proxy_status_when_not_running() {
    let config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 8893,
        proxy_port: 8888,
        cassette_dir: "./cassettes".to_string(),
        auth_enabled: false,
        api_key: None,
    };

    let server = ApiServer::new(config.clone());
    tokio::spawn(async move {
        let _ = server.start().await;
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "http://{}:{}/proxy/status",
            config.host, config.port
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");

    assert_eq!(json["success"], true);
    assert_eq!(json["data"]["running"], false);
    assert_eq!(json["data"]["mode"], "None");
}

#[tokio::test]
async fn test_authentication() {
    let config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 8894,
        proxy_port: 8888,
        cassette_dir: "./cassettes".to_string(),
        auth_enabled: true,
        api_key: Some("test-api-key-12345".to_string()),
    };

    let server = ApiServer::new(config.clone());
    tokio::spawn(async move {
        let _ = server.start().await;
    });

    tokio::time::sleep(Duration::from_millis(500)).await;

    let client = reqwest::Client::new();

    // Test without authentication - should fail
    let response = client
        .get(format!(
            "http://{}:{}/proxy/status",
            config.host, config.port
        ))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 401);
    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(json["success"], false);

    // Test with valid authentication - should succeed
    let response = client
        .get(format!(
            "http://{}:{}/proxy/status",
            config.host, config.port
        ))
        .header("Authorization", "Bearer test-api-key-12345")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
    let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(json["success"], true);

    // Test with invalid authentication - should fail
    let response = client
        .get(format!(
            "http://{}:{}/proxy/status",
            config.host, config.port
        ))
        .header("Authorization", "Bearer wrong-key")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 401);
}
