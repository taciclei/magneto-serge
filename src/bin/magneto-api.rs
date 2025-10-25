//! Magnéto-Serge REST API Server
//!
//! Standalone API server for cassette management.

use anyhow::Result;
use magneto_serge::api::create_router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = create_router()?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("🚀 Magnéto API Server");
    println!("   Listening on http://{}", addr);
    println!("   API docs: http://{}/docs", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
