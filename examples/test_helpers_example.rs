//! Example: Test Helpers
//!
//! Demonstrates how to use Magnéto-Serge test helpers in Rust tests.
//!
//! This example shows various assertion helpers for testing cassettes.

use magneto_serge::test_helpers::*;
use magneto_serge::cassette::InteractionKind;
use magneto_serge::{Cassette, Cookie, HttpRequest, HttpResponse, Interaction};
use chrono::Utc;
use std::collections::HashMap;

fn create_example_cassette() -> Cassette {
    let mut cassette = Cassette::new("example".to_string());

    // Add session cookies
    cassette.cookies = Some(vec![
        Cookie {
            name: "JSESSIONID".to_string(),
            value: "ABC123XYZ".to_string(),
            domain: Some(".example.com".to_string()),
            path: Some("/".to_string()),
            expires: None,
            max_age: Some(3600),
            secure: true,
            http_only: true,
            same_site: None,
            created_at: Utc::now(),
        },
        Cookie {
            name: "XSRF-TOKEN".to_string(),
            value: "token-456".to_string(),
            domain: Some(".example.com".to_string()),
            path: Some("/".to_string()),
            expires: None,
            max_age: Some(3600),
            secure: false,
            http_only: false,
            same_site: Some(magneto_serge::cookies::SameSite::Lax),
            created_at: Utc::now(),
        },
    ]);

    // Add HTTP interactions
    // GET request
    cassette.interactions.push(Interaction {
        kind: InteractionKind::Http {
            request: HttpRequest {
                method: "GET".to_string(),
                url: "https://api.example.com/users".to_string(),
                headers: HashMap::from([
                    ("Accept".to_string(), "application/json".to_string()),
                ]),
                body: None,
            },
            response: HttpResponse {
                status: 200,
                headers: HashMap::from([
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
                body: Some(b"{\"users\":[]}".to_vec()),
            },
        },
        recorded_at: Utc::now(),
        response_time_ms: Some(50),
    });

    // POST request
    cassette.interactions.push(Interaction {
        kind: InteractionKind::Http {
            request: HttpRequest {
                method: "POST".to_string(),
                url: "https://api.example.com/users".to_string(),
                headers: HashMap::from([
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
                body: Some(b"{\"name\":\"John\"}".to_vec()),
            },
            response: HttpResponse {
                status: 201,
                headers: HashMap::from([
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
                body: Some(b"{\"id\":1,\"name\":\"John\"}".to_vec()),
            },
        },
        recorded_at: Utc::now(),
        response_time_ms: Some(75),
    });

    // GET request
    cassette.interactions.push(Interaction {
        kind: InteractionKind::Http {
            request: HttpRequest {
                method: "GET".to_string(),
                url: "https://api.example.com/users/1".to_string(),
                headers: HashMap::new(),
                body: None,
            },
            response: HttpResponse {
                status: 200,
                headers: HashMap::from([
                    ("Content-Type".to_string(), "application/json".to_string()),
                ]),
                body: Some(b"{\"id\":1,\"name\":\"John\"}".to_vec()),
            },
        },
        recorded_at: Utc::now(),
        response_time_ms: Some(45),
    });

    cassette
}

fn main() {
    println!("🧪 Magnéto-Serge Test Helpers Example\n");

    let cassette = create_example_cassette();

    println!("📋 Basic Assertions:");
    println!("  ✓ Cassette version: {}", cassette.version);
    assert_cassette_version(&cassette, "1.0");

    println!("  ✓ Interaction count: {}", cassette.interactions.len());
    assert_interaction_count(&cassette, 3);

    println!("\n🍪 Cookie Assertions:");
    if let Some(cookies) = &cassette.cookies {
        println!("  ✓ Has cookies: {}", cookies.len());
        assert_has_cookies(&cassette);

        for cookie in cookies {
            println!("    - {}", cookie.name);
            assert_has_cookie(&cassette, &cookie.name);
        }
    }

    println!("\n🌐 HTTP Assertions:");
    println!("  ✓ Has HTTP interactions");
    assert_has_http_interactions(&cassette);

    println!("  ✓ GET requests: 2");
    assert_http_method_count(&cassette, "GET", 2);

    println!("  ✓ POST requests: 1");
    assert_http_method_count(&cassette, "POST", 1);

    println!("\n📊 Status Code Assertions:");
    println!("  ✓ 200 responses: 2");
    assert_status_code_count(&cassette, 200, 2);

    println!("  ✓ 201 responses: 1");
    assert_status_code_count(&cassette, 201, 1);

    println!("\n🎯 Macro Usage Example:");
    println!("  Using assert_cassette! macro...");

    // This would typically be in a test
    // assert_cassette!("example", {
    //     version: "1.0",
    //     interactions: 3,
    //     has_cookies: true,
    //     has_cookie: "JSESSIONID",
    //     has_http: true,
    //     http_method: ("GET", 2),
    //     status_code: (200, 2),
    // });

    println!("  ✓ Macro assertions would pass!\n");

    println!("✅ All assertions passed successfully!");
    println!("\nℹ️  In actual tests, use these helpers like:");
    println!("   #[test]");
    println!("   fn test_my_cassette() {{");
    println!("       let cassette = load_cassette(\"my-cassette\").unwrap();");
    println!("       assert_has_cookies(&cassette);");
    println!("       assert_interaction_count(&cassette, 5);");
    println!("   }}");
}
