<?php

declare(strict_types=1);

namespace MagnetoSerge\PHPUnit;

use PHPUnit\Framework\TestCase;
use MagnetoSerge\MagnetoProxy;

/**
 * Base TestCase with automatic cassette management
 *
 * Extends PHPUnit\Framework\TestCase with Magnéto-Serge cassette support.
 *
 * @example
 * ```php
 * use MagnetoSerge\PHPUnit\MagnetoTestCase;
 *
 * class ApiTest extends MagnetoTestCase
 * {
 *     protected string $cassetteDir = 'tests/fixtures/cassettes';
 *
 *     #[Cassette('github_users')]
 *     public function testFetchUsers(): void
 *     {
 *         $response = file_get_contents('https://api.github.com/users');
 *         $this->assertNotEmpty($response);
 *     }
 * }
 * ```
 */
abstract class MagnetoTestCase extends TestCase
{
    /**
     * Directory where cassettes are stored
     */
    protected string $cassetteDir = 'tests/cassettes';

    /**
     * Default recording mode
     */
    protected string $defaultMode = 'auto';

    /**
     * Default proxy port
     */
    protected int $proxyPort = 8888;

    /**
     * Default record mode (VCR-compatible)
     */
    protected string $recordMode = 'new_episodes';

    /**
     * Active MagnetoProxy instance
     */
    private ?MagnetoProxy $proxy = null;

    /**
     * Current cassette name
     */
    private ?string $currentCassette = null;

    /**
     * Whether cassette is active
     */
    private bool $cassetteActive = false;

    /**
     * Set up before each test
     */
    protected function setUp(): void
    {
        parent::setUp();

        // Check for Cassette attribute
        $reflection = new \ReflectionMethod($this, $this->name());
        $attributes = $reflection->getAttributes(Cassette::class);

        if (!empty($attributes)) {
            /** @var Cassette $cassette */
            $cassette = $attributes[0]->newInstance();
            $this->startCassette($cassette);
        }
    }

    /**
     * Tear down after each test
     */
    protected function tearDown(): void
    {
        if ($this->cassetteActive) {
            $this->stopCassette();
        }

        parent::tearDown();
    }

    /**
     * Start a cassette for the current test
     */
    private function startCassette(Cassette $cassette): void
    {
        // Get cassette name (use provided or generate from test name)
        $cassetteName = $cassette->name ?? $this->generateCassetteName();

        // Get mode (use cassette mode or record mode translation)
        $mode = $cassette->mode ?? $this->translateRecordMode($cassette->record ?? $this->recordMode);

        // Create proxy
        $this->proxy = new MagnetoProxy($this->cassetteDir);

        // Start in appropriate mode
        match ($mode) {
            'auto' => $this->proxy->auto($cassetteName),
            'record' => $this->proxy->record($cassetteName),
            'replay' => $this->proxy->replay($cassetteName),
            'passthrough' => $this->proxy->passthrough(),
            default => throw new \InvalidArgumentException("Unknown mode: {$mode}"),
        };

        $this->currentCassette = $cassetteName;
        $this->cassetteActive = true;
    }

    /**
     * Stop the active cassette
     */
    private function stopCassette(): void
    {
        if ($this->proxy !== null) {
            $this->proxy->stop();
            $this->proxy = null;
        }

        $this->currentCassette = null;
        $this->cassetteActive = false;
    }

    /**
     * Generate cassette name from test class and method
     */
    private function generateCassetteName(): string
    {
        $className = (new \ReflectionClass($this))->getShortName();
        $methodName = $this->name();

        // Remove 'test' prefix if present
        $methodName = preg_replace('/^test/', '', $methodName);

        // Convert camelCase to snake_case
        $methodName = strtolower(preg_replace('/(?<!^)[A-Z]/', '_$0', $methodName));

        return sprintf('%s/%s', $className, $methodName);
    }

    /**
     * Translate VCR record mode to Magneto mode
     */
    private function translateRecordMode(string $recordMode): string
    {
        return match ($recordMode) {
            'new_episodes' => 'auto',
            'once' => 'replay',
            'all' => 'record',
            'none' => 'replay',
            default => 'auto',
        };
    }

    /**
     * Get current cassette name
     */
    protected function getCurrentCassette(): ?string
    {
        return $this->currentCassette;
    }

    /**
     * Manually use a cassette within a test
     *
     * @param string $name Cassette name
     * @param callable $callback Callback to execute with cassette active
     * @param array{mode?: string, record?: string, port?: int} $options Cassette options
     */
    protected function useCassette(string $name, callable $callback, array $options = []): void
    {
        $mode = $options['mode'] ?? $this->translateRecordMode($options['record'] ?? $this->recordMode);
        $port = $options['port'] ?? $this->proxyPort;

        $proxy = new MagnetoProxy($this->cassetteDir);

        try {
            // Start cassette
            match ($mode) {
                'auto' => $proxy->auto($name),
                'record' => $proxy->record($name),
                'replay' => $proxy->replay($name),
                'passthrough' => $proxy->passthrough(),
                default => throw new \InvalidArgumentException("Unknown mode: {$mode}"),
            };

            // Execute callback
            $callback();
        } finally {
            // Always stop cassette
            $proxy->stop();
        }
    }
}
