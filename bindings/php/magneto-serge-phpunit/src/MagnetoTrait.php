<?php

declare(strict_types=1);

namespace MagnetoSerge\PHPUnit;

use MagnetoSerge\MagnetoProxy;

/**
 * Trait for adding Magneto cassette support to any TestCase
 *
 * Use this trait when you can't extend MagnetoTestCase (e.g., when already extending another base class).
 *
 * @example
 * ```php
 * use PHPUnit\Framework\TestCase;
 * use MagnetoSerge\PHPUnit\MagnetoTrait;
 *
 * class MyTest extends TestCase
 * {
 *     use MagnetoTrait;
 *
 *     protected string $cassetteDir = 'tests/cassettes';
 *
 *     #[Cassette('api_test')]
 *     public function testApi(): void
 *     {
 *         // Cassette auto-managed
 *     }
 *
 *     public function testManual(): void
 *     {
 *         $this->useCassette('manual_cassette', function() {
 *             // Cassette active here
 *         });
 *     }
 * }
 * ```
 */
trait MagnetoTrait
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
    private ?MagnetoProxy $magnetoProxy = null;

    /**
     * Current cassette name
     */
    private ?string $magnetoCurrentCassette = null;

    /**
     * Whether cassette is active
     */
    private bool $magnetoCassetteActive = false;

    /**
     * Set up before each test (call this from your setUp method)
     */
    protected function setUpMagneto(): void
    {
        // Check for Cassette attribute
        $reflection = new \ReflectionMethod($this, $this->name());
        $attributes = $reflection->getAttributes(Cassette::class);

        if (!empty($attributes)) {
            /** @var Cassette $cassette */
            $cassette = $attributes[0]->newInstance();
            $this->magnetoStartCassette($cassette);
        }
    }

    /**
     * Tear down after each test (call this from your tearDown method)
     */
    protected function tearDownMagneto(): void
    {
        if ($this->magnetoCassetteActive) {
            $this->magnetoStopCassette();
        }
    }

    /**
     * Start a cassette
     */
    private function magnetoStartCassette(Cassette $cassette): void
    {
        $cassetteName = $cassette->name ?? $this->magnetoGenerateCassetteName();
        $mode = $cassette->mode ?? $this->magnetoTranslateRecordMode($cassette->record ?? $this->recordMode);

        $this->magnetoProxy = new MagnetoProxy($this->cassetteDir);

        match ($mode) {
            'auto' => $this->magnetoProxy->auto($cassetteName),
            'record' => $this->magnetoProxy->record($cassetteName),
            'replay' => $this->magnetoProxy->replay($cassetteName),
            'passthrough' => $this->magnetoProxy->passthrough(),
            default => throw new \InvalidArgumentException("Unknown mode: {$mode}"),
        };

        $this->magnetoCurrentCassette = $cassetteName;
        $this->magnetoCassetteActive = true;
    }

    /**
     * Stop the active cassette
     */
    private function magnetoStopCassette(): void
    {
        if ($this->magnetoProxy !== null) {
            $this->magnetoProxy->stop();
            $this->magnetoProxy = null;
        }

        $this->magnetoCurrentCassette = null;
        $this->magnetoCassetteActive = false;
    }

    /**
     * Generate cassette name from test
     */
    private function magnetoGenerateCassetteName(): string
    {
        $className = (new \ReflectionClass($this))->getShortName();
        $methodName = $this->name();

        $methodName = preg_replace('/^test/', '', $methodName);
        $methodName = strtolower(preg_replace('/(?<!^)[A-Z]/', '_$0', $methodName));

        return sprintf('%s/%s', $className, $methodName);
    }

    /**
     * Translate VCR record mode to Magneto mode
     */
    private function magnetoTranslateRecordMode(string $recordMode): string
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
        return $this->magnetoCurrentCassette;
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
        $mode = $options['mode'] ?? $this->magnetoTranslateRecordMode($options['record'] ?? $this->recordMode);

        $proxy = new MagnetoProxy($this->cassetteDir);

        try {
            match ($mode) {
                'auto' => $proxy->auto($name),
                'record' => $proxy->record($name),
                'replay' => $proxy->replay($name),
                'passthrough' => $proxy->passthrough(),
                default => throw new \InvalidArgumentException("Unknown mode: {$mode}"),
            };

            $callback();
        } finally {
            $proxy->stop();
        }
    }
}
