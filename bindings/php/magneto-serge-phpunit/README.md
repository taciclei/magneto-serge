# magneto-serge/phpunit

PHPUnit integration for Magnéto-Serge, providing VCR-like automatic cassette management for HTTP/WebSocket recording and replay.

## Installation

```bash
composer require --dev magneto-serge/phpunit
```

## Quick Start

### Using MagnetoTestCase (Recommended)

Extend `MagnetoTestCase` instead of PHPUnit's `TestCase`:

```php
<?php

use MagnetoSerge\PHPUnit\MagnetoTestCase;
use MagnetoSerge\PHPUnit\Cassette;

class GitHubApiTest extends MagnetoTestCase
{
    protected string $cassetteDir = 'tests/fixtures/cassettes';

    #[Cassette('github_users')]
    public function testFetchUsers(): void
    {
        // Cassette: tests/fixtures/cassettes/github_users.json
        $response = file_get_contents('https://api.github.com/users');
        $users = json_decode($response, true);

        $this->assertIsArray($users);
        $this->assertNotEmpty($users);
    }

    #[Cassette('octocat_profile')]
    public function testFetchUser(): void
    {
        // Cassette: tests/fixtures/cassettes/octocat_profile.json
        $response = file_get_contents('https://api.github.com/users/octocat');
        $user = json_decode($response, true);

        $this->assertEquals('octocat', $user['login']);
    }

    #[Cassette(name: 'force_record', record: 'all')]
    public function testForceRecord(): void
    {
        // Always re-records this cassette
        $response = file_get_contents('https://api.github.com/users');
        $this->assertNotEmpty($response);
    }
}
```

### Using MagnetoTrait (Alternative)

If you can't extend `MagnetoTestCase` (e.g., already extending another class), use the trait:

```php
<?php

use PHPUnit\Framework\TestCase;
use MagnetoSerge\PHPUnit\MagnetoTrait;
use MagnetoSerge\PHPUnit\Cassette;

class MyCustomTest extends TestCase
{
    use MagnetoTrait;

    protected string $cassetteDir = 'tests/cassettes';

    protected function setUp(): void
    {
        parent::setUp();
        $this->setUpMagneto();
    }

    protected function tearDown(): void
    {
        $this->tearDownMagneto();
        parent::tearDown();
    }

    #[Cassette('api_test')]
    public function testApi(): void
    {
        // Cassette auto-managed
        $response = file_get_contents('https://api.example.com/data');
        $this->assertNotEmpty($response);
    }
}
```

## API Reference

### MagnetoTestCase

Base test class with automatic cassette management.

**Properties:**
```php
protected string $cassetteDir = 'tests/cassettes';  // Directory for cassettes
protected string $defaultMode = 'auto';              // Default recording mode
protected int $proxyPort = 8888;                     // Default proxy port
protected string $recordMode = 'new_episodes';       // VCR-compatible record mode
```

**Methods:**
```php
protected function useCassette(string $name, callable $callback, array $options = []): void
protected function getCurrentCassette(): ?string
```

### #[Cassette] Attribute

Mark tests to use cassettes with automatic management.

**Parameters:**
- `name` (string|null): Cassette name (auto-generated from test name if null)
- `mode` (string|null): Recording mode (`'auto'`, `'record'`, `'replay'`, `'passthrough'`)
- `record` (string|null): VCR-compatible mode (`'new_episodes'`, `'once'`, `'all'`, `'none'`)
- `port` (int|null): Proxy port (overrides default)

**Examples:**

```php
// Auto-generated cassette name
#[Cassette]
public function testFetchUsers(): void {}
// Cassette: ClassName/fetch_users.json

// Custom cassette name
#[Cassette('github_users')]
public function testApi(): void {}
// Cassette: github_users.json

// Force replay mode
#[Cassette('api_test', mode: 'replay')]
public function testReplay(): void {}

// VCR-compatible: force re-record
#[Cassette('api_live', record: 'all')]
public function testLive(): void {}

// Custom port
#[Cassette('custom', port: 9999)]
public function testCustomPort(): void {}
```

