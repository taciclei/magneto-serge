# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**matgto-serge** is a high-performance HTTP/HTTPS/WebSocket proxy library for testing, written in Rust with multi-language bindings. It records network interactions into "cassettes" that can be replayed deterministically, similar to VCR for Ruby but with 10-100x better performance and support for 8+ programming languages.

**Technology Stack:** Rust 1.75+, UniFFI for multi-language bindings, Tokio async runtime
**Target Languages:** Java, JavaScript, Python, Ruby, Kotlin, Swift, Go, C#

---

## Build and Development Commands

### Rust Core

```bash
# Build the project
cargo build

# Build optimized release
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test test_name

# Run clippy (linter)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Run benchmarks
cargo bench

# Generate documentation
cargo doc --open
```

### UniFFI Bindings Generation

```bash
# Generate Java bindings
cargo run --features=uniffi/cli --bin uniffi-bindgen -- generate src/matgto_serge.udl --language java --out-dir bindings/java

# Generate JavaScript bindings
cargo run --features=uniffi/cli --bin uniffi-bindgen -- generate src/matgto_serge.udl --language typescript --out-dir bindings/js

# Generate Python bindings
cargo run --features=uniffi/cli --bin uniffi-bindgen -- generate src/matgto_serge.udl --language python --out-dir bindings/python
```

### Features

```bash
# Build with CLI features
cargo build --features cli

# Build with MessagePack support
cargo build --features msgpack

# Build with all features
cargo build --all-features
```

---

## Architecture Overview

### High-Level Design

matgto-serge uses a **layered proxy architecture**:

1. **Public API Layer** (`MagnetoProxy`) - Simple API exposed via UniFFI to all target languages
2. **Core Proxy Layer** - HTTP/HTTPS interceptor (Hudsucker) + WebSocket interceptor (tokio-tungstenite)
3. **Record/Replay Engine** - `Recorder` captures interactions, `Player` matches and replays them
4. **Cassette Storage** - JSON/MessagePack files on disk

### Key Components

- **MagnetoProxy**: Main entry point with builder pattern (`new()`, `with_port()`, `with_mode()`)
- **ProxyMode**: `Auto` (record if cassette missing, else replay), `Record`, `Replay`, `Passthrough`
- **HttpHandler**: Intercepts HTTP/HTTPS via Hudsucker MITM proxy with auto-generated TLS certificates
- **WebSocketInterceptor**: Bidirectional message capture with direction tracking (sent/received)
- **Recorder**: Serializes HTTP requests/responses and WebSocket messages to cassette files
- **Player**: Loads cassettes and matches incoming requests using RequestSignature (method + URL + body hash)

### Cassette Format

Cassettes are stored as JSON (or MessagePack with `msgpack` feature) with this structure:

```json
{
  "version": "1.0",
  "name": "cassette-name",
  "recorded_at": "2025-10-10T14:30:00Z",
  "interactions": [
    {
      "type": "Http",
      "request": { "method": "GET", "url": "...", "headers": {...}, "body": null },
      "response": { "status": 200, "headers": {...}, "body": [...] }
    },
    {
      "type": "WebSocket",
      "url": "wss://...",
      "messages": [
        { "direction": "Sent", "timestamp_ms": 0, "msg_type": "Text", "data": "..." }
      ]
    }
  ]
}
```

### Critical Design Patterns

- **MITM TLS**: Auto-generated CA certificate with `rcgen`, requires installation in OS trust store
- **Zero-Copy**: Uses `Bytes` (Arc-based) to minimize memory allocations
- **Async/Await**: Tokio runtime with multi-threaded executor for high throughput (>5000 req/s target)
- **Thread-Safe Sharing**: `Arc<Mutex<Recorder>>` and `Arc<RwLock<Player>>` for concurrent access
- **UniFFI Bindings**: UDL file (`matgto_serge.udl`) defines cross-language API, generated in `build.rs`

---

## Workspace Structure

The project follows a **Cargo workspace** pattern (when implemented):

```
matgto-serge/
├── core/              # Core Rust proxy logic
├── bindings/          # Language-specific bindings
│   ├── java/
│   ├── javascript/
│   └── python/
├── cli/               # CLI binary (optional feature)
├── cassettes/         # Test cassettes (gitignored)
├── docs/              # Architecture, tech stack, roadmap, examples
└── tests/             # Integration tests
```

---

## Key Dependencies

### Core Proxy
- **hudsucker 0.20**: MITM HTTP/HTTPS proxy with TLS interception
- **hyper 0.14**: HTTP client/server (LTS version, migration to 1.0 planned)
- **tokio-tungstenite 0.21**: Async WebSocket implementation
- **tokio 1.35**: Async runtime with multi-thread, sync, net, io-util features

### Serialization & Bindings
- **serde + serde_json**: Cassette serialization (JSON format)
- **rmp-serde**: Optional MessagePack format for large cassettes
- **uniffi 0.25**: Multi-language binding generator

### TLS & Security
- **rustls 0.21**: Modern TLS implementation (no OpenSSL dependency)
- **rcgen 0.11**: Certificate generation for MITM

### CLI (Optional)
- **clap 4.4**: Command-line argument parsing with derive macros
- **colored 2.1**: Terminal color output
- **indicatif 0.17**: Progress bars

