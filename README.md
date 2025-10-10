# Magnéto-Serge

Multi-language HTTP/WebSocket testing library with record/replay capabilities.

> 🚀 **VCR for the modern web** - Record HTTP/HTTPS and WebSocket traffic, replay it deterministically. Written in Rust for maximum performance.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## 🎯 Features

- ✅ **HTTP/HTTPS Proxy** - MITM interception with automatic TLS certificate generation
- ✅ **WebSocket Support** - Record and replay bidirectional WebSocket messages
- ✅ **Multi-Language** - Python, Kotlin, Swift, Java, JavaScript (via UniFFI + wrappers)
- 🚀 **High Performance** - Built in Rust for maximum speed and safety
- 📦 **Universal Cassette Format** - Share test fixtures across languages (JSON)
- 🎯 **Zero Config** - Auto-detect record vs replay mode

## 📦 Installation

**Magnéto-Serge** is available in **5 languages** :

### 🐍 Python (PyPI)

```bash
pip install matgto-serge
```

### ☕ Java (Gradle)

```gradle
dependencies {
    implementation 'io.github.matgto:serge:0.1.0'
}
```

### 🟨 JavaScript/TypeScript (npm)

```bash
npm install @matgto/serge
```

### 🟣 Kotlin (Gradle)

```gradle
dependencies {
    implementation("io.github.matgto:serge:0.1.0")
}
```

### 🍎 Swift (SPM)

```swift
dependencies: [
    .package(url: "https://github.com/matgto/serge-swift", from: "0.1.0")
]
```

### 🦀 Rust (Cargo)

```toml
[dependencies]
magneto-serge = "0.1"
```

**📚 Voir [BINDINGS.md](BINDINGS.md) pour la documentation complète de chaque langage.**

## 🚀 Quick Start

### Rust

```rust
use magneto_serge::{MatgtoProxy, ProxyMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut proxy = MatgtoProxy::new("./cassettes")?
        .with_port(8888)
        .with_mode(ProxyMode::Auto);
    
    // Start recording
    proxy.start_recording("my-api-test")?;
    
    // Your HTTP/WebSocket requests go through localhost:8888
    // ... make requests ...
    
    // Stop and save cassette
    proxy.stop_recording()?;
    
    Ok(())
}
```

### Java (JUnit 5)

```java
@Test
public void testWithMatgto() {
    MatgtoProxy proxy = new MatgtoProxy("./cassettes");
    proxy.startRecording("my-api-test");
    
    // Configure your HTTP client to use proxy localhost:8888
    // ... make requests ...
    
    proxy.stopRecording();
}
```

### JavaScript (Jest)

```javascript
const { MatgtoProxy } = require('@matgto/serge');

test('API with matgto', async () => {
  const proxy = new MatgtoProxy('./cassettes');
  proxy.startRecording('my-api-test');
  
  // Make HTTP/WebSocket requests through proxy
  const response = await fetch('https://api.example.com/users', {
    proxy: { host: 'localhost', port: 8888 }
  });
  
  proxy.stopRecording();
});
```

### Python (pytest)

```python
from matgto_serge import MatgtoProxy

def test_api_with_matgto():
    proxy = MatgtoProxy(cassette_dir="./cassettes")
    proxy.start_recording("my-api-test")
    
    # Configure requests to use proxy
    response = requests.get(
        "https://api.example.com/users",
        proxies={"https": "http://localhost:8888"}
    )
    
    proxy.stop_recording()
```

## 🎬 How It Works

1. **Record Mode**: Proxy intercepts HTTP/HTTPS/WebSocket traffic and saves to cassette
2. **Replay Mode**: Proxy matches incoming requests and returns saved responses
3. **Auto Mode**: Records if cassette doesn't exist, otherwise replays

```
┌─────────────┐        ┌──────────────┐        ┌──────────┐
│  Your App   │───────▶│ matgto-serge │───────▶│   API    │
│             │        │    (proxy)   │        │  Server  │
│             │◀───────│  Port 8888   │◀───────│          │
└─────────────┘        └──────────────┘        └──────────┘
                              │
                              ▼
                       ┌─────────────┐
                       │  cassette   │
                       │   (JSON)    │
                       └─────────────┘
```

## 📋 Cassette Format

Cassettes are stored as JSON (or MessagePack for binary efficiency):

```json
{
  "version": "1.0",
  "name": "my-api-test",
  "recorded_at": "2025-10-10T14:30:00Z",
  "interactions": [
    {
      "type": "Http",
      "request": {
        "method": "GET",
        "url": "https://api.example.com/users",
        "headers": {},
        "body": null
      },
      "response": {
        "status": 200,
        "headers": {"content-type": "application/json"},
        "body": [...]
      }
    }
  ]
}
```

## 🛠️ Development

```bash
# Clone repository
git clone https://github.com/your-org/matgto-serge
cd matgto-serge

# Build
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Generate bindings
cargo run --features=uniffi/cli --bin uniffi-bindgen -- \
  generate src/matgto_serge.udl --language java --out-dir bindings/java
```

## 📚 Documentation

- **[BINDINGS.md](BINDINGS.md)** - 🌐 Multi-language bindings complete guide
- **[ROADMAP.md](ROADMAP.md)** - 🗺️ Development plan and milestones
- **[MULTI_LANGUAGE_SUMMARY.md](MULTI_LANGUAGE_SUMMARY.md)** - 🎉 Multi-language implementation summary
- **Language-specific docs:**
  - [Python README](bindings/python/README.md) 🐍
  - [Java README](bindings/java/README.md) ☕
  - [JavaScript README](bindings/javascript/README.md) 🟨
  - [Kotlin README](bindings/kotlin/README.md) 🟣
  - [Swift README](bindings/swift/README.md) 🍎

## 🎯 Project Status

**Current Phase:** Phase 3 - Multi-language Bindings (85% complete)

| Phase | Status | Progress |
|-------|--------|----------|
| **Phase 1** - HTTP/HTTPS Proxy | ✅ Complete | 100% |
| **Phase 2** - WebSocket Support | ✅ Complete | 100% |
| **Phase 3** - Multi-language Bindings | 🟡 In Progress | 85% |
| **Phase 4** - CLI & Production | ⏳ Pending | 0% |

**Recent achievements:**
- ✅ UniFFI integration complete
- ✅ Python bindings (tested - 4/4 ✓)
- ✅ Kotlin bindings (generated)
- ✅ Swift bindings (generated)
- ✅ Java bindings (wrapper created + tests)
- ✅ JavaScript bindings (wrapper created + tests)

**Next steps:**
- ⏳ Package distribution (PyPI, Maven, NPM)
- ⏳ CLI tool with clap
- ⏳ CI/CD setup

See [ROADMAP.md](ROADMAP.md) for complete roadmap.

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🌟 Acknowledgments

Inspired by:
- [VCR](https://github.com/vcr/vcr) (Ruby)
- [VHS](https://github.com/diegoalvarado/vhs) (Ruby)  
- [Polly](https://github.com/Netflix/polly-js) (JavaScript)

Built with:
- [Hudsucker](https://github.com/omjadas/hudsucker) - HTTP/HTTPS MITM proxy
- [UniFFI](https://github.com/mozilla/uniffi-rs) - Multi-language bindings
- [Tokio](https://tokio.rs/) - Async runtime

---

**Made with ❤️ in Rust**
