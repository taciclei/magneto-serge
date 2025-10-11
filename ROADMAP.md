# 🗺️ Roadmap - Magnéto-Serge

## Vision

**Magnéto-Serge** est une bibliothèque Rust multi-langage pour enregistrer et rejouer les interactions HTTP/WebSocket dans vos tests. Inspiré par VCR (Ruby) et Polly (Node.js), mais écrit en Rust pour performance et sécurité.

---

## 📊 État Global

| Phase | Description | Statut | Progression |
|-------|-------------|--------|-------------|
| **Phase 1** | HTTP/HTTPS Proxy | ✅ Terminé | 100% |
| **Phase 2** | WebSocket Support | ✅ Terminé | 100% |
| **Phase 3** | Multi-language Bindings | ✅ Terminé | 100% |
| **Phase 4** | CLI & Production | ✅ Terminé | 100% |
| **Phase 5** | Advanced Features | 🟡 En cours | 25% |

---

## Phase 1 : HTTP/HTTPS Proxy ✅

**Objectif** : Créer un proxy HTTP/HTTPS capable d'enregistrer et rejouer les requêtes.

### 1.1 - Core Infrastructure ✅
- [x] Structure du projet Rust
- [x] Modules de base (proxy, cassette, recorder, player)
- [x] Types de données (HttpRequest, HttpResponse)
- [x] Gestion d'erreurs (MatgtoError)

### 1.2 - HTTP Proxy ✅
- [x] Serveur HTTP (Hyper)
- [x] Client HTTP (Hyper + Rustls)
- [x] Gestion des headers
- [x] Gestion du body (texte, JSON, binaire)
- [x] Forward des requêtes

### 1.3 - HTTPS & TLS ✅
- [x] Support HTTPS
- [x] Génération de certificat CA auto-signé
- [x] Génération de certificats par domaine
- [x] Interception MITM (CONNECT)
- [x] TLS client pour forward

### 1.4 - Record/Replay ✅
- [x] Enregistrement JSON (Cassette)
- [x] Recorder : capture des requêtes/réponses
- [x] Player : replay depuis cassette
- [x] Matching des requêtes (méthode, URL, body hash)
- [x] Modes : AUTO, RECORD, REPLAY, PASSTHROUGH

### 1.5 - Tests & Validation ✅
- [x] Tests unitaires
- [x] Tests d'intégration
- [x] Exemple d'utilisation
- [x] Documentation Rust

---

## Phase 2 : WebSocket Support ✅

**Objectif** : Ajouter le support WebSocket pour enregistrer/rejouer les échanges temps réel.

### 2.1 - WebSocket Proxy ✅
- [x] Upgrade HTTP → WebSocket
- [x] Bi-directional message forwarding
- [x] Support des frames (Text, Binary, Ping, Pong, Close)
- [x] Gestion des connexions WebSocket

### 2.2 - WebSocket Record/Replay ✅
- [x] WebSocketRecorder : capture des messages
- [x] WebSocketPlayer : replay des messages
- [x] WebSocketCassette : format JSON
- [x] Matching des messages
- [x] Replay timing (optionnel)

### 2.3 - Tests WebSocket ✅
- [x] Tests unitaires WebSocket
- [x] Tests d'intégration
- [x] Exemple WebSocket
- [x] Documentation

---

## Phase 3 : Multi-language Bindings ✅

**Objectif** : Rendre matgto-serge utilisable dans tous les langages majeurs.

### 3.1 - UniFFI Integration ✅
- [x] Ajout de UniFFI au projet
- [x] Création du fichier UDL (matgto_serge.udl)
- [x] Exposition de l'API Rust
- [x] Correction des 54 erreurs de compilation
- [x] Build Rust réussi (0 erreurs)
- [x] Upgrade uniffi 0.25 → 0.28

### 3.2 - Bindings Generation ✅
- [x] **Python** (PyPI)
  - [x] Génération via UniFFI
  - [x] Tests (4/4 ✓)
  - [x] Documentation complète
  - [x] Exemple d'utilisation
  - [ ] Package PyPI

- [x] **Kotlin** (Maven)
  - [x] Génération via UniFFI
  - [x] Documentation complète
  - [ ] Tests
  - [ ] Package Maven

