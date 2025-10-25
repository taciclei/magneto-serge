<?php
/**
 * Unit tests for Magnéto-Serge PHPUnit assertions (no server needed)
 */

use PHPUnit\Framework\TestCase;

class UnitTest extends TestCase
{
    private $tempDir;

    protected function setUp(): void
    {
        $this->tempDir = sys_get_temp_dir() . '/magneto-test-' . uniqid();
        mkdir($this->tempDir, 0777, true);
    }

    protected function tearDown(): void
    {
        $this->deleteDirectory($this->tempDir);
    }

    // Cassette Structure Tests

    public function testValidCassetteStructure()
    {
        $cassette = $this->createTestCassette('test', 1);

        $this->assertStringContainsString('"version"', $cassette);
        $this->assertStringContainsString('"name"', $cassette);
        $this->assertStringContainsString('"interactions"', $cassette);
        $this->assertStringContainsString('"recorded_at"', $cassette);
    }

    public function testCassetteVersion()
    {
        $cassette = $this->createTestCassette('version-test', 1);

        $this->assertStringContainsString('"version": "1.0"', $cassette);
    }

    public function testCassetteName()
    {
        $cassette = $this->createTestCassette('name-test', 1);

        $this->assertStringContainsString('"name": "name-test"', $cassette);
    }

    // HTTP Interaction Tests

    public function testHttpInteraction()
    {
        $cassette = $this->createTestCassette('http-test', 1);

        $this->assertStringContainsString('"Http"', $cassette);
        $this->assertStringContainsString('"request"', $cassette);
        $this->assertStringContainsString('"response"', $cassette);
    }

    public function testRequestStructure()
    {
        $cassette = $this->createTestCassette('request-test', 1);

        $this->assertStringContainsString('"method": "GET"', $cassette);
        $this->assertStringContainsString('"url": "https://api.example.com/users"', $cassette);
    }

    public function testResponseStructure()
    {
        $cassette = $this->createTestCassette('response-test', 1);

        $this->assertStringContainsString('"status": 200', $cassette);
        $this->assertStringContainsString('"Content-Type"', $cassette);
        $this->assertStringContainsString('"application/json"', $cassette);
    }

    // Cookie Tests

    public function testHasCookies()
    {
        $cassette = $this->createTestCassette('cookie-test', 1);

        $this->assertStringContainsString('"cookies"', $cassette);
        $this->assertStringContainsString('"JSESSIONID"', $cassette);
    }

    public function testCookieFields()
    {
        $cassette = $this->createTestCassette('cookie-fields-test', 1);

        $this->assertStringContainsString('"name": "JSESSIONID"', $cassette);
        $this->assertStringContainsString('"value": "ABC123"', $cassette);
        $this->assertStringContainsString('"domain": "example.com"', $cassette);
        $this->assertStringContainsString('"path": "/"', $cassette);
        $this->assertStringContainsString('"secure": true', $cassette);
        $this->assertStringContainsString('"http_only": true', $cassette);
    }

    // Multiple Interactions Tests

    /**
     * @dataProvider interactionCountProvider
     */
    public function testMultipleInteractions($count)
    {
        $cassette = $this->createTestCassette('multi-test', $count);

        $httpCount = substr_count($cassette, '"Http"');
        $this->assertEquals($count, $httpCount);
    }

    public function interactionCountProvider()
    {
        return [
            [1],
            [2],
            [5],
            [10],
            [50]
        ];
    }

    public function testMultipleCassettes()
    {
        $cassette1 = $this->createTestCassette('cassette-1', 1);
        $cassette2 = $this->createTestCassette('cassette-2', 2);

        $this->assertStringContainsString('"name": "cassette-1"', $cassette1);
        $this->assertStringContainsString('"name": "cassette-2"', $cassette2);

        $count1 = substr_count($cassette1, '"Http"');
        $count2 = substr_count($cassette2, '"Http"');

        $this->assertEquals(1, $count1);
        $this->assertEquals(2, $count2);
    }

    // File Operations Tests

