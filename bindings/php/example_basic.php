<?php
/**
 * Basic example of using matgto-serge from PHP
 *
 * This example shows how to:
 * 1. Create a proxy in record mode
 * 2. Start recording HTTP/WebSocket interactions
 * 3. Stop recording and save to cassette
 */

require_once __DIR__ . '/MagnetoProxy.php';

use MatgtoSerge\MagnetoProxy;
use MatgtoSerge\ProxyMode;

echo "🎬 matgto-serge PHP Example - Basic Recording\n";
echo str_repeat("=", 50) . "\n\n";

try {
    // Create proxy instance
    echo "1️⃣ Creating proxy...\n";
    $proxy = new MagnetoProxy("./cassettes");
    $proxy->withPort(8888)
          ->withMode(ProxyMode::Record);
    echo "   ✅ Proxy created on port " . $proxy->getPort() . "\n";

    // Start recording
    echo "\n2️⃣ Starting recording...\n";
    $cassetteName = "php-example";
    $proxy->startRecording($cassetteName);
    echo "   ✅ Recording to cassette: $cassetteName\n";

    // Make HTTP requests through proxy
    echo "\n3️⃣ Making HTTP requests through proxy...\n";
    echo "   (Configure your HTTP client to use proxy)\n";
    echo "   Example with Guzzle:\n";
    echo "   \$client = new \\GuzzleHttp\\Client(['proxy' => 'http://localhost:8888']);\n";
    echo "   \$response = \$client->get('https://httpbin.org/get');\n";

    // Simulate some work
    sleep(1);

    // Stop recording
    echo "\n4️⃣ Stopping recording...\n";
    $proxy->stopRecording();
    echo "   ✅ Cassette saved: ./cassettes/{$cassetteName}.json\n";

    // Shutdown proxy
    echo "\n5️⃣ Shutting down proxy...\n";
    $proxy->shutdown();
    echo "   ✅ Proxy stopped\n";

    echo "\n" . str_repeat("=", 50) . "\n";
    echo "✅ Example complete!\n";

} catch (Exception $e) {
    echo "\n❌ Error: " . $e->getMessage() . "\n";
    exit(1);
}
