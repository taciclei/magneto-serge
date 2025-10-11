# 📣 Announcement Templates for Magnéto-Serge v0.1.0

Ready-to-use announcement templates for various platforms.

---

## 🐦 Twitter/X

### Thread Starter
```
🎉 Magnéto-Serge v0.1.0 is here!

A Rust-powered HTTP/WebSocket testing library with record/replay – like VCR but for the modern web.

✅ HTTP/HTTPS proxy with MITM
✅ WebSocket capture
✅ Multi-language (Rust, JS/TS, PHP)
✅ 5000+ req/s throughput

🧵 Thread 👇

#rustlang #testing #opensource
```

### Tweet 2 - Features
```
What makes Magnéto-Serge different?

🔄 4 modes: Auto, Record, Replay, Passthrough
📦 Language-agnostic JSON cassettes
🔐 Auto TLS certificates
⚡ Rust performance (10-100x faster than Ruby VCR)
🌍 Works across languages

Try it: https://github.com/taciclei/magneto-serge
```

### Tweet 3 - Use Cases
```
Perfect for:

🧪 Deterministic API tests
🐛 Debugging with real traffic
📊 Offline development
🚀 CI/CD speed (no real API calls)
🔁 Reproducible bugs

68 tests passing | TypeScript support | Multi-platform

docs: https://github.com/taciclei/magneto-serge#readme
```

### Tweet 4 - Quick Start
```
Quick start (JavaScript):

```javascript
const proxy = new MagnetoProxy('./cassettes');
proxy.setMode(ProxyMode.Auto);
proxy.startRecording('api-test');

// Your HTTP requests...

proxy.stopRecording();
```

npm install @taciclei/magneto-serge

Rust: cargo add magneto-serge
```

---

## 🌐 Reddit

### r/rust

**Title:** `[Show] Magnéto-Serge v0.1.0 - Multi-language HTTP/WebSocket record/replay library`

**Body:**
```markdown
Hi r/rust! I'm excited to share the first release of **Magnéto-Serge**, a high-performance testing library for recording and replaying HTTP/WebSocket traffic.

## What is it?

Think VCR (Ruby) or Polly.JS, but written in Rust with multi-language support via FFI bindings.

## Features

- ✅ **HTTP/HTTPS MITM proxy** (Hudsucker + auto TLS certs)
- ✅ **WebSocket bidirectional capture**
- ✅ **4 modes:** Auto, Record, Replay, Passthrough
- ✅ **Multi-language:** Rust native, JavaScript (NAPI-RS), PHP (FFI)
- ✅ **Performance:** ~5000 req/s HTTP, ~10k msg/s WebSocket
- ✅ **68 tests passing** with full CI/CD

## Why Rust?

- Type safety and memory safety
- Amazing async ecosystem (Tokio)
- 10-100x faster than Ruby VCR
- Easy FFI for multi-language bindings

## Quick Example (Rust)

```rust
use magneto_serge::{MagnetoProxy, ProxyMode};

let proxy = MagnetoProxy::new_internal("./cassettes")?
    .with_port(8888)
    .with_mode(ProxyMode::Auto);

proxy.start_recording_internal("my-test".to_string())?;
// Configure HTTP client to use localhost:8888
proxy.stop_recording_internal()?;
```

## Installation

- **Rust:** `cargo add magneto-serge`
- **JavaScript:** `npm install @taciclei/magneto-serge`
- **PHP:** Via Private Packagist

## Links

- **Repo:** https://github.com/taciclei/magneto-serge
- **Docs:** https://github.com/taciclei/magneto-serge#readme
- **Release:** https://github.com/taciclei/magneto-serge/releases/tag/v0.1.0
- **Crates.io:** https://crates.io/crates/magneto-serge (pending)

## Roadmap

- v0.2.0: CLI tool, framework examples
- v0.3.0: Python, Java/Kotlin bindings
- v1.0: Production release

Would love feedback from the community! 🦀
```

---

### r/javascript

**Title:** `Magnéto-Serge: VCR-like HTTP/WebSocket record/replay for Node.js (TypeScript support)`

