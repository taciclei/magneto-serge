# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added (v0.5.0 - In Progress)
- **Hydra Hypermedia API (Phase 1 - Backend)** 🎉
  - Complete W3C Hydra Core Vocabulary implementation
  - JSON-LD linked data support
  - HATEOAS-compliant REST API
  - 7 hypermedia endpoints with auto-discovery
  - Pagination with HydraView (first/prev/next/last)
  - Content negotiation (JSON-LD, JSON)
  - 31 unit tests, 3,150 lines Rust
  - Example: `examples/hydra_api_server.rs`

### Planned
- WebSocket template support
- Additional language bindings (Go, C#)
- HAR (HTTP Archive) export format
- Postman Collection export
- Interactive TUI mode for CLI
- Better error messages with suggestions (v0.4.1)
- Hydra Phase 2: Angular frontend with Alcaeus
- Hydra Phase 2: E2E integration tests
- Hydra Phase 2: Turtle/RDF-XML serialization

---

## [0.4.0] - 2025-10-26

### Dynamic Templates & Response Rendering

Complete Handlebars template engine integration for dynamic cassette responses, enabling environment variable substitution, dynamic timestamps, UUID generation, and request context access.

### Added

#### Templates Module (`src/templates.rs` - 400+ lines)
- **Handlebars Template Engine** with optional feature flag
  - Environment variable substitution: `{{ env "VAR_NAME" }}`
  - Dynamic ISO 8601 timestamps: `{{ now }}`
  - Unix epoch timestamps: `{{ now_timestamp }}`
  - UUID v4 generation: `{{ uuid }}`
  - Request context access: `{{ request.method }}`, `{{ request.url }}`, `{{ request.headers.xxx }}`
  - Custom helper registration API
  - Template detection (avoids rendering non-template responses)
  - Stub implementation when feature disabled (zero overhead)

#### Player Integration
- `TemplateEngine` field in `Player` struct
- `render_templates_in_response()` method for HTTP response rendering
- `template_engine()` and `template_engine_mut()` accessors
- Automatic template engine initialization in all Player constructors
- Debug trait implementation for TemplateEngine

#### Built-in Helpers
- **`env`**: Environment variable lookup with empty string fallback
- **`now`**: Current timestamp in ISO 8601 format (RFC3339)
- **`now_timestamp`**: Unix epoch timestamp (seconds since 1970)
- **`uuid`**: UUID v4 generation using `uuid` crate

#### Error Handling
- `TemplateError` variant in `MatgtoError` enum (feature-gated)
- Detailed error messages for template rendering failures

#### Testing
- **Integration Tests** (`tests/test_templates.rs` - 542 lines)
  - 8 tests with templates feature enabled
  - 1 test for stub behavior when disabled
  - Environment variable substitution tests
  - Dynamic timestamp generation tests
  - UUID generation tests
  - Request header access tests
  - Complex multi-feature template tests
  - Pass-through tests for non-template responses
  - Custom helper registration tests
  - Multiple interactions with different templates

- **Unit Tests** (8 tests in `src/templates.rs`)
  - Template detection (`has_templates`)
  - Plain text pass-through
  - Request header rendering
  - UUID generation
  - Timestamp generation
  - Environment variable rendering
  - Custom helper functionality
  - Complex template scenarios

#### Documentation & Examples
- **Example Cassettes** (`examples/cassettes-with-templates/`)
  - `api-auth-with-env.json`: Environment variable patterns
  - `webhook-with-request-data.json`: Request context patterns
  - `dynamic-timestamps.json`: Multiple timestamp formats

- **Comprehensive Guide** (`examples/cassettes-with-templates/README.md` - 388 lines)
  - Template syntax reference
  - Built-in helper documentation
  - Custom helper registration guide
  - Real-world use cases
  - Best practices and security tips
  - Integration examples
  - Debugging tips

- **README Updates**
  - Added "Dynamic Templates" to features table
  - New template example section with full API reference
  - Links to examples and documentation

### Dependencies
- Added `handlebars = "5.1"` (optional, behind `templates` feature)
- Added `chrono` (already present, now used for timestamp helpers)
- Added `uuid` (already present, now used for UUID helper)

### Changed
- Player struct now includes `template_engine` field
- All Player constructors initialize TemplateEngine

### Performance
- Zero overhead when `templates` feature is disabled (stub implementation)
- Template detection checks for `{{` and `}}` before parsing
- Handlebars rendering is only triggered for responses containing templates

### Migration Guide
No breaking changes. Templates are an opt-in feature requiring:
1. Compile with `--features templates`
2. Use Handlebars syntax in cassette response bodies
3. Optionally call `player.render_templates_in_response()` during replay

### Statistics
- **Files Added**: 8 (src/templates.rs, tests/test_templates.rs, 3 example cassettes, 3 READMEs)
- **Files Modified**: 4 (Cargo.toml, src/error.rs, src/lib.rs, src/player.rs, README.md)
- **Lines Added**: 1,742+ (412 core, 542 tests, 388 examples, 400 docs)
- **Tests**: 119 total (111 existing + 8 new template tests)
- **Test Coverage**: All tests passing (103 unit, 8 template integration, 8 template unit)

---

## [0.3.1] - 2025-10-25

### Test Framework Integration Release

Complete test framework integrations for 5 languages, achieving 100% feature parity with VCR's test integration capabilities while maintaining Magneto-Serge's performance advantages.

### Added

#### Phase 1: Rust Test Macro
- **`#[magneto_test]` Procedural Macro** (`magneto-test-macro` crate)
  - Auto-start proxy with cassette name derived from test function
  - Auto-stop and save cassette after test completion
  - Support for custom cassette names: `#[magneto_test(cassette = "name")]`
  - Support for mode override: `#[magneto_test(mode = "replay")]`
  - Support for port override: `#[magneto_test(port = 9000)]`
  - Support for cassette_dir override
  - Full async/await support
  - Syn 2.0 compatibility
  - 200+ lines of documentation and examples

#### Phase 2: Ruby RSpec Integration
- **`magneto-serge-rspec` Gem** (16 files, 2,091 lines)
  - RSpec metadata-driven cassette activation (`:magneto`, `:cassette`)
  - Auto-generated cassette names from test hierarchy
  - `use_cassette` helper method for manual control
  - Configuration DSL with global defaults
  - VCR-compatible record modes (:new_episodes, :once, :all, :none)
  - Sensitive header filtering
  - Custom cassette name generator support
  - 350+ lines of documentation
  - Basic and advanced examples
  - Full RSpec hooks integration (before/after)

#### Phase 3: JavaScript Jest Plugin
- **`@magneto-serge/jest` Package** (11 files, 1,527 lines)
  - Multiple API patterns:
    - `magnetoTest()` - Wrapper for test() with auto cassette management
    - `magnetoDescribe()` - Suite-level cassette management
    - `useCassette()` - Manual cassette control
    - `setupMagneto()` - Jest environment setup
  - Full TypeScript support with type definitions
  - VCR-compatible record mode translation
  - Auto-generated cassette names from test names
  - Global configuration via `configure()`
  - `getCurrentCassette()` helper
  - 600+ lines of documentation
  - Basic and advanced examples with TypeScript

#### Phase 4: Python pytest Plugin
- **`pytest-magneto-serge` Package** (13 files, ~1,300 lines)
  - Four flexible API patterns:
    - Markers: `@pytest.mark.magneto_cassette()`
    - Decorators: `@magneto_cassette()`
    - Fixtures: `magneto_proxy`, `magneto_config`
    - Context manager: `use_cassette()`
  - pytest plugin registration via entry_points
  - Auto-generated cassette names from test hierarchy
  - VCR-compatible record mode translation
  - Global configuration via conftest.py
  - Support for pytest 6.0+, Python 3.8+
  - 650+ lines of documentation
  - Basic and advanced examples (350+ lines)
  - Unit tests for plugin functionality

#### Phase 5: PHP PHPUnit Integration
- **`magneto-serge/phpunit` Package** (11 files, 1,693 lines)
  - Modern PHP 8+ Attributes: `#[Cassette]`
  - `MagnetoTestCase` base class
  - `MagnetoTrait` for flexible integration
  - Auto-generated cassette names from class/method
  - VCR-compatible record mode translation
  - `useCassette()` for manual control
  - Support for PHPUnit 9, 10, 11
  - 600+ lines of documentation
  - Basic and advanced examples
  - Comprehensive unit tests

### Changed
- Updated roadmap: v0.3.1 marked as 100% complete
- Gap analysis score: 9.7/10 → 9.8/10
- Test framework coverage: 25% → 100%

### Documentation
- Created `SESSION-RECAP-2025-10-25.md` (429 lines) documenting all implementations
- Updated `ROADMAP-v0.3-v0.4.md` with completion status for all 5 phases
- Updated `GAP-ANALYSIS.md` with 9.8/10 score and 100% framework coverage
- Comprehensive README files for all 5 integration packages (3,250+ lines total)
- Migration examples from VCR, vcrpy, go-vcr, php-vcr

### Statistics
- **Total Files Created**: 51+ files
- **Total Lines of Code**: 6,611+ lines
- **Total Documentation**: 3,250+ lines
- **Languages Supported**: 5 (Rust, Ruby, JavaScript/TypeScript, Python, PHP)
- **API Patterns**: 12+ different usage patterns across all languages
- **Examples**: 10+ complete working examples

### Framework Coverage Achievement
- ✅ Rust - `#[magneto_test]` proc macro
- ✅ Ruby/RSpec - `magneto-serge-rspec` gem
- ✅ JavaScript/Jest - `@magneto-serge/jest` npm package
- ✅ PHP/PHPUnit - `magneto-serge/phpunit` composer package
- ✅ Python/pytest - `pytest-magneto-serge` PyPI package

**Result**: 🎉 100% test framework integration coverage, matching VCR's capabilities across all major testing ecosystems

---

## [0.3.0] - 2025-10-25

### Hook System Release (CRITICAL)

Complete implementation of VCR-compatible hook system for customizing record/replay behavior.

### Added

#### Core Hook System
- **`RecordHook` and `ReplayHook` Traits**
  - `before_record` / `after_record` lifecycle methods
  - `before_replay` / `after_replay` lifecycle methods
  - Thread-safe with `Send + Sync + Debug` bounds
  - Named hooks with `name()` method
  - Arc-based storage for efficient sharing

#### Built-in Hooks
- **`SensitiveHeaderFilter`**
  - Filters Authorization, Cookie, Set-Cookie, X-API-Key headers
  - Extensible with custom header patterns
  - Replaces sensitive values with `[FILTERED]` placeholder

- **`BodyPatternReplacer`**
  - Regex-based body content replacement
  - Support for passwords, tokens, API keys in JSON
  - Custom pattern registration
  - Applies to both requests and responses

- **`LoggingHook`**
  - Logs all interactions to stderr
  - Verbose mode with full request/response details
  - Integration with tracing framework

### Changed
- Recorder and Player now accept hooks via `add_hook()` method
- Hooks execute in registration order
- Hook errors propagate to caller
- Updated gap analysis score: 9.2/10 → 9.5/10

### Documentation
- Hook system API documentation (200+ lines)
- Examples for all built-in hooks
- Custom hook implementation guide
- Updated roadmap with v0.3.0 completion

---

## [0.2.0] - 2025-10-25

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
