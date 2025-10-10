//! Exemple complet de record/replay HTTP
//!
//! Cet exemple démontre :
//! 1. Enregistrement d'une requête HTTP
//! 2. Sauvegarde dans une cassette
//! 3. Replay depuis la cassette

use matgto_serge::{
    MatgtoProxy, ProxyMode, CertificateAuthority,
    cassette::{Cassette, HttpRequest, HttpResponse},
    recorder::Recorder, player::Player,
};
use std::collections::HashMap;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialiser le logging
    tracing_subscriber::fmt()
        .with_env_filter("matgto_serge=info")
        .init();

    println!("🎬 matgto-serge - Exemple Record/Replay HTTP\n");

    // ========== PHASE 1: RECORD ==========
    println!("📹 Phase 1: Enregistrement");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cassette_dir = "./examples/cassettes";
    std::fs::create_dir_all(cassette_dir)?;

    // Créer un enregistreur
    let mut recorder = Recorder::new("api-example".to_string());

    // Simuler une requête HTTP
    let request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.github.com/users/octocat".to_string(),
        headers: {
            let mut h = HashMap::new();
            h.insert("User-Agent".to_string(), "matgto-serge/0.1.0".to_string());
            h.insert("Accept".to_string(), "application/json".to_string());
            h
        },
        body: None,
    };

    // Simuler une réponse HTTP
    let response = HttpResponse {
        status: 200,
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type".to_string(), "application/json".to_string());
            h.insert("X-GitHub-Media-Type".to_string(), "github.v3".to_string());
            h
        },
        body: Some(
            r#"{
  "login": "octocat",
  "id": 1,
  "avatar_url": "https://github.com/images/error/octocat_happy.gif",
  "type": "User",
  "name": "The Octocat",
  "company": "GitHub",
  "blog": "https://github.blog",
  "location": "San Francisco"
}"#
            .as_bytes()
            .to_vec(),
        ),
    };

    println!("  ✓ Requête : {} {}", request.method, request.url);
    println!("  ✓ Réponse : HTTP {}", response.status);

    // Enregistrer l'interaction
    recorder.record_http(request.clone(), response.clone());
    println!("  ✓ Interaction enregistrée\n");

    // Sauvegarder la cassette
    recorder.save(Path::new(cassette_dir))?;
    println!("  ✓ Cassette sauvegardée : {}/api-example.json\n", cassette_dir);

    // ========== PHASE 2: AFFICHER LA CASSETTE ==========
    println!("📼 Phase 2: Contenu de la cassette");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cassette_path = Path::new(cassette_dir).join("api-example.json");
    let cassette_json = std::fs::read_to_string(&cassette_path)?;
    println!("{}\n", cassette_json);

    // ========== PHASE 3: REPLAY ==========
    println!("▶️  Phase 3: Replay depuis la cassette");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Charger la cassette
    let mut player = Player::new();
    player.load(Path::new(cassette_dir), "api-example")?;
    println!("  ✓ Cassette chargée : api-example.json");

    // Créer une requête identique
    let replay_request = HttpRequest {
        method: "GET".to_string(),
        url: "https://api.github.com/users/octocat".to_string(),
        headers: HashMap::new(), // Headers peuvent être différents
        body: None,
    };

    println!("  ✓ Recherche interaction pour : {} {}",
        replay_request.method, replay_request.url);

    // Trouver l'interaction correspondante
    match player.find_interaction(&replay_request) {
        Ok(interaction) => {
            if let Some(recorded_response) = interaction.response() {
                println!("  ✓ Interaction trouvée !\n");
                println!("  Réponse enregistrée :");
                println!("  - Status : {}", recorded_response.status);
                println!("  - Headers : {} headers", recorded_response.headers.len());

                if let Some(body) = &recorded_response.body {
                    println!("  - Body : {} bytes", body.len());

                    // Afficher le body JSON formaté
                    if let Ok(body_str) = String::from_utf8(body.clone()) {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_str) {
                            println!("\n  Body JSON :");
                            println!("  {}", serde_json::to_string_pretty(&json)?);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("  ✗ Erreur : {}", e);
        }
    }

    // ========== PHASE 4: STATISTIQUES ==========
    println!("\n📊 Phase 4: Statistiques");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if let Some(cassette) = player.cassette() {
        println!("  Nom : {}", cassette.name);
        println!("  Version : {}", cassette.version);
        println!("  Enregistré le : {}", cassette.recorded_at);
        println!("  Interactions : {}", cassette.interactions.len());
        println!("  Replays effectués : {}", player.replay_count());
    }

    println!("\n✅ Exemple terminé avec succès !\n");

    // Note pour l'utilisateur
    println!("💡 Pour utiliser le proxy complet :");
    println!("   1. Démarrer le proxy : cargo run --example simple_record");
    println!("   2. Configurer votre client HTTP pour utiliser le proxy");
    println!("   3. Faire vos requêtes normalement");
    println!("   4. Le proxy enregistrera automatiquement\n");

    Ok(())
}
