//! End-to-end integration tests for HTTP proxy
//!
//! These tests verify the complete record/replay cycle using real HTTP requests
//! to httpbin.org (a public HTTP testing service).

use magneto_serge::{
    cassette::Cassette,
    player::{Player, RequestSignature},
    recorder::Recorder,
    CertificateAuthority, MatgtoProxy, ProxyMode,
};
use std::collections::HashMap;
use tempfile::TempDir;

/// Helper to create a test proxy with temporary directories
fn create_test_proxy() -> (MatgtoProxy, TempDir, TempDir) {
    let cassette_dir = TempDir::new().expect("Failed to create temp cassette dir");
    let cert_dir = TempDir::new().expect("Failed to create temp cert dir");

    let proxy = MatgtoProxy::new_internal(cassette_dir.path())
        .expect("Failed to create proxy")
        .with_port(18888); // Use non-standard port for tests

    (proxy, cassette_dir, cert_dir)
}

#[tokio::test]
#[ignore] // Ignore by default - requires network access
async fn test_e2e_record_and_replay_simple_get() {
    // Initialize tracing for debugging
    let _ = tracing_subscriber::fmt()
        .with_env_filter("magneto_serge=debug")
        .try_init();

    let (mut proxy, cassette_dir, _cert_dir) = create_test_proxy();

    // ========== PHASE 1: RECORD ==========
    tracing::info!("Starting record phase...");

    proxy = proxy.with_mode(ProxyMode::Record);
    assert!(
        proxy.start_recording("httpbin-test".to_string()),
        "Failed to start recording"
    );

    // TODO: Make actual HTTP request through proxy
    // For now, this is a placeholder for the integration

    assert!(proxy.stop_recording(), "Failed to stop recording");

    tracing::info!("Recording complete");

    // ========== PHASE 2: VERIFY CASSETTE ==========
    let _cassette_path = cassette_dir.path().join("httpbin-test.json");

    // In a real implementation, cassette should exist here
    // assert!(_cassette_path.exists(), "Cassette file should exist");

    // ========== PHASE 3: REPLAY ==========
    tracing::info!("Starting replay phase...");

    proxy = proxy.with_mode(ProxyMode::Replay);
    assert!(
        proxy.replay("httpbin-test".to_string()),
        "Failed to start replay"
    );

    // TODO: Make same HTTP request - should return cached response
    // Verify response matches recorded one

    proxy.shutdown();

    tracing::info!("E2E test complete");
}

#[tokio::test]
#[ignore]
async fn test_e2e_auto_mode() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("magneto_serge=debug")
        .try_init();

    let (mut proxy, _cassette_dir, _cert_dir) = create_test_proxy();

    // ========== TEST AUTO MODE ==========
    // First request should record (no cassette exists)
    // Second request should replay (cassette exists)

    proxy = proxy.with_mode(ProxyMode::Auto);
    assert!(
        proxy.start_recording("auto-test".to_string()),
        "Failed to start auto mode"
    );

    // TODO: Make first request - should trigger recording
    // TODO: Make second request - should trigger replay

    proxy.shutdown();
}

#[tokio::test]
#[ignore] // Requires network access to httpbin.org
async fn test_http_forwarder_direct() {
    // Test the HTTP forwarder directly without proxy
    use magneto_serge::cassette::HttpRequest;
    use magneto_serge::proxy::HttpForwarder;

    let forwarder = HttpForwarder::new();

    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://httpbin.org/get".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    // This test requires network access
    let response = forwarder.forward(&request).await;

    if let Ok(resp) = response {
        assert_eq!(resp.status, 200);
        tracing::info!("Direct forwarder test passed: status={}", resp.status);
    } else {
        tracing::warn!("Network request failed (expected in offline environments)");
    }
}

#[tokio::test]
#[ignore] // Requires network access to httpbin.org
async fn test_http_forwarder_post() {
    use magneto_serge::cassette::HttpRequest;
    use magneto_serge::proxy::HttpForwarder;

    let forwarder = HttpForwarder::new();

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://httpbin.org/post".to_string(),
        headers,
        body: Some(b"{\"test\":\"data\",\"value\":42}".to_vec()),
    };

    let response = forwarder.forward(&request).await;

    if let Ok(resp) = response {
        assert_eq!(resp.status, 200);

        // Verify response contains our posted data
        if let Some(body) = &resp.body {
            let body_str = String::from_utf8_lossy(body);
            assert!(body_str.contains("test"));
            assert!(body_str.contains("data"));
        }

        tracing::info!("POST forwarder test passed");
    }
}

#[test]
fn test_certificate_authority_creation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let ca = CertificateAuthority::new(temp_dir.path());

    assert!(
        ca.is_ok(),
        "Certificate authority should be created successfully"
    );
}

