//! Example: Hydra Hypermedia API Server
//!
//! Demonstrates the Hydra/JSON-LD hypermedia API with auto-discoverable endpoints.
//!
//! Usage:
//!   cargo run --example hydra_api_server --features hydra
//!
//! Then test with:
//!   curl http://localhost:8889/api
//!   curl http://localhost:8889/api/cassettes
//!   curl http://localhost:8889/api/cassettes?page=1&limit=10
//!   curl http://localhost:8889/vocab
//!
//! With JSON-LD context:
//!   curl -H "Accept: application/ld+json" http://localhost:8889/api

use magneto_serge::api::{ApiConfig, ApiServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 8889,
        proxy_port: 8888,
        cassette_dir: "./cassettes".to_string(),
        auth_enabled: false,
        api_key: None,
    };

    println!("🚀 Starting Magnéto-Serge Hydra API Server");
    println!("📂 Cassette directory: {}", config.cassette_dir);
    println!("🌐 Server address: http://{}:{}", config.host, config.port);
    println!("\n📋 Hydra Hypermedia Endpoints:");
    println!("  GET  /api                              - API Documentation (auto-discoverable)");
    println!("  GET  /api/cassettes                    - Cassettes Collection (paginated)");
    println!("  GET  /api/cassettes?page=1&limit=20    - Paginated cassettes");
    println!("  GET  /api/cassettes/:name              - Single cassette resource");
    println!("  GET  /api/cassettes/:name/interactions - Interactions collection (paginated)");
    println!("  GET  /api/cassettes/:name/interactions/:id - Single interaction");
    println!("  GET  /api/templates                    - Available Handlebars helpers");
    println!("  GET  /vocab                            - Magneto RDF vocabulary");
    println!("\n🔗 Features:");
    println!("  ✅ HATEOAS - Follow links to navigate the API");
    println!("  ✅ JSON-LD - Linked Data with semantic context");
    println!("  ✅ Hydra Core Vocabulary - W3C standard");
    println!("  ✅ Pagination - HydraView with first/prev/next/last");
    println!("  ✅ Content Negotiation - application/ld+json, application/json");
    println!("\n💡 Try with httpie:");
    println!("  http :8889/api Accept:application/ld+json");
    println!("  http :8889/api/cassettes page==1 limit==5");
    println!("\n⚡ Press Ctrl+C to stop the server\n");

    let server = ApiServer::new(config);

    #[cfg(feature = "hydra")]
    {
        server.start_with_hydra().await?;
    }

    #[cfg(not(feature = "hydra"))]
    {
        eprintln!("❌ Error: This example requires the 'hydra' feature flag");
        eprintln!("Run with: cargo run --example hydra_api_server --features hydra");
        std::process::exit(1);
    }

    Ok(())
}
