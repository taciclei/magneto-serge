//! Test d'intégration complet pour valider l'API publique

use magneto_serge::{MagnetoProxy, ProxyMode};
use tempfile::TempDir;

#[test]
fn test_proxy_lifecycle() {
    // Créer un répertoire temporaire pour les cassettes
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_dir = temp_dir.path().to_path_buf();

    // Test 1: Création du proxy
    let proxy = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy");

    assert_eq!(proxy.port(), 8888, "Default port should be 8888");
    assert_eq!(proxy.mode(), ProxyMode::Auto, "Default mode should be Auto");

    // Test 2: Configuration du port
    proxy.set_port(9999);
    assert_eq!(proxy.port(), 9999, "Port should be updated to 9999");

    // Test 3: Configuration des modes
    proxy.set_mode(ProxyMode::Record);
    assert_eq!(proxy.mode(), ProxyMode::Record);

    proxy.set_mode(ProxyMode::Replay);
    assert_eq!(proxy.mode(), ProxyMode::Replay);

    proxy.set_mode(ProxyMode::Passthrough);
    assert_eq!(proxy.mode(), ProxyMode::Passthrough);

    proxy.set_mode(ProxyMode::Auto);
    assert_eq!(proxy.mode(), ProxyMode::Auto);

    // Test 4: Builder pattern
    let proxy2 = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy")
        .with_port(7777)
        .with_mode(ProxyMode::Record);

    assert_eq!(proxy2.port(), 7777);
    assert_eq!(proxy2.mode(), ProxyMode::Record);
}

#[test]
fn test_recording_lifecycle() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_dir = temp_dir.path().to_path_buf();

    let proxy = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy");

    // Test démarrage enregistrement
    let result = proxy.start_recording_internal("test-recording".to_string());
    assert!(result.is_ok(), "Should start recording successfully");

    // Attendre un peu
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Test arrêt enregistrement
    let result = proxy.stop_recording_internal();
    assert!(result.is_ok(), "Should stop recording successfully");

    // Vérifier que la cassette existe
    let cassette_file = cassette_dir.join("test-recording.json");
    assert!(cassette_file.exists(), "Cassette file should be created");
}

#[test]
fn test_recording_errors() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_dir = temp_dir.path().to_path_buf();

    let proxy = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy");

    // Test: Arrêter sans avoir commencé
    let result = proxy.stop_recording_internal();
    assert!(result.is_err(), "Should fail when stopping without starting");
    assert!(result.unwrap_err().to_string().contains("No recording in progress"));
}

#[test]
fn test_replay_errors() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_dir = temp_dir.path().to_path_buf();

    let proxy = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy");

    // Test: Replay cassette inexistante
    let result = proxy.replay_internal("nonexistent-cassette".to_string());
    assert!(result.is_err(), "Should fail when replaying nonexistent cassette");
    assert!(result.unwrap_err().to_string().contains("Cassette not found"));
}

#[test]
fn test_multiple_instances() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_dir = temp_dir.path().to_path_buf();

    // Créer plusieurs instances avec des ports différents
    let proxy1 = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy1")
        .with_port(8881);

    let proxy2 = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy2")
        .with_port(8882);

    let proxy3 = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy3")
        .with_port(8883);

    assert_eq!(proxy1.port(), 8881);
    assert_eq!(proxy2.port(), 8882);
    assert_eq!(proxy3.port(), 8883);

    // Tous devraient avoir leur propre configuration
    proxy1.set_mode(ProxyMode::Record);
    proxy2.set_mode(ProxyMode::Replay);
    proxy3.set_mode(ProxyMode::Passthrough);

    assert_eq!(proxy1.mode(), ProxyMode::Record);
    assert_eq!(proxy2.mode(), ProxyMode::Replay);
    assert_eq!(proxy3.mode(), ProxyMode::Passthrough);
}

#[test]
fn test_concurrent_access() {
    use std::sync::Arc;
    use std::thread;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_dir = temp_dir.path().to_path_buf();

    let proxy = Arc::new(
        MagnetoProxy::new_internal(&cassette_dir)
            .expect("Failed to create proxy")
    );

    // Créer plusieurs threads qui accèdent au proxy
    let mut handles = vec![];

    for i in 0..10 {
        let proxy_clone = Arc::clone(&proxy);
        let handle = thread::spawn(move || {
            // Changer le port
            proxy_clone.set_port(9000 + i);
            // Changer le mode
            let modes = [
                ProxyMode::Auto,
                ProxyMode::Record,
                ProxyMode::Replay,
                ProxyMode::Passthrough,
            ];
            proxy_clone.set_mode(modes[i as usize % 4]);
            // Lire les valeurs
            let _port = proxy_clone.port();
            let _mode = proxy_clone.mode();
        });
        handles.push(handle);
    }

    // Attendre tous les threads
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    // Le proxy devrait toujours être valide
    let final_port = proxy.port();
    let final_mode = proxy.mode();

    // Le port et le mode final dépendent du dernier thread
    // mais ils devraient être valides
    assert!(final_port >= 9000 && final_port < 9010);
    assert!(matches!(
        final_mode,
        ProxyMode::Auto | ProxyMode::Record | ProxyMode::Replay | ProxyMode::Passthrough
    ));
}

#[test]
fn test_shutdown_cleanup() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_dir = temp_dir.path().to_path_buf();

    let proxy = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy");

    // Démarrer un enregistrement
    proxy.start_recording_internal("shutdown-test".to_string())
        .expect("Should start recording");

    // Shutdown
    let result = proxy.shutdown_internal();
    assert!(result.is_ok(), "Shutdown should succeed");

    // Après shutdown, on peut encore accéder aux méthodes de lecture
    let _port = proxy.port();
    let _mode = proxy.mode();
}

#[test]
fn test_proxy_modes_equality() {
    assert_eq!(ProxyMode::Auto, ProxyMode::Auto);
    assert_eq!(ProxyMode::Record, ProxyMode::Record);
    assert_eq!(ProxyMode::Replay, ProxyMode::Replay);
    assert_eq!(ProxyMode::Passthrough, ProxyMode::Passthrough);

    assert_ne!(ProxyMode::Auto, ProxyMode::Record);
    assert_ne!(ProxyMode::Record, ProxyMode::Replay);
    assert_ne!(ProxyMode::Replay, ProxyMode::Passthrough);
}

#[test]
fn test_cassette_directory_creation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_dir = temp_dir.path().join("nested").join("cassettes");

    // Le répertoire n'existe pas encore
    assert!(!cassette_dir.exists());

    // Créer le proxy devrait créer le répertoire des certificats
    let _proxy = MagnetoProxy::new_internal(&cassette_dir)
        .expect("Failed to create proxy");

    // Le répertoire parent pour les certs devrait exister
    let ca_dir = cassette_dir.parent().unwrap().join(".magneto").join("certs");
    assert!(ca_dir.exists(), "CA directory should be created");
}
