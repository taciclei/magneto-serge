# Stack Technique - matgto-serge

**Version:** 1.0
**Date:** 2025-10-10
**Rust Version:** 1.75+ (stable)

---

## 🦀 Dépendances Rust Core

### Cargo.toml Principal

```toml
[package]
name = "matgto-serge"
version = "1.0.0"
edition = "2021"
rust-version = "1.75"
authors = ["matgto-serge contributors"]
license = "MIT OR Apache-2.0"
description = "Multi-language HTTP/WebSocket testing library with record/replay"
repository = "https://github.com/your-org/matgto-serge"
keywords = ["testing", "http", "websocket", "proxy", "vcr"]
categories = ["development-tools::testing", "network-programming"]

[lib]
crate-type = ["cdylib", "rlib", "staticlib"]
name = "matgto_serge"

[[bin]]
name = "matgto"
path = "src/bin/cli.rs"

[workspace]
members = [
    "core",
    "bindings/java",
    "bindings/javascript",
    "bindings/python",
    "cli",
]

[dependencies]
# === Proxy & HTTP ===
hudsucker = { version = "0.20", features = ["rustls", "rcgen"] }
hyper = { version = "0.14", features = ["full"] }
hyper-rustls = { version = "0.24", features = ["native-tokio"] }
http = "0.2"
http-body-util = "0.1"

# === WebSocket ===
tokio-tungstenite = { version = "0.21", features = ["native-tls"] }
tungstenite = "0.21"

# === Async Runtime ===
tokio = { version = "1.35", features = [
    "full",
    "macros",
    "rt-multi-thread",
    "sync",
    "time",
    "io-util",
    "net",
] }
async-trait = "0.1"
futures = "0.3"

# === Serialization ===
serde = { version = "1.0", features = ["derive", "rc"] }
serde_json = { version = "1.0", features = ["preserve_order"] }
rmp-serde = { version = "1.1", optional = true }  # MessagePack

# === TLS & Certificates ===
rustls = { version = "0.21", features = ["dangerous_configuration"] }
rustls-pemfile = "1.0"
rcgen = { version = "0.11", features = ["pem", "x509-parser"] }
webpki = "0.22"

# === Multi-Language Bindings ===
uniffi = { version = "0.25", features = ["cli"] }

# === CLI ===
clap = { version = "4.4", features = ["derive", "env", "color"], optional = true }
colored = { version = "2.1", optional = true }
indicatif = { version = "0.17", optional = true }

# === Configuration ===
toml = "0.8"
dirs = "5.0"  # Cross-platform user directories

# === Logging & Tracing ===
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-appender = "0.2"

# === Error Handling ===
thiserror = "1.0"
anyhow = "1.0"

# === Utilities ===
bytes = "1.5"
url = "2.5"
regex = "1.10"
lazy_static = "1.4"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }

# === Hashing ===
sha2 = "0.10"
base64 = "0.21"

[dev-dependencies]
# === Testing ===
tokio-test = "0.4"
mockito = "1.2"
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"
proptest = "1.4"
criterion = { version = "0.5", features = ["html_reports"] }

# === Test Fixtures ===
wiremock = "0.5"
httptest = "0.15"

[features]
default = ["cli", "msgpack"]
cli = ["clap", "colored", "indicatif"]
msgpack = ["rmp-serde"]
metrics = ["prometheus"]

[build-dependencies]
uniffi = { version = "0.25", features = ["build"] }

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"

[profile.dev]
opt-level = 1  # Compilation plus rapide en dev

[[bench]]
name = "http_proxy"
harness = false

[[bench]]
name = "websocket_proxy"
harness = false
```

---

## 📦 Dépendances Détaillées

### 1. HTTP/HTTPS Proxy

#### hudsucker = "0.20"
**Rôle :** Proxy MITM HTTP/HTTPS avec interception complète

**Features utilisées :**
- `rustls` : TLS moderne (alternative OpenSSL)
- `rcgen` : Génération certificats CA à la volée

**Pourquoi ce choix :**
- ✅ Basé sur hyper (standard de facto Rust)
- ✅ Support HTTP/2 natif
- ✅ MITM TLS clé-en-main
- ✅ Maintenance active (2024-2025)

