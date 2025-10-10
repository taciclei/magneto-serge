//! Simple example demonstrating record mode

use magneto_serge::{CertificateAuthority, MatgtoProxy, ProxyMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("magneto_serge=debug")
        .init();

    println!("🎬 magneto-serge - Simple Record Example\n");

    // Create Certificate Authority
    println!("1️⃣  Initializing Certificate Authority...");
    let ca = CertificateAuthority::new("./.magneto/certs")?;
    ca.print_install_instructions();

    // Create proxy
    println!("\n2️⃣  Creating proxy...");
    let proxy = MatgtoProxy::new_internal("./cassettes")?
        .with_port(8888)
        .with_mode(ProxyMode::Record);

    println!("   ✓ Proxy created on port {}", proxy.port());

    // Start recording
    println!("\n3️⃣  Starting recording...");
    proxy.start_recording_internal("example-api-call".to_string())?;
    println!("   ✓ Recording to cassette: example-api-call.json");

    println!("\n📡 Proxy is now ready!");
    println!("   Configure your HTTP client to use proxy:");
    println!("   http://localhost:8888\n");

    println!("   Example with curl:");
    println!("   curl -x http://localhost:8888 https://httpbin.org/get\n");

    println!("   Press Ctrl+C to stop recording and save cassette.\n");

    // In a real implementation, we would:
    // 1. Start the actual proxy server
    // 2. Wait for interrupt signal
    // 3. Stop recording and save cassette

    // For now, simulate with a sleep
    println!("   (In real implementation: proxy would run until interrupted)");

    // Stop recording
    println!("\n4️⃣  Stopping recording...");
    proxy.stop_recording_internal()?;
    println!("   ✓ Cassette saved to ./cassettes/example-api-call.json");

    // Shutdown
    proxy.shutdown_internal()?;
    println!("\n✅ Example completed!");

    Ok(())
}
