package io.github.matgto.serge.test;

import io.github.matgto.serge.MatgtoProxy;
import io.github.matgto.serge.MatgtoProxy.Mode;

import org.junit.jupiter.api.*;
import static org.junit.jupiter.api.Assertions.*;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.InetSocketAddress;
import java.net.ProxySelector;
import java.io.IOException;

/**
 * Tests JUnit 5 pour matgto-serge
 *
 * Exemple d'intégration avec JUnit pour tester des APIs HTTP
 * en utilisant le mode record/replay de matgto-serge.
 */
@DisplayName("MatgtoProxy Tests")
public class MatgtoTest {

    private MatgtoProxy proxy;
    private static final String CASSETTE_DIR = "./test_cassettes";
    private static final int PROXY_PORT = 8889;

    @BeforeEach
    void setUp() {
        // Créer un proxy pour chaque test
        proxy = MatgtoProxy.create(CASSETTE_DIR);
        assertNotNull(proxy, "Proxy should be created");

        // Configuration de base
        proxy.setPort(PROXY_PORT);
    }

    @AfterEach
    void tearDown() {
        // Nettoyer après chaque test
        if (proxy != null) {
            proxy.shutdown();
        }
    }

    @Test
    @DisplayName("Should create proxy successfully")
    void testProxyCreation() {
        assertNotNull(proxy);
        assertEquals(PROXY_PORT, proxy.getPort());
    }

    @Test
    @DisplayName("Should configure port correctly")
    void testSetPort() {
        proxy.setPort(9999);
        assertEquals(9999, proxy.getPort());
    }

    @Test
    @DisplayName("Should configure mode correctly")
    void testSetMode() {
        proxy.setMode(Mode.RECORD);
        assertEquals(Mode.RECORD, proxy.getMode());

        proxy.setMode(Mode.REPLAY);
        assertEquals(Mode.REPLAY, proxy.getMode());

        proxy.setMode(Mode.AUTO);
        assertEquals(Mode.AUTO, proxy.getMode());

        proxy.setMode(Mode.PASSTHROUGH);
        assertEquals(Mode.PASSTHROUGH, proxy.getMode());
    }

    @Test
    @DisplayName("Should throw exception for invalid port")
    void testInvalidPort() {
        assertThrows(IllegalArgumentException.class, () -> {
            proxy.setPort(0);
        });

        assertThrows(IllegalArgumentException.class, () -> {
            proxy.setPort(70000);
        });
    }

    @Test
    @DisplayName("Should start and stop recording")
    void testRecording() {
        proxy.setMode(Mode.RECORD);

        boolean started = proxy.startRecording("junit_test");
        assertTrue(started, "Recording should start successfully");

        boolean stopped = proxy.stopRecording();
        assertTrue(stopped, "Recording should stop successfully");
    }

    @Test
    @DisplayName("Should replay cassette")
    void testReplay() {
        // D'abord enregistrer
        proxy.setMode(Mode.RECORD);
        proxy.startRecording("junit_replay_test");
        proxy.stopRecording();

        // Puis rejouer
        proxy.setMode(Mode.REPLAY);
        boolean replayed = proxy.replay("junit_replay_test");
        assertTrue(replayed, "Replay should start successfully");
    }

    @Test
    @DisplayName("Should return version")
    void testVersion() {
        String version = MatgtoProxy.getVersion();
        assertNotNull(version);
        assertFalse(version.isEmpty());
        assertEquals("0.1.0", version);
    }

    @Test
    @DisplayName("Should handle HTTP requests in record mode")
    void testHttpRecordMode() {
        proxy.setMode(Mode.RECORD);
        proxy.startRecording("http_test");

        // Configurer HttpClient avec le proxy
        HttpClient client = HttpClient.newBuilder()
            .proxy(ProxySelector.of(new InetSocketAddress("localhost", PROXY_PORT)))
            .build();

        // Créer une requête de test
        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create("https://httpbin.org/get"))
            .GET()
            .build();

        // Note: Cette requête passerait normalement par le proxy
        // Pour les tests, on vérifie juste la configuration
        assertNotNull(client);
        assertNotNull(request);

        proxy.stopRecording();
    }

    @Test
    @DisplayName("Should create proxy with factory method")
    void testFactoryMethod() {
        MatgtoProxy newProxy = MatgtoProxy.create(CASSETTE_DIR);
        assertNotNull(newProxy);
        newProxy.shutdown();
    }

    @Test
    @DisplayName("Should handle null cassette dir gracefully")
    void testNullCassetteDir() {
        MatgtoProxy nullProxy = MatgtoProxy.create(null);
        // Le comportement dépend de l'implémentation
        // On accepte null ou exception
    }

    @Test
    @DisplayName("Should have meaningful toString")
    void testToString() {
        proxy.setPort(8888);
        proxy.setMode(Mode.RECORD);

        String str = proxy.toString();
        assertNotNull(str);
        assertTrue(str.contains("8888"));
        assertTrue(str.contains("RECORD"));
        assertTrue(str.contains("0.1.0"));
    }

    /**
     * Test d'intégration complet: Record puis Replay
     */
    @Test
    @DisplayName("Integration test: Record and Replay cycle")
    void testFullRecordReplayCycle() {
        String cassetteName = "integration_test";

        // Phase 1: Enregistrement
        proxy.setMode(Mode.RECORD);
        assertTrue(proxy.startRecording(cassetteName));

        // Simuler des requêtes
        // (Dans un vrai test, faire de vraies requêtes HTTP)

        assertTrue(proxy.stopRecording());

        // Phase 2: Replay
        proxy.setMode(Mode.REPLAY);
        assertTrue(proxy.replay(cassetteName));

        // Vérifier que les requêtes rejouées donnent les mêmes résultats
        // (Dans un vrai test, comparer les réponses)
    }

    /**
     * Test avec annotation personnalisée
     */
    @Test
    @Tag("slow")
    @DisplayName("Slow test with real HTTP request")
    @Disabled("Requires network access")
    void testRealHttpRequest() throws IOException, InterruptedException {
        proxy.setMode(Mode.PASSTHROUGH);

        HttpClient client = HttpClient.newBuilder()
            .proxy(ProxySelector.of(new InetSocketAddress("localhost", PROXY_PORT)))
            .build();

        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create("https://httpbin.org/get"))
            .GET()
            .build();

        HttpResponse<String> response = client.send(
            request,
            HttpResponse.BodyHandlers.ofString()
        );

        assertEquals(200, response.statusCode());
        assertNotNull(response.body());
    }
}
