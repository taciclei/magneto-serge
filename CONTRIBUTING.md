# Contributing to Magneto-Serge

🎉 Merci de votre intérêt pour contribuer à **Magneto-Serge** !

Ce guide vous aidera à contribuer efficacement au projet.

## Table des Matières

- [Code of Conduct](#code-of-conduct)
- [Comment Contribuer](#comment-contribuer)
- [Setup Development](#setup-development)
- [Architecture](#architecture)
- [Conventions de Code](#conventions-de-code)
- [Tests](#tests)
- [Pull Requests](#pull-requests)
- [Reporting Bugs](#reporting-bugs)
- [Feature Requests](#feature-requests)

---

## Code of Conduct

En participant à ce projet, vous acceptez de respecter notre [Code of Conduct](CODE_OF_CONDUCT.md).

**Principes**:
- Soyez respectueux et inclusif
- Accueillez les nouveaux contributeurs
- Focalisez sur des critiques constructives
- Maintenez un environnement professionnel

---

## Comment Contribuer

### Types de Contributions

Nous accueillons les contributions suivantes:

1. **🐛 Bug Reports** - Signalez des bugs via [Issues](https://github.com/taciclei/magneto-serge/issues)
2. **✨ Feature Requests** - Proposez de nouvelles fonctionnalités
3. **📖 Documentation** - Améliorez la documentation
4. **🧪 Tests** - Ajoutez ou améliorez les tests
5. **💻 Code** - Implémentez des features ou corrigez des bugs
6. **🌍 Traductions** - Traduisez la documentation
7. **🎨 Design** - Améliorez l'UX/UI du CLI

### Workflow de Contribution

1. **Fork** le repository
2. **Clone** votre fork localement
3. **Créer une branche** pour votre contribution
4. **Implémenter** vos changements
5. **Tester** vos changements
6. **Commit** avec des messages clairs
7. **Push** vers votre fork
8. **Ouvrir une Pull Request**

---

## Setup Development

### Prérequis

- **Rust** 1.75+ ([rustup](https://rustup.rs/))
- **Git** 2.30+
- **Unix-like OS** (Linux, macOS) ou Windows avec WSL
- **Optional**:
  - Python 3.9+ (pour bindings Python)
  - Node.js 18+ (pour bindings JavaScript)
  - JDK 11+ (pour bindings Java/Kotlin)
  - Xcode 13+ (pour bindings Swift)

### Clone et Build

```bash
# Clone le repository
git clone https://github.com/taciclei/magneto-serge.git
cd magneto-serge

# Build le projet
cargo build

# Run tests
cargo test

# Run clippy (linter)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Build release
cargo build --release
```

### Générer les Bindings

```bash
# Générer bindings Python
cargo run --bin uniffi-bindgen -- generate src/magneto_serge.udl --language python --out-dir bindings/python

# Générer bindings Kotlin
cargo run --bin uniffi-bindgen -- generate src/magneto_serge.udl --language kotlin --out-dir bindings/kotlin

# Générer bindings Swift
cargo run --bin uniffi-bindgen -- generate src/magneto_serge.udl --language swift --out-dir bindings/swift
```

### Structure du Projet

```
magneto-serge/
├── src/                # Code Rust core
│   ├── lib.rs          # Entry point bibliothèque
│   ├── proxy/          # Proxy HTTP/HTTPS/WebSocket
│   ├── cassette/       # Gestion cassettes
│   ├── recorder/       # Enregistrement
│   ├── player/         # Rejeu
│   └── cli/            # CLI tool
├── bindings/           # Bindings multi-langages
│   ├── python/         # Bindings Python
│   ├── kotlin/         # Bindings Kotlin
│   ├── swift/          # Bindings Swift
│   ├── java/           # Bindings Java
│   └── javascript/     # Bindings JavaScript
├── tests/              # Tests d'intégration
├── benches/            # Benchmarks Criterion
├── docs/               # Documentation
├── examples/           # Exemples d'utilisation
└── .github/            # CI/CD workflows
```

---

## Architecture

Consultez [ARCHITECTURE.md](docs/ARCHITECTURE.md) pour une compréhension détaillée de l'architecture.

### Composants Clés

1. **Proxy Layer** (`src/proxy/`)
   - Serveur HTTP/HTTPS (Hyper + Rustls)
   - Intercepteur WebSocket (tokio-tungstenite)
   - Handler MITM pour CONNECT

2. **Record/Replay Engine** (`src/cassette/`, `src/recorder/`, `src/player/`)
   - Cassette: Sérialisation JSON/MessagePack
   - Recorder: Capture des interactions
   - Player: Matching et rejeu

3. **Public API** (`src/lib.rs`)
   - `MagnetoProxy`: API publique
   - Exposition via UniFFI pour bindings

4. **CLI** (`src/cli/`)
   - 8 commandes (record, replay, auto, list, inspect, delete, init, version)
   - Configuration via magneto.toml

---

## Conventions de Code

### Rust

- **Formatting**: Utilisez `cargo fmt` (rustfmt)
- **Linting**: Utilisez `cargo clippy -- -D warnings`
- **Naming**:
  - `snake_case` pour fonctions et variables
  - `PascalCase` pour types et traits
  - `SCREAMING_SNAKE_CASE` pour constantes
- **Documentation**: Documentez toutes les fonctions publiques
- **Tests**: Ajoutez des tests pour chaque nouvelle fonctionnalité

### Commits

Utilisez [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: Nouvelle fonctionnalité
- `fix`: Correction de bug
- `docs`: Documentation
- `style`: Formatting, sans changement de code
- `refactor`: Refactoring
- `perf`: Amélioration de performance
- `test`: Ajout/modification de tests
- `chore`: Maintenance (CI, deps, etc.)

**Exemples**:
```
feat(proxy): add WebSocket compression support

Implement WebSocket per-message compression (RFC 7692) for reduced
bandwidth usage.

Closes #123
```

```
fix(player): correct request matching for multipart bodies

The request signature hash was incorrectly computed for multipart
form data, causing false match failures.

Fixes #456
```

### Documentation

- **Code comments**: Expliquez le "pourquoi", pas le "quoi"
- **Doc comments**: Utilisez `///` pour documenter les fonctions publiques
- **Markdown**: Utilisez Markdown pour la documentation externe
- **Exemples**: Incluez des exemples dans la documentation

---

## Tests

### Run Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# Integration tests
cargo test --test '*'

# With output
cargo test -- --nocapture

# Benchmarks
cargo bench
```

### Écrire des Tests

#### Tests Unitaires

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cassette_serialization() {
        let cassette = Cassette::new("test");
        let json = serde_json::to_string(&cassette).unwrap();
        assert!(json.contains("test"));
    }
}
```

#### Tests d'Intégration

```rust
// tests/integration_test.rs
use magneto_serge::{MagnetoProxy, ProxyMode};

#[tokio::test]
async fn test_record_replay() {
    let mut proxy = MagnetoProxy::new("./test_cassettes");
    proxy.set_port(8888);
    proxy.start_recording("test");

    // ... test code ...

    proxy.shutdown();
}
```

#### Tests de Benchmarks

```rust
// benches/proxy_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use magneto_serge::MagnetoProxy;

fn bench_proxy_startup(c: &mut Criterion) {
    c.bench_function("proxy_startup", |b| {
        b.iter(|| {
            let proxy = MagnetoProxy::new(black_box("./cassettes"));
            proxy
        });
    });
}

criterion_group!(benches, bench_proxy_startup);
criterion_main!(benches);
```

---

## Pull Requests

### Checklist PR

Avant de soumettre une PR, vérifiez:

- [ ] Code compilé sans warnings (`cargo build`)
- [ ] Tests passent (`cargo test`)
- [ ] Clippy passe (`cargo clippy -- -D warnings`)
- [ ] Code formaté (`cargo fmt`)
- [ ] Documentation ajoutée/mise à jour
- [ ] Tests ajoutés pour les nouvelles features
- [ ] CHANGELOG.md mis à jour (si applicable)
- [ ] Commit messages suivent les conventions

### Template PR

```markdown
## Description

Brief description of changes.

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## How Has This Been Tested?

Describe the tests you ran to verify your changes.

## Checklist

- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
```

### Review Process

1. **Automated Checks**: CI/CD vérifie le build, tests, linting
2. **Code Review**: Un mainteneur review votre code
3. **Feedback**: Répondez aux commentaires et ajustez si nécessaire
4. **Approval**: Une fois approuvée, la PR sera merged

---

## Reporting Bugs

### Avant de Reporter

1. **Vérifiez** les [Issues existantes](https://github.com/taciclei/magneto-serge/issues)
2. **Testez** avec la dernière version
3. **Reproduisez** le bug de manière consistante

### Template Bug Report

```markdown
**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Initialize proxy with '...'
2. Call method '...'
3. See error

**Expected behavior**
A clear description of what you expected to happen.

**Actual behavior**
What actually happened.

**Environment**
- OS: [e.g., macOS 14.0]
- Rust version: [e.g., 1.75.0]
- Magneto-Serge version: [e.g., 0.1.0]

**Additional context**
Add any other context about the problem here.
```

---

## Feature Requests

### Template Feature Request

```markdown
**Is your feature request related to a problem?**
A clear and concise description of what the problem is. Ex. I'm always frustrated when [...]

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```

---

## Ressources

- **Documentation**: [README.md](README.md)
- **Architecture**: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- **Roadmap**: [docs/ROADMAP.md](docs/ROADMAP.md)
- **Examples**: [docs/EXAMPLES.md](docs/EXAMPLES.md)
- **Rust Book**: https://doc.rust-lang.org/book/
- **UniFFI Guide**: https://mozilla.github.io/uniffi-rs/

---

## Licence

En contribuant à Magneto-Serge, vous acceptez que vos contributions soient licenciées sous la licence MIT OR Apache-2.0.

---

## Questions?

Si vous avez des questions, n'hésitez pas à:
- Ouvrir une [Discussion](https://github.com/taciclei/magneto-serge/discussions)
- Contacter les mainteneurs via [Issues](https://github.com/taciclei/magneto-serge/issues)

---

🙏 **Merci** de contribuer à Magneto-Serge !

🦀 Built with Rust for maximum performance and safety
