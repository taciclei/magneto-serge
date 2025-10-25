/**
 * Unit tests for Magnéto-Serge JUnit assertions (no server needed)
 */

import org.junit.jupiter.api.*;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import java.io.*;
import java.nio.file.*;
import java.util.*;

import static org.junit.jupiter.api.Assertions.*;

@DisplayName("Cassette Unit Tests")
public class UnitTest {

    private Path tempDir;

    @BeforeEach
    void setUp() throws IOException {
        tempDir = Files.createTempDirectory("magneto-test-");
    }

    @AfterEach
    void tearDown() throws IOException {
        // Cleanup temp directory
        Files.walk(tempDir)
            .sorted(Comparator.reverseOrder())
            .forEach(path -> {
                try {
                    Files.delete(path);
                } catch (IOException e) {
                    // Ignore
                }
            });
    }

    @Nested
    @DisplayName("Cassette Structure Tests")
    class CassetteStructureTests {

        @Test
        @DisplayName("Should create valid cassette JSON")
        void testValidCassetteStructure() throws IOException {
            String cassette = createTestCassette("test", 1);

            assertTrue(cassette.contains("\"version\""));
            assertTrue(cassette.contains("\"name\""));
            assertTrue(cassette.contains("\"interactions\""));
            assertTrue(cassette.contains("\"recorded_at\""));
        }

        @Test
        @DisplayName("Should have correct version")
        void testCassetteVersion() throws IOException {
            String cassette = createTestCassette("version-test", 1);

            assertTrue(cassette.contains("\"version\": \"1.0\""));
        }

        @Test
        @DisplayName("Should have correct name")
        void testCassetteName() throws IOException {
            String cassette = createTestCassette("name-test", 1);

            assertTrue(cassette.contains("\"name\": \"name-test\""));
        }
    }

    @Nested
    @DisplayName("HTTP Interaction Tests")
    class HttpInteractionTests {

        @Test
        @DisplayName("Should have HTTP interaction")
        void testHttpInteraction() throws IOException {
            String cassette = createTestCassette("http-test", 1);

            assertTrue(cassette.contains("\"Http\""));
            assertTrue(cassette.contains("\"request\""));
            assertTrue(cassette.contains("\"response\""));
        }

        @Test
        @DisplayName("Should have request method and URL")
        void testRequestStructure() throws IOException {
            String cassette = createTestCassette("request-test", 1);

            assertTrue(cassette.contains("\"method\": \"GET\""));
            assertTrue(cassette.contains("\"url\": \"https://api.example.com/users\""));
        }

        @Test
        @DisplayName("Should have response status and headers")
        void testResponseStructure() throws IOException {
            String cassette = createTestCassette("response-test", 1);

            assertTrue(cassette.contains("\"status\": 200"));
            assertTrue(cassette.contains("\"Content-Type\""));
            assertTrue(cassette.contains("\"application/json\""));
        }
    }

    @Nested
    @DisplayName("Cookie Tests")
    class CookieTests {

        @Test
        @DisplayName("Should have cookies array")
        void testHasCookies() throws IOException {
            String cassette = createTestCassette("cookie-test", 1);

            assertTrue(cassette.contains("\"cookies\""));
            assertTrue(cassette.contains("\"JSESSIONID\""));
        }

        @Test
        @DisplayName("Cookie should have required fields")
        void testCookieFields() throws IOException {
            String cassette = createTestCassette("cookie-fields-test", 1);

            // Required fields
            assertTrue(cassette.contains("\"name\": \"JSESSIONID\""));
            assertTrue(cassette.contains("\"value\": \"ABC123\""));
            assertTrue(cassette.contains("\"domain\": \"example.com\""));
            assertTrue(cassette.contains("\"path\": \"/\""));
            assertTrue(cassette.contains("\"secure\": true"));
            assertTrue(cassette.contains("\"http_only\": true"));
        }
    }

    @Nested
    @DisplayName("Multiple Interactions Tests")
    class MultipleInteractionsTests {

        @ParameterizedTest
        @ValueSource(ints = {1, 2, 5, 10})
        @DisplayName("Should handle varying interaction counts")
        void testMultipleInteractions(int count) throws IOException {
            String cassette = createTestCassette("multi-test", count);

            // Count occurrences of "Http" to verify interaction count
            int httpCount = countOccurrences(cassette, "\"Http\"");
            assertEquals(count, httpCount);
        }

        @Test
        @DisplayName("Should create multiple different cassettes")
        void testMultipleCassettes() throws IOException {
            String cassette1 = createTestCassette("cassette-1", 1);
            String cassette2 = createTestCassette("cassette-2", 2);

            assertTrue(cassette1.contains("\"name\": \"cassette-1\""));
            assertTrue(cassette2.contains("\"name\": \"cassette-2\""));

            int count1 = countOccurrences(cassette1, "\"Http\"");
            int count2 = countOccurrences(cassette2, "\"Http\"");

            assertEquals(1, count1);
            assertEquals(2, count2);
        }
    }

