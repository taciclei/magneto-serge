package com.magnetoserge.junit;

import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

import static com.magnetoserge.junit.MagnetoAssertions.*;

/**
 * Example tests using Magnéto-Serge JUnit assertions
 */
public class ExampleTest {

    private static HttpClient client;

    @BeforeAll
    public static void setUp() {
        // Configure HTTP client to use Magnéto proxy
        client = HttpClient.newBuilder()
                .proxy(java.net.ProxySelector.of(
                        new java.net.InetSocketAddress("localhost", 8888)
                ))
                .build();

        // Set cassette directory
        MagnetoAssertions.setCassetteDirectory("./cassettes");
    }

    @Test
    public void testUserLogin() throws Exception {
        HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create("http://localhost:8080/api/authenticate"))
                .POST(HttpRequest.BodyPublishers.ofString("{\"username\":\"admin\",\"password\":\"admin\"}"))
                .header("Content-Type", "application/json")
                .build();

        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        // Assert response matches cassette
        assertMatchesCassette(response, "user-login");
    }

    @Test
    public void testUserAccount() throws Exception {
        HttpRequest request = HttpRequest.newBuilder()
                .uri(URI.create("http://localhost:8080/api/account"))
                .GET()
                .build();

        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        // Assert status matches cassette
        assertCassetteStatus(response, "user-account", 200);
    }

    @Test
    public void testInteractionCount() {
        // Assert cassette has expected number of interactions
        assertInteractionCount("user-login", 3);
    }

    @Test
    public void testCassetteCookies() {
        // Assert cassette contains cookies
        assertHasCookies("user-login");
    }

    @Test
    public void testSessionCookie() {
        // Assert cassette has specific cookie
        assertHasCookie("user-login", "JSESSIONID");
    }

    @Test
    public void testCassetteVersion() {
        // Assert cassette version
        assertCassetteVersion("user-login", "2.0");
    }
}