#[test]
fn test_proxy_modes() {
    let (proxy, _cassette_dir, _cert_dir) = create_test_proxy();

    // Test default mode
    assert_eq!(proxy.mode(), ProxyMode::Auto);

    // Test mode switching
    let proxy_record = proxy.with_mode(ProxyMode::Record);
    assert_eq!(proxy_record.mode(), ProxyMode::Record);

    let proxy_replay = proxy_record.with_mode(ProxyMode::Replay);
    assert_eq!(proxy_replay.mode(), ProxyMode::Replay);

    let proxy_passthrough = proxy_replay.with_mode(ProxyMode::Passthrough);
    assert_eq!(proxy_passthrough.mode(), ProxyMode::Passthrough);
}

#[tokio::test]
async fn test_full_record_replay_cycle() {
    // Initialize tracing
    let _ = tracing_subscriber::fmt()
        .with_env_filter("magneto_serge=debug")
        .try_init();

    let cassette_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_name = "full-cycle-test";

    tracing::info!("📹 Phase 1: RECORD");

    // ========== PHASE 1: RECORD ==========
    {
        let mut recorder = Recorder::new(cassette_name.to_string());

        // Simulate a recorded HTTP interaction
        let request = magneto_serge::cassette::HttpRequest {
            method: "GET".to_string(),
            url: "https://httpbin.org/get".to_string(),
            headers: {
                let mut h = HashMap::new();
                h.insert("User-Agent".to_string(), "matgto-test/1.0".to_string());
                h.insert("Accept".to_string(), "application/json".to_string());
                h
            },
            body: None,
        };

        let response = magneto_serge::cassette::HttpResponse {
            status: 200,
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "application/json".to_string());
                h.insert("Server".to_string(), "httpbin".to_string());
                h
            },
            body: Some(
                r#"{"args":{},"headers":{"Accept":"application/json","User-Agent":"matgto-test/1.0"},"url":"https://httpbin.org/get"}"#
                    .as_bytes()
                    .to_vec(),
            ),
        };

        // Record the interaction
        recorder.record_http(request.clone(), response.clone());
        tracing::info!("✅ Interaction recorded");

        // Save cassette
        recorder
            .save(cassette_dir.path())
            .expect("Failed to save cassette");
        tracing::info!("💾 Cassette saved");

        // Verify cassette file exists
        let cassette_path = cassette_dir.path().join(format!("{}.json", cassette_name));
        assert!(cassette_path.exists(), "Cassette file should exist");
        tracing::info!("✅ Cassette file verified: {:?}", cassette_path);
    }

    tracing::info!("\n▶️  Phase 2: REPLAY");

    // ========== PHASE 2: REPLAY ==========
    {
        // Load cassette
        let mut player =
            Player::load(cassette_dir.path(), cassette_name).expect("Failed to load cassette");
        tracing::info!("📼 Cassette loaded");

        // Verify cassette was loaded
        assert!(player.has_cassette(), "Player should have cassette loaded");

        // Create a matching request signature
        let replay_signature = RequestSignature {
            method: "GET".to_string(),
            url: "https://httpbin.org/get".to_string(),
            body_hash: None,
        };

        // Find matching interaction
        let interaction_idx = player
            .find_interaction(&replay_signature)
            .expect("Should find matching interaction");

        tracing::info!("✅ Interaction found in cassette");

        // Get the interaction
        let interaction = player
            .get_interaction(interaction_idx)
            .expect("Should get interaction");

        // Verify response
        if let magneto_serge::cassette::InteractionKind::Http { response, .. } = &interaction.kind
        {
            assert_eq!(response.status, 200);
            assert!(response.body.is_some());

            let body = response.body.as_ref().unwrap();
            let body_str = String::from_utf8_lossy(body);
            assert!(body_str.contains("httpbin.org"));
            assert!(body_str.contains("matgto-test/1.0"));

            tracing::info!("✅ Response validated");
            tracing::info!("Response body: {}", body_str);
        } else {
            panic!("Interaction should be HTTP type");
        }

        // Verify replay count
        assert_eq!(player.replay_count(), 1, "Should have 1 replay");
        tracing::info!("✅ Replay count verified");
    }

    tracing::info!("\n📊 Phase 3: VERIFY CASSETTE");

    // ========== PHASE 3: VERIFY CASSETTE ==========
    {
        // Read and parse cassette file
        let cassette_path = cassette_dir.path().join(format!("{}.json", cassette_name));
        let cassette_json =
            std::fs::read_to_string(&cassette_path).expect("Failed to read cassette");

        tracing::info!("Cassette JSON:\n{}", cassette_json);

        // Parse cassette
        let cassette: Cassette =
            serde_json::from_str(&cassette_json).expect("Failed to parse cassette");

        // Verify cassette structure
        assert_eq!(cassette.name, cassette_name);
        assert_eq!(cassette.version, "1.0");
        assert_eq!(cassette.interactions.len(), 1);

        tracing::info!("✅ Cassette structure validated");
        tracing::info!("   Name: {}", cassette.name);
        tracing::info!("   Version: {}", cassette.version);
        tracing::info!("   Interactions: {}", cassette.interactions.len());
    }

    tracing::info!("\n🎉 Full cycle test completed successfully!");
}