- [x] **Swift** (Swift Package Manager)
  - [x] Génération via UniFFI
  - [x] Documentation complète
  - [ ] Tests
  - [ ] Package SPM

- [x] **Java** (Maven)
  - [x] Wrapper autour de Kotlin
  - [x] Tests JUnit 5 (11 tests)
  - [x] Documentation complète
  - [x] Exemples d'utilisation
  - [ ] Build Gradle
  - [ ] Package Maven

- [x] **JavaScript/Node.js** (NPM)
  - [x] Wrapper Node.js
  - [x] Support TypeScript (index.d.ts)
  - [x] Tests Jest
  - [x] Documentation complète
  - [x] Exemples (Jest, Playwright, Express, Axios)
  - [ ] Tests npm
  - [ ] Package NPM

### 3.3 - Distribution ✅
- [x] Package PyPI (Python)
  - [x] Configuration prête
  - [x] Workflow CD configuré
  - [ ] Publication (en attente de secrets GitHub)
- [x] Package Maven Central (Java/Kotlin)
  - [x] pom.xml créé
  - [x] Guide de publication (PUBLISHING.md)
  - [x] Workflow CD configuré
  - [ ] Publication (en attente de secrets GitHub)
- [x] Package NPM (JavaScript)
  - [x] package.json configuré
  - [x] Guide de publication (PUBLISHING.md)
  - [x] Workflow CD configuré
  - [ ] Publication (en attente de secrets GitHub)
- [x] Package Swift Package Manager
  - [x] Génération Swift via UniFFI
  - [x] Workflow CD configuré
  - [ ] Package.swift (optionnel)
- [x] Package Cargo (crates.io)
  - [x] Cargo.toml configuré
  - [x] Licenses créées
  - [x] Workflow CD configuré
  - [ ] Publication (en attente de secrets GitHub)
- [x] CI/CD pour publication automatique
  - [x] Workflow CD complet (.github/workflows/cd.yml)
  - [x] Build multi-plateformes
  - [x] Publication automatique sur tag

### 3.4 - Documentation ✅
- [x] README par langage
- [x] BINDINGS.md (synthèse)
- [x] Exemples d'utilisation
- [ ] Documentation API en ligne

---

## Phase 4 : CLI & Production ✅

**Objectif** : Créer un CLI et préparer la production.

### 4.1 - CLI (Command Line Interface) ✅
- [x] Outil `magneto` avec clap (renommé de `matgto`)
- [x] Commandes :
  - [x] `magneto record <name>` : Démarre l'enregistrement
  - [x] `magneto replay <name>` : Rejoue une cassette
  - [x] `magneto auto <name>` : Mode automatique (record si absent, sinon replay)
  - [x] `magneto list` : Liste les cassettes
  - [x] `magneto inspect <name>` : Affiche le contenu
  - [x] `magneto delete <name>` : Supprime une cassette
  - [x] `magneto init` : Initialise configuration magneto.toml
  - [x] `magneto version` : Affiche la version
- [x] Configuration via fichier (magneto.toml)
- [x] Variables d'environnement (via clap)
- [x] Logging avec tracing
- [x] CLI testé et fonctionnel
- [x] Erreurs de compilation corrigées
- [x] Renommage complet MatgtoProxy → MagnetoProxy

### 4.2 - CI/CD ✅
- [x] GitHub Actions
  - [x] Tests Rust multi-plateformes (Ubuntu, macOS, Windows)
  - [x] Tests Rust multi-versions (stable, beta)
  - [x] Lint (rustfmt + clippy)
  - [x] Build CLI pour 3 plateformes
  - [x] Génération bindings (Python, Kotlin, Swift) via binaire uniffi-bindgen
  - [x] Code coverage (tarpaulin)
  - [x] CI complètement verte (12/12 jobs success)
- [x] Release automatique (CD)
  - [x] Publication crates.io (workflow configuré)
  - [x] Publication NPM (workflow configuré)
  - [x] Publication PyPI (workflow configuré)
  - [x] Publication Maven Central (workflow configuré)
  - [x] Création releases GitHub
  - [x] Build binaires multi-plateformes
  - [x] Docker multi-arch (linux/amd64, linux/arm64)
- [x] Documentation CI/CD
- [x] Workflows corrigés pour utiliser `magneto` au lieu de `matgto`
- [x] Binaire `uniffi-bindgen` créé pour génération de bindings
- [ ] Configuration secrets GitHub (pour publication effective)

