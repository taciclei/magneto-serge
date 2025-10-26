//! Integration tests for template rendering feature
//!
//! These tests verify that the template engine works correctly with
//! real cassettes containing template syntax.

#[cfg(feature = "templates")]
mod templates_integration {
    use magneto_serge::{
        cassette::{HttpRequest, HttpResponse, InteractionKind},
        Player, Recorder,
    };
    use std::collections::HashMap;
    use tempfile::tempdir;

    /// Test basic template rendering with environment variables
    #[test]
    fn test_template_env_variable_in_cassette() {
        // Set up environment
        std::env::set_var("TEST_API_KEY", "secret-key-12345");

        // Create a cassette with template in response body
        let mut recorder = Recorder::new("test-env-template".to_string());

        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/auth".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(r#"{"api_key":"{{ env "TEST_API_KEY" }}"}"#.as_bytes().to_vec()),
        };

        recorder.record_http(request.clone(), response);

        // Save cassette
        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        // Load and replay with template rendering
        let player = Player::load(dir.path(), "test-env-template").unwrap();

        let idx = player.cassette().unwrap().interactions.len() - 1;
        let interaction = player.get_interaction(idx).unwrap();

        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction.kind.clone()
        {
            // Render templates
            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();

            // Verify template was rendered
            let body = String::from_utf8(resp.body.unwrap()).unwrap();
            assert_eq!(body, r#"{"api_key":"secret-key-12345"}"#);
        } else {
            panic!("Expected HTTP interaction");
        }

        std::env::remove_var("TEST_API_KEY");
    }

    /// Test template rendering with dynamic timestamps
    #[test]
    fn test_template_timestamp_in_cassette() {
        let mut recorder = Recorder::new("test-timestamp-template".to_string());

        let request = HttpRequest {
            method: "POST".to_string(),
            url: "https://api.example.com/events".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 201,
            headers: HashMap::new(),
            body: Some(
                r#"{"event_id":"evt_123","created_at":"{{ now }}","timestamp":{{ now_timestamp }}}"#
                    .as_bytes()
                    .to_vec(),
            ),
        };

        recorder.record_http(request.clone(), response);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        let player = Player::load(dir.path(), "test-timestamp-template").unwrap();

        let idx = player.cassette().unwrap().interactions.len() - 1;
        let interaction = player.get_interaction(idx).unwrap();

        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction.kind.clone()
        {
            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();

            let body = String::from_utf8(resp.body.unwrap()).unwrap();

            // Verify timestamps were generated
            assert!(body.contains("\"created_at\":\""));
            assert!(body.contains("T")); // ISO 8601 format contains T
            assert!(body.contains("\"timestamp\":"));
            assert!(body.contains("evt_123")); // Static content preserved
        } else {
            panic!("Expected HTTP interaction");
        }
    }

