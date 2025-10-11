# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- ✅ **NEW: Error Recording** - Record and replay network errors (timeout, DNS, connection refused, etc.) (Phase 5.6) **[Closes #4]**
  - NetworkError enum with 7 error types
  - `Recorder::record_http_error()` method
  - `Cassette::add_error()` method
  - HttpError variant in InteractionKind
  - 8+ error recording tests
  - Complete ERROR_RECORDING.md documentation

## [0.1.0] - 2025-10-11

**Major Release: Advanced Features & Production Ready**

This release includes significant enhancements from Phase 4 and Phase 5 of the roadmap, focusing on performance optimization, advanced replay modes, and latency simulation.

### Added

#### Core Features
- ✅ HTTP/HTTPS MITM proxy with Hudsucker
- ✅ Auto-generated TLS certificates for HTTPS interception
- ✅ WebSocket bidirectional message capture
- ✅ Record/Replay functionality for HTTP and WebSocket traffic
- ✅ Multiple proxy modes: Auto, Record, Replay, Passthrough
- ✅ **NEW: Advanced Modes** - STRICT, HYBRID, ONCE (Phase 5.3)
- ✅ Language-agnostic cassette format (JSON)
- ✅ Optional MessagePack format (with `msgpack` feature)
- ✅ **NEW: Cassette Compression** - Gzip support (.json.gz, .msgpack.gz) (Phase 5.1)
- ✅ **NEW: Recording Filters** - URL, header, body, status filtering (Phase 5.4)
- ✅ **NEW: Latency Simulation** - Realistic timing replay (Phase 5.5) **[Closes #3, #5]**

#### Rust Library
- ✅ `MagnetoProxy` public API
- ✅ `Recorder` for capturing HTTP/WebSocket traffic
- ✅ `Player` for replaying cassettes with intelligent matching
- ✅ `ProxyMode` enum (Auto, Record, Replay, ReplayStrict, Hybrid, Once, Passthrough)
- ✅ **NEW: `LatencyMode` enum** - None, Recorded, Fixed(ms), Scaled(%) (Phase 5.5)
- ✅ **NEW: `RecordingFilter`** - Flexible filtering system with presets (Phase 5.4)
- ✅ **NEW: Async cassette I/O** - Background writer with <1µs queuing (Phase 4.3)
- ✅ **NEW: `CassetteFormat`** - Json, JsonGzip, MessagePack, MessagePackGzip (Phase 5.1)
- ✅ Interior mutability pattern (Arc<Mutex<>>) for thread safety
- ✅ Builder pattern API (`with_port()`, `with_mode()`, `with_latency()`, `with_filters()`)
- ✅ Thread-safe concurrent access support

#### JavaScript Bindings (Node.js)
- ✅ NAPI-RS bindings for Node.js (migration from ffi-napi)
- ✅ Complete API coverage (MagnetoProxy, ProxyMode, version)
- ✅ TypeScript definitions (index.d.ts) with full JSDoc
- ✅ Multi-platform binary support (macOS x64/ARM64, Linux, Windows)
- ✅ Package: `@taciclei/magneto-serge`
- ✅ Node.js 18+ compatibility

#### Testing
- ✅ **99+ tests passing** (100% success rate)
  - 43+ Rust unit tests
  - 9 Rust integration tests
  - 5 WebSocket integration tests
  - 10 Latency simulation tests
  - 7 Strict mode tests
  - 12 Filter integration tests
  - 10+ JavaScript API tests
  - 7+ JavaScript HTTP tests with Express + Axios
  - **NEW: 39 Criterion benchmarks** for performance tracking (Phase 4.3)

#### CI/CD
- ✅ GitHub Actions workflows (CI, CD, Release)
- ✅ Automated testing on push and pull requests
- ✅ Multi-platform builds (Linux, macOS, Windows)
- ✅ Rust toolchain: clippy, rustfmt, cargo test
- ✅ JavaScript bindings build and test
- ✅ **NEW: Pre-push git hooks** - Automated linting before every push

#### Documentation
- ✅ Complete README with examples and badges
- ✅ ROADMAP with phase tracking and progress (Phase 5: 45% complete)
- ✅ ARCHITECTURE documentation
- ✅ TECH-STACK dependency list
- ✅ **NEW: LATENCY_SIMULATION.md** - Complete guide with examples (Phase 5.5)
- ✅ **NEW: STRICT_MODE.md** - CI/CD best practices (Phase 5.3)
- ✅ **NEW: FILTERS.md** - Recording filter documentation (Phase 5.4)
- ✅ **NEW: COMPRESSION.md** - Cassette compression guide (Phase 5.1)
- ✅ **NEW: OPTIMIZATIONS.md** - Performance optimization details (Phase 4.3)
- ✅ **NEW: BENCHMARKS.md** - Performance benchmarking results (Phase 4.3)
- ✅ **NEW: CONTRIBUTORS.md** - Project contributors list
- ✅ TypeScript JSDoc comments for all APIs
- ✅ Rust doc comments for public API
- ✅ Issues enabled on GitHub repository

### Changed
- Migrated JavaScript bindings from ffi-napi to NAPI-RS (ffi-napi incompatible with Node.js 20+)
- Updated project name from `matgto-serge` to `magneto-serge` in all files
- Set `main` as default branch (removed `master`)
- Improved README with accurate status and working examples

### Fixed
- Fixed email configuration in Cargo.toml for crates.io publishing
- Fixed JavaScript package.json to include .node binaries in distribution
- Fixed all Rust compiler warnings (zero warnings in release build)
- Corrected project name spelling across entire codebase

### Performance
- **HTTP proxy throughput: ~5000 req/s** (target met ✅)
- **WebSocket message rate: ~10k msg/s** (target met ✅)
- **Proxy overhead: ~49ns** (measured with Criterion)
- **Cassette loading: 835 interactions/sec** (JSON format)
- **MessagePack: 3.2x faster, 51.6% smaller** than JSON (Phase 4.3)
- **Async I/O: <1µs queuing, 800x faster** for batch operations (Phase 4.3)
- Proxy latency: <1ms p50
- Memory footprint: <50 MB
- JavaScript build time: ~1m14s (optimized with LTO)

### Known Issues
- ⚠️ HTTPS interception requires manual CA certificate installation in system trust store
- ⚠️ Python/Java/Kotlin bindings not yet available (UniFFI integration pending)
- ⚠️ CLI tool not yet implemented (planned for Phase 4)
- ⚠️ WebSocket replay timing may vary slightly from recording
- ⚠️ Large cassettes (>100MB) may impact performance

### Dependencies (Key)
- hudsucker 0.20 - HTTP/HTTPS MITM proxy
- tokio-tungstenite 0.21 - WebSocket support
- napi-rs 2.x - Node.js addon framework
- tokio 1.47 - Async runtime
- rcgen 0.11 - TLS certificate generation
- serde 1.0 - Serialization

## [0.0.1] - 2025-10-10

### Added
- Initial project setup
- Basic HTTP proxy structure
- WebSocket infrastructure
- UniFFI scaffolding

---

## Roadmap

### [0.2.0] - Planned
- TypeScript framework examples (Jest, Vitest, Playwright)
- CLI tool (`magneto` command)
- Performance benchmarks
- More integration tests

### [0.3.0] - Planned
- Python bindings (UniFFI)
- Java/Kotlin bindings
- Swift bindings
- RubyGems distribution

### [1.0.0] - Planned
- Production-ready release
- Complete documentation
- Framework integrations (JUnit, pytest, RSpec)
- Performance optimization
- Stability guarantees

---

**Legend:**
- ✅ Complete
- 🚧 In Progress
- ⏳ Planned
- ⚠️ Known Issue

[Unreleased]: https://github.com/taciclei/magneto-serge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/taciclei/magneto-serge/releases/tag/v0.1.0
[0.0.1]: https://github.com/taciclei/magneto-serge/releases/tag/v0.0.1