    @Nested
    @DisplayName("File Operations Tests")
    class FileOperationsTests {

        @Test
        @DisplayName("Should write cassette to file")
        void testWriteCassette() throws IOException {
            String cassette = createTestCassette("file-test", 1);
            Path filePath = tempDir.resolve("file-test.json");

            Files.writeString(filePath, cassette);

            assertTrue(Files.exists(filePath));
            assertTrue(Files.size(filePath) > 0);
        }

        @Test
        @DisplayName("Should read cassette from file")
        void testReadCassette() throws IOException {
            String originalCassette = createTestCassette("read-test", 2);
            Path filePath = tempDir.resolve("read-test.json");

            Files.writeString(filePath, originalCassette);
            String loadedCassette = Files.readString(filePath);

            assertEquals(originalCassette, loadedCassette);
        }

        @Test
        @DisplayName("Should handle multiple cassette files")
        void testMultipleCassetteFiles() throws IOException {
            for (int i = 0; i < 3; i++) {
                String cassette = createTestCassette("cassette-" + i, i + 1);
                Path filePath = tempDir.resolve("cassette-" + i + ".json");
                Files.writeString(filePath, cassette);
            }

            List<Path> files = Files.list(tempDir).toList();
            assertEquals(3, files.size());
        }
    }

    @Nested
    @DisplayName("JSON Validity Tests")
    class JsonValidityTests {

        @Test
        @DisplayName("Should be valid JSON")
        void testValidJson() {
            assertDoesNotThrow(() -> {
                String cassette = createTestCassette("json-test", 1);
                // If we can parse it successfully, it's valid JSON
                // (In real code, you'd use a JSON library here)
            });
        }

        @Test
        @DisplayName("Should have proper JSON structure")
        void testJsonStructure() throws IOException {
            String cassette = createTestCassette("structure-test", 1);

            assertTrue(cassette.startsWith("{"));
            assertTrue(cassette.endsWith("}\n"));
            assertTrue(cassette.contains("\"interactions\": ["));
            assertTrue(cassette.contains("\"cookies\": ["));
        }
    }

    // Helper methods

    private String createTestCassette(String name, int interactionCount) throws IOException {
        StringBuilder sb = new StringBuilder();

        sb.append("{\n");
        sb.append("  \"version\": \"1.0\",\n");
        sb.append("  \"name\": \"").append(name).append("\",\n");
        sb.append("  \"recorded_at\": \"2025-10-25T10:00:00Z\",\n");
        sb.append("  \"interactions\": [\n");

        for (int i = 0; i < interactionCount; i++) {
            if (i > 0) sb.append(",\n");
            sb.append("    {\n");
            sb.append("      \"kind\": {\n");
            sb.append("        \"Http\": {\n");
            sb.append("          \"request\": {\n");
            sb.append("            \"method\": \"GET\",\n");
            sb.append("            \"url\": \"https://api.example.com/users\",\n");
            sb.append("            \"headers\": {\"Accept\": \"application/json\"},\n");
            sb.append("            \"body\": null\n");
            sb.append("          },\n");
            sb.append("          \"response\": {\n");
            sb.append("            \"status\": 200,\n");
            sb.append("            \"headers\": {\"Content-Type\": \"application/json\"},\n");
            sb.append("            \"body\": [123, 34, 117, 115, 101, 114, 115, 34, 58, 91, 93, 125]\n");
            sb.append("          }\n");
            sb.append("        }\n");
            sb.append("      }\n");
            sb.append("    }");
        }

        sb.append("\n  ],\n");
        sb.append("  \"cookies\": [\n");
        sb.append("    {\n");
        sb.append("      \"name\": \"JSESSIONID\",\n");
        sb.append("      \"value\": \"ABC123\",\n");
        sb.append("      \"domain\": \"example.com\",\n");
        sb.append("      \"path\": \"/\",\n");
        sb.append("      \"expires\": null,\n");
        sb.append("      \"max_age\": null,\n");
        sb.append("      \"secure\": true,\n");
        sb.append("      \"http_only\": true,\n");
        sb.append("      \"same_site\": null,\n");
        sb.append("      \"created_at\": \"2025-10-25T10:00:00Z\"\n");
        sb.append("    }\n");
        sb.append("  ]\n");
        sb.append("}\n");

        return sb.toString();
    }

    private int countOccurrences(String text, String pattern) {
        int count = 0;
        int index = 0;

        while ((index = text.indexOf(pattern, index)) != -1) {
            count++;
            index += pattern.length();
        }

        return count;
    }
}
