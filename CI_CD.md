# CI/CD Configuration - Magnéto-Serge

Ce document décrit la configuration CI/CD pour le projet Magnéto-Serge.

## Vue d'ensemble

Le projet utilise **GitHub Actions** pour automatiser :
- ✅ Tests et validation du code (CI)
- 🚀 Publication sur les registres de packages (CD)
- 📦 Génération des binaires multi-plateformes
- 🐳 Construction et publication d'images Docker

## Workflows disponibles

### 1. CI - Intégration Continue (`ci.yml`)

**Déclenchement :** Push ou PR sur `master`, `main`, ou `develop`

**Jobs :**

#### `test` - Tests Multi-plateformes
- **Plateformes :** Ubuntu, macOS, Windows
- **Versions Rust :** stable, beta
- **Actions :**
  - Exécute tous les tests avec `cargo test --all-features`
  - Exécute les tests sans features par défaut
  - Utilise le cache Cargo pour accélérer les builds

#### `lint` - Vérification du Code
- **Actions :**
  - Vérifie le formatage avec `cargo fmt`
  - Analyse le code avec `cargo clippy`
  - Échoue si le code n'est pas formaté ou a des warnings

#### `build-cli` - Compilation du CLI
- **Plateformes :** Ubuntu, macOS, Windows
- **Actions :**
  - Compile le binaire CLI en mode release
  - Teste la commande `version`
  - Upload les artefacts pour chaque plateforme

#### `build-bindings` - Génération des Bindings
- **Actions :**
  - Installe UniFFI bindgen
  - Génère les bindings Python, Kotlin, Swift
  - Valide que tous les bindings se génèrent correctement

#### `security` - Audit de Sécurité
- **Actions :**
  - Exécute `cargo audit` pour détecter les vulnérabilités
  - Utilise la base de données RustSec

#### `coverage` - Couverture de Code
- **Actions :**
  - Génère un rapport de couverture avec `cargo-tarpaulin`
  - Upload vers Codecov (optionnel)

### 2. CD - Déploiement Continu (`cd.yml`)

**Déclenchement :** Push d'un tag `v*.*.*` (ex: `v0.1.0`)

**Jobs :**

