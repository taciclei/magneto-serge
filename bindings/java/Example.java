package io.github.matgto.serge.examples;

import io.github.matgto.serge.MatgtoProxy;
import io.github.matgto.serge.MatgtoProxy.Mode;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.InetSocketAddress;
import java.net.ProxySelector;

/**
 * Exemple d'utilisation de matgto-serge depuis Java
 */
public class Example {

    public static void main(String[] args) {
        System.out.println("=".repeat(60));
        System.out.println("🧪 Exemple matgto-serge Java");
        System.out.println("=".repeat(60));
        System.out.println();

        // Exemple 1: Utilisation basique
        basicExample();

        System.out.println();

        // Exemple 2: Enregistrement et replay
        recordAndReplayExample();

        System.out.println();
        System.out.println("✅ Tous les exemples terminés !");
    }

    /**
     * Exemple basique: Créer un proxy et le configurer
     */
    public static void basicExample() {
        System.out.println("📝 Exemple 1: Utilisation basique");
        System.out.println("-".repeat(60));

        // Créer un proxy
        MatgtoProxy proxy = MatgtoProxy.create("./cassettes");

        if (proxy == null) {
            System.err.println("❌ Impossible de créer le proxy");
            return;
        }

        System.out.println("✅ Proxy créé: " + proxy);

        // Configurer le port
        proxy.setPort(8888);
        System.out.println("  Port configuré: " + proxy.getPort());

        // Configurer le mode
        proxy.setMode(Mode.RECORD);
        System.out.println("  Mode configuré: " + proxy.getMode());

        // Afficher la version
        System.out.println("  Version: " + MatgtoProxy.getVersion());

        // Shutdown
        proxy.shutdown();
        System.out.println("✅ Proxy arrêté");
    }

    /**
     * Exemple d'enregistrement et replay
     */
    public static void recordAndReplayExample() {
        System.out.println("📝 Exemple 2: Enregistrement et Replay");
        System.out.println("-".repeat(60));

        // Créer un proxy
        MatgtoProxy proxy = new MatgtoProxy("./cassettes");

        // Configuration
        proxy.setPort(8888);
        proxy.setMode(Mode.RECORD);

        // Démarrer l'enregistrement
        String cassetteName = "java_example_test";
        if (proxy.startRecording(cassetteName)) {
            System.out.println("🎬 Enregistrement démarré: " + cassetteName);

            // Simuler des requêtes HTTP via le proxy
            // (Dans un vrai test, vous feriez de vraies requêtes HTTP)
            simulateHttpRequests(proxy.getPort());

            // Arrêter l'enregistrement
            if (proxy.stopRecording()) {
                System.out.println("💾 Enregistrement arrêté et cassette sauvegardée");
            }
        } else {
            System.err.println("❌ Impossible de démarrer l'enregistrement");
        }

        // Passer en mode replay
        proxy.setMode(Mode.REPLAY);

        // Rejouer la cassette
        if (proxy.replay(cassetteName)) {
            System.out.println("▶️  Replay démarré: " + cassetteName);

            // Les requêtes seront maintenant rejouées depuis la cassette
            simulateHttpRequests(proxy.getPort());

            System.out.println("✅ Replay terminé");
        }

        // Shutdown
        proxy.shutdown();
    }

    /**
     * Simule des requêtes HTTP via le proxy
     */
    private static void simulateHttpRequests(int proxyPort) {
        try {
            // Configurer HttpClient avec le proxy matgto
            HttpClient client = HttpClient.newBuilder()
                .proxy(ProxySelector.of(new InetSocketAddress("localhost", proxyPort)))
                .build();

            // Créer une requête
            HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create("https://api.example.com/test"))
                .GET()
                .build();

            System.out.println("  📡 Requête HTTP via proxy (port " + proxyPort + ")");

            // Note: Dans un environnement réel, cette requête passerait par le proxy
            // Pour cet exemple, on simule juste la configuration

        } catch (Exception e) {
            System.err.println("  ⚠️  Simulation requête: " + e.getMessage());
        }
    }
}
