# Magnéto-Serge v0.1.0 - First Production Release

🎉 **Major Release: Advanced Features & Production Ready**

This is the first production-ready release of Magnéto-Serge, a high-performance HTTP/HTTPS/WebSocket testing library with record/replay capabilities.

## 🌟 Highlights

### Phase 5.5 - Latency Simulation ✨ **NEW**
**Closes #3, #5**

Realistic network timing simulation for testing timeout handling, slow networks, and performance under various conditions.

```rust
// Replay with recorded timing
let player = Player::load(dir, "cassette")
    .with_latency(LatencyMode::Recorded);

// 2x faster for faster tests
let player = Player::new().with_latency(LatencyMode::Scaled(50));

// Fixed 100ms delay
let player = Player::new().with_latency(LatencyMode::Fixed(100));
```

**4 modes available:**
- `None` - Instant responses (default)
- `Recorded` - Use captured response times
- `Fixed(ms)` - Consistent delay
- `Scaled(%)` - Adjust speed (50 = 2x faster, 200 = 2x slower)

📖 [Complete Documentation](https://github.com/taciclei/magneto-serge/blob/develop/docs/LATENCY_SIMULATION.md)

### Phase 5.3 - Advanced Replay Modes

**STRICT Mode** - Perfect for CI/CD:
```rust
let player = Player::load_strict(dir, "cassette");
// Fails fast if request doesn't match cassette
```

**HYBRID Mode** - Record missing, replay existing:
```rust
proxy.hybrid("cassette"); // Record new + replay known
```

**ONCE Mode** - Immutable cassettes:
```rust
proxy.once("cassette"); // Record only if doesn't exist
```

### Phase 5.4 - Recording Filters

Filter sensitive data during recording:

```rust
use magneto_serge::filters::{RecordingFilter, FilterPreset};

// Security preset: mask Authorization, Cookie, API keys
let filter = RecordingFilter::from_preset(FilterPreset::Security);

// Custom filters
let filter = RecordingFilter::new()
    .with_url_filter(r".*/(admin|secret)/.*")
    .with_header_filter("X-API-Key")
    .with_max_body_size(1024 * 1024); // 1MB limit
```

**6 presets available:**
- Security, Strict, NoAnalytics, NoMedia, SuccessOnly, SmallBodies

### Phase 5.1 - Cassette Compression

Reduce cassette size by 50-95%:

```rust
// Gzip JSON (50-70% smaller)
cassette.save_with_format(dir, CassetteFormat::JsonGzip)?;

// Gzip MessagePack (85-95% smaller)
cassette.save_with_format(dir, CassetteFormat::MessagePackGzip)?;
```

### Phase 4.3 - Performance Optimizations

**Async I/O:**
- Background cassette writer with <1µs queuing
- 800x faster for batch operations

**MessagePack:**
- 3.2x faster than JSON
- 51.6% smaller files

**Benchmarks:**
- 39 Criterion benchmarks
- ~49ns proxy overhead
- 835 interactions/sec loading speed

## 📊 Statistics

| Metric | Value |
|--------|-------|
| **Tests** | 99+ (100% passing) |
| **Benchmarks** | 39 Criterion tests |
| **Performance** | ~5000 req/s HTTP, ~10k msg/s WebSocket |
| **Overhead** | ~49ns per request |
| **Languages** | Rust, JavaScript/TypeScript |
| **Phase 5 Progress** | 45% complete |

## 📚 New Documentation

- ✅ [LATENCY_SIMULATION.md](https://github.com/taciclei/magneto-serge/blob/develop/docs/LATENCY_SIMULATION.md) - Complete latency guide
- ✅ [STRICT_MODE.md](https://github.com/taciclei/magneto-serge/blob/develop/docs/STRICT_MODE.md) - CI/CD best practices
- ✅ [FILTERS.md](https://github.com/taciclei/magneto-serge/blob/develop/docs/FILTERS.md) - Recording filters
- ✅ [COMPRESSION.md](https://github.com/taciclei/magneto-serge/blob/develop/docs/COMPRESSION.md) - Cassette compression
- ✅ [OPTIMIZATIONS.md](https://github.com/taciclei/magneto-serge/blob/develop/docs/OPTIMIZATIONS.md) - Performance details
- ✅ [BENCHMARKS.md](https://github.com/taciclei/magneto-serge/blob/develop/docs/BENCHMARKS.md) - Benchmark results

## 🔧 Developer Experience

### Pre-push Git Hooks

Automatic code quality enforcement:
```bash
./scripts/install-git-hooks.sh
# Runs cargo fmt + clippy before every push
```

### Contributors

Special thanks to:
- **Taciclei** - Project Lead & Core Developer
- **phpjit** - Core Developer
- **Citoyen** - Contributor

## 🚀 Installation

### Rust (crates.io)
```bash
cargo add magneto-serge
```

### JavaScript/Node.js (npm)
```bash
npm install @taciclei/magneto-serge
```

## 📝 Breaking Changes

None! This release is fully backward compatible.

## 🐛 Bug Fixes

- Fixed timing test flakiness in CI environments (especially macOS)
- Increased timing tolerance for CI variability (±200ms)
- Fixed all Rust compiler warnings

## 🔮 What's Next (v0.2.0)

- **CLI tool** (`magneto` command)
- **Error simulation** (timeout, 500 errors)
- **Python bindings** (UniFFI)
- **Framework plugins** (Jest, pytest, JUnit)

## 📖 Full Changelog

See [CHANGELOG.md](https://github.com/taciclei/magneto-serge/blob/develop/CHANGELOG.md) for complete details.

## 🙏 Acknowledgments

Built with Rust 🦀 for maximum performance and safety.

---

**Issues Closed:** #3 (HTTP response delay), #5 (speed setting for replay)

**Commits:** 25+ commits in this release

**Download:** See Assets below ⬇️
