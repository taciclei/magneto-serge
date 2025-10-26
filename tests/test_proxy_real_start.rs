//! Test vérifiant que le proxy démarre et bind réellement le port

use magneto_serge::MagnetoProxy;
use std::time::Duration;

#[test]
fn test_proxy_starts_and_binds_port() {
    // Configuration
    let cassette_dir = "/tmp/magneto-test-real";
    std::fs::create_dir_all(cassette_dir).unwrap();

    // Créer le proxy
    let proxy = MagnetoProxy::new_internal(cassette_dir).expect("Failed to create proxy");
    proxy.set_port(18888); // Port différent pour éviter conflits

    // Démarrer recording
    println!("🎬 Starting recording...");
    proxy
        .start_recording_internal("test-real".to_string())
        .expect("Failed to start recording");

    // Attendre plus longtemps pour que le thread démarre
    println!("⏳ Waiting 3 seconds for proxy to fully start...");
    std::thread::sleep(Duration::from_secs(3));

    // Vérifier que le port écoute
    println!("🔍 Checking if port 18888 is listening...");

    #[cfg(target_os = "windows")]
    let port_check = std::process::Command::new("netstat")
        .args(["-ano"])
        .output()
        .expect("Failed to run netstat");

    #[cfg(not(target_os = "windows"))]
    let port_check = std::process::Command::new("lsof")
        .args(["-i", ":18888"])
        .output()
        .expect("Failed to run lsof");

    let output = String::from_utf8_lossy(&port_check.stdout);

    #[cfg(target_os = "windows")]
    let port_listening = output
        .lines()
        .any(|line| line.contains(":18888") && line.contains("LISTENING"));

    #[cfg(not(target_os = "windows"))]
    let port_listening = output.contains("LISTEN");

    if port_listening {
        println!("✅ SUCCESS: Port 18888 is listening!");
    } else {
        println!("❌ FAILURE: Port 18888 is NOT listening!");
        #[cfg(target_os = "windows")]
        println!("📊 netstat output:\n{}", output);
        #[cfg(not(target_os = "windows"))]
        println!("📊 lsof output:\n{}", output);

        println!("Stderr: {}", String::from_utf8_lossy(&port_check.stderr));

        panic!("Port not listening after 3 seconds");
    }

    // Test avec une requête HTTP (bloquer le runtime)
    println!("\n🌐 Testing HTTP request through proxy...");
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::http("http://localhost:18888").unwrap())
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();

        match client.get("http://httpbin.org/get").send().await {
            Ok(response) => {
                println!("✅ HTTP request succeeded: status={}", response.status());
            }
            Err(e) => {
                println!("⚠️  HTTP request failed (expected for MITM): {:?}", e);
            }
        }
    });
    // Ne pas dropper le runtime dans un context async
    std::mem::forget(rt);

    // Stop recording
    println!("\n🛑 Stopping recording...");
    proxy
        .stop_recording_internal()
        .expect("Failed to stop recording");

    println!("✅ Test complete!");

    // Cleanup
    std::fs::remove_dir_all(cassette_dir).ok();
}
