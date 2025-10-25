//! Magnéto-Serge REST API Server
//!
//! Standalone API server for cassette management.

use magneto_serge::api::handlers::start_server;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let host = env::var("MAGNETO_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("MAGNETO_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8889);
    let cassette_dir =
        env::var("MAGNETO_CASSETTE_DIR").unwrap_or_else(|_| "./cassettes".to_string());

    println!("🚀 Magnéto-Serge API Server v0.2.0");
    println!("   Listening on http://{}:{}", host, port);
    println!("   Cassette directory: {}", cassette_dir);
    println!("   Health check: http://{}:{}/health", host, port);
    println!();

    start_server(&host, port, cassette_dir).await?;

    Ok(())
}