    /// Test template rendering with UUID generation
    #[test]
    fn test_template_uuid_in_cassette() {
        let mut recorder = Recorder::new("test-uuid-template".to_string());

        let request = HttpRequest {
            method: "POST".to_string(),
            url: "https://api.example.com/resources".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 201,
            headers: HashMap::new(),
            body: Some(r#"{"resource_id":"{{ uuid }}","status":"created"}"#.as_bytes().to_vec()),
        };

        recorder.record_http(request.clone(), response);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        let player = Player::load(dir.path(), "test-uuid-template").unwrap();

        let idx = player.cassette().unwrap().interactions.len() - 1;
        let interaction = player.get_interaction(idx).unwrap();

        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction.kind.clone()
        {
            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();

            let body = String::from_utf8(resp.body.unwrap()).unwrap();

            // Verify UUID was generated (length check for UUID format)
            assert!(body.contains("\"resource_id\":\""));
            assert!(body.contains("\"status\":\"created\""));
            // UUID is 36 characters (8-4-4-4-12 with hyphens)
            assert!(body.len() > 50); // At least contains UUID + JSON structure
        } else {
            panic!("Expected HTTP interaction");
        }
    }

    /// Test template rendering with request context (headers)
    #[test]
    fn test_template_request_headers_in_cassette() {
        let mut recorder = Recorder::new("test-request-headers-template".to_string());

        let mut headers = HashMap::new();
        headers.insert("x-user-id".to_string(), "user-12345".to_string());
        headers.insert("x-session-id".to_string(), "sess-abcde".to_string());

        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/profile".to_string(),
            headers: headers.clone(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(
                r#"{"user_id":"{{ request.headers.x-user-id }}","session":"{{ request.headers.x-session-id }}"}"#
                    .as_bytes()
                    .to_vec(),
            ),
        };

        recorder.record_http(request.clone(), response);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        let player = Player::load(dir.path(), "test-request-headers-template").unwrap();

        let idx = player.cassette().unwrap().interactions.len() - 1;
        let interaction = player.get_interaction(idx).unwrap();

        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction.kind.clone()
        {
            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();

            let body = String::from_utf8(resp.body.unwrap()).unwrap();

            // Verify request headers were substituted
            assert_eq!(
                body,
                r#"{"user_id":"user-12345","session":"sess-abcde"}"#
            );
        } else {
            panic!("Expected HTTP interaction");
        }
    }

    /// Test complex template with multiple features
    #[test]
    fn test_template_complex_cassette() {
        std::env::set_var("API_BASE_URL", "https://api.example.com");

        let mut recorder = Recorder::new("test-complex-template".to_string());

        let mut headers = HashMap::new();
        headers.insert("x-request-id".to_string(), "req-xyz".to_string());

        let request = HttpRequest {
            method: "POST".to_string(),
            url: "https://api.example.com/webhooks".to_string(),
            headers: headers.clone(),
            body: Some(b"{\"event\":\"user.created\"}".to_vec()),
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(
                r#"{
  "webhook_id": "{{ uuid }}",
  "request_id": "{{ request.headers.x-request-id }}",
  "api_url": "{{ env "API_BASE_URL" }}",
  "timestamp": "{{ now }}",
  "unix_time": {{ now_timestamp }},
  "method": "{{ request.method }}",
  "url": "{{ request.url }}"
}"#
                .as_bytes()
                .to_vec(),
            ),
        };

        recorder.record_http(request.clone(), response);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        let player = Player::load(dir.path(), "test-complex-template").unwrap();

        let idx = player.cassette().unwrap().interactions.len() - 1;
        let interaction = player.get_interaction(idx).unwrap();

        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction.kind.clone()
        {
            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();

            let body = String::from_utf8(resp.body.unwrap()).unwrap();

            // Verify all template features were rendered
            assert!(body.contains("\"webhook_id\": \""));
            assert!(body.contains("\"request_id\": \"req-xyz\""));
            assert!(body.contains("\"api_url\": \"https://api.example.com\""));
            assert!(body.contains("\"timestamp\": \""));
            assert!(body.contains("T")); // ISO timestamp
            assert!(body.contains("\"unix_time\": "));
            assert!(body.contains("\"method\": \"POST\""));
            assert!(body.contains("\"url\": \"https://api.example.com/webhooks\""));
        } else {
            panic!("Expected HTTP interaction");
        }

        std::env::remove_var("API_BASE_URL");
    }

    /// Test that non-template responses are passed through unchanged
    #[test]
    fn test_no_template_passthrough() {
        let mut recorder = Recorder::new("test-no-template".to_string());

        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/static".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(b"{\"message\":\"Hello, World!\"}".to_vec()),
        };

        recorder.record_http(request.clone(), response);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        let player = Player::load(dir.path(), "test-no-template").unwrap();