**Alternatives considérées :**
- ❌ `mitm` (abandonné depuis 2019)
- ❌ `proxy-wasm` (trop bas niveau)
- ❌ Implémentation custom (réinventer la roue)

#### hyper = "0.14"
**Rôle :** Client/serveur HTTP sous-jacent

**Features :**
- `full` : Client + Server + HTTP/1 + HTTP/2
- `stream` : Streaming bodies pour large payloads

**Notes :**
- Version 0.14 (LTS) au lieu de 1.0 (plus stable)
- Migration vers 1.0 prévue Phase 4

#### http-body-util = "0.1"
**Rôle :** Utilitaires pour manipuler HTTP bodies

**Utilisation :**
```rust
use http_body_util::BodyExt;

let body_bytes = body.collect().await?.to_bytes();
```

---

### 2. WebSocket

#### tokio-tungstenite = "0.21"
**Rôle :** Client/serveur WebSocket avec Tokio

**Features :**
- `native-tls` : Support wss:// (WebSocket Secure)

**Pourquoi ce choix :**
- ✅ Intégration Tokio native
- ✅ Async/await first-class
- ✅ Support WebSocket RFC 6455 complet
- ✅ Utilisé par Discord, Twilight, etc.

**Alternatives :**
- `async-tungstenite` (même chose, nom différent)
- `ws` (synchrone uniquement)

#### tungstenite = "0.21"
**Rôle :** Types WebSocket core (Message, CloseFrame, etc.)

---

### 3. Async Runtime

#### tokio = "1.35"
**Rôle :** Runtime async (cœur de l'application)

**Features critiques :**
- `rt-multi-thread` : Thread pool pour parallélisme
- `macros` : `#[tokio::main]`, `tokio::select!`
- `sync` : Channels (mpsc, broadcast, watch)
- `net` : TCP/UDP async
- `io-util` : AsyncRead/AsyncWrite traits

**Configuration runtime :**
```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // 4 threads pour proxy haute performance
}
```

**Pourquoi Tokio vs async-std :**
- ✅ Écosystème plus large (hyper, tonic, etc.)
- ✅ Performance légèrement supérieure
- ✅ Tracing/debugging tools matures

---

### 4. Serialization

#### serde = "1.0"
**Rôle :** Serialization framework standard Rust

**Features :**
- `derive` : Macros `#[derive(Serialize, Deserialize)]`
- `rc` : Support Arc<T>, Rc<T>

#### serde_json = "1.0"
**Rôle :** Format cassettes lisible

**Features :**
- `preserve_order` : Ordre clés JSON stable (debugging)

**Utilisation :**
```rust
// Cassette → JSON pretty-printed
let json = serde_json::to_string_pretty(&cassette)?;
fs::write("cassette.json", json)?;
```

#### rmp-serde = "1.1" (feature msgpack)
**Rôle :** Format cassettes binaire (optionnel)

**Use case :**
- Cassettes > 10 MB
- Compression ~3x
- Désérialisation ~2x plus rapide

**Utilisation :**
```rust
// Cassette → MessagePack
let msgpack = rmp_serde::to_vec(&cassette)?;
fs::write("cassette.msgpack", msgpack)?;
```

---

### 5. TLS & Certificates

#### rustls = "0.21"
**Rôle :** Implémentation TLS moderne en Rust

**Pourquoi rustls vs OpenSSL :**
- ✅ Memory safe (100% Rust)
- ✅ Plus rapide (benchmarks)
- ✅ Pas de dépendance C (build simplifié)
- ✅ Utilisé par Cloudflare, Firefox

**Features :**
- `dangerous_configuration` : Nécessaire pour accepter certificats MITM auto-signés

#### rcgen = "0.11"
**Rôle :** Génération certificats CA et leaf certificates

**Utilisation :**
```rust
use rcgen::{CertificateParams, generate_simple_self_signed};

let subject_alt_names = vec!["localhost".to_string()];
let cert = generate_simple_self_signed(subject_alt_names)?;

println!("Certificate PEM: {}", cert.serialize_pem()?);
println!("Private Key: {}", cert.serialize_private_key_pem());
```

**Alternatives :**
- `openssl` crate (dépendances C lourdes)
- `x509-parser` (parsing uniquement, pas génération)

---

### 6. UniFFI (Multi-Language Bindings)

#### uniffi = "0.25"
**Rôle :** Génération automatique bindings Java/JS/Python/Ruby/etc.

**Architecture :**
```
Rust (lib.rs) → UDL (matgto_serge.udl) → uniffi-bindgen
                                              ↓
                        ┌────────────────────┼──────────────────┐
                        ↓                    ↓                  ↓
                   Java classes      JavaScript N-API     Python module
```

**Configuration build.rs :**
```rust
// build.rs
fn main() {
    uniffi::generate_scaffolding("src/matgto_serge.udl").unwrap();
}
```

**Génération bindings (post-build) :**
```bash
# Java
cargo run --features=uniffi/cli \
    --bin uniffi-bindgen generate src/matgto_serge.udl \
    --language java --out-dir bindings/java

# JavaScript
cargo run --features=uniffi/cli \
    --bin uniffi-bindgen generate src/matgto_serge.udl \
    --language typescript --out-dir bindings/js

# Python
cargo run --features=uniffi/cli \
    --bin uniffi-bindgen generate src/matgto_serge.udl \
    --language python --out-dir bindings/python
```

**Limitations :**
- ⚠️ Pas de support traits Rust génériques
- ⚠️ Callback Rust → langage cible limité
- ⚠️ Performance overhead (FFI boundary)

**Workarounds :**
- Simplifier API publique (pas de génériques)
- Éviter callbacks complexes (préférer polling)
- Benchmarker FFI overhead (généralement < 1%)

---

### 7. CLI (feature cli)

#### clap = "4.4"
**Rôle :** Parser arguments ligne de commande

**Features :**
- `derive` : `#[derive(Parser)]` macros
- `env` : Variables d'environnement automatiques
- `color` : Output coloré

**Exemple :**
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "matgto")]
#[command(about = "HTTP/WebSocket testing with record/replay")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start recording a new cassette
    Record {
        /// Cassette name
        name: String,
        /// Proxy port (default: 8888)
        #[arg(short, long, default_value = "8888")]
        port: u16,
    },
    /// Replay existing cassette
    Replay {
        name: String,
    },
}
```

#### colored = "2.1"
**Rôle :** Output terminal coloré

**Utilisation :**
```rust
use colored::*;

