# matgto-serge Language Bindings

This directory contains language bindings for matgto-serge, generated using [UniFFI](https://mozilla.github.io/uniffi-rs/).

## Supported Languages

- 🐍 **Python** - `python/`
- 🐘 **PHP** - `php/` ✨ NEW
- ☕ **Java** - `java/` (via Kotlin)
- 🤖 **Kotlin** - `kotlin/`
- 🍎 **Swift** - `swift/`

## Generating Bindings

### Prerequisites

1. Build the Rust library:
   ```bash
   cargo build --release
   ```

2. Install `uniffi-bindgen` CLI (if not already installed):
   ```bash
   cargo install uniffi-bindgen
   ```

### Generate All Bindings

Run the generation script:

```bash
cd bindings
./generate.sh
```

This will generate bindings for all supported languages.

### Generate Individual Language

You can also generate bindings for a specific language:

```bash
# Python
uniffi-bindgen generate \
    --library ../target/release/libmatgto_serge.dylib \
    --language python \
    --out-dir python/ \
    ../src/matgto_serge.udl

# Kotlin
uniffi-bindgen generate \
    --library ../target/release/libmatgto_serge.dylib \
    --language kotlin \
    --out-dir kotlin/ \
    ../src/matgto_serge.udl

# Swift
uniffi-bindgen generate \
    --library ../target/release/libmatgto_serge.dylib \
    --language swift \
    --out-dir swift/ \
    ../src/matgto_serge.udl
```

Note: Replace `.dylib` with `.so` on Linux or `.dll` on Windows.

## Usage Examples

### Python

```python
from matgto_serge import create_proxy, ProxyMode

# Create proxy in record mode
proxy = create_proxy("./cassettes")
proxy = proxy.with_port(8888)
proxy = proxy.with_mode(ProxyMode.RECORD)

# Start recording
proxy.start_recording("api-test")

# ... make HTTP/WebSocket requests through proxy ...

# Stop recording
proxy.stop_recording()
```

### PHP

```php
<?php
use MatgtoSerge\MagnetoProxy;
use MatgtoSerge\ProxyMode;

// Create proxy in record mode
$proxy = new MagnetoProxy("./cassettes");
$proxy->withPort(8888)
      ->withMode(ProxyMode::Record);

// Start recording
$proxy->startRecording("api-test");

// ... make HTTP/WebSocket requests through proxy ...
// Example with Guzzle:
$client = new \GuzzleHttp\Client(['proxy' => 'http://localhost:8888']);
$response = $client->get('https://api.example.com/users');

// Stop recording
$proxy->stopRecording();
$proxy->shutdown();
```

### Kotlin

```kotlin
import matgto_serge.*

fun main() {
    // Create proxy in replay mode
    val proxy = createProxy("./cassettes")
        .withPort(8888)
        .withMode(ProxyMode.REPLAY)

    // Replay cassette
    proxy.replay("api-test")

    // ... make HTTP/WebSocket requests ...

    proxy.shutdown()
}
```

### Swift

```swift
import matgto_serge

// Create proxy in auto mode
let proxy = try createProxy(cassetteDir: "./cassettes")
    .withPort(port: 8888)
    .withMode(mode: .auto)

// Start recording
try proxy.startRecording(cassetteName: "api-test")

// ... make requests ...

try proxy.stopRecording()
```

## Package Distribution

### Python (PyPI)

Create a `setup.py` and package:

```bash
cd python/
python setup.py sdist bdist_wheel
pip install dist/matgto_serge-*.whl
```

### PHP (Packagist)

Publish to Packagist:

```bash
cd php/
# Ensure composer.json is configured
composer validate

# Install locally
composer require matgto/serge

# Or install from Packagist (once published)
composer global require matgto/serge
```

Usage:
```php
<?php
require 'vendor/autoload.php';

use MatgtoSerge\MagnetoProxy;
$proxy = new MagnetoProxy('./cassettes');
```

### Kotlin/Java (Maven)

Use Gradle or Maven to package the Kotlin bindings:

```gradle
plugins {
    kotlin("jvm")
}

dependencies {
    implementation(files("libs/matgto_serge.jar"))
}
```

### Swift (CocoaPods/SPM)

Create a Swift Package or CocoaPods spec:

```swift
// Package.swift
let package = Package(
    name: "matgto-serge",
    products: [
        .library(name: "MatgtoSerge", targets: ["MatgtoSerge"])
    ],
    targets: [
        .target(name: "MatgtoSerge")
    ]
)
```

## Architecture

The bindings are generated from `src/matgto_serge.udl`, which defines the public API.

```
Rust Core (matgto-serge)
    ↓
UniFFI UDL Definition
    ↓
Generated Bindings
    ├─ Python
    ├─ Kotlin/Java
    └─ Swift
```

## Contributing

To add support for a new language:

1. Add language-specific directory: `bindings/your-language/`
2. Update `generate.sh` script
3. Create language-specific examples
4. Update this README

## Documentation

- [UniFFI Documentation](https://mozilla.github.io/uniffi-rs/)
- [matgto-serge API Reference](../docs/)

## License

MIT OR Apache-2.0