        let idx = player.cassette().unwrap().interactions.len() - 1;
        let interaction = player.get_interaction(idx).unwrap();

        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction.kind.clone()
        {
            let original_body = resp.body.clone();

            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();

            // Verify body is unchanged
            assert_eq!(resp.body, original_body);
            assert_eq!(
                String::from_utf8(resp.body.unwrap()).unwrap(),
                r#"{"message":"Hello, World!"}"#
            );
        } else {
            panic!("Expected HTTP interaction");
        }
    }

    /// Test custom helper registration
    #[test]
    fn test_custom_helper_in_cassette() {
        let mut recorder = Recorder::new("test-custom-helper".to_string());

        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/custom".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(r#"{"custom":"{{ my_helper }}","standard":"{{ uuid }}"}"#.as_bytes().to_vec()),
        };

        recorder.record_http(request.clone(), response);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        let mut player = Player::load(dir.path(), "test-custom-helper").unwrap();

        // Register custom helper
        player
            .template_engine_mut()
            .register_helper("my_helper", || "custom-value-xyz".to_string());

        let idx = player.cassette().unwrap().interactions.len() - 1;
        let interaction = player.get_interaction(idx).unwrap();

        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction.kind.clone()
        {
            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();

            let body = String::from_utf8(resp.body.unwrap()).unwrap();

            // Verify custom helper was used
            assert!(body.contains("\"custom\":\"custom-value-xyz\""));
            assert!(body.contains("\"standard\":")); // UUID still works
        } else {
            panic!("Expected HTTP interaction");
        }
    }

    /// Test template rendering with multiple interactions in same cassette
    #[test]
    fn test_multiple_interactions_with_templates() {
        std::env::set_var("ENV_VAR_1", "value1");
        std::env::set_var("ENV_VAR_2", "value2");

        let mut recorder = Recorder::new("test-multi-templates".to_string());

        // First interaction
        let request1 = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/endpoint1".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response1 = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(r#"{"value":"{{ env "ENV_VAR_1" }}"}"#.as_bytes().to_vec()),
        };

        recorder.record_http(request1.clone(), response1);

        // Second interaction
        let request2 = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/endpoint2".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response2 = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(r#"{"value":"{{ env "ENV_VAR_2" }}","id":"{{ uuid }}"}"#.as_bytes().to_vec()),
        };

        recorder.record_http(request2.clone(), response2);

        // Third interaction (no template)
        let request3 = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/endpoint3".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response3 = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(b"{\"value\":\"static\"}".to_vec()),
        };

        recorder.record_http(request3.clone(), response3);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        let player = Player::load(dir.path(), "test-multi-templates").unwrap();

        // Test first interaction
        let interaction1 = player.get_interaction(0).unwrap();
        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction1.kind.clone()
        {
            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();
            assert_eq!(
                String::from_utf8(resp.body.unwrap()).unwrap(),
                r#"{"value":"value1"}"#
            );
        }

        // Test second interaction
        let interaction2 = player.get_interaction(1).unwrap();
        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction2.kind.clone()
        {
            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();
            let body = String::from_utf8(resp.body.unwrap()).unwrap();
            assert!(body.contains("\"value\":\"value2\""));
            assert!(body.contains("\"id\":\"")); // UUID generated
        }

        // Test third interaction (no template)
        let interaction3 = player.get_interaction(2).unwrap();
        if let InteractionKind::Http {
            request: req,
            response: mut resp,
        } = interaction3.kind.clone()
        {
            player
                .render_templates_in_response(&req, &mut resp)
                .unwrap();
            assert_eq!(
                String::from_utf8(resp.body.unwrap()).unwrap(),
                r#"{"value":"static"}"#
            );
        }

        std::env::remove_var("ENV_VAR_1");
        std::env::remove_var("ENV_VAR_2");
    }
}

/// Tests when templates feature is disabled
#[cfg(not(feature = "templates"))]
mod templates_disabled {
    use magneto_serge::TemplateEngine;
    use std::collections::HashMap;

    #[test]
    fn test_template_engine_stub() {
        // Template engine should exist but do nothing
        let engine = TemplateEngine::new();

        // Has_templates should always return false
        assert!(!TemplateEngine::has_templates("{{ env \"TEST\" }}"));
        assert!(!TemplateEngine::has_templates("{{ now }}"));

        // Render should pass through unchanged
        let request = magneto_serge::cassette::HttpRequest {
            method: "GET".to_string(),
            url: "https://example.com".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let template = "{{ env \"TEST\" }}";
        let result = engine.render(template, &request).unwrap();
        assert_eq!(result, template); // Unchanged
    }
}
