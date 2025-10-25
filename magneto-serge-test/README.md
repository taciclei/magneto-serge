# magneto-serge-test

Test framework integration for [magneto-serge](https://github.com/taciclei/magneto-serge) with automatic cassette management.

## Status

⚠️ **Work in Progress** - v0.3.0-alpha

This crate provides a `#[magneto_test]` procedural macro for automatic proxy setup and cassette management in Rust tests.

## Installation

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
magneto-serge = "0.2"
magneto-serge-test = "0.3"
tokio = { version = "1.35", features = ["full"] }
```

## Usage

### Basic Test

```rust
use magneto_serge_test::magneto_test;

#[magneto_test]
async fn test_api_call() {
    // Proxy auto-started with cassette "test_api_call"
    // Configure your HTTP client to use http://localhost:8888

    let response = reqwest::get("http://api.example.com/users").await?;
    assert_eq!(response.status(), 200);

    // Proxy auto-stopped, cassette saved to ./cassettes/test_api_call.json
}
```

### Custom Cassette Name

```rust
#[magneto_test(cassette = "shared_cassette")]
async fn test_with_shared_cassette() {
    // Uses cassette "shared_cassette" instead of function name
}
```

### Custom Mode

```rust
#[magneto_test(mode = "replay")]
async fn test_strict_replay() {
    // Runs in strict replay mode (fails if cassette missing)
}

#[magneto_test(mode = "record")]
async fn test_force_record() {
    // Always records (overwrites existing cassette)
}

#[magneto_test(mode = "auto")]  // Default
async fn test_auto_mode() {
    // Records if cassette missing, else replays
}
```

### All Options

```rust
#[magneto_test(
    cassette = "integration_test",
    mode = "replay",
    cassette_dir = "./fixtures",
    port = 9000
)]
async fn test_with_all_options() {
    // Full control over proxy configuration
}
```

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `cassette` | `&str` | Function name | Cassette name |
| `mode` | `"auto"` \| `"record"` \| `"replay"` \| `"passthrough"` | `"auto"` | Proxy mode |
| `cassette_dir` | `&str` | `"./cassettes"` | Directory for cassettes |
| `port` | `u16` | `8888` | Proxy port |

## Modes

- **auto**: Record if cassette missing, else replay (default, best for development)
- **record**: Always record, overwrite existing (useful for updating tests)
- **replay**: Strict replay only, fail if cassette missing (best for CI/CD)
- **passthrough**: No recording/replay, just proxy (useful for debugging)

## Configuring HTTP Client

The macro starts a proxy on `localhost:8888` (or custom port). Configure your HTTP client to use it:

### reqwest

```rust
let client = reqwest::Client::builder()
    .proxy(reqwest::Proxy::all("http://localhost:8888")?)
    .build()?;
```

### hyper

```rust
use hyper::client::HttpConnector;
use hyper_proxy::{Proxy, ProxyConnector, Intercept};

let proxy = Proxy::new(Intercept::All, "http://localhost:8888".parse()?);
let connector = ProxyConnector::from_proxy(HttpConnector::new(), proxy)?;
let client = hyper::Client::builder().build(connector);
```

### Environment Variable

```rust
std::env::set_var("HTTP_PROXY", "http://localhost:8888");
std::env::set_var("HTTPS_PROXY", "http://localhost:8888");

// Most HTTP clients will use these automatically
let response = reqwest::get("https://api.example.com").await?;
```

## Requirements

This macro requires:

1. **Tokio runtime**: The macro wraps your test with `#[tokio::test]`
2. **Async function**: Your test function must be `async`
3. **magneto-serge**: The core library must be in your dependencies

## Implementation Status

### ✅ Completed

- [x] Proc macro infrastructure
- [x] Attribute parsing (cassette, mode, cassette_dir, port)
- [x] Documentation and examples
- [x] Syn 2.0 compatibility

### ⚠️ Pending (requires magneto-serge API additions)

- [ ] `MagnetoProxy::set_mode()` implementation
- [ ] `MagnetoProxy::set_port()` implementation
- [ ] `MagnetoProxy::start_recording()` implementation
- [ ] `MagnetoProxy::start_replay()` implementation
- [ ] `MagnetoProxy::start_passthrough()` implementation
- [ ] `MagnetoProxy::stop_recording()` implementation
- [ ] `MagnetoProxy::stop_replay()` implementation
- [ ] `MagnetoProxy::stop_passthrough()` implementation

These methods need to be added to the `MagnetoProxy` type in the main `magneto-serge` crate.

## Roadmap

### v0.3.0-alpha (Current)

- [x] Basic macro structure
- [x] Attribute parsing
- [ ] Integration tests
- [ ] API implementation in magneto-serge

### v0.3.0-beta

- [ ] Full API integration
- [ ] Comprehensive examples
- [ ] Error handling improvements

### v0.3.0 (Stable)

- [ ] Production-ready
- [ ] Full documentation
- [ ] Published to crates.io

## Comparison with VCR (Ruby)

| Feature | magneto_test | VCR (RSpec) | Status |
|---------|--------------|-------------|--------|
| Attribute-based | ✅ `#[magneto_test]` | ✅ `:vcr` tag | ✅ Parity |
| Auto cassette naming | ✅ From function | ✅ From test name | ✅ Parity |
| Custom cassette | ✅ `cassette = "name"` | ✅ `cassette: 'name'` | ✅ Parity |
| Mode selection | ✅ `mode = "replay"` | ✅ `record: :none` | ✅ Parity |
| Async support | ✅ Native | ❌ Ruby fibers | ✅ Better |
| Type safety | ✅ Compile-time | ❌ Runtime | ✅ Better |

## Contributing

Contributions welcome! See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
- MIT License ([LICENSE-MIT](../LICENSE-MIT))

at your option.

---

**Part of the [Magneto-Serge](https://github.com/taciclei/magneto-serge) project**