### useCassette() Method

Manual cassette control within tests.

**Signature:**
```php
protected function useCassette(
    string $name,
    callable $callback,
    array $options = []
): void
```

**Options:**
- `mode` (string): Recording mode
- `record` (string): VCR-compatible record mode
- `port` (int): Proxy port

**Example:**

```php
public function testManualControl(): void
{
    $this->useCassette('cassette1', function() {
        $response = file_get_contents('https://api.example.com/users');
        $this->assertNotEmpty($response);
    });

    // Different cassette
    $this->useCassette('cassette2', function() {
        $response = file_get_contents('https://api.example.com/posts');
        $this->assertNotEmpty($response);
    }, ['record' => 'all']);
}
```

## Recording Modes

### Mode Translation (VCR-compatible)

| `record` value | Magneto `mode` | Behavior |
|----------------|----------------|----------|
| `new_episodes` | `auto` | Record new, replay existing (default) |
| `once` | `replay` | Replay only, error if missing |
| `all` | `record` | Always re-record, overwrite existing |
| `none` | `replay` | Replay only, never record |

### Magneto Modes

- **`auto`**: Record if cassette doesn't exist, replay if it does (default)
- **`record`**: Always record, overwrite existing cassette
- **`replay`**: Only replay, error if cassette not found
- **`passthrough`**: Direct connection, no recording or replay

## Configuration

### Global Configuration

Set in your base test class:

```php
abstract class ApiTestCase extends MagnetoTestCase
{
    protected string $cassetteDir = 'tests/fixtures/cassettes';
    protected string $defaultMode = 'auto';
    protected int $proxyPort = 8888;
    protected string $recordMode = 'new_episodes';
}
```

### Per-Test Configuration

Use attribute parameters:

```php
#[Cassette('test', mode: 'replay', port: 9999)]
public function testCustomConfig(): void
{
    // Uses custom configuration
}
```

### PHPUnit Configuration

Configure PHPUnit to exclude cassette directory from version control:

```xml
<!-- phpunit.xml -->
<phpunit>
    <source>
        <exclude>
            <directory>tests/fixtures/cassettes</directory>
        </exclude>
    </source>
</phpunit>
```

## Examples

### Basic HTTP Recording

```php
<?php

use MagnetoSerge\PHPUnit\MagnetoTestCase;
use MagnetoSerge\PHPUnit\Cassette;

class JsonPlaceholderTest extends MagnetoTestCase
{
    #[Cassette('posts_list')]
    public function testFetchPosts(): void
    {
        $response = file_get_contents('https://jsonplaceholder.typicode.com/posts');
        $posts = json_decode($response, true);

        $this->assertIsArray($posts);
        $this->assertCount(100, $posts);
    }

    #[Cassette('post_1')]
    public function testFetchSinglePost(): void
    {
        $response = file_get_contents('https://jsonplaceholder.typicode.com/posts/1');
        $post = json_decode($response, true);

        $this->assertEquals(1, $post['id']);
        $this->assertEquals(1, $post['userId']);
    }
}
```

### Using cURL

```php
#[Cassette('curl_example')]
public function testWithCurl(): void
{
    $ch = curl_init('https://api.github.com/users/octocat');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($ch, CURLOPT_HTTPHEADER, [
        'User-Agent: magneto-serge-test',
    ]);

    $response = curl_exec($ch);
    $statusCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
    curl_close($ch);

    $this->assertEquals(200, $statusCode);
    $this->assertNotEmpty($response);
}
```

### Using Guzzle

```php
use GuzzleHttp\Client;

#[Cassette('guzzle_test')]
public function testWithGuzzle(): void
{
    $client = new Client();
    $response = $client->get('https://jsonplaceholder.typicode.com/users');

    $this->assertEquals(200, $response->getStatusCode());
    $this->assertNotEmpty((string) $response->getBody());
}
```