### 4.3 - Production Ready ✅
- [x] **Benchmarks de performance** ✅
  - [x] HTTP proxy benchmarks (7 groups, 21 benchmarks)
  - [x] WebSocket proxy benchmarks (8 groups, 18 benchmarks)
  - [x] Latency measurements (~49ns overhead)
  - [x] Throughput analysis (835 interactions/sec)
  - [x] Complete BENCHMARKS.md documentation
  - [x] Optimization priorities identified
- [x] **Optimisations** ✅
  - [x] **Async cassette I/O** (background writer, <1µs queuing)
  - [x] **MessagePack binary format** (3.2x faster, 51.6% smaller)
  - [x] **In-memory cassette buffering** (800x faster for batch)
  - [x] Serialization benchmarks (JSON vs MessagePack)
  - [x] OPTIMIZATIONS.md documentation
  - [ ] Memory-mapped large cassettes (future v0.3.0)
- [ ] Sécurité : audit des dépendances
- [ ] Documentation complète
- [ ] Site web / GitHub Pages

### 4.4 - Release 1.0
- [ ] Release notes
- [ ] Migration guide
- [ ] Blog post / annonce
- [ ] Soumission à awesome-rust

---

## Phase 5 : Advanced Features 🟡

**Objectif** : Fonctionnalités avancées et améliorations.

**Statut** : En cours (5.3 et 5.4 terminés)

### 5.1 - Cassette Management
- [ ] Édition de cassettes (modifier réponses)
- [ ] Fusion de cassettes
- [ ] Filtrage de cassettes (supprimer certaines requêtes)
- [ ] Compression des cassettes
- [ ] Chiffrement des cassettes sensibles

### 5.2 - Matching Avancé
- [ ] Matching par regex sur URL
- [ ] Matching par body partiel (JSON path)
- [ ] Matching par headers spécifiques
- [ ] Custom matchers
- [ ] Stratégies de matching configurables

### 5.3 - Modes Avancés
- [x] **Mode STRICT** ✅ (erreur si pas de match)
  - [x] ProxyMode::ReplayStrict enum variant
  - [x] Player::load_strict() method
  - [x] MagnetoProxy::replay_strict() method
  - [x] Enhanced error logging with 🔒 prefix
  - [x] 3 unit tests + 7 integration tests
  - [x] Documentation complète (STRICT_MODE.md)
- [ ] Mode HYBRID (mix record/replay)
- [ ] Mode UPDATE (met à jour cassettes existantes)
- [ ] Mode ONCE (record uniquement si absent)

### 5.4 - Recording Features ✅
- [x] **Filtres d'enregistrement** ✅
  - [x] URL filtering (regex patterns)
  - [x] Header filtering (masquage automatique)
  - [x] Body transformation (redaction, truncation)
  - [x] Status code filtering
  - [x] Content-type filtering
  - [x] Body size limiting
  - [x] 6 Filter presets (security, strict, no_analytics, no_media, success_only, small_bodies)
  - [x] 14 tests unitaires + 12 tests d'intégration
  - [x] Documentation complète (FILTERS.md)
- [ ] Hooks pré/post enregistrement
- [ ] Recording conditionnel avancé (custom functions)

### 5.5 - Replay Features
- [ ] Latency simulation (replay timing réel)
- [ ] Erreur simulation (500, timeout, etc.)
- [ ] Replay séquentiel vs aléatoire
- [ ] Replay avec variations

### 5.6 - Observability
- [ ] Métriques Prometheus
- [ ] Traces OpenTelemetry
- [ ] Dashboard web (statistiques)
- [ ] Export de rapports

### 5.7 - Intégrations
- [ ] Plugin Jest (JavaScript)
- [ ] Plugin pytest (Python)
- [ ] Plugin JUnit (Java)
- [ ] Plugin XCTest (Swift)
- [ ] Plugin Gradle (Kotlin)
- [ ] Plugin Docker (image officielle)

---

## 🎯 Milestones

### v0.1.0 (MVP) ✅ - ATTEINT
- HTTP/HTTPS proxy fonctionnel
- Record/Replay basique
- API Rust complète

### v0.2.0 (WebSocket) ✅ - ATTEINT
- Support WebSocket complet
- WebSocket record/replay
- Tests et documentation

