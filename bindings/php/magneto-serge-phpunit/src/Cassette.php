<?php

declare(strict_types=1);

namespace MagnetoSerge\PHPUnit;

use Attribute;

/**
 * Cassette attribute for PHPUnit tests
 *
 * Marks a test method to use a specific cassette for HTTP/WebSocket recording.
 *
 * @example
 * ```php
 * use MagnetoSerge\PHPUnit\Cassette;
 *
 * class ApiTest extends MagnetoTestCase
 * {
 *     #[Cassette('github_users')]
 *     public function testFetchUsers(): void
 *     {
 *         // Cassette: tests/cassettes/github_users.json
 *     }
 *
 *     #[Cassette('api_call', mode: 'replay')]
 *     public function testReplayOnly(): void
 *     {
 *         // Strict replay mode
 *     }
 *
 *     #[Cassette(name: 'shared', record: 'all')]
 *     public function testForceRecord(): void
 *     {
 *         // Always re-records
 *     }
 * }
 * ```
 */
#[Attribute(Attribute::TARGET_METHOD)]
class Cassette
{
    /**
     * @param string|null $name Cassette name (auto-generated if null)
     * @param string|null $mode Recording mode: 'auto', 'record', 'replay', 'passthrough'
     * @param string|null $record VCR-compatible mode: 'new_episodes', 'once', 'all', 'none'
     * @param int|null $port Proxy port (overrides default)
     */
    public function __construct(
        public ?string $name = null,
        public ?string $mode = null,
        public ?string $record = null,
        public ?int $port = null,
    ) {
    }
}
