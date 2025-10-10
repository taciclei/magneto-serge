# ⚡ Commandes Utiles - matgto-serge

Guide de référence rapide pour toutes les commandes de développement.

---

## 🛠️ Développement

### Build & Compilation

```bash
# Build en mode debug
cargo build

# Build en mode release (optimisé)
cargo build --release

# Vérifier compilation sans générer binaires
cargo check

# Vérifier avec tous les warnings
cargo check --all-features

# Build tous les exemples
cargo build --examples

# Build tous les benches
cargo build --benches
```

### Tests

```bash
# Tous les tests unitaires
cargo test

# Tests avec output détaillé
cargo test -- --nocapture

# Tests d'un module spécifique
cargo test --package matgto-serge --lib recorder

# Tests E2E
cargo test --test e2e_http_proxy

# Tests E2E avec tests ignorés (réseau)
cargo test --test e2e_http_proxy -- --ignored

# Tests avec coverage
cargo tarpaulin --out Html
```

### Linting & Formatage

```bash
# Linter Rust (clippy)
cargo clippy

# Clippy strict (fail sur warnings)
cargo clippy -- -D warnings

# Formatage code
cargo fmt

# Vérifier formatage sans modifier
cargo fmt -- --check

# Vérifier documentation
cargo doc --no-deps --open
```

### Benchmarks

```bash
# Tous les benchmarks
cargo bench

# Benchmark HTTP spécifique
cargo bench --bench http_proxy

# Benchmark WebSocket
cargo bench --bench websocket_proxy

# Benchmarks avec flamegraph
cargo flamegraph --bench http_proxy
```

---

## 📦 Exemples

### Exécuter Exemples

```bash
# Exemple record simple
cargo run --example simple_record

# Exemple record/replay complet
cargo run --example http_record_replay

# Avec logs debug
RUST_LOG=magneto_serge=debug cargo run --example simple_record

# Avec logs trace (très verbeux)
RUST_LOG=trace cargo run --example http_record_replay
```

---

## 🔐 Certificats TLS

### Générer & Installer

```bash
# Le certificat est généré automatiquement au premier lancement
# Il est stocké dans .magneto/certs/magneto-ca.pem

# macOS - Installer dans keychain système
sudo security add-trusted-cert -d -r trustRoot \
  -k /Library/Keychains/System.keychain \
  .magneto/certs/magneto-ca.pem

# macOS - Désinstaller
sudo security delete-certificate -c "magneto-serge CA"

# Linux (Ubuntu/Debian) - Installer
sudo cp .magneto/certs/magneto-ca.pem \
  /usr/local/share/ca-certificates/magneto-ca.crt
sudo update-ca-certificates

# Linux - Désinstaller
sudo rm /usr/local/share/ca-certificates/magneto-ca.crt
sudo update-ca-certificates --fresh

# Windows (PowerShell Admin) - Installer
Import-Certificate -FilePath .magneto\certs\magneto-ca.pem `
  -CertStoreLocation Cert:\LocalMachine\Root

# Vérifier certificat
openssl x509 -in .magneto/certs/magneto-ca.pem -text -noout
```

---

## 🚀 Utilisation Proxy

### Démarrer le Proxy

```bash
# Mode record (port 8888 par défaut)
cargo run -- record my-cassette

# Mode replay
cargo run -- replay my-cassette

# Mode auto (replay si cassette existe, sinon record)
cargo run -- auto my-cassette

# Custom port
cargo run -- record my-cassette --port 9999
```

### Configurer Clients HTTP

#### cURL

```bash
# HTTP simple
curl -x http://localhost:8888 http://httpbin.org/get

# HTTPS (nécessite certificat installé)
curl -x http://localhost:8888 https://httpbin.org/get

# HTTPS sans vérification SSL (dev seulement)
curl -x http://localhost:8888 -k https://httpbin.org/get

# POST avec données
curl -x http://localhost:8888 \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"test":"data"}' \
  https://httpbin.org/post
```

#### HTTPie

```bash
# GET simple
http --proxy=http:http://localhost:8888 http://httpbin.org/get

# POST
http --proxy=http:http://localhost:8888 POST http://httpbin.org/post test=data
```

#### wget

```bash
wget -e use_proxy=yes \
  -e http_proxy=localhost:8888 \
  -e https_proxy=localhost:8888 \
  http://httpbin.org/get
```

### Variables d'Environnement

```bash
# Configurer proxy pour session
export HTTP_PROXY=http://localhost:8888
export HTTPS_PROXY=http://localhost:8888

# Tester
curl http://httpbin.org/get

# Désactiver
unset HTTP_PROXY HTTPS_PROXY
```

---

## 🐍 Python

### Installation

```python
pip install matgto-serge  # (à venir Phase 3)
```

### Usage

```python
import requests

# Configurer proxy
proxies = {
    'http': 'http://localhost:8888',
    'https': 'http://localhost:8888',
}

# Requête via proxy
response = requests.get(
    'https://httpbin.org/get',
    proxies=proxies,
    verify=False  # Dev seulement
)
print(response.json())
```

```python
# Avec matgto-serge (à venir)
from matgto_serge import MagnetoProxy

proxy = MagnetoProxy(cassette_dir="./cassettes")
proxy.start_recording("my-test")

# Vos tests HTTP ici
response = requests.get("https://api.example.com/users")

proxy.stop_recording()
```

---

## ☕ Java

### Maven

```xml
<!-- À venir Phase 3 -->
<dependency>
    <groupId>com.matgto</groupId>
    <artifactId>serge</artifactId>
    <version>1.0.0</version>