println!("{}", "✓ Cassette recorded successfully".green());
println!("{}", "✗ Error: Cassette not found".red());
```

#### indicatif = "0.17"
**Rôle :** Progress bars et spinners

**Utilisation :**
```rust
use indicatif::{ProgressBar, ProgressStyle};

let pb = ProgressBar::new(100);
pb.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap());

for i in 0..100 {
    pb.inc(1);
    thread::sleep(Duration::from_millis(50));
}
pb.finish_with_message("done");
```

---

### 8. Logging & Observability

#### tracing = "0.1"
**Rôle :** Structured logging et instrumentation

**Pourquoi tracing vs log :**
- ✅ Structured events (pas juste strings)
- ✅ Spans pour tracer requêtes complètes
- ✅ Async-aware (tokio task tracing)

**Utilisation :**
```rust
use tracing::{info, debug, instrument, span, Level};

#[instrument(skip(self))]
async fn handle_request(&mut self, req: Request<Body>) -> Result<Response<Body>> {
    debug!(?req.method, ?req.uri, "Handling request");

    let span = span!(Level::INFO, "forward_request");
    let _enter = span.enter();

    let res = self.client.request(req).await?;

    info!(
        status = %res.status(),
        "Request completed"
    );

    Ok(res)
}
```

#### tracing-subscriber = "0.3"
**Rôle :** Formatters et filtres pour tracing

**Features :**
- `env-filter` : Filtre logs via RUST_LOG env var
- `json` : Output JSON pour ingestion Elasticsearch/Datadog

**Configuration :**
```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "matgto_serge=debug,hudsucker=info".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
```

**Variables d'environnement :**
```bash
# Dev : logs détaillés
RUST_LOG=matgto_serge=trace,tokio=debug cargo run