#### `publish-crates-io` - Publication Rust
- Publie le package sur [crates.io](https://crates.io)
- **Secret requis :** `CARGO_REGISTRY_TOKEN`

#### `publish-npm` - Publication JavaScript
- Publie le package sur [NPM](https://npmjs.com)
- **Secret requis :** `NPM_TOKEN`
- Package: `@magneto/serge`

#### `publish-pypi` - Publication Python
- Construit les wheels avec `maturin`
- Publie sur [PyPI](https://pypi.org)
- **Secret requis :** `PYPI_TOKEN`
- Package: `magneto-serge`

#### `publish-maven` - Publication Java/Kotlin
- Publie sur Maven Central (OSSRH)
- **Secrets requis :**
  - `OSSRH_USERNAME`
  - `OSSRH_PASSWORD`
  - `GPG_PRIVATE_KEY`
  - `GPG_PASSPHRASE`

#### `create-github-release` - Release GitHub
- Crée une release GitHub avec les artefacts
- Génère automatiquement les notes de version
- **Dépend de :** tous les jobs de publication

#### `build-docker` - Image Docker
- Construit et publie l'image Docker
- **Plateformes :** linux/amd64, linux/arm64
- **Tags :** `latest` et version spécifique
- **Secrets requis :**
  - `DOCKER_USERNAME`
  - `DOCKER_PASSWORD`

### 3. Release - Binaires Multi-plateformes (`release.yml`)

**Déclenchement :** Push d'un tag `v*.*.*`

**Plateformes supportées :**
- Linux: x86_64, ARM64
- macOS: x86_64 (Intel), ARM64 (Apple Silicon)
- Windows: x86_64

**Actions :**
- Compile les binaires pour toutes les plateformes
- Strip et compresse les binaires (tar.gz pour Linux/macOS, zip pour Windows)
- Génère les checksums SHA256
- Crée une release GitHub avec tous les artefacts

## Configuration des Secrets

Pour utiliser le CI/CD complet, configure ces secrets dans les paramètres GitHub du repo :

### Obligatoires pour CD

| Secret | Description | Obtention |
|--------|-------------|-----------|
| `CARGO_REGISTRY_TOKEN` | Token crates.io | https://crates.io/settings/tokens |
| `NPM_TOKEN` | Token NPM | https://www.npmjs.com/settings/~/tokens |
| `PYPI_TOKEN` | Token PyPI | https://pypi.org/manage/account/token/ |
| `OSSRH_USERNAME` | Utilisateur Sonatype | https://issues.sonatype.org |
| `OSSRH_PASSWORD` | Mot de passe Sonatype | https://issues.sonatype.org |
| `GPG_PRIVATE_KEY` | Clé GPG privée | `gpg --armor --export-secret-keys YOUR_KEY_ID` |
| `GPG_PASSPHRASE` | Passphrase GPG | Ton mot de passe GPG |

### Optionnels

| Secret | Description | Obtention |
|--------|-------------|-----------|
| `DOCKER_USERNAME` | Nom d'utilisateur Docker Hub | https://hub.docker.com |
| `DOCKER_PASSWORD` | Token Docker Hub | https://hub.docker.com/settings/security |

## Workflow de Release

### 1. Préparer la Release

```bash
# 1. Mettre à jour la version dans Cargo.toml
vim Cargo.toml

# 2. Mettre à jour CHANGELOG.md
vim CHANGELOG.md

# 3. Committer les changements
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to v0.2.0"

# 4. Créer un tag
git tag -a v0.2.0 -m "Release v0.2.0"

# 5. Pousser le tag
git push origin v0.2.0
```

### 2. CI/CD Automatique

Une fois le tag poussé :
1. ✅ Le workflow `release.yml` compile les binaires
2. 🚀 Le workflow `cd.yml` publie sur tous les registres
3. 📦 Les artefacts sont disponibles dans la release GitHub

### 3. Vérification

Après la publication, vérifier :
- https://crates.io/crates/magneto-serge
- https://npmjs.com/package/@magneto/serge
- https://pypi.org/project/magneto-serge/
- https://central.sonatype.com/artifact/io.github.magneto/serge
- https://hub.docker.com/r/[username]/magneto-serge
- https://github.com/taciclei/magneto-serge/releases

## Tests en Local

### Tester le formatage
```bash
cargo fmt --all -- --check
```

### Tester clippy
```bash
cargo clippy --all-features -- -D warnings
```

### Tester la compilation
```bash
# Tests
cargo test --all-features

# CLI
cargo build --bin matgto --features cli --release

# Bindings
cargo install uniffi-bindgen-cli --version 0.28.3
uniffi-bindgen generate src/magneto_serge.udl --language python --out-dir /tmp/test
```

### Tester Docker
```bash
# Build
docker build -t magneto-serge:test .

# Run
docker run --rm magneto-serge:test version
```

### Tester la publication (dry-run)
```bash
cargo publish --dry-run
```

## Optimisations

### Cache
Les workflows utilisent `actions/cache` pour :
- Registre Cargo
- Index Cargo
- Dossier target

Cela réduit significativement les temps de build (de ~5min à ~1min).

### Matrices
Les jobs utilisent des matrices pour tester plusieurs :
- Systèmes d'exploitation
- Versions de Rust
- Architectures

### Parallélisation
Les jobs indépendants s'exécutent en parallèle pour réduire le temps total.

## Dépannage

### Échec du workflow CI

1. **Tests échouent** : Vérifier les logs, exécuter `cargo test` localement
2. **Clippy warnings** : Exécuter `cargo clippy --all-features --fix`
3. **Formatage** : Exécuter `cargo fmt --all`

### Échec du workflow CD

1. **Token invalide** : Vérifier que les secrets sont correctement configurés
2. **Version déjà publiée** : Incrémenter la version dans `Cargo.toml`
3. **Build échoue** : Tester `cargo publish --dry-run` localement

### Permissions

Si les workflows ne se déclenchent pas :
1. Vérifier les permissions dans Settings > Actions
2. Autoriser "Read and write permissions"
3. Autoriser "Allow GitHub Actions to create and approve pull requests"

## Prochaines Améliorations

- [ ] Ajouter des tests d'intégration dans le CI
- [ ] Configurer des benchmarks automatiques
- [ ] Ajouter des notifications Slack/Discord
- [ ] Mettre en place le versioning sémantique automatique
- [ ] Ajouter des checks de performance (régression)

---

**Note :** Ce CI/CD est prêt à l'emploi mais nécessite la configuration des secrets pour fonctionner complètement.
