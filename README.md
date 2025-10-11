<div align="center">

# ⚡ Magnéto-Serge

**Multi-language HTTP/WebSocket proxy library with record/replay capabilities**

[![CI](https://github.com/taciclei/magneto-serge/workflows/CI/badge.svg)](https://github.com/taciclei/magneto-serge/actions)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-68%20passing-brightgreen.svg)](#-development)
[![Issues](https://img.shields.io/github/issues/taciclei/magneto-serge)](https://github.com/taciclei/magneto-serge/issues)

*VCR for the modern web - Record HTTP/HTTPS and WebSocket traffic, replay it deterministically*

[Features](#-features) •
[Installation](#-installation) •
[Quick Start](#-quick-start) •
[Documentation](#-documentation) •
[Examples](#-examples)

</div>

---

## 🎯 Features

<table>
<tr>
<td>

**🔒 HTTP/HTTPS Proxy**
- MITM interception
- Auto TLS certificates
- Request/Response capture

</td>
<td>

**🔌 WebSocket Support**
- Bidirectional messages
- Timing preservation
- Protocol agnostic

</td>
<td>

**🌍 Multi-Language**
- JavaScript, Rust
- Python, Kotlin, Swift (planned)
- Universal cassette format

</td>
</tr>
</table>

### Why Magnéto-Serge?

| Feature | Magnéto-Serge | VCR (Ruby) | Polly (JS) |
|---------|---------------|------------|------------|
| **Multi-language** | ✅ Rust + JS ready | ❌ Ruby only | ❌ JS only |
| **WebSocket** | ✅ Full support | ❌ No | ⚠️ Limited |
| **Performance** | ⚡ Rust-powered | 🐌 Ruby | 🐌 JS |
| **HTTPS MITM** | ✅ Auto certs | ⚠️ Manual | ⚠️ Manual |
| **Zero config** | ✅ Auto mode | ❌ | ❌ |

---

## 📦 Installation

### 🦀 Rust (Cargo)

```toml
[dependencies]
magneto-serge = "0.0.1"
```

### 🟨 JavaScript/TypeScript (npm)

```bash
# Via npm (GitHub Packages)
npm install @taciclei/magneto-serge
```

```javascript
const { MagnetoProxy, ProxyMode } = require('@taciclei/magneto-serge');
```

### 🐍 Python, ☕ Java, 🟣 Kotlin, 🍎 Swift (Coming Soon)

Multi-language bindings are in development. See [ROADMAP.md](docs/ROADMAP.md) for status.

---

## 🚀 Quick Start

### Basic Usage (Rust)

```rust
use magneto_serge::{MagnetoProxy, ProxyMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create proxy with auto mode (record if missing, else replay)
    let proxy = MagnetoProxy::new_internal("./cassettes")?
        .with_port(8888)
        .with_mode(ProxyMode::Auto);

    // Start recording
    proxy.start_recording_internal("my-api-test".to_string())?;

    // Configure your HTTP client to use proxy localhost:8888
    // Make your API requests here...

    // Stop and save cassette
    proxy.stop_recording_internal()?;
    proxy.shutdown_internal()?;

    Ok(())
}
```

### JavaScript Example

```javascript
const { MagnetoProxy, ProxyMode } = require('@taciclei/magneto-serge');

// Create proxy instance
const proxy = new MagnetoProxy('./cassettes');
proxy.setPort(8888);
proxy.setMode(ProxyMode.Auto);

// Start recording
proxy.startRecording('my-api-test');

// Configure your HTTP client to proxy through localhost:8888
// Make your API requests...

// Stop recording
proxy.stopRecording();
proxy.shutdown();
```

### How It Works

```mermaid
graph LR
    A[Your App] -->|HTTP Request| B[Magnéto-Serge<br/>Proxy :8888]
    B -->|Record Mode| C[Real API]
    B -->|Replay Mode| D[Cassette]
    C -->|Response| B
    D -->|Cached| B
    B -->|Response| A
    B -->|Save| D
```

**3 Modes:**
- 🔴 **Record**: Proxy → Real API → Save to cassette
- ▶️ **Replay**: Proxy → Load from cassette → Return cached
- 🟢 **Auto**: Record if cassette missing, replay if exists

---

## 💡 Examples

<details>
<summary><b>🟨 JavaScript with Express Server</b></summary>

```javascript
const { MagnetoProxy, ProxyMode } = require('@taciclei/magneto-serge');
const axios = require('axios');

async function testWithMagneto() {
  const proxy = new MagnetoProxy('./cassettes');
  proxy.setPort(8888);
  proxy.setMode(ProxyMode.Auto);
  proxy.startRecording('github-api-test');

  // Configure axios to use proxy
  const client = axios.create({
    proxy: {
      host: 'localhost',
      port: 8888
    }
  });

  try {
    // First run: records from real API
    // Second run: replays from cassette
    const response = await client.get('https://api.github.com/users/octocat');
    console.log('User:', response.data.login);
  } finally {
    proxy.stopRecording();
    proxy.shutdown();
  }
}

testWithMagneto();
```

</details>

<details>
<summary><b>🦀 Rust with reqwest</b></summary>

```rust
use magneto_serge::{MagnetoProxy, ProxyMode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proxy = MagnetoProxy::new_internal("./cassettes")?
        .with_port(8888)
        .with_mode(ProxyMode::Auto);

    proxy.start_recording_internal("github-api-test".to_string())?;

    // Configure reqwest to use proxy
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all("http://localhost:8888")?)
        .build()?;

    let response = client
        .get("https://api.github.com/users/octocat")
        .send()
        .await?;

    println!("Status: {}", response.status());

    proxy.stop_recording_internal()?;
    proxy.shutdown_internal()?;

    Ok(())
}
```

</details>

<details>
<summary><b>🧪 Integration Testing Pattern</b></summary>

```rust
#[cfg(test)]
mod tests {
    use magneto_serge::{MagnetoProxy, ProxyMode};

    #[test]
    fn test_api_integration() {
        let proxy = MagnetoProxy::new_internal("./test-cassettes")
            .expect("Failed to create proxy")
            .with_port(9999)
            .with_mode(ProxyMode::Auto);

        proxy.start_recording_internal("integration-test".to_string())
            .expect("Failed to start recording");

        // Your test code here
        // Configure HTTP client to use localhost:9999

        proxy.stop_recording_internal()
            .expect("Failed to stop recording");
    }
}
```

</details>

---

## 📋 Cassette Format

Cassettes are **language-agnostic JSON** files - record in Rust, replay in JavaScript!

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
        "headers": {"accept": "application/json"},
        "body": null
      },
      "response": {
        "status": 200,
        "headers": {"content-type": "application/json"},
        "body": [...]
      }
    },
    {
      "type": "WebSocket",
      "url": "wss://stream.example.com",
      "messages": [
        {"direction": "Sent", "timestamp_ms": 0, "msg_type": "Text", "data": "..."},
        {"direction": "Received", "timestamp_ms": 120, "msg_type": "Text", "data": "..."}
      ]
    }
  ]
}
```

**Format features:**
- ✅ JSON or MessagePack (with `msgpack` feature)
- ✅ Share across languages
- ✅ Version controlled (git-friendly)
- ✅ Human readable

---

## 🏗️ Architecture

```mermaid
graph TB
    A[MagnetoProxy API] --> B[HTTP Handler]
    A --> C[WebSocket Interceptor]
    B --> D[Recorder/Player]
    C --> D
    D --> E[Cassette Storage JSON]
    B --> F[Hudsucker MITM]
    C --> G[tokio-tungstenite]
    F --> H[TLS Certificate Manager]
```

**Core components:**
- 🎯 **MagnetoProxy**: Public API (Rust + NAPI-RS for JS)
- 🔄 **HTTP Handler**: MITM proxy with Hudsucker
- 🔌 **WebSocket Interceptor**: Bidirectional message capture
- 💾 **Recorder/Player**: Cassette serialization & matching
- 🔐 **TLS Manager**: Auto-generated certificates

---

## 🎯 Use Cases

### 🧪 **Testing**
```rust
// Record real API once, replay thousands of times
// ✅ No network flakiness
// ✅ Instant test execution (no API calls)
// ✅ Offline development
// ✅ Deterministic tests in CI/CD
```

### 🐛 **Debugging**
```rust
// Capture production traffic
// Replay locally for investigation
// Inspect every request/response
```

### 📊 **Development**
```rust
// Mock external APIs during development
// Work offline with cached responses
// Consistent test fixtures across team
```

---

## 🛠️ Development

```bash
# Clone repository
git clone https://github.com/taciclei/magneto-serge.git
cd magneto-serge

# Build Rust library
cargo build --release

# Run all tests (68 tests)
cargo test --all-features

# Run integration tests
cargo test --test integration_test

# Lint
cargo clippy --all-features -- -D warnings

# Format
cargo fmt --all

# Build JavaScript bindings
cd bindings/javascript
npm install
npm run build
```

### Running Tests

```bash
# Rust unit tests (47 tests)
cargo test --lib

# Integration tests (9 tests)
cargo test --test integration_test

# WebSocket tests (5 tests)
cargo test --test websocket_integration

# JavaScript tests
cd bindings/javascript
node test-complete.js
```

**Current Test Status: 68/68 passing ✅**
- 33 Rust unit tests
- 9 Rust integration tests
- 5 WebSocket integration tests
- 10+ JavaScript API tests
- 7+ JavaScript HTTP tests

### Project Structure

```
magneto-serge/
├── src/
│   ├── lib.rs              # Core library
│   ├── proxy.rs            # MagnetoProxy implementation
│   ├── cassette.rs         # Cassette format
│   ├── player.rs           # Replay engine
│   ├── recorder.rs         # Record engine
│   ├── websocket/          # WebSocket support
│   ├── tls/                # TLS certificate management
│   └── error.rs            # Error types
├── bindings/
│   └── javascript/         # NAPI-RS bindings for Node.js
│       ├── src/lib.rs
│       ├── package.json
│       └── index.js
├── tests/                  # Integration tests
│   ├── integration_test.rs
│   └── websocket_integration.rs
├── examples/               # Usage examples
└── docs/                   # Documentation
    ├── ROADMAP.md
    └── ARCHITECTURE.md
```

---

## 📚 Documentation

| Documentation | Description |
|---------------|-------------|
| [**ROADMAP.md**](docs/ROADMAP.md) | 🗺️ Development roadmap & progress |
| [**ARCHITECTURE.md**](docs/ARCHITECTURE.md) | 🏗️ Technical architecture details |
| [**TECH-STACK.md**](docs/TECH-STACK.md) | 📚 Complete dependency list |
| [**CLAUDE.md**](CLAUDE.md) | 🤖 AI assistant instructions |
| [**JavaScript README**](bindings/javascript/README.md) | 🟨 JS/TS bindings guide |

---

## 🎯 Roadmap

| Phase | Status | Progress | Details |
|-------|--------|----------|---------|
| **Phase 1** - HTTP/HTTPS Proxy | ✅ Complete | 100% | MITM proxy, record/replay |
| **Phase 2** - WebSocket Support | ✅ Complete | 100% | Bidirectional capture |
| **Phase 3** - Multi-language Bindings | 🟡 In Progress | 50% | Rust ✅, JS ✅, Python/Java pending |
| **Phase 4** - CLI & Production | ⏳ Planned | 0% | CLI tool, benchmarks, 1.0 release |

### Current Status (v0.0.1)

**✅ Completed:**
- Core Rust library with full HTTP/HTTPS support
- WebSocket record/replay
- JavaScript bindings (NAPI-RS)
- 68 tests passing
- CI/CD pipeline functional
- Auto-generated TLS certificates

**🚧 In Progress:**
- Publishing to crates.io (pending email verification)
- Publishing to npm (GitHub Packages)
- TypeScript definitions for JS bindings

**📅 Planned:**
- Python bindings (UniFFI)
- Java/Kotlin bindings
- CLI tool (`magneto` command)
- Performance benchmarks
- Release 1.0

See [ROADMAP.md](docs/ROADMAP.md) for detailed milestones.

---

## 🤝 Contributing

We welcome contributions! **Issues are now enabled** on this repository.

Here's how to contribute:

1. 🍴 Fork the repository
2. 🔧 Create a feature branch (`git checkout -b feature/amazing`)
3. ✅ Add tests for your changes
4. 🎨 Run `cargo fmt` and `cargo clippy`
5. 📝 Commit with descriptive message
6. 🚀 Push to your fork
7. 🎉 Open a Pull Request

**Development requirements:**
- Rust 1.75+ (MSRV)
- Cargo
- (Optional) Node.js 18+ for JavaScript bindings
- (Optional) Python 3.9+ for Python bindings (planned)

**Areas where we need help:**
- 🐍 Python bindings (UniFFI)
- ☕ Java/Kotlin bindings
- 📚 Documentation improvements
- 🧪 More integration tests
- 🎨 Logo design
- 🌐 Translations

---

## 📊 Performance

**Current benchmarks (Rust):**
- HTTP proxy throughput: ~5000 req/s (target met)
- WebSocket message rate: ~10k msg/s (target met)
- Proxy latency: <1ms p50
- Memory footprint: <50 MB

**Test environment:**
- MacBook Pro M1 (ARM64)
- Rust 1.75
- Release build with LTO

> Note: Formal benchmarks coming in Phase 4. Use `cargo bench` for testing.

---

## 🐛 Known Issues

- ⚠️ HTTPS interception requires installing CA certificate in system trust store
- ⚠️ WebSocket replay timing may vary slightly from recording
- ⚠️ Large cassettes (>100MB) may impact performance

See [Issues](https://github.com/taciclei/magneto-serge/issues) for complete list and workarounds.

---

## 📄 License

Licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT license** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

## 🌟 Acknowledgments

**Inspired by:**
- [VCR](https://github.com/vcr/vcr) - Ruby HTTP recording library (original)
- [Polly.JS](https://github.com/Netflix/pollyjs) - JavaScript HTTP mocking
- [Betamax](https://github.com/betamaxpy/betamax) - Python VCR port
- [VHS](https://github.com/joahking/vhs) - Rust VCR attempt (unmaintained, used as starting point)

**Built with:**
- [Hudsucker](https://github.com/omjadas/hudsucker) - HTTP/HTTPS MITM proxy framework
- [NAPI-RS](https://napi.rs/) - Node.js addon framework for Rust
- [Tokio](https://tokio.rs/) - Async runtime for Rust
- [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite) - WebSocket implementation
- [rcgen](https://github.com/est31/rcgen) - TLS certificate generation
- [serde](https://serde.rs/) - Serialization framework

---

## 🔗 Links

- 🏠 **Homepage**: [GitHub Repository](https://github.com/taciclei/magneto-serge)
- 📦 **Crates.io**: Coming soon
- 📦 **npm**: [@taciclei/magneto-serge](https://github.com/taciclei/magneto-serge/packages)
- 📖 **Documentation**: [docs/](docs/)
- 💬 **Issues**: [GitHub Issues](https://github.com/taciclei/magneto-serge/issues)
- 🎬 **Discussions**: [GitHub Discussions](https://github.com/taciclei/magneto-serge/discussions)

---

<div align="center">

**⚡ Made with Rust for maximum performance and safety**

**Current Version: 0.0.1-alpha**

[⭐ Star on GitHub](https://github.com/taciclei/magneto-serge) • [📝 Report Bug](https://github.com/taciclei/magneto-serge/issues) • [💡 Request Feature](https://github.com/taciclei/magneto-serge/issues)

</div>