# Prod : errors uniquement
RUST_LOG=error ./matgto

# JSON pour ingestion
RUST_LOG_FORMAT=json ./matgto
```

---

### 9. Error Handling

#### thiserror = "1.0"
**Rôle :** Macros pour définir error types

**Utilisation :**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatgtoError {
    #[error("Cassette not found: {name}")]
    CassetteNotFound { name: String },

    #[error("No matching interaction for {method} {url}")]
    NoMatchingInteraction { method: String, url: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] hyper::Error),
}
```

**Pourquoi thiserror vs anyhow :**
- `thiserror` : Bibliothèque (types erreurs publics)
- `anyhow` : Application (erreurs opaques, contexte)

#### anyhow = "1.0"
**Rôle :** Error handling simplifié pour CLI

**Utilisation :**
```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = load_config()
        .context("Failed to load configuration")?;

    start_proxy(config)
        .context("Failed to start proxy")?;

    Ok(())
}
```

---

### 10. Utilities

#### bytes = "1.5"
**Rôle :** Zero-copy byte buffers (Arc sous le capot)

**Pourquoi Bytes vs Vec<u8> :**
- ✅ Cheap clones (Arc pointeur, pas copie data)
- ✅ Slicing sans allocation
- ✅ Utilisé par hyper, tokio, tonic

```rust
use bytes::Bytes;

let data = Bytes::from("hello world");
let slice = data.slice(0..5);  // "hello" - zero copy!
```

#### url = "2.5"
**Rôle :** Parser et manipuler URLs

```rust
use url::Url;

let url = Url::parse("https://api.example.com/users?page=1")?;
println!("Host: {}", url.host_str().unwrap());
println!("Path: {}", url.path());
println!("Query: {:?}", url.query_pairs());
```

#### regex = "1.10"
**Rôle :** Matching personnalisé URLs/bodies

**Note performance :**
- Compiler regex une fois (lazy_static)
- Pas dans hot path si possible

```rust
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref URL_PATTERN: Regex = Regex::new(r"/users/\d+").unwrap();
}

fn matches_url(url: &str) -> bool {
    URL_PATTERN.is_match(url)
}
```

#### chrono = "0.4"
**Rôle :** Dates et timestamps

**Features :**
- `serde` : Serialization DateTime

```rust
use chrono::{DateTime, Utc};

let now: DateTime<Utc> = Utc::now();
println!("Recorded at: {}", now.to_rfc3339());
```

#### uuid = "1.6"
**Rôle :** Génération UUIDs pour trace IDs

```rust
use uuid::Uuid;

let id = Uuid::new_v4();
println!("Request ID: {}", id);
```

---

## 🔧 Build & Dev Tools

### build.rs (Build Script)

```rust
// build.rs
fn main() {
    // Générer scaffolding UniFFI
    uniffi::generate_scaffolding("src/matgto_serge.udl")
        .expect("Failed to generate UniFFI scaffolding");

    // Recompiler si UDL change
    println!("cargo:rerun-if-changed=src/matgto_serge.udl");

    // Embedder version Git
    let git_hash = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
}
```

### .cargo/config.toml

```toml
[build]
rustflags = ["-C", "target-cpu=native"]  # Optimisations CPU spécifiques

[alias]
# Alias custom
bench = "bench --features=metrics"
bindgen-java = "run --features=uniffi/cli --bin uniffi-bindgen -- generate src/matgto_serge.udl --language java --out-dir bindings/java"
bindgen-js = "run --features=uniffi/cli --bin uniffi-bindgen -- generate src/matgto_serge.udl --language typescript --out-dir bindings/js"
bindgen-py = "run --features=uniffi/cli --bin uniffi-bindgen -- generate src/matgto_serge.udl --language python --out-dir bindings/python"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]  # Linker plus rapide

[profile.release-lto]
inherits = "release"
lto = "fat"
codegen-units = 1
```

---

## 🧪 Testing Stack

### Unit & Integration Tests

```toml
[dev-dependencies]
tokio-test = "0.4"       # Helpers Tokio tests
mockito = "1.2"          # Mock HTTP servers
tempfile = "3.8"         # Temporary files/dirs
```

