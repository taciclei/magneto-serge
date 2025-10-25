//! Advanced hooks example - custom hook implementation
//!
//! This example demonstrates how to create custom hooks for
//! advanced use cases like timestamp normalization and metrics.

use magneto_serge::cassette::{HttpRequest, HttpResponse, Interaction, InteractionKind};
use magneto_serge::error::Result;
use magneto_serge::hooks::{RecordHook, ReplayHook};
use magneto_serge::player::Player;
use magneto_serge::recorder::Recorder;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Custom hook: Normalizes timestamps in request/response bodies
///
/// This is useful for making cassettes deterministic when APIs
/// return timestamps.
#[derive(Debug)]
struct TimestampNormalizer {
    timestamp_pattern: Regex,
    normalized_value: String,
}

impl TimestampNormalizer {
    fn new() -> Self {
        Self {
            // Match ISO 8601 timestamps
            timestamp_pattern: Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?Z?")
                .unwrap(),
            normalized_value: "2025-01-01T00:00:00Z".to_string(),
        }
    }

    fn normalize_body(&self, body: &mut Option<Vec<u8>>) {
        if let Some(bytes) = body {
            if let Ok(text) = String::from_utf8(bytes.clone()) {
                let normalized = self
                    .timestamp_pattern
                    .replace_all(&text, &self.normalized_value);
                *bytes = normalized.to_string().into_bytes();
            }
        }
    }
}

impl RecordHook for TimestampNormalizer {
    fn before_record(&self, interaction: &mut Interaction) -> Result<()> {
        if let InteractionKind::Http { request, response } = &mut interaction.kind {
            self.normalize_body(&mut request.body);
            self.normalize_body(&mut response.body);
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "TimestampNormalizer"
    }
}

/// Custom hook: Collects metrics about recorded/replayed interactions
#[derive(Debug, Clone, Default)]
struct MetricsCollector {
    stats: Arc<Mutex<InteractionStats>>,
}

#[derive(Debug, Default)]
struct InteractionStats {
    total_recorded: usize,
    total_replayed: usize,
    methods: HashMap<String, usize>,
    status_codes: HashMap<u16, usize>,
    total_request_bytes: usize,
    total_response_bytes: usize,
}

impl MetricsCollector {
    fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(InteractionStats::default())),
        }
    }

    fn print_stats(&self) {
        let stats = self.stats.lock().unwrap();
        println!("\n📊 Interaction Metrics:");
        println!("   Total recorded: {}", stats.total_recorded);
        println!("   Total replayed: {}", stats.total_replayed);
        println!("\n   HTTP Methods:");
        for (method, count) in &stats.methods {
            println!("      {} = {}", method, count);
        }
        println!("\n   Status Codes:");
        for (code, count) in &stats.status_codes {
            println!("      {} = {}", code, count);
        }
        println!("\n   Data Transfer:");
        println!(
            "      Request bytes:  {}",
            format_bytes(stats.total_request_bytes)
        );
        println!(
            "      Response bytes: {}",
            format_bytes(stats.total_response_bytes)
        );
    }
}

