//! Basic hooks example - filtering sensitive data
//!
//! This example demonstrates how to use built-in hooks to filter
//! sensitive data from recorded cassettes.

use magneto_serge::cassette::{HttpRequest, HttpResponse};
use magneto_serge::hooks::builtins::{BodyPatternReplacer, LoggingHook, SensitiveHeaderFilter};
use magneto_serge::recorder::Recorder;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🎯 Magneto-Serge: Basic Hooks Example\n");

    // Create a recorder
    let mut recorder = Recorder::new("hooks_demo".to_string());

    // 1. Add sensitive header filter
    println!("📝 Adding SensitiveHeaderFilter...");
    let mut header_filter = SensitiveHeaderFilter::new();
    header_filter.add_header("x-api-key");
    header_filter.add_header("x-custom-token");
    recorder.add_hook(header_filter);

    // 2. Add body pattern replacer
    println!("📝 Adding BodyPatternReplacer...");
    let mut body_replacer = BodyPatternReplacer::new();

    // Replace passwords in request bodies
    body_replacer.add_pattern(r#""password"\s*:\s*"[^"]*""#, r#""password":"[FILTERED]""#)?;

    // Replace tokens in response bodies
    body_replacer.add_pattern(r#""token"\s*:\s*"[^"]*""#, r#""token":"[FILTERED]""#)?;

    // Replace API keys
    body_replacer.add_pattern(r#""api_key"\s*:\s*"[^"]*""#, r#""api_key":"[FILTERED]""#)?;

    recorder.add_hook(body_replacer);

    // 3. Add logging hook
    println!("📝 Adding LoggingHook (verbose)...\n");
    recorder.add_hook(LoggingHook::new().verbose());

    // Record some interactions
    println!("🎬 Recording interactions...\n");

    // Interaction 1: Login request with sensitive data
    let login_request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/auth/login".to_string(),
        headers: HashMap::from([
            ("content-type".to_string(), "application/json".to_string()),
            (
                "x-api-key".to_string(),
                "super-secret-api-key-12345".to_string(),
            ),
        ]),
        body: Some(br#"{"username":"alice","password":"my-super-secret-password"}"#.to_vec()),
    };

    let login_response = HttpResponse {
        status: 200,
        headers: HashMap::from([
            ("content-type".to_string(), "application/json".to_string()),
            (
                "set-cookie".to_string(),
                "session=abc123xyz; HttpOnly".to_string(),
            ),
        ]),
        body: Some(
            br#"{"token":"jwt-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9","user_id":"123"}"#.to_vec(),
        ),
    };

    println!("➡️  Recording: POST /auth/login");
    recorder.record_http(login_request, login_response);

    // Interaction 2: API call with bearer token
    let api_request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.example.com/users/me".to_string(),
        headers: HashMap::from([
            (
                "authorization".to_string(),
                "Bearer jwt-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string(),
            ),
            (
                "x-custom-token".to_string(),
                "custom-secret-token".to_string(),
            ),
        ]),
        body: None,
    };

    let api_response = HttpResponse {
        status: 200,
        headers: HashMap::from([("content-type".to_string(), "application/json".to_string())]),
        body: Some(br#"{"id":"123","username":"alice","email":"alice@example.com"}"#.to_vec()),
    };

    println!("➡️  Recording: GET /users/me");
    recorder.record_http(api_request, api_response);

    // Save the cassette
    let cassette_dir = PathBuf::from("cassettes");
    std::fs::create_dir_all(&cassette_dir)?;

    println!("\n💾 Saving cassette...");
    recorder.save(&cassette_dir)?;

    println!("✅ Cassette saved to: cassettes/hooks_demo.json");

    // Show what was filtered
    println!("\n🔍 Inspecting recorded cassette...");
    let cassette = recorder.cassette();

    for (idx, interaction) in cassette.interactions.iter().enumerate() {
        if let magneto_serge::cassette::InteractionKind::Http { request, response } =
            &interaction.kind
        {
            println!("\n📦 Interaction #{}", idx + 1);
            println!("   Method: {} {}", request.method, request.url);

            // Show filtered headers
            println!("   Request Headers:");
            for (name, value) in &request.headers {
                if value == "[FILTERED]" {
                    println!("      {} = [FILTERED] ✅", name);
                } else {
                    println!("      {} = {}", name, value);
                }
            }

            // Show filtered body
            if let Some(body) = &request.body {
                let body_str = String::from_utf8_lossy(body);
                println!("   Request Body: {}", body_str);
                if body_str.contains("[FILTERED]") {
                    println!("      ✅ Sensitive data filtered!");
                }
            }

            // Show response headers
            println!("   Response Headers:");
            for (name, value) in &response.headers {
                if value == "[FILTERED]" {
                    println!("      {} = [FILTERED] ✅", name);
                } else {
                    println!("      {} = {}", name, value);
                }
            }

            // Show filtered response body
            if let Some(body) = &response.body {
                let body_str = String::from_utf8_lossy(body);
                println!("   Response Body: {}", body_str);
                if body_str.contains("[FILTERED]") {
                    println!("      ✅ Sensitive data filtered!");
                }
            }
        }
    }

    println!("\n🎉 Done! Check cassettes/hooks_demo.json to see the filtered data.");
    println!(
        "💡 Tip: You can create custom hooks by implementing RecordHook or ReplayHook traits."
    );

    Ok(())
}