**Exemple test :**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;
    use tempfile::tempdir;

    #[test]
    fn test_recorder_save() {
        let dir = tempdir().unwrap();
        let mut recorder = Recorder::new("test".to_string());

        recorder.record_http_interaction(req, res);
        recorder.save(dir.path()).unwrap();

        assert!(dir.path().join("test.json").exists());
    }
}
```

### Property-Based Testing

```toml
[dev-dependencies]
proptest = "1.4"
```

**Exemple :**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_url_parsing_never_panics(url in "https?://[a-z]+\\.[a-z]{2,5}/.*") {
        let _ = Url::parse(&url);  // Ne doit jamais panic
    }
}
```

### Benchmarks (Criterion)

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "http_proxy"
harness = false
```

**benches/http_proxy.rs :**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_http_intercept(c: &mut Criterion) {
    c.bench_function("http intercept", |b| {
        b.iter(|| {
            // Benchmark code
            black_box(intercept_request());
        });
    });
}

criterion_group!(benches, bench_http_intercept);
criterion_main!(benches);
```

**Run benchmarks :**
```bash
cargo bench
# Résultats : target/criterion/report/index.html
```

---

## 📊 CI/CD Stack

### GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta, nightly]

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}

      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run tests
        run: cargo test --all-features

      - name: Run clippy
        run: cargo clippy -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check

  bindings:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Generate Java bindings
        run: cargo bindgen-java
      - name: Generate JS bindings
        run: cargo bindgen-js
      - name: Generate Python bindings
        run: cargo bindgen-py
```

### Outils Quality

```bash
# Linting
cargo clippy -- -D warnings

# Formatting
cargo fmt --check

# Security audit
cargo audit

# Dependency updates
cargo outdated

# Code coverage
cargo tarpaulin --out Html
```

---

## 🚀 Release & Distribution

### Cross-Compilation

```bash
# Linux ARM (Raspberry Pi)
cargo build --release --target aarch64-unknown-linux-gnu

# macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-gnu
```

### Docker Build

```dockerfile
# Dockerfile.build
FROM rust:1.75-alpine AS builder

WORKDIR /app
COPY . .

RUN apk add --no-cache musl-dev
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/matgto /matgto
ENTRYPOINT ["/matgto"]
```

### Package Managers

**crates.io (Rust) :**
```bash
cargo publish
```

**Homebrew (macOS) :**
```ruby
# Formula/matgto-serge.rb
class MatgtoSerge < Formula
  desc "Multi-language HTTP/WebSocket testing with record/replay"
  homepage "https://github.com/your-org/matgto-serge"
  url "https://github.com/your-org/matgto-serge/releases/download/v1.0.0/matgto-serge-1.0.0.tar.gz"
  sha256 "..."

  def install
    bin.install "matgto"
  end
end
```

**Maven Central (Java) :**
```xml
<!-- pom.xml -->
<dependency>
    <groupId>com.matgto</groupId>
    <artifactId>serge</artifactId>
    <version>1.0.0</version>
</dependency>
```

**npm (JavaScript) :**
```json
{
  "name": "@matgto/serge",
  "version": "1.0.0",
  "main": "index.node",
  "scripts": {
    "install": "node-pre-gyp install --fallback-to-build"
  }
}
```

**PyPI (Python) :**
```bash
# Build wheel
maturin build --release

# Publish
maturin publish
```

---

## 📝 Récapitulatif Dependencies

| Catégorie | Dépendances | Taille Binary Impact |
|-----------|-------------|---------------------|
| **Core Proxy** | hudsucker, hyper, tokio | ~3 MB |
| **WebSocket** | tokio-tungstenite | ~500 KB |
| **TLS** | rustls, rcgen | ~1 MB |
| **Serialization** | serde, serde_json | ~200 KB |
| **Bindings** | uniffi | ~100 KB |
| **CLI** (optionnel) | clap, colored | ~300 KB |
| **Total Release** | - | **~5 MB** (stripped) |

---

**Dernière mise à jour :** 2025-10-10
**Rust Version :** 1.75+
**Status :** ✅ Stack validée et prête