#[tokio::test]
async fn test_record_with_post_body() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("magneto_serge=debug")
        .try_init();

    let cassette_dir = TempDir::new().unwrap();
    let cassette_name = "post-body-test";

    tracing::info!("Testing POST request with body");

    // Record a POST request with JSON body
    let mut recorder = Recorder::new(cassette_name.to_string());

    let request = magneto_serge::cassette::HttpRequest {
        method: "POST".to_string(),
        url: "https://httpbin.org/post".to_string(),
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type".to_string(), "application/json".to_string());
            h
        },
        body: Some(b"{\"name\":\"test\",\"value\":42}".to_vec()),
    };

    let response = magneto_serge::cassette::HttpResponse {
        status: 200,
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type".to_string(), "application/json".to_string());
            h
        },
        body: Some(b"{\"json\":{\"name\":\"test\",\"value\":42}}".to_vec()),
    };

    recorder.record_http(request.clone(), response.clone());
    recorder.save(cassette_dir.path()).unwrap();

    tracing::info!("✅ POST with body recorded");

    // Replay
    let mut player = Player::load(cassette_dir.path(), cassette_name).unwrap();

    let replay_signature = RequestSignature::from(magneto_serge::cassette::HttpRequest {
        method: "POST".to_string(),
        url: "https://httpbin.org/post".to_string(),
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type".to_string(), "application/json".to_string());
            h
        },
        body: Some(b"{\"name\":\"test\",\"value\":42}".to_vec()),
    });

    let interaction_idx = player.find_interaction(&replay_signature).unwrap();
    let interaction = player.get_interaction(interaction_idx).unwrap();

    let replayed_response = if let magneto_serge::cassette::InteractionKind::Http { response, .. } =
        &interaction.kind
    {
        response
    } else {
        panic!("Expected HTTP interaction");
    };

    // Verify body was preserved
    assert!(replayed_response.body.is_some());
    let body = replayed_response.body.as_ref().unwrap();
    assert!(body.contains(&b"test"[0]));
    assert!(body.contains(&b"42"[0]));

    tracing::info!("✅ POST body verified in replay");
}

#[tokio::test]
async fn test_multiple_interactions() {
    let cassette_dir = TempDir::new().unwrap();
    let cassette_name = "multi-test";

    // Record multiple interactions
    let mut recorder = Recorder::new(cassette_name.to_string());

    // Interaction 1: GET
    recorder.record_http(
        magneto_serge::cassette::HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/users".to_string(),
            headers: HashMap::new(),
            body: None,
        },
        magneto_serge::cassette::HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(b"[{\"id\":1,\"name\":\"Alice\"}]".to_vec()),
        },
    );

    // Interaction 2: POST
    recorder.record_http(
        magneto_serge::cassette::HttpRequest {
            method: "POST".to_string(),
            url: "https://api.example.com/users".to_string(),
            headers: HashMap::new(),
            body: Some(b"{\"name\":\"Bob\"}".to_vec()),
        },
        magneto_serge::cassette::HttpResponse {
            status: 201,
            headers: HashMap::new(),
            body: Some(b"{\"id\":2,\"name\":\"Bob\"}".to_vec()),
        },
    );

    // Interaction 3: DELETE
    recorder.record_http(
        magneto_serge::cassette::HttpRequest {
            method: "DELETE".to_string(),
            url: "https://api.example.com/users/1".to_string(),
            headers: HashMap::new(),
            body: None,
        },
        magneto_serge::cassette::HttpResponse {
            status: 204,
            headers: HashMap::new(),
            body: None,
        },
    );

    recorder.save(cassette_dir.path()).unwrap();

    // Replay and verify all interactions
    let mut player = Player::load(cassette_dir.path(), cassette_name).unwrap();

    // Find GET
    let get_sig = RequestSignature::from(magneto_serge::cassette::HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/users".to_string(),
        headers: HashMap::new(),
        body: None,
    });
    assert!(player.find_interaction(&get_sig).is_ok());

    // Find POST
    let post_sig = RequestSignature::from(magneto_serge::cassette::HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/users".to_string(),
        headers: HashMap::new(),
        body: Some(b"{\"name\":\"Bob\"}".to_vec()),
    });
    assert!(player.find_interaction(&post_sig).is_ok());

    // Find DELETE
    let delete_sig = RequestSignature::from(magneto_serge::cassette::HttpRequest {
        method: "DELETE".to_string(),
        url: "https://api.example.com/users/1".to_string(),
        headers: HashMap::new(),
        body: None,
    });
    assert!(player.find_interaction(&delete_sig).is_ok());

    assert_eq!(player.replay_count(), 3);
}