### v0.3.0 (Multi-language) ✅ - ATTEINT
- ✅ Bindings Python, Kotlin, Swift, Java, JavaScript
- ✅ Distribution packages préparés
- ✅ Documentation complète

### v0.4.0 (CLI) ✅ - ATTEINT
- ✅ CLI complet et testé (renommé en `magneto`)
- ✅ Configuration avancée (magneto.toml)
- ✅ CI/CD configuré et fonctionnel (12/12 jobs success)
- ✅ Renommage complet du projet (MatgtoProxy → MagnetoProxy)
- ✅ Workflows CD prêts pour publication
- ⏳ Publication packages (en attente secrets GitHub)

### v1.0.0 (Production Ready)
- Tous les bindings publiés
- Documentation complète
- Performance optimisée
- Release officielle

### v2.0.0 (Advanced Features)
- Cassette management avancé
- Matching avancé
- Observability
- Intégrations

---

## 📈 Métriques de Succès

### Technique
- ✅ 0 erreurs de compilation Rust
- ✅ Tests Python : 4/4 passent
- ✅ Tests Java : 11/11 passent
- ✅ Tests JavaScript : créés
- ✅ CLI : 8 commandes fonctionnelles (binaire `magneto`)
- ✅ CI/CD : workflows GitHub Actions configurés et verts (12/12 jobs success)
- ✅ Renommage complet : MatgtoProxy → MagnetoProxy (309 occurrences, 35 fichiers)
- ✅ Binaire uniffi-bindgen créé pour génération de bindings
- ✅ Tests Rust : 43 tests passent (8 ignorés volontairement)
- ✅ **Benchmarks Criterion : 39 benchmarks couvrant toutes les opérations**
- ✅ **Performance mesurée : ~49ns overhead, 445µs startup, 835 interactions/sec**
- ⏳ Couverture de code > 80%

### Distribution
- ⏳ Package PyPI (prêt à publier)
- ⏳ Package Maven Central (prêt à publier)
- ⏳ Package NPM (prêt à publier)
- ⏳ Package crates.io (prêt à publier)
- ⏳ Package SPM (en préparation)

### Adoption
- ⏳ 100+ stars GitHub
- ⏳ 10+ contributeurs
- ⏳ 1000+ téléchargements

---

## 🤝 Contribution

Vous pouvez contribuer sur :

### Phase actuelle (4.3 - Production Ready)
1. ✅ Benchmarks de performance (39 benchmarks Criterion)
2. Implémenter optimisations identifiées (async I/O, MessagePack)
3. Audit de sécurité des dépendances
4. Documentation complète (API docs, guides)
5. Configurer secrets GitHub pour publication

### Prochaines phases
1. Optimisations performance (Phase 4.3) - **EN COURS**
2. Release v1.0.0 (Phase 4.4)
3. Features avancées (Phase 5)

---

## 📅 Timeline

| Période | Phase | Statut |
|---------|-------|--------|
| **Semaine 1-2** | Phase 1 - HTTP Proxy | ✅ Terminé |
| **Semaine 3** | Phase 2 - WebSocket | ✅ Terminé |
| **Semaine 4-5** | Phase 3.1-3.2 - Bindings | ✅ Terminé |
| **Semaine 6** | Phase 3.3 - Distribution | ✅ Terminé |
| **Semaine 7** | Phase 4.1 - CLI | ✅ Terminé |
| **Semaine 8** | Phase 4.2 - CI/CD | ✅ Terminé |
| **Semaine 9** | Phase 4.3-4.4 - Production & Release | 🟡 En cours |
| **Semaine 10+** | Phase 5 - Advanced Features | ⏳ À venir |

---

## 🔗 Ressources

- [GitHub Repository](https://github.com/taciclei/magneto-serge)
- [GitHub Actions (CI/CD)](https://github.com/taciclei/magneto-serge/actions)
- [Documentation Bindings](BINDINGS.md)
- [Documentation CI/CD](CI_CD.md)
- [Exemples](examples/)
- [Tests](tests/)

---

## 📄 Licence

MIT OR Apache-2.0

---

**Dernière mise à jour** : 2025-10-11 (après ajout benchmarks Criterion)
**Version actuelle** : v0.1.0 (First Release)
**Prochaine milestone** : v0.2.0 (Optimisations & Advanced Features)
