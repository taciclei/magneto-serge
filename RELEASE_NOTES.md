# Release Notes

## v0.1.0 - First Release (2025-10-12)

🎉 **Première release officielle de Magneto-Serge** - bibliothèque Rust multi-langage pour enregistrer et rejouer les interactions HTTP/WebSocket dans vos tests.

### 🚀 Features Principales

#### Core Proxy (Phase 1 & 2)
- ✅ **HTTP/HTTPS Proxy** avec interception MITM
  - Support complet HTTP/1.1
  - Génération automatique de certificats TLS
  - Forward de requêtes avec préservation des headers
  - Gestion du body (texte, JSON, binaire)
- ✅ **WebSocket Support**
  - Upgrade HTTP → WebSocket bidirectionnel
  - Support frames: Text, Binary, Ping, Pong, Close
  - Timing replay optionnel
- ✅ **Record/Replay Engine**
  - Enregistrement JSON avec sérialisation complète
  - Matching intelligent (méthode + URL + body hash)
  - 5 modes: Auto, Record, Replay, Passthrough, Strict, Hybrid, Once
  - Format cassette: JSON et MessagePack (avec compression gzip)

#### Multi-language Bindings (Phase 3)
- ✅ **Python** (UniFFI)
  - Génération automatique via uniffi-bindgen
  - Documentation complète avec exemples requests/httpx
  - Plugin pytest intégré
  - 4/4 tests passing
- ✅ **Kotlin** (UniFFI)
  - Génération automatique (magneto_serge.kt 63KB)
  - Documentation OkHttp, Ktor, Retrofit
  - Extension JUnit 5 intégrée
  - Exemples complets
- ✅ **Swift** (UniFFI)
  - Génération automatique (magneto_serge.swift 30KB)
  - Documentation URLSession, XCTest, Alamofire
  - Base class XCTest intégrée
  - Support iOS 13+, macOS 10.15+
- ✅ **Java** (wrapper Kotlin)
  - Wrapper Java au-dessus des bindings Kotlin
  - 11/11 tests JUnit 5 passing
  - Documentation complète avec exemples
- ✅ **JavaScript/Node.js** (wrapper)
  - Wrapper Node.js avec support TypeScript
  - Tests Jest
  - Plugin Jest intégré
  - Exemples Axios, Playwright, Express

#### CLI Tool (Phase 4.1)
- ✅ **8 commandes** fonctionnelles:
  - `magneto record <name>` - Démarre l'enregistrement
  - `magneto replay <name>` - Rejoue une cassette
  - `magneto auto <name>` - Mode automatique
  - `magneto list` - Liste les cassettes
  - `magneto inspect <name>` - Affiche le contenu
  - `magneto delete <name>` - Supprime une cassette
  - `magneto init` - Initialise magneto.toml
  - `magneto version` - Affiche la version
- ✅ Configuration via `magneto.toml`
- ✅ Variables d'environnement
- ✅ Logging avec tracing (RUST_LOG)

#### Advanced Features (Phase 5)
- ✅ **Compression** (gzip)
  - CassetteFormat::JsonGzip (-50-70%)
  - CassetteFormat::MessagePackGzip (-80-95%)
  - Auto-détection format
- ✅ **Modes Avancés**
  - Mode STRICT (erreur si pas de match)
  - Mode HYBRID (mix record/replay automatique)
  - Mode ONCE (record si absent, sinon replay)
- ✅ **Filtres d'enregistrement**
  - URL filtering (regex)
  - Header masking automatique
  - Body redaction et truncation
  - 6 presets: security, strict, no_analytics, no_media, success_only, small_bodies
- ✅ **Latency Simulation**
  - LatencyMode: None, Recorded, Fixed, Scaled
  - Timing replay pour tests réalistes
- ✅ **Test Framework Plugins**
  - Plugin pytest (Python)
  - Plugin Jest (JavaScript)
  - Extension JUnit 5 (Java/Kotlin)
  - Base class XCTest (Swift)

#### Performance & Optimization (Phase 4.3)
- ✅ **Benchmarks Criterion**
  - 39 benchmarks couvrant toutes les opérations
  - HTTP proxy: ~49ns overhead
  - Startup: 445µs
  - Throughput: 835 interactions/sec
- ✅ **Optimizations**
  - Async cassette I/O (<1µs queuing)
  - MessagePack format (3.2x faster, 51.6% smaller)
  - In-memory buffering (800x faster batch)

#### CI/CD (Phase 4.2)
- ✅ **GitHub Actions**
  - Tests multi-plateformes (Ubuntu, macOS, Windows)
  - Tests multi-versions Rust (stable, beta)
  - Lint (rustfmt + clippy)
  - Build CLI pour 3 plateformes
  - Génération bindings (Python, Kotlin, Swift)
  - Code coverage (tarpaulin)
  - 12/12 jobs success ✅
