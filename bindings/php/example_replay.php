<?php
/**
 * Replay example of using matgto-serge from PHP
 *
 * This example shows how to:
 * 1. Create a proxy in replay mode
 * 2. Replay a previously recorded cassette
 * 3. Verify requests are served from cassette
 */

require_once __DIR__ . '/MatgtoProxy.php';

use MatgtoSerge\MatgtoProxy;
use MatgtoSerge\ProxyMode;

echo "▶️  matgto-serge PHP Example - Replay\n";
echo str_repeat("=", 50) . "\n\n";

try {
    // Create proxy instance in replay mode
    echo "1️⃣ Creating proxy in REPLAY mode...\n";
    $proxy = new MatgtoProxy("./cassettes");
    $proxy->withPort(8888)
          ->withMode(ProxyMode::Replay);
    echo "   ✅ Proxy created on port " . $proxy->getPort() . "\n";
    echo "   ✅ Mode: " . $proxy->getMode()->value . "\n";

    // Replay cassette
    echo "\n2️⃣ Loading cassette for replay...\n";
    $cassetteName = "php-example";
    $proxy->replay($cassetteName);
    echo "   ✅ Replaying cassette: $cassetteName\n";

    // Make HTTP requests (will be served from cassette)
    echo "\n3️⃣ Making HTTP requests...\n";
    echo "   Requests will be served from cassette (no real network calls)\n";
    echo "   Configure your HTTP client:\n\n";

    echo "   Example with Guzzle:\n";
    echo "   ```php\n";
    echo "   use GuzzleHttp\\Client;\n";
    echo "   \n";
    echo "   \$client = new Client([\n";
    echo "       'proxy' => 'http://localhost:8888'\n";
    echo "   ]);\n";
    echo "   \n";
    echo "   \$response = \$client->get('https://httpbin.org/get');\n";
    echo "   echo \$response->getBody();\n";
    echo "   ```\n";

    echo "\n   Example with cURL:\n";
    echo "   ```php\n";
    echo "   \$ch = curl_init('https://httpbin.org/get');\n";
    echo "   curl_setopt(\$ch, CURLOPT_PROXY, 'http://localhost:8888');\n";
    echo "   curl_setopt(\$ch, CURLOPT_RETURNTRANSFER, true);\n";
    echo "   \$response = curl_exec(\$ch);\n";
    echo "   curl_close(\$ch);\n";
    echo "   echo \$response;\n";
    echo "   ```\n";

    // Shutdown
    echo "\n4️⃣ Shutting down proxy...\n";
    $proxy->shutdown();
    echo "   ✅ Proxy stopped\n";

    echo "\n" . str_repeat("=", 50) . "\n";
    echo "✅ Replay example complete!\n";

} catch (Exception $e) {
    echo "\n❌ Error: " . $e->getMessage() . "\n";
    exit(1);
}
