# matgto-serge PHP Bindings

PHP bindings for [matgto-serge](https://github.com/your-org/matgto-serge) - HTTP/WebSocket testing library with record/replay capabilities.

## Requirements

- PHP 8.1 or higher
- FFI extension enabled
- matgto-serge Rust library compiled

## Installation

### Via Composer

```bash
composer require matgto/serge
```

### Manual Installation

1. Build the Rust library:
   ```bash
   cd ../../
   cargo build --release
   ```

2. Copy `MatgtoProxy.php` to your project

3. Require it in your code:
   ```php
   require_once 'path/to/MatgtoProxy.php';
   ```

## Quick Start

### Basic Recording

```php
<?php
use MatgtoSerge\MatgtoProxy;
use MatgtoSerge\ProxyMode;

// Create proxy
$proxy = new MatgtoProxy('./cassettes');
$proxy->withPort(8888)
      ->withMode(ProxyMode::Record);

// Start recording
$proxy->startRecording('api-test');

// Make HTTP requests through proxy (configure your HTTP client)
// Example with Guzzle:
$client = new \GuzzleHttp\Client(['proxy' => 'http://localhost:8888']);
$response = $client->get('https://api.example.com/users');

// Stop recording
$proxy->stopRecording();
$proxy->shutdown();
```

### Replay Mode

```php
<?php
use MatgtoSerge\MatgtoProxy;
use MatgtoSerge\ProxyMode;

// Create proxy in replay mode
$proxy = new MatgtoProxy('./cassettes');
$proxy->withPort(8888)
      ->withMode(ProxyMode::Replay);

// Replay cassette
$proxy->replay('api-test');

// Make same requests - will be served from cassette
$client = new \GuzzleHttp\Client(['proxy' => 'http://localhost:8888']);
$response = $client->get('https://api.example.com/users');

$proxy->shutdown();
```

### Auto Mode (Recommended)

```php
<?php
// Auto mode: records if cassette doesn't exist, replays if it does
$proxy = new MatgtoProxy('./cassettes');
$proxy->withPort(8888)
      ->withMode(ProxyMode::Auto);

$proxy->startRecording('my-test');
// ... make requests ...
$proxy->stopRecording();
```

## PHPUnit Integration

```php
<?php
use PHPUnit\Framework\TestCase;
use MatgtoSerge\MatgtoProxy;
use MatgtoSerge\ProxyMode;

class ApiTest extends TestCase
{
    private MatgtoProxy $proxy;

    protected function setUp(): void
    {
        $this->proxy = new MatgtoProxy('./test-cassettes');
        $this->proxy->withPort(8888)
                    ->withMode(ProxyMode::Auto);
    }

    protected function tearDown(): void
    {
        $this->proxy->shutdown();
    }

    public function testApiEndpoint(): void
    {
        $this->proxy->startRecording('users-api');

        $client = new \GuzzleHttp\Client([
            'proxy' => 'http://localhost:8888'
        ]);

        $response = $client->get('https://api.example.com/users');

        $this->assertEquals(200, $response->getStatusCode());

        $this->proxy->stopRecording();
    }
}
```

## API Reference

### MatgtoProxy

#### Constructor

```php
new MatgtoProxy(string $cassetteDir)
```

Create a new proxy instance.

**Parameters:**
- `$cassetteDir` - Directory to store cassettes

#### Methods

##### withPort(int $port): self

Set the proxy port (default: 8888).

```php
$proxy->withPort(9000);
```

##### withMode(ProxyMode $mode): self

Set the operating mode.

```php
$proxy->withMode(ProxyMode::Record);
```

**Modes:**
- `ProxyMode::Auto` - Record if cassette doesn't exist, replay if it does
- `ProxyMode::Record` - Always record
- `ProxyMode::Replay` - Always replay from cassette
- `ProxyMode::Passthrough` - Transparent proxy (no recording)

##### startRecording(string $cassetteName): void

Start recording to a cassette.

```php
$proxy->startRecording('my-api-test');
```

##### stopRecording(): void

Stop recording and save cassette.

```php
$proxy->stopRecording();
```

##### replay(string $cassetteName): void

Load and replay a cassette.

```php
$proxy->replay('my-api-test');
```

##### getPort(): int

Get current proxy port.

```php
$port = $proxy->getPort(); // 8888
```

##### getMode(): ProxyMode

Get current operating mode.

```php
$mode = $proxy->getMode(); // ProxyMode::Auto
```

##### shutdown(): void

Shutdown the proxy server.

```php
$proxy->shutdown();
```

## HTTP Client Configuration

### Guzzle

```php
use GuzzleHttp\Client;

$client = new Client([
    'proxy' => 'http://localhost:8888',
    'verify' => false, // Disable SSL verification for MITM
]);
```

### cURL

```php
$ch = curl_init('https://api.example.com/users');
curl_setopt($ch, CURLOPT_PROXY, 'http://localhost:8888');
curl_setopt($ch, CURLOPT_SSL_VERIFYPEER, false); // MITM
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
$response = curl_exec($ch);
curl_close($ch);
```

### Symfony HttpClient

```php
use Symfony\Component\HttpClient\HttpClient;

$client = HttpClient::create([
    'proxy' => 'http://localhost:8888',
    'verify_peer' => false,
]);
```

## Laravel Integration

Create a service provider:

```php
<?php
namespace App\Providers;

use Illuminate\Support\ServiceProvider;
use MatgtoSerge\MatgtoProxy;
use MatgtoSerge\ProxyMode;

class MatgtoServiceProvider extends ServiceProvider
{
    public function register(): void
    {
        $this->app->singleton(MatgtoProxy::class, function ($app) {
            $proxy = new MatgtoProxy(storage_path('cassettes'));
            $proxy->withPort(8888)
                  ->withMode(ProxyMode::Auto);
            return $proxy;
        });
    }
}
```

Use in tests:

```php
<?php
namespace Tests\Feature;

use Tests\TestCase;
use MatgtoSerge\MatgtoProxy;

class ApiTest extends TestCase
{
    public function test_api_call()
    {
        $proxy = app(MatgtoProxy::class);
        $proxy->startRecording('api-test');

        // Make API calls...

        $proxy->stopRecording();
    }
}
```

## WebSocket Support

```php
<?php
// WebSocket recording/replay is supported
// Configure your WebSocket client to use the proxy

$proxy = new MatgtoProxy('./cassettes');
$proxy->withPort(8888)
      ->withMode(ProxyMode::Record);

$proxy->startRecording('websocket-test');

// Use your WebSocket client with proxy
// Example with Ratchet or similar

$proxy->stopRecording();
```

## Troubleshooting

### FFI Extension Not Loaded

Enable FFI in `php.ini`:

```ini
extension=ffi
ffi.enable=true
```

### Library Not Found

Ensure the Rust library is compiled:

```bash
cd /path/to/matgto-serge
cargo build --release
```

The library should be at:
- macOS: `target/release/libmatgto_serge.dylib`
- Linux: `target/release/libmatgto_serge.so`
- Windows: `target/release/matgto_serge.dll`

### SSL Certificate Errors

For MITM proxy to work with HTTPS, you need to:

1. Trust the matgto certificate
2. Or disable SSL verification in your HTTP client (not recommended for production)

```php
$client = new \GuzzleHttp\Client([
    'proxy' => 'http://localhost:8888',
    'verify' => false, // Disable SSL verification
]);
```

## Examples

See the `examples/` directory:
- `example_basic.php` - Basic record/replay
- `example_replay.php` - Replay mode demonstration
- `example_phpunit.php` - PHPUnit integration

Run examples:

```bash
php example_basic.php
php example_replay.php
```

## Performance

The PHP bindings use FFI to call the Rust library directly, providing excellent performance:

- Minimal overhead
- Native Rust speed
- Efficient memory usage

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please see the main [matgto-serge repository](https://github.com/your-org/matgto-serge).

## Links

- [Main Documentation](../../README.md)
- [Rust API](../../docs/)
- [Other Language Bindings](../)
