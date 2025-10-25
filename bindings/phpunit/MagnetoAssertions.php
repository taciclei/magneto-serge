<?php

namespace MagnetoSerge\PHPUnit;

use PHPUnit\Framework\Assert;
use PHPUnit\Framework\AssertionFailedError;

/**
 * Custom PHPUnit assertions for Magnéto-Serge cassettes
 *
 * Usage:
 * <code>
 * use MagnetoSerge\PHPUnit\MagnetoAssertions;
 *
 * class UserTest extends TestCase
 * {
 *     use MagnetoAssertions;
 *
 *     public function testUserLogin()
 *     {
 *         $response = $this->client->post('/api/authenticate', [...]);
 *         $this->assertMatchesCassette($response, 'user-login');
 *     }
 * }
 * </code>
 */
trait MagnetoAssertions
{
    /**
     * Cassette directory path
     * @var string
     */
    protected static $cassetteDir = './cassettes';

    /**
     * Set the cassette directory
     *
     * @param string $dir Directory path
     * @return void
     */
    public static function setCassetteDirectory(string $dir): void
    {
        self::$cassetteDir = $dir;
    }

    /**
     * Load a cassette from disk
     *
     * @param string $name Cassette name (without extension)
     * @return array Parsed cassette
     * @throws \RuntimeException if cassette not found
     */
    protected function loadCassette(string $name): array
    {
        $jsonPath = self::$cassetteDir . '/' . $name . '.json';
        $msgpackPath = self::$cassetteDir . '/' . $name . '.msgpack';

        if (file_exists($jsonPath)) {
            $content = file_get_contents($jsonPath);
            return json_decode($content, true);
        } elseif (file_exists($msgpackPath)) {
            throw new \RuntimeException('MessagePack cassettes not yet supported');
        } else {
            throw new \RuntimeException("Cassette not found: {$name}");
        }
    }

    /**
     * Find matching interaction in cassette
     *
     * @param array $cassette Loaded cassette
     * @param string $method HTTP method
     * @param string $url Request URL
     * @return array|null Matching interaction or null
     */
    protected function findMatchingInteraction(array $cassette, string $method, string $url): ?array
    {
        foreach ($cassette['interactions'] as $interaction) {
            if (!isset($interaction['kind']['Http'])) {
                continue;
            }

            $request = $interaction['kind']['Http']['request'];

            if ($request['method'] === $method && $request['url'] === $url) {
                return $interaction;
            }
        }

        return null;
    }

    /**
     * Assert that HTTP response matches a cassette
     *
     * @param \Psr\Http\Message\ResponseInterface|array $response HTTP response
     * @param string $cassetteName Name of cassette to match against
     * @return void
     * @throws AssertionFailedError
     */
    public function assertMatchesCassette($response, string $cassetteName): void
    {
        $cassette = $this->loadCassette($cassetteName);

        // Extract request info (support different HTTP client interfaces)
        if (is_array($response)) {
            $method = $response['method'] ?? 'GET';
            $url = $response['url'] ?? '';
            $status = $response['status'] ?? 0;
        } else {
            // PSR-7 Response
            $request = $response->getRequest();
            $method = $request->getMethod();
            $url = (string) $request->getUri();
            $status = $response->getStatusCode();
        }

        $interaction = $this->findMatchingInteraction($cassette, $method, $url);

        Assert::assertNotNull(
            $interaction,
            "No matching interaction found in cassette '{$cassetteName}' for {$method} {$url}"
        );

        $expectedStatus = $interaction['kind']['Http']['response']['status'];

        Assert::assertEquals(
            $expectedStatus,
            $status,
            "Status code mismatch in cassette '{$cassetteName}'"
        );
    }

    /**
     * Assert that HTTP response status matches cassette
     *
     * @param \Psr\Http\Message\ResponseInterface|array $response HTTP response
     * @param string $cassetteName Name of cassette
     * @param int $expectedStatus Expected status code
     * @return void
     * @throws AssertionFailedError
     */
    public function assertCassetteStatus($response, string $cassetteName, int $expectedStatus): void
    {
        $cassette = $this->loadCassette($cassetteName);

        // Extract request info
        if (is_array($response)) {
            $method = $response['method'] ?? 'GET';
            $url = $response['url'] ?? '';
        } else {
            $request = $response->getRequest();
            $method = $request->getMethod();
            $url = (string) $request->getUri();
        }

        $interaction = $this->findMatchingInteraction($cassette, $method, $url);

        Assert::assertNotNull(
            $interaction,
            "No matching interaction found in cassette '{$cassetteName}'"
        );

        $cassetteStatus = $interaction['kind']['Http']['response']['status'];

        Assert::assertEquals(
            $expectedStatus,
            $cassetteStatus,
            "Expected status {$expectedStatus} but cassette has {$cassetteStatus}"
        );
    }

    /**
     * Assert that cassette has expected number of interactions
     *
     * @param string $cassetteName Name of cassette
     * @param int $expectedCount Expected interaction count
     * @return void
     * @throws AssertionFailedError
     */
    public function assertInteractionCount(string $cassetteName, int $expectedCount): void
    {
        $cassette = $this->loadCassette($cassetteName);
        $actualCount = count($cassette['interactions']);

        Assert::assertEquals(
            $expectedCount,
            $actualCount,
            "Expected {$expectedCount} interactions but found {$actualCount}"
        );
    }

    /**
     * Assert that cassette contains cookies
     *
     * @param string $cassetteName Name of cassette
     * @return void
     * @throws AssertionFailedError
     */
    public function assertHasCookies(string $cassetteName): void
    {
        $cassette = $this->loadCassette($cassetteName);

        Assert::assertArrayHasKey('cookies', $cassette, 'Cassette has no cookies');
        Assert::assertNotEmpty($cassette['cookies'], 'Cassette cookies array is empty');
    }

    /**
     * Assert that cassette has specific cookie
     *
     * @param string $cassetteName Name of cassette
     * @param string $cookieName Name of cookie to find
     * @return void
     * @throws AssertionFailedError
     */
    public function assertHasCookie(string $cassetteName, string $cookieName): void
    {
        $cassette = $this->loadCassette($cassetteName);

        Assert::assertArrayHasKey('cookies', $cassette, 'Cassette has no cookies');

        $cookieNames = array_column($cassette['cookies'], 'name');

        Assert::assertContains(
            $cookieName,
            $cookieNames,
            "Cookie '{$cookieName}' not found in cassette"
        );
    }

    /**
     * Assert that cassette version matches expected
     *
     * @param string $cassetteName Name of cassette
     * @param string $expectedVersion Expected version (e.g., "2.0")
     * @return void
     * @throws AssertionFailedError
     */
    public function assertCassetteVersion(string $cassetteName, string $expectedVersion): void
    {
        $cassette = $this->loadCassette($cassetteName);
        $actualVersion = $cassette['version'] ?? null;

        Assert::assertEquals(
            $expectedVersion,
            $actualVersion,
            "Expected version {$expectedVersion} but found {$actualVersion}"
        );
    }
}