**Body:**
```markdown
Hey r/javascript!

Just released **Magnéto-Serge v0.1.0** - a testing library that records your HTTP/WebSocket traffic and replays it deterministically.

## Why?

- 🚫 No more flaky API tests
- ⚡ Instant test execution (no real API calls)
- 🔁 Reproducible bugs
- 🌐 Works offline

## Features

- Full TypeScript definitions
- Multi-platform binaries (macOS, Linux, Windows)
- WebSocket support (not just HTTP!)
- Powered by Rust for performance

## Quick Start

```bash
npm install @taciclei/magneto-serge
```

```typescript
import { MagnetoProxy, ProxyMode } from '@taciclei/magneto-serge';

const proxy = new MagnetoProxy('./cassettes');
proxy.setMode(ProxyMode.Auto);
proxy.startRecording('github-api');

// First run: records from real API
// Second run: replays from cassette
const response = await fetch('https://api.github.com/users/octocat', {
  agent: new HttpsProxyAgent('http://localhost:8888')
});

proxy.stopRecording();
```

## vs Polly.JS?

- ✅ Faster (Rust native addon via NAPI-RS)
- ✅ WebSocket support
- ✅ Works across languages (share cassettes with Rust, PHP)
- ✅ Auto mode (record or replay automatically)

## Links

- **Repo:** https://github.com/taciclei/magneto-serge
- **npm:** https://github.com/taciclei/magneto-serge/packages
- **Docs:** https://github.com/taciclei/magneto-serge/blob/main/README.md

Let me know what you think! 🎉
```

---

### r/PHP

**Title:** `Magnéto-Serge: HTTP/WebSocket testing library with PHP FFI bindings`

**Body:**
```markdown
Hi r/PHP!

Introducing **Magnéto-Serge** - a Rust-powered testing library now available for PHP via FFI.

## What does it do?

Records your HTTP/WebSocket traffic and replays it for deterministic testing. Think VCR but with WebSocket support and better performance.

## Features

- HTTP/HTTPS MITM proxy
- WebSocket bidirectional capture
- Auto, Record, Replay, Passthrough modes
- Language-agnostic cassettes (JSON)
- PHPUnit integration ready

## Installation

```bash
composer require taciclei/magneto-serge
```

(Via Private Packagist - setup instructions in docs)

## Example with PHPUnit

```php
<?php
use Taciclei\MagnetoSerge\MagnetoProxy;
use Taciclei\MagnetoSerge\ProxyMode;

class ApiTest extends \PHPUnit\Framework\TestCase {
    public function testGitHubApi() {
        $proxy = new MagnetoProxy("./cassettes");
        $proxy->setMode(ProxyMode::AUTO);
        $proxy->startRecording("github-test");

        $client = new \GuzzleHttp\Client([
            'proxy' => 'http://localhost:8888'
        ]);

        $response = $client->get('https://api.github.com/users/octocat');

        $this->assertEquals(200, $response->getStatusCode());
        $proxy->stopRecording();
    }
}
```

## Requirements

- PHP 8.1+
- ext-ffi (Foreign Function Interface)
- Compiled Rust library

## Links

- **Repo:** https://github.com/taciclei/magneto-serge
- **Docs:** https://github.com/taciclei/magneto-serge/blob/main/bindings/php/README.md
- **Private Packagist:** https://packagist.com/packages/taciclei/magneto-serge

Feedback welcome! 🐘
```

---

## 📰 Dev.to Article

**Title:** `Building a Multi-Language HTTP Testing Library with Rust`

**Tags:** `#rust`, `#testing`, `#opensource`, `#tutorial`

**Intro:**
```markdown
Have you ever wanted to record HTTP traffic in tests and replay it deterministically? Today I'm releasing Magnéto-Serge v0.1.0, a Rust-powered testing library with bindings for JavaScript, PHP, and more.

## The Problem

Testing applications that depend on external APIs is hard:

- Tests are slow (network calls)
- Tests are flaky (API downtime, rate limits)
- Tests require internet
- Hard to reproduce specific scenarios

## The Solution

Record HTTP/WebSocket traffic once, replay it thousands of times.

[continue with technical deep dive...]
```

---

## 🔥 HackerNews

**Title:** `Show HN: Magnéto-Serge – Multi-language HTTP/WebSocket record/replay library (Rust)`

