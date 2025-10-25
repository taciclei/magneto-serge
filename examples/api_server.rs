//! Example: REST API Server
//!
//! Demonstrates how to start the REST API server for cassette management.
//!
//! Usage:
//!   cargo run --example api_server --features api
//!
//! Then test with:
//!   curl http://localhost:8889/health
//!   curl http://localhost:8889/cassettes

use magneto_serge::api::handlers::start_server;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cassette_dir = PathBuf::from("./cassettes");
    let host = "127.0.0.1";
    let port = 8889;

    println!("🚀 Starting Magnéto-Serge API Server");
    println!("📂 Cassette directory: {}", cassette_dir.display());
    println!("🌐 Server address: http://{}:{}", host, port);
    println!("\n📋 Available endpoints:");
    println!("  GET  /health                      - Health check");
    println!("  GET  /cassettes                   - List all cassettes");
    println!("  GET  /cassettes/:name             - Get cassette metadata");
    println!("  GET  /cassettes/:name/stats       - Get cassette statistics");
    println!("  GET  /cassettes/:name/validate    - Validate cassette");
    println!("  DELETE /cassettes/:name           - Delete cassette");
    println!("  POST /cassettes/:name/export      - Export cassette");
    println!("  GET  /cassettes/stats             - Global statistics");
    println!("\n⚡ Press Ctrl+C to stop the server\n");

    start_server(host, port, cassette_dir).await?;

    Ok(())
}