impl RecordHook for MetricsCollector {
    fn after_record(&self, interaction: &Interaction) -> Result<()> {
        let mut stats = self.stats.lock().unwrap();
        stats.total_recorded += 1;

        if let InteractionKind::Http { request, response } = &interaction.kind {
            // Count method
            *stats.methods.entry(request.method.clone()).or_insert(0) += 1;

            // Count status code
            *stats.status_codes.entry(response.status).or_insert(0) += 1;

            // Count bytes
            if let Some(body) = &request.body {
                stats.total_request_bytes += body.len();
            }
            if let Some(body) = &response.body {
                stats.total_response_bytes += body.len();
            }
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "MetricsCollector"
    }
}

impl ReplayHook for MetricsCollector {
    fn after_replay(&self, interaction: &Interaction) -> Result<()> {
        let mut stats = self.stats.lock().unwrap();
        stats.total_replayed += 1;

        if let InteractionKind::Http { request, .. } = &interaction.kind {
            *stats.methods.entry(request.method.clone()).or_insert(0) += 1;
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "MetricsCollector"
    }
}

/// Custom hook: Validates response schemas
#[derive(Debug)]
struct ResponseSchemaValidator {
    expected_fields: Vec<String>,
}

impl ResponseSchemaValidator {
    fn new(fields: Vec<String>) -> Self {
        Self {
            expected_fields: fields,
        }
    }
}

impl RecordHook for ResponseSchemaValidator {
    fn before_record(&self, interaction: &mut Interaction) -> Result<()> {
        if let InteractionKind::Http { response, .. } = &interaction.kind {
            if let Some(body) = &response.body {
                if let Ok(text) = String::from_utf8(body.clone()) {
                    // Simple JSON validation (production should use serde_json)
                    for field in &self.expected_fields {
                        if !text.contains(&format!("\"{}\"", field)) {
                            tracing::warn!("⚠️  Response missing expected field: {}", field);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "ResponseSchemaValidator"
    }
}

fn format_bytes(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;

    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🎯 Magneto-Serge: Advanced Hooks Example\n");

    // Create a recorder with custom hooks
    let mut recorder = Recorder::new("advanced_hooks".to_string());

    // Add timestamp normalizer
    println!("📝 Adding TimestampNormalizer hook...");
    recorder.add_hook(TimestampNormalizer::new());

    // Add metrics collector
    println!("📝 Adding MetricsCollector hook...");
    let metrics = MetricsCollector::new();
    recorder.add_hook(metrics.clone());

    // Add schema validator
    println!("📝 Adding ResponseSchemaValidator hook...");
    recorder.add_hook(ResponseSchemaValidator::new(vec![
        "id".to_string(),
        "created_at".to_string(),
    ]));

    println!("\n🎬 Recording interactions with custom hooks...\n");

    // Record an interaction with timestamps
    let request = HttpRequest {
        method: "POST".to_string(),
        url: "https://api.example.com/posts".to_string(),
        headers: HashMap::from([("content-type".to_string(), "application/json".to_string())]),
        body: Some(
            format!(
                r#"{{"title":"Hello","body":"World","timestamp":"{}"}}"#,
                chrono::Utc::now().to_rfc3339()
            )
            .into_bytes(),
        ),
    };

    let response = HttpResponse {
        status: 201,
        headers: HashMap::from([("content-type".to_string(), "application/json".to_string())]),
        body: Some(
            format!(
                r#"{{"id":"123","title":"Hello","created_at":"{}","updated_at":"{}"}}"#,
                chrono::Utc::now().to_rfc3339(),
                chrono::Utc::now().to_rfc3339()
            )
            .into_bytes(),
        ),
    };

    println!("➡️  Recording: POST /posts");
    recorder.record_http(request, response);

    // Record more interactions
    for i in 1..=3 {
        let request = HttpRequest {
            method: "GET".to_string(),
            url: format!("https://api.example.com/posts/{}", i),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::from([("content-type".to_string(), "application/json".to_string())]),
            body: Some(
                format!(
                    r#"{{"id":"{}","title":"Post {}","created_at":"{}"}}"#,
                    i,
                    i,
                    chrono::Utc::now().to_rfc3339()
                )
                .into_bytes(),
            ),
        };

        println!("➡️  Recording: GET /posts/{}", i);
        recorder.record_http(request, response);
    }

    // Save cassette
    let cassette_dir = PathBuf::from("cassettes");
    std::fs::create_dir_all(&cassette_dir)?;

    println!("\n💾 Saving cassette...");
    recorder.save(&cassette_dir)?;

    // Print metrics
    metrics.print_stats();

    // Verify timestamp normalization
    println!("\n🔍 Verifying timestamp normalization...");
    let cassette = recorder.cassette();

    for (idx, interaction) in cassette.interactions.iter().enumerate() {
        if let InteractionKind::Http { response, .. } = &interaction.kind {
            if let Some(body) = &response.body {
                let body_str = String::from_utf8_lossy(body);
                if body_str.contains("2025-01-01T00:00:00Z") {
                    println!("   ✅ Interaction #{}: Timestamps normalized", idx + 1);
                } else if body_str.contains("created_at") {
                    println!("   ❌ Interaction #{}: Timestamps NOT normalized", idx + 1);
                }
            }
        }
    }

    // Demonstrate replay with hooks
    println!("\n🔁 Now testing replay with hooks...\n");

    let mut player = Player::load(&cassette_dir, "advanced_hooks")?;
    player.add_hook(metrics.clone());

    println!(
        "   Simulating replay of {} interactions...",
        cassette.interactions.len()
    );
    for i in 0..cassette.interactions.len() {
        if let Ok(interaction) = player.get_interaction_with_hooks(i) {
            player.mark_replayed(&interaction)?;
        }
    }

    // Print final metrics
    metrics.print_stats();

    println!("\n🎉 Done! Custom hooks demonstrated successfully.");
    println!("💡 Use cases for custom hooks:");
    println!("   - Normalizing dynamic data (timestamps, IDs, UUIDs)");
    println!("   - Collecting metrics and telemetry");
    println!("   - Validating API contracts");
    println!("   - Logging and debugging");
    println!("   - Custom data transformations");

    Ok(())
}
