package com.magnetoserge.junit;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Assertions;

import java.io.File;
import java.io.IOException;
import java.net.http.HttpResponse;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;

/**
 * Custom JUnit assertions for Magnéto-Serge cassettes
 *
 * Usage:
 * <pre>
 * import static com.magnetoserge.junit.MagnetoAssertions.*;
 *
 * {@literal @}Test
 * public void testUserLogin() {
 *     HttpResponse&lt;String&gt; response = client.send(request, HttpResponse.BodyHandlers.ofString());
 *     assertMatchesCassette(response, "user-login");
 * }
 * </pre>
 */
public class MagnetoAssertions {

    private static final ObjectMapper objectMapper = new ObjectMapper();
    private static String cassetteDir = "./cassettes";

    /**
     * Set the directory where cassettes are stored
     * @param dir Cassette directory path
     */
    public static void setCassetteDirectory(String dir) {
        cassetteDir = dir;
    }

    /**
     * Load a cassette from disk
     * @param name Cassette name (without extension)
     * @return Parsed cassette as JsonNode
     * @throws IOException if cassette cannot be read
     */
    private static JsonNode loadCassette(String name) throws IOException {
        Path jsonPath = Paths.get(cassetteDir, name + ".json");
        Path msgpackPath = Paths.get(cassetteDir, name + ".msgpack");

        Path cassettePath;
        if (Files.exists(jsonPath)) {
            cassettePath = jsonPath;
        } else if (Files.exists(msgpackPath)) {
            throw new UnsupportedOperationException("MessagePack cassettes not yet supported");
        } else {
            throw new IOException("Cassette not found: " + name);
        }

        String content = Files.readString(cassettePath);
        return objectMapper.readTree(content);
    }

    /**
     * Assert that HTTP response matches a cassette
     * @param response HTTP response to validate
     * @param cassetteName Name of cassette to match against
     */
    public static void assertMatchesCassette(HttpResponse<?> response, String cassetteName) {
        try {
            JsonNode cassette = loadCassette(cassetteName);
            String method = response.request().method();
            String url = response.request().uri().toString();

            JsonNode interaction = findMatchingInteraction(cassette, method, url);

            if (interaction == null) {
                Assertions.fail(String.format(
                    "No matching interaction found in cassette '%s' for %s %s",
                    cassetteName, method, url
                ));
            }

            // Validate status code
            int expectedStatus = interaction.get("kind").get("Http").get("response").get("status").asInt();
            int actualStatus = response.statusCode();

            Assertions.assertEquals(expectedStatus, actualStatus,
                String.format("Status code mismatch in cassette '%s'", cassetteName));

        } catch (IOException e) {
            Assertions.fail("Failed to load cassette: " + e.getMessage());
        }
    }

    /**
     * Assert that HTTP response status matches cassette
     * @param response HTTP response to validate
     * @param cassetteName Name of cassette
     * @param expectedStatus Expected status code
     */
    public static void assertCassetteStatus(HttpResponse<?> response, String cassetteName, int expectedStatus) {
        try {
            JsonNode cassette = loadCassette(cassetteName);
            String method = response.request().method();
            String url = response.request().uri().toString();

            JsonNode interaction = findMatchingInteraction(cassette, method, url);

            if (interaction == null) {
                Assertions.fail(String.format(
                    "No matching interaction found in cassette '%s'",
                    cassetteName
                ));
            }

            int cassetteStatus = interaction.get("kind").get("Http").get("response").get("status").asInt();

            Assertions.assertEquals(expectedStatus, cassetteStatus,
                String.format("Expected status %d but cassette has %d", expectedStatus, cassetteStatus));

        } catch (IOException e) {
            Assertions.fail("Failed to load cassette: " + e.getMessage());
        }
    }

    /**
     * Assert that cassette has expected number of interactions
     * @param cassetteName Name of cassette
     * @param expectedCount Expected interaction count
     */
    public static void assertInteractionCount(String cassetteName, int expectedCount) {
        try {
            JsonNode cassette = loadCassette(cassetteName);
            int actualCount = cassette.get("interactions").size();

            Assertions.assertEquals(expectedCount, actualCount,
                String.format("Expected %d interactions but found %d", expectedCount, actualCount));

        } catch (IOException e) {
            Assertions.fail("Failed to load cassette: " + e.getMessage());
        }
    }

    /**
     * Assert that cassette contains cookies
     * @param cassetteName Name of cassette
     */
    public static void assertHasCookies(String cassetteName) {
        try {
            JsonNode cassette = loadCassette(cassetteName);
            JsonNode cookies = cassette.get("cookies");

            Assertions.assertNotNull(cookies, "Cassette has no cookies");
            Assertions.assertTrue(cookies.size() > 0,
                "Cassette cookies array is empty");

        } catch (IOException e) {
            Assertions.fail("Failed to load cassette: " + e.getMessage());
        }
    }

    /**
     * Assert that cassette has specific cookie
     * @param cassetteName Name of cassette
     * @param cookieName Name of cookie to find
     */
    public static void assertHasCookie(String cassetteName, String cookieName) {
        try {
            JsonNode cassette = loadCassette(cassetteName);
            JsonNode cookies = cassette.get("cookies");

            if (cookies == null || cookies.size() == 0) {
                Assertions.fail("Cassette has no cookies");
            }

            boolean found = false;
            for (JsonNode cookie : cookies) {
                if (cookie.get("name").asText().equals(cookieName)) {
                    found = true;
                    break;
                }
            }

            Assertions.assertTrue(found,
                String.format("Cookie '%s' not found in cassette", cookieName));

        } catch (IOException e) {
            Assertions.fail("Failed to load cassette: " + e.getMessage());
        }
    }

    /**
     * Assert that cassette version matches expected
     * @param cassetteName Name of cassette
     * @param expectedVersion Expected version (e.g., "2.0")
     */
    public static void assertCassetteVersion(String cassetteName, String expectedVersion) {
        try {
            JsonNode cassette = loadCassette(cassetteName);
            String actualVersion = cassette.get("version").asText();

            Assertions.assertEquals(expectedVersion, actualVersion,
                String.format("Expected version %s but found %s", expectedVersion, actualVersion));

        } catch (IOException e) {
            Assertions.fail("Failed to load cassette: " + e.getMessage());
        }
    }

    /**
     * Find matching interaction in cassette
     * @param cassette Loaded cassette
     * @param method HTTP method
     * @param url Request URL
     * @return Matching interaction or null
     */
    private static JsonNode findMatchingInteraction(JsonNode cassette, String method, String url) {
        JsonNode interactions = cassette.get("interactions");

        for (JsonNode interaction : interactions) {
            if (!interaction.has("kind") || !interaction.get("kind").has("Http")) {
                continue;
            }

            JsonNode httpInteraction = interaction.get("kind").get("Http");
            JsonNode request = httpInteraction.get("request");

            String reqMethod = request.get("method").asText();
            String reqUrl = request.get("url").asText();

            if (reqMethod.equals(method) && reqUrl.equals(url)) {
                return interaction;
            }
        }

        return null;
    }
}