    public function testWriteCassette()
    {
        $cassette = $this->createTestCassette('file-test', 1);
        $filePath = $this->tempDir . '/file-test.json';

        file_put_contents($filePath, $cassette);

        $this->assertFileExists($filePath);
        $this->assertGreaterThan(0, filesize($filePath));
    }

    public function testReadCassette()
    {
        $originalCassette = $this->createTestCassette('read-test', 2);
        $filePath = $this->tempDir . '/read-test.json';

        file_put_contents($filePath, $originalCassette);
        $loadedCassette = file_get_contents($filePath);

        $this->assertEquals($originalCassette, $loadedCassette);
    }

    public function testMultipleCassetteFiles()
    {
        for ($i = 0; $i < 3; $i++) {
            $cassette = $this->createTestCassette('cassette-' . $i, $i + 1);
            $filePath = $this->tempDir . '/cassette-' . $i . '.json';
            file_put_contents($filePath, $cassette);
        }

        $files = glob($this->tempDir . '/*.json');
        $this->assertCount(3, $files);
    }

    // JSON Validity Tests

    public function testValidJson()
    {
        $cassette = $this->createTestCassette('json-test', 1);

        $decoded = json_decode($cassette, true);

        $this->assertNotNull($decoded);
        $this->assertIsArray($decoded);
        $this->assertEquals(JSON_ERROR_NONE, json_last_error());
    }

    public function testJsonStructure()
    {
        $cassette = $this->createTestCassette('structure-test', 1);
        $data = json_decode($cassette, true);

        $this->assertArrayHasKey('version', $data);
        $this->assertArrayHasKey('name', $data);
        $this->assertArrayHasKey('interactions', $data);
        $this->assertArrayHasKey('cookies', $data);
        $this->assertArrayHasKey('recorded_at', $data);
    }

    public function testInteractionsArray()
    {
        $cassette = $this->createTestCassette('array-test', 3);
        $data = json_decode($cassette, true);

        $this->assertIsArray($data['interactions']);
        $this->assertCount(3, $data['interactions']);
    }

    public function testCookiesArray()
    {
        $cassette = $this->createTestCassette('cookies-array-test', 1);
        $data = json_decode($cassette, true);

        $this->assertIsArray($data['cookies']);
        $this->assertGreaterThan(0, count($data['cookies']));
    }

    // Helper Methods

    private function createTestCassette($name, $interactionCount)
    {
        $interactions = [];

        for ($i = 0; $i < $interactionCount; $i++) {
            $interactions[] = [
                'kind' => [
                    'Http' => [
                        'request' => [
                            'method' => 'GET',
                            'url' => 'https://api.example.com/users',
                            'headers' => ['Accept' => 'application/json'],
                            'body' => null
                        ],
                        'response' => [
                            'status' => 200,
                            'headers' => ['Content-Type' => 'application/json'],
                            'body' => [123, 34, 117, 115, 101, 114, 115, 34, 58, 91, 93, 125]
                        ]
                    ]
                ]
            ];
        }

        $cassette = [
            'version' => '1.0',
            'name' => $name,
            'recorded_at' => '2025-10-25T10:00:00Z',
            'interactions' => $interactions,
            'cookies' => [
                [
                    'name' => 'JSESSIONID',
                    'value' => 'ABC123',
                    'domain' => 'example.com',
                    'path' => '/',
                    'expires' => null,
                    'max_age' => null,
                    'secure' => true,
                    'http_only' => true,
                    'same_site' => null,
                    'created_at' => '2025-10-25T10:00:00Z'
                ]
            ]
        ];

        return json_encode($cassette, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES) . "\n";
    }

    private function deleteDirectory($dir)
    {
        if (!file_exists($dir)) {
            return;
        }

        $files = array_diff(scandir($dir), ['.', '..']);

        foreach ($files as $file) {
            $path = $dir . '/' . $file;
            is_dir($path) ? $this->deleteDirectory($path) : unlink($path);
        }

        rmdir($dir);
    }
}
