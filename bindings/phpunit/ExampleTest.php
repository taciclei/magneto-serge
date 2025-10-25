<?php

namespace MagnetoSerge\PHPUnit\Tests;

use MagnetoSerge\PHPUnit\MagnetoAssertions;
use PHPUnit\Framework\TestCase;
use GuzzleHttp\Client;

/**
 * Example tests using Magnéto-Serge PHPUnit assertions
 */
class ExampleTest extends TestCase
{
    use MagnetoAssertions;

    private Client $client;

    protected function setUp(): void
    {
        parent::setUp();

        // Configure HTTP client to use Magnéto proxy
        $this->client = new Client([
            'proxy' => 'http://localhost:8888',
            'base_uri' => 'http://localhost:8080',
        ]);

        // Set cassette directory
        self::setCassetteDirectory('./cassettes');
    }

    public function testUserLogin(): void
    {
        $response = $this->client->post('/api/authenticate', [
            'json' => [
                'username' => 'admin',
                'password' => 'admin',
            ],
        ]);

        // Assert response matches cassette
        $this->assertMatchesCassette($response, 'user-login');
    }

    public function testUserAccount(): void
    {
        $response = $this->client->get('/api/account');

        // Assert status matches cassette
        $this->assertCassetteStatus($response, 'user-account', 200);
    }

    public function testInteractionCount(): void
    {
        // Assert cassette has expected number of interactions
        $this->assertInteractionCount('user-login', 3);
    }

    public function testCassetteCookies(): void
    {
        // Assert cassette contains cookies
        $this->assertHasCookies('user-login');
    }

    public function testSessionCookie(): void
    {
        // Assert cassette has specific cookie
        $this->assertHasCookie('user-login', 'JSESSIONID');
    }

    public function testCassetteVersion(): void
    {
        // Assert cassette version
        $this->assertCassetteVersion('user-login', '2.0');
    }
}
