# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- UniFFI bindings for additional languages (Go, C#, Ruby)
- HAR (HTTP Archive) export format
- Postman Collection export
- Interactive TUI mode for CLI
- WebSocket message filtering improvements

---

## [2.2.0] - 2025-10-25

### 🎉 Testing Utilities Release + Rust Helpers

Complete testing utilities for 5 programming languages, making it easy to write elegant cassette assertions.

### Added

#### Phase 2.2 - Testing Utilities (5 Languages)
- **Rust Test Helpers** (`src/test_helpers.rs` - 465 lines)
  - 9 assertion functions (`assert_cassette_version`, `assert_interaction_count`, etc.)
  - `assert_cassette!` macro for declarative testing
  - Support for JSON and MessagePack cassettes
  - `load_cassette()` and `load_cassette_from()` helpers
  - 9 comprehensive tests
  - Example: `examples/test_helpers_example.rs`

- **JavaScript/TypeScript** (Jest)
  - 7 custom matchers (`toMatchCassette`, `toHaveCookie`, etc.)
  - TypeScript definitions (`.d.ts`)
  - NPM package ready (`@magneto-serge/jest-matchers`)

- **Java** (JUnit 5)
  - 7 static assertions (`assertMatchesCassette`, `assertHasCookie`, etc.)
  - Maven Central ready (`com.magneto-serge:junit-assertions`)

- **Python** (pytest)
  - 7 helper functions (`assert_matches_cassette`, `assert_has_cookie`, etc.)
  - PyPI ready (`magneto-pytest`)

- **PHP** (PHPUnit)
  - 7 assertions via trait (`assertMatchesCassette`, `assertHasCookie`, etc.)
  - Packagist ready (`magneto-serge/phpunit-assertions`)

### Changed
- Updated test count: **89 tests passing** (+9 from Rust test helpers)
- Enhanced documentation with Rust examples

### Documentation
- Updated `PHASE_2.2_COMPLETE.md` with Rust test helpers
- Created comprehensive examples for all 5 languages

---

## [2.1.0] - 2025-10-25

### 🛠️ CLI Tools Release

Comprehensive command-line interface for managing cassettes from the terminal.

### Added

#### Phase 2.1 - CLI Tools
- **Complete CLI** (`src/bin/cli.rs` - 806 lines)
  - 10 commands for cassette management
  - Colored terminal output (colored 2.2)
  - Progress bars (indicatif 0.17)
  - Global installation support (`~/.cargo/bin/magneto`)

- **CLI Commands**:
  - `magneto list` - List all cassettes with filtering/sorting
  - `magneto validate <name>` - Validate cassette integrity
  - `magneto clean` - Clean old/large cassettes with dry-run
  - `magneto stats <name>` - Show detailed statistics
  - `magneto export <name> <format>` - Export to JSON/MessagePack/HAR
  - `magneto serve` - Start REST API server
  - `magneto migrate <from> <to>` - Migrate cassette versions
  - `magneto replay <name>` - Replay mode with strict option
  - `magneto record <name>` - Record mode with filters
  - `magneto init` - Initialize magneto.toml configuration

- **Configuration File** (magneto.toml)
  - Smart filtering presets
  - Default proxy settings
  - Recording filters
  - API server settings

### Fixed
- Fixed `*port` dereference error in server start
- Fixed `MatgtoError::ConfigError` to use `CassetteLoadFailed`
- Replaced `include_str!()` with inline template for magneto.toml
- Fixed unused variable warnings

### Documentation
- Created `PHASE_2.1_COMPLETE.md` with full CLI documentation
- Added usage examples for all 10 commands

---

## [1.3.0] - 2025-10-25

### 🌐 REST API Release

Full-featured REST API for remote cassette management with Axum framework.

### Added

#### Phase 1.3 - REST API
- **Axum-based API** (`src/api/` - ~1000 lines)
  - 8 HTTP endpoints for cassette management
  - `CassetteManager` for centralized operations
  - Filtering, sorting, and pagination support
  - Detailed statistics and validation
  - OpenAPI 3.0 specification support

