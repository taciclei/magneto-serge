<?php
/**
 * PHPUnit integration example for matgto-serge
 *
 * This example shows how to use matgto-serge in PHPUnit tests
 */

use PHPUnit\Framework\TestCase;
use MatgtoSerge\MagnetoProxy;
use MatgtoSerge\ProxyMode;
use GuzzleHttp\Client;

class ApiTest extends TestCase
{
    private MagnetoProxy $proxy;
    private Client $httpClient;

    protected function setUp(): void
    {
        // Create proxy in auto mode
        $this->proxy = new MagnetoProxy('./test-cassettes');
        $this->proxy->withPort(8888)
                    ->withMode(ProxyMode::Auto);

        // Create HTTP client configured to use proxy
        $this->httpClient = new Client([
            'proxy' => 'http://localhost:8888',
            'verify' => false, // Disable SSL verification for MITM proxy
        ]);
    }

    protected function tearDown(): void
    {
        $this->proxy->shutdown();
    }

    /**
     * Test GET request with auto mode
     * First run: records to cassette
     * Subsequent runs: replays from cassette
     */
    public function testGetRequest(): void
    {
        $this->proxy->startRecording('get-users');

        // Make HTTP request through proxy
        $response = $this->httpClient->get('https://jsonplaceholder.typicode.com/users/1');

        $this->assertEquals(200, $response->getStatusCode());

        $data = json_decode($response->getBody(), true);
        $this->assertArrayHasKey('id', $data);
        $this->assertEquals(1, $data['id']);

        $this->proxy->stopRecording();
    }

    /**
     * Test POST request
     */
    public function testPostRequest(): void
    {
        $this->proxy->startRecording('post-user');

        $response = $this->httpClient->post('https://jsonplaceholder.typicode.com/posts', [
            'json' => [
                'title' => 'Test Post',
                'body' => 'This is a test',
                'userId' => 1,
            ]
        ]);

        $this->assertEquals(201, $response->getStatusCode());

        $data = json_decode($response->getBody(), true);
        $this->assertArrayHasKey('id', $data);

        $this->proxy->stopRecording();
    }

    /**
     * Test replay mode specifically
     */
    public function testReplayMode(): void
    {
        // First, ensure cassette exists
        $this->proxy->withMode(ProxyMode::Record);
        $this->proxy->startRecording('replay-test');

        $response1 = $this->httpClient->get('https://httpbin.org/uuid');
        $uuid1 = json_decode($response1->getBody(), true)['uuid'];

        $this->proxy->stopRecording();

        // Now replay
        $this->proxy->withMode(ProxyMode::Replay);
        $this->proxy->replay('replay-test');

        $response2 = $this->httpClient->get('https://httpbin.org/uuid');
        $uuid2 = json_decode($response2->getBody(), true)['uuid'];

        // UUIDs should be identical in replay mode
        $this->assertEquals($uuid1, $uuid2);
    }
}

// Example usage without PHPUnit
if (PHP_SAPI === 'cli' && !class_exists('PHPUnit\Framework\TestCase')) {
    echo "📝 PHPUnit Integration Example\n";
    echo str_repeat("=", 50) . "\n\n";

    echo "This file shows how to integrate matgto-serge with PHPUnit.\n\n";

    echo "To run the tests:\n";
    echo "1. Install dependencies: composer require --dev phpunit/phpunit\n";
    echo "2. Run tests: ./vendor/bin/phpunit example_phpunit.php\n\n";

    echo "Key features:\n";
    echo "  - setUp() creates proxy for each test\n";
    echo "  - tearDown() cleans up proxy\n";
    echo "  - Auto mode records on first run, replays on subsequent runs\n";
    echo "  - Perfect for deterministic API tests\n\n";

    echo "Example test method:\n";
    echo "```php\n";
    echo "public function testApi(): void\n";
    echo "{\n";
    echo "    \$this->proxy->startRecording('my-test');\n";
    echo "    \$response = \$this->httpClient->get('https://api.example.com');\n";
    echo "    \$this->assertEquals(200, \$response->getStatusCode());\n";
    echo "    \$this->proxy->stopRecording();\n";
    echo "}\n";
    echo "```\n";
}