### Observability
- **tracing 0.1**: Structured logging with async awareness
- **tracing-subscriber 0.3**: Log filtering via `RUST_LOG` env var

---

## Development Workflow

### Project Phases (Roadmap)

**Phase 1** (3 weeks): Core HTTP/HTTPS proxy with record/replay
**Phase 2** (2 weeks): WebSocket support
**Phase 3** (3 weeks): Java/JavaScript/Python bindings via UniFFI
**Phase 4** (2 weeks): CLI + production release 1.0

Current status: **Planning** (0/4 phases complete)

### Adding New Features

1. Implement in Rust core (`src/`)
2. Add to UDL file if exposing to bindings (`src/matgto_serge.udl`)
3. Regenerate bindings via `build.rs` or manual uniffi-bindgen
4. Write tests (unit in `src/`, integration in `tests/`)
5. Update documentation

### Testing Strategy

- **Unit tests**: Recorder serialization, Player matching logic, cassette validation
- **Integration tests**: E2E HTTP/WebSocket record+replay scenarios
- **Bindings tests**: JUnit (Java), Jest (JS), pytest (Python) integration tests
- **Property-based tests**: Use `proptest` for cassette roundtrip validation
- **Benchmarks**: Criterion benchmarks in `benches/` (target: >5000 req/s HTTP, >10k msg/s WebSocket)

### Performance Targets

- HTTP throughput: ≥5000 req/s
- WebSocket throughput: ≥10k msg/s
- Proxy latency: <1ms p50
- Memory footprint: <50 MB
- Startup time: <100ms

---

## Important Implementation Notes

### MITM Certificate Handling

The proxy generates a self-signed CA certificate on first run. Users must install it in their OS trust store:

- **macOS**: `security add-trusted-cert -d -r trustRoot -k ~/Library/Keychains/login.keychain magneto-ca.pem`
- **Linux**: Copy to `/usr/local/share/ca-certificates/` and run `update-ca-certificates`
- **Windows**: Import via certmgr.msc

Provide clear installation guides and consider auto-install scripts.

### Sensitive Data Filtering

Always filter credentials from cassettes:
- Headers: `Authorization`, `Cookie`, `Set-Cookie`, `X-API-Key`, `Proxy-Authorization`
- Replace with `[FILTERED]` placeholder in recordings
- See `SENSITIVE_HEADERS` constant in architecture docs

### WebSocket Challenges

- **Bidirectional messaging**: Track direction (Sent/Received) and timestamp ordering
- **Timing**: Use relative timestamps from connection start for reproducible replay
- **Sequence validation**: In strict mode, verify client messages match cassette sequence

### UniFFI Limitations

- No support for generic Rust types in public API
- Complex callbacks (Rust → target language) are limited
- Keep public API simple: basic types, no lifetimes, no generics
- FFI overhead is typically <1% but benchmark critical paths

---

## Configuration

Default configuration (when `magneto.toml` exists):

```toml
[magneto]
cassette_dir = "./cassettes"
proxy_port = 8888
mode = "auto"  # auto | record | replay | passthrough
strict = true

[matching]
ignore_headers = ["User-Agent", "Date", "X-Request-Id"]
ignore_query_params = ["timestamp", "_t"]

[recording]
filter_headers = ["Authorization", "X-API-Key", "Cookie"]
compress = true
format = "json"  # json | msgpack
```

Environment variables override config:
- `MAGNETO_MODE`: Set proxy mode
- `MAGNETO_CASSETTE_DIR`: Cassette directory path
- `MAGNETO_PROXY_PORT`: Proxy port
- `RUST_LOG`: Logging level (e.g., `magneto_serge=debug`)

---

## Common Patterns

### Mode Selection Strategy

- **Development**: `ProxyMode::Auto` - records new cassettes, replays existing ones
- **CI/CD**: `ProxyMode::Replay` with `strict=true` - fast deterministic tests, errors on missing cassettes
- **Recording**: `ProxyMode::Record` - explicitly (re)record cassettes, overwrites existing

### Error Handling

Use `thiserror` for library errors (public types), `anyhow` for CLI/application errors:

```rust
#[derive(Error, Debug)]
pub enum MatgtoError {
    #[error("Cassette not found: {name}")]
    CassetteNotFound { name: String },

    #[error("No matching interaction for {method} {url}")]
    NoMatchingInteraction { method: String, url: String },
}
```

### Async Runtime Configuration

```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // 4 threads for high-performance proxy
}
```

---

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite: `cargo test --all-features`
4. Build release binaries for all platforms (Linux x64/ARM64, macOS Intel/Apple Silicon, Windows x64)
5. Generate bindings for all languages
6. Publish to package managers:
   - Rust: `cargo publish`
   - Java: Maven Central
   - JavaScript: `npm publish`
   - Python: `maturin publish` (PyPI)
   - Ruby: `gem push` (RubyGems)

---

## Additional Resources

- **Architecture**: `docs/ARCHITECTURE.md` - Detailed component design, lifecycle diagrams
- **Tech Stack**: `docs/TECH-STACK.md` - Complete dependency list with rationale
- **Roadmap**: `docs/ROADMAP.md` - 4-phase implementation plan with timeline
- **Examples**: `docs/EXAMPLES.md` - Usage examples for Java, JS, Python, Ruby, Kotlin

---

*Last updated: 2025-10-10*