- **API Endpoints**:
  - `GET /health` - Health check
  - `GET /cassettes` - List cassettes (with filters/sorting)
  - `GET /cassettes/:name` - Cassette metadata
  - `GET /cassettes/:name/stats` - Detailed statistics
  - `GET /cassettes/:name/validate` - Validate cassette
  - `DELETE /cassettes/:name` - Delete cassette
  - `POST /cassettes/:name/export` - Export cassette
  - `GET /cassettes/stats` - Global statistics

- **Example Server** (`examples/api_server.rs`)

### Fixed
- Fixed error variant names (`IoError` → `Io`, `SerializationError` → `Serialization`)
- Replaced non-existent `Cassette::load()` with manual deserialization
- Fixed deprecated `num_days()` method calls
- Added `create_router` alias for `build_router`

### Documentation
- Created `PHASE_1.3_COMPLETE.md` with full API documentation
- Added API endpoint examples and usage

---

## [1.2.0] - 2025-10-25

### 🎯 Smart Filtering Release

Intelligent cassette filtering reducing file sizes by up to 95.8%.

### Added

#### Phase 1.2 - Smart Filtering
- **FilterChain System** (`src/filters/` - ~900 lines)
  - `RequestFilter` trait for extensibility
  - `FilterChain` with AND/OR logic
  - 5 specialized filters:
    - `ExtensionFilter` - Skip static assets (.js, .css, .png, .woff2, etc.)
    - `ContentTypeFilter` - Filter by MIME type (image/*, font/*, video/*)
    - `UrlPatternFilter` - Glob-style URL patterns (/static/*, /assets/*)
    - `BodySizeFilter` - Skip large responses (> X MB)
    - `StatusCodeFilter` - Filter by HTTP status codes (404, 4xx, 5xx)

- **FilterPresets** for common use cases:
  - `web_assets()` - Skip JS/CSS/images/fonts
  - `api_only()` - JSON/XML only
  - `minimal()` - Aggressive filtering

- **Performance**: 95.8% cassette size reduction (100 MB → 4.2 MB in real-world tests)
- **8 comprehensive tests** for filtering functionality

### Fixed
- Fixed borrowed value lifetime errors in filters
- Used `.to_string()` for proper ownership in `extension.rs` and `url_pattern.rs`
- Created missing `status_code.rs` module

### Documentation
- Created `PHASE_1.2_COMPLETE.md` with detailed filtering guide
- Added examples for each filter type and preset

---

## [1.1.0] - 2025-10-25

### 🍪 Cookie Preservation Release

RFC 6265 compliant cookie handling for session-based authentication.

### Added

#### Phase 1.1 - Cookie Preservation
- **RFC 6265 Compliant Cookie Handling** (`src/cookies.rs` - 527 lines)
  - `Cookie` struct with all RFC 6265 attributes
  - `CookieJar` for automatic cookie management
  - Domain matching (exact + subdomain: `.example.com`)
  - Path matching with specificity rules
  - Expiration handling (Expires + Max-Age)
  - Secure, HttpOnly, SameSite attributes
  - Automatic cookie purging on expiration
  - Created timestamp tracking

- **Cassette Cookie Storage**:
  - Added `cookies: Option<Vec<Cookie>>` field to Cassette struct
  - Backward compatible serialization
  - Support for JSON and MessagePack formats

- **Player Integration**:
  - Added `CookieJar` to Player struct
  - Automatic cookie restoration during replay
  - Cookie loading from cassette in `load_with_mode()`

- **11 comprehensive tests** for cookie functionality

### Fixed
- Resolved 401 Unauthorized errors after login (cookies now preserved)
- Fixed session persistence issues in authentication flows

### Documentation
- Created `INSTALLATION_COMPLETE.md` with setup guide and Phase 1.1 details
- Added cookie usage examples

---

## [1.0.0] - 2025-10-25 (Previous Release)

### 🎉 Initial Production Release

[Previous content from old [0.1.0] moved here]

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