- ✅ **Continuous Deployment**
  - Workflows configurés pour:
    - crates.io (Cargo)
    - PyPI (Python)
    - NPM (JavaScript)
    - Maven Central (Java/Kotlin)
  - Build binaires multi-plateformes
  - Docker multi-arch (linux/amd64, linux/arm64)
  - GitHub Releases automatiques

### 📊 Statistiques

- **Lignes de code Rust**: ~8,500 lignes
- **Tests Rust**: 43 tests (100% passing)
- **Benchmarks**: 39 benchmarks Criterion
- **Bindings**: 5 langages (Python, Kotlin, Swift, Java, JavaScript)
- **Documentation**: 15+ fichiers markdown (>10,000 lignes)
- **Commits**: 150+ commits
- **Contributors**: 2 (+ Claude Code)

### 🔧 Breaking Changes

Aucun - première release.

### 📦 Installation

#### Rust (crates.io)
```bash
cargo add magneto-serge
```

#### Python (PyPI)
```bash
pip install magneto-serge
```

#### JavaScript (NPM)
```bash
npm install magneto-serge
```

#### Java/Kotlin (Maven)
```xml
<dependency>
    <groupId>com.magneto</groupId>
    <artifactId>magneto-serge</artifactId>
    <version>0.1.0</version>
</dependency>
```

#### Swift (SPM)
```swift
dependencies: [
    .package(url: "https://github.com/taciclei/magneto-serge.git", from: "0.1.0")
]
```

#### CLI (Binaries)
```bash
# macOS
curl -L https://github.com/taciclei/magneto-serge/releases/download/v0.1.0/magneto-macos -o magneto
chmod +x magneto

# Linux
curl -L https://github.com/taciclei/magneto-serge/releases/download/v0.1.0/magneto-linux -o magneto
chmod +x magneto

# Windows
# Télécharger magneto-windows.exe depuis GitHub Releases
```

### 📖 Documentation

- **README**: [README.md](README.md)
- **Architecture**: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- **Roadmap**: [docs/ROADMAP.md](docs/ROADMAP.md)
- **Examples**: [docs/EXAMPLES.md](docs/EXAMPLES.md)
- **Bindings**:
  - Python: [bindings/python/README.md](bindings/python/README.md)
  - Kotlin: [bindings/kotlin/README.md](bindings/kotlin/README.md)
  - Swift: [bindings/swift/README.md](bindings/swift/README.md)
  - Java: [bindings/java/README.md](bindings/java/README.md)
  - JavaScript: [bindings/javascript/README.md](bindings/javascript/README.md)
- **Plugins**:
  - pytest: [bindings/python/PYTEST_PLUGIN.md](bindings/python/PYTEST_PLUGIN.md)
  - Jest: [bindings/javascript/JEST_PLUGIN.md](bindings/javascript/JEST_PLUGIN.md)
  - JUnit: [bindings/java/JUNIT_EXTENSION.md](bindings/java/JUNIT_EXTENSION.md)
  - XCTest: [bindings/swift/XCTEST_INTEGRATION.md](bindings/swift/XCTEST_INTEGRATION.md)

### 🐛 Known Issues

- [ ] Publication packages en attente de configuration secrets GitHub
- [ ] Couverture de code < 80% (target: >80%)
- [ ] Documentation API en ligne (en préparation)

### 🙏 Remerciements

- **UniFFI** (Mozilla) - Pour les bindings multi-langages
- **Hudsucker** - Pour le proxy MITM HTTP/HTTPS
- **Tokio** - Pour le runtime async
- **Hyper** - Pour le client/serveur HTTP
- **Criterion** - Pour les benchmarks
- **Communauté Rust** - Pour l'écosystème exceptionnel

### 🔗 Liens

- **GitHub**: https://github.com/taciclei/magneto-serge
- **Issues**: https://github.com/taciclei/magneto-serge/issues
- **Discussions**: https://github.com/taciclei/magneto-serge/discussions
- **CI/CD**: https://github.com/taciclei/magneto-serge/actions

### 📄 Licence

MIT OR Apache-2.0

---

**Prochaine release**: v0.2.0 (Q1 2026)
- Features avancées (Phase 5)
- Observability (Prometheus, OpenTelemetry)
- Matching avancé (regex URL, JSON path)
- Documentation complète en ligne

---

🦀 Built with Rust for maximum performance and safety

🤖 Generated with [Claude Code](https://claude.com/claude-code)