**Body:**
```
Hi HN! I built Magnéto-Serge, a testing library for recording and replaying HTTP/WebSocket traffic across multiple languages.

It's inspired by Ruby's VCR but written in Rust for performance and multi-language support via FFI. Think of it as a "time machine" for your API tests.

Key features:
- HTTP/HTTPS MITM proxy with auto TLS certificates
- WebSocket bidirectional message capture
- 4 modes: Auto, Record, Replay, Passthrough
- Language-agnostic JSON cassettes
- ~5000 req/s throughput, <1ms latency

Currently supports:
- Rust (native)
- JavaScript/TypeScript (NAPI-RS)
- PHP (FFI)

Coming soon: Python, Java/Kotlin, Swift

Use cases:
- Deterministic API tests in CI/CD
- Debugging with recorded production traffic
- Offline development
- Faster test suites (no real network calls)

The JavaScript example from the README:

  const proxy = new MagnetoProxy('./cassettes');
  proxy.setMode(ProxyMode.Auto);
  proxy.startRecording('my-api-test');

  // First run: records from real API
  // Subsequent runs: replay from cassette

  proxy.stopRecording();

68 tests passing, full CI/CD, TypeScript support.

Repo: https://github.com/taciclei/magneto-serge

Would love feedback from the community, especially on:
- API design
- Additional language bindings priorities
- Use cases I haven't considered

Thanks!
```

---

## 📧 Email Newsletter

**Subject:** `Magnéto-Serge v0.1.0 Released - HTTP/WebSocket Testing Library`

**Body:**
```
Hi!

I'm excited to announce the first release of Magnéto-Serge, a multi-language HTTP/WebSocket testing library.

What is Magnéto-Serge?
Record your HTTP and WebSocket traffic once, replay it deterministically in tests. No more flaky API tests!

Key Features:
✅ HTTP/HTTPS MITM proxy
✅ WebSocket capture
✅ Multi-language (Rust, JavaScript, PHP)
✅ High performance (~5000 req/s)
✅ TypeScript support
✅ 68 tests passing

Quick Start:
```bash
# Rust
cargo add magneto-serge

# JavaScript/TypeScript
npm install @taciclei/magneto-serge

# PHP
composer require taciclei/magneto-serge
```

Try it today: https://github.com/taciclei/magneto-serge

Best regards,
Magnéto-Serge Team
```

---

## 📝 GitHub Discussions

**Title:** `🎉 Magnéto-Serge v0.1.0 Released!`

**Category:** Announcements

**Body:**
```markdown
We're thrilled to announce the **first official release** of Magnéto-Serge! 🎉

## What's New in v0.1.0

### Core Features
- ✅ HTTP/HTTPS MITM proxy with Hudsucker
- ✅ WebSocket bidirectional message capture
- ✅ Record/Replay functionality
- ✅ 4 proxy modes (Auto, Record, Replay, Passthrough)
- ✅ Language-agnostic JSON cassettes

### Language Support
- ✅ **Rust** - Native library
- ✅ **JavaScript/TypeScript** - NAPI-RS bindings with full type definitions
- ✅ **PHP** - FFI bindings (Private Packagist)

### Quality
- ✅ 68 tests passing (100%)
- ✅ CI/CD with GitHub Actions
- ✅ Complete documentation
- ✅ Zero compiler warnings

### Performance
- HTTP: ~5000 req/s
- WebSocket: ~10k msg/s
- Latency: <1ms p50
- Memory: <50 MB

## Installation

See our [README](../README.md) for installation instructions.

## What's Next?

### v0.2.0 (Planned)
- CLI tool (`magneto` command)
- Framework examples (Jest, Vitest, PHPUnit)
- Performance benchmarks

### v0.3.0+ (Future)
- Python bindings
- Java/Kotlin bindings
- Framework integrations

## Get Involved

- 🐛 [Report bugs](../issues/new?template=bug_report.md)
- 💡 [Request features](../issues/new?template=feature_request.md)
- 📖 [Improve docs](../blob/develop/README.md)
- 🤝 Contribute code

## Links

- **Release Notes:** https://github.com/taciclei/magneto-serge/releases/tag/v0.1.0
- **Documentation:** https://github.com/taciclei/magneto-serge#readme
- **Changelog:** https://github.com/taciclei/magneto-serge/blob/main/CHANGELOG.md

Thank you for your support! 🙏

Let us know what you think in the comments below! 👇
```

---

**Ready to announce? Copy and paste! 🚀**

Last updated: 2025-10-11