</dependency>
```

### Usage

```java
import com.matgto.serge.MagnetoProxy;

// Configurer proxy système
System.setProperty("http.proxyHost", "localhost");
System.setProperty("http.proxyPort", "8888");
System.setProperty("https.proxyHost", "localhost");
System.setProperty("https.proxyPort", "8888");

// Ou avec OkHttp
Proxy proxy = new Proxy(
    Proxy.Type.HTTP,
    new InetSocketAddress("localhost", 8888)
);

OkHttpClient client = new OkHttpClient.Builder()
    .proxy(proxy)
    .build();
```

---

## 🟨 JavaScript/Node.js

### Installation

```bash
npm install @matgto/serge  # À venir Phase 3
```

### Usage

```javascript
// Axios
const axios = require('axios');

const response = await axios.get('https://httpbin.org/get', {
  proxy: {
    host: 'localhost',
    port: 8888
  }
});
```

```javascript
// Node.js https module
const https = require('https');
const HttpsProxyAgent = require('https-proxy-agent');

const agent = new HttpsProxyAgent('http://localhost:8888');

https.get('https://httpbin.org/get', { agent }, (res) => {
  // ...
});
```

---

## 🧹 Nettoyage

### Supprimer Fichiers Générés

```bash
# Nettoyer build
cargo clean

# Supprimer cassettes
rm -rf ./cassettes/*.json

# Supprimer certificats
rm -rf .magneto/certs/

# Supprimer tout
cargo clean && rm -rf ./cassettes .magneto/

# Nettoyer cache Cargo (ATTENTION: global)
cargo cache --autoclean
```

---

## 📊 Debugging & Profiling

### Logs

```bash
# Logs info
RUST_LOG=info cargo run

# Logs debug magneto-serge seulement
RUST_LOG=magneto_serge=debug cargo run

# Logs trace (très verbeux)
RUST_LOG=trace cargo run

# Logs colorés
RUST_LOG=magneto_serge=debug cargo run | bunyan
```

### Profiling

```bash
# Flamegraph
cargo flamegraph --example http_record_replay

# Callgrind (valgrind)
cargo build --release
valgrind --tool=callgrind \
  target/release/examples/http_record_replay

# Cachegrind
valgrind --tool=cachegrind \
  target/release/examples/http_record_replay
```

### Memory Leaks

```bash
# Valgrind memcheck
cargo build
valgrind --leak-check=full \
  target/debug/examples/http_record_replay

# Heaptrack
heaptrack target/release/examples/http_record_replay
```

---

## 📚 Documentation

### Générer Docs

```bash
# Documentation API
cargo doc --no-deps --open

# Documentation avec private items
cargo doc --no-deps --document-private-items --open

# Documentation JSON (pour tools)
cargo doc --no-deps --output-format json
```

### Vérifier Liens

```bash
# Vérifier liens dans docs
cargo deadlinks

# Vérifier README
markdown-link-check README.md
```

---

## 🔧 Maintenance

### Mise à Jour Dépendances

```bash
# Lister dépendances obsolètes
cargo outdated

# Mise à jour interactive
cargo upgrade

# Mise à jour avec breaking changes
cargo upgrade --incompatible

# Vérifier sécurité
cargo audit

# Fixer vulnérabilités
cargo audit fix
```

### Permissions Cargo (macOS/Linux)

```bash
# Fixer permissions Cargo
sudo chown -R $USER:$USER ~/.cargo

# Fixer permissions projet
sudo chown -R $USER:$USER .

# Vérifier permissions
ls -la ~/.cargo/registry/
```

---

## 🚢 Release

### Préparer Release

```bash
# Vérifier version dans Cargo.toml
cat Cargo.toml | grep version

# Tests complets
cargo test --all-features

# Clippy strict
cargo clippy -- -D warnings

# Build release
cargo build --release --all-features

# Vérifier taille binaires
ls -lh target/release/magneto

# Strip symbols
strip target/release/magneto
```

### Publier (à venir)

```bash
# Dry-run
cargo publish --dry-run

# Publier sur crates.io
cargo publish

# Tag git
git tag v1.0.0
git push --tags
```

---

## 🐛 Troubleshooting

### Erreurs Courantes

```bash
# Permission denied sur .cargo
sudo chown -R $USER:$USER ~/.cargo

# Linker error
rustup default stable
rustup update

# OpenSSL error (macOS)
brew install openssl
export OPENSSL_DIR=/usr/local/opt/openssl

# Compilation lente
# Utiliser mold linker (Linux)
sudo apt install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Ou lld (cross-platform)
cargo install lld
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
```

### Reset Complet

```bash
# Reset projet
cargo clean
rm -rf .magneto/ cassettes/
rm Cargo.lock

# Re-build
cargo build

# Reset Cargo global
rm -rf ~/.cargo/registry/
cargo fetch
```

---

## 📖 Ressources

### Documentation Interne
- [README.md](README.md) - Vue d'ensemble
- [docs/ROADMAP.md](docs/ROADMAP.md) - Feuille de route
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - Architecture
- [examples/README.md](examples/README.md) - Guide exemples
- [STATUS.md](STATUS.md) - Statut actuel

### Rust Externe
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Async Book](https://rust-lang.github.io/async-book/)

### Dépendances
- [Hudsucker Docs](https://docs.rs/hudsucker/)
- [Hyper Docs](https://docs.rs/hyper/)
- [Tokio Docs](https://docs.rs/tokio/)

---

**Dernière mise à jour :** 2025-10-10
**Version :** 0.1.0-alpha