### Manual Cassette Control

```php
public function testMultipleCassettes(): void
{
    // First cassette
    $this->useCassette('users', function() {
        $response = file_get_contents('https://api.example.com/users');
        $this->assertNotEmpty($response);
    });

    // Second cassette with options
    $this->useCassette('posts', function() {
        $response = file_get_contents('https://api.example.com/posts');
        $this->assertNotEmpty($response);
    }, ['record' => 'all']);
}
```

### Nested Cassettes

```php
public function testNestedCassettes(): void
{
    $this->useCassette('outer', function() {
        $users = file_get_contents('https://api.example.com/users');

        $this->useCassette('inner', function() use ($users) {
            $posts = file_get_contents('https://api.example.com/posts');
            $this->assertNotEmpty($posts);
        });

        $this->assertNotEmpty($users);
    });
}
```

### Error Handling

```php
#[Cassette('404_error')]
public function testHandles404(): void
{
    $context = stream_context_create([
        'http' => [
            'ignore_errors' => true,
        ],
    ]);

    $response = file_get_contents(
        'https://jsonplaceholder.typicode.com/posts/999999',
        false,
        $context
    );

    $this->assertFalse($response);
}
```

### Testing with Authentication

```php
#[Cassette('authenticated_request')]
public function testWithAuth(): void
{
    $context = stream_context_create([
        'http' => [
            'header' => [
                'Authorization: Bearer fake-token-for-testing',
                'Accept: application/json',
            ],
        ],
    ]);

    $response = file_get_contents(
        'https://api.example.com/protected',
        false,
        $context
    );

    // Authorization header will be filtered in cassette
    $this->assertNotEmpty($response);
}
```

### Different Recording Modes

```php
class RecordingModesTest extends MagnetoTestCase
{
    #[Cassette('auto_mode')]
    public function testAutoMode(): void
    {
        // Records if missing, replays if exists (default)
    }

    #[Cassette('force_record', record: 'all')]
    public function testForceRecord(): void
    {
        // Always re-records, overwrites existing
    }

    #[Cassette('replay_only', record: 'none')]
    public function testReplayOnly(): void
    {
        // Only replays, errors if cassette missing
    }

    #[Cassette('passthrough', mode: 'passthrough')]
    public function testPassthrough(): void
    {
        // Direct connection, no recording/replay
    }
}
```

## Comparison with php-vcr

| Feature | php-vcr | magneto-serge/phpunit |
|---------|---------|----------------------|
| HTTP Recording | ✅ | ✅ |
| WebSocket Recording | ❌ | ✅ |
| PHPUnit Integration | ✅ | ✅ |
| PHP 8 Attributes | ❌ | ✅ |
| Multi-language Support | ❌ | ✅ (Rust, Ruby, JS, etc.) |
| Performance | ~500 req/s | ~5000+ req/s |
| VCR Compatibility | N/A | ✅ |

## Migration from php-vcr

Minimal changes required:

```php
// Before (php-vcr)
use VCR\PHPUnit\TestCase as VCRTestCase;

class ApiTest extends VCRTestCase
{
    /**
     * @vcr github_users
     */
    public function testFetchUsers()
    {
        // ...
    }
}

// After (magneto-serge)
use MagnetoSerge\PHPUnit\MagnetoTestCase;
use MagnetoSerge\PHPUnit\Cassette;

class ApiTest extends MagnetoTestCase
{
    #[Cassette('github_users')]
    public function testFetchUsers(): void
    {
        // Same test code!
    }
}
```

## Requirements

- PHP 8.0 or higher
- PHPUnit 9.0, 10.0, or 11.0
- magneto-serge/magneto-serge ^0.2

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/taciclei/magneto-serge.

## License

The library is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
