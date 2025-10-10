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
| **Phase 4** | CLI & Production | 🟡 En cours | 75% |
| **Phase 5** | Advanced Features | ⏳ À venir | 0% |

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

## Phase 3 : Multi-language Bindings 🟡

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

### 3.3 - Distribution 🟡
- [ ] Package PyPI (Python)
  - [x] Configuration prête
  - [ ] Publication
- [x] Package Maven Central (Java/Kotlin)
  - [x] pom.xml créé
  - [x] Guide de publication (PUBLISHING.md)
  - [ ] Publication
- [x] Package NPM (JavaScript)
  - [x] package.json configuré
  - [x] Guide de publication (PUBLISHING.md)
  - [ ] Publication
- [ ] Package Swift Package Manager
  - [ ] Package.swift
  - [ ] Publication
- [x] Package Cargo (crates.io)
  - [x] Cargo.toml configuré
  - [x] Licenses créées
  - [ ] Publication
- [ ] CI/CD pour publication automatique

### 3.4 - Documentation ✅
- [x] README par langage
- [x] BINDINGS.md (synthèse)
- [x] Exemples d'utilisation
- [ ] Documentation API en ligne

---

## Phase 4 : CLI & Production 🟡

**Objectif** : Créer un CLI et préparer la production.

### 4.1 - CLI (Command Line Interface) ✅
- [x] Outil `matgto` avec clap
- [x] Commandes :
  - [x] `matgto record <name>` : Démarre l'enregistrement
  - [x] `matgto replay <name>` : Rejoue une cassette
  - [x] `matgto auto <name>` : Mode automatique (record si absent, sinon replay)
  - [x] `matgto list` : Liste les cassettes
  - [x] `matgto inspect <name>` : Affiche le contenu
  - [x] `matgto delete <name>` : Supprime une cassette
  - [x] `matgto init` : Initialise configuration matgto.toml
  - [x] `matgto version` : Affiche la version
- [x] Configuration via fichier (matgto.toml)
- [x] Variables d'environnement (via clap)
- [x] Logging avec tracing
- [x] CLI testé et fonctionnel
- [x] Erreurs de compilation corrigées

### 4.2 - CI/CD ✅
- [x] GitHub Actions
  - [x] Tests Rust multi-plateformes (Ubuntu, macOS, Windows)
  - [x] Tests Rust multi-versions (stable, beta)
  - [x] Lint (rustfmt + clippy)
  - [x] Build CLI pour 3 plateformes
  - [x] Génération bindings (Python, Kotlin, Swift)
  - [x] Security audit (cargo-audit)
  - [x] Code coverage (tarpaulin)
- [x] Release automatique (CD)
  - [x] Publication crates.io
  - [x] Publication NPM
  - [x] Publication PyPI
  - [x] Publication Maven Central
  - [x] Création releases GitHub
  - [x] Build binaires multi-plateformes (5 architectures)
  - [x] Docker multi-arch (linux/amd64, linux/arm64)
- [x] Documentation CI/CD (CI_CD.md)
- [ ] Configuration secrets GitHub (pour publication)

### 4.3 - Production Ready
- [ ] Benchmarks de performance
- [ ] Optimisations
- [ ] Sécurité : audit des dépendances
- [ ] Documentation complète
- [ ] Site web / GitHub Pages

### 4.4 - Release 1.0
- [ ] Release notes
- [ ] Migration guide
- [ ] Blog post / annonce
- [ ] Soumission à awesome-rust

---

## Phase 5 : Advanced Features ⏳

**Objectif** : Fonctionnalités avancées et améliorations.

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
- [ ] Mode HYBRID (mix record/replay)
- [ ] Mode UPDATE (met à jour cassettes existantes)
- [ ] Mode ONCE (record uniquement si absent)
- [ ] Mode STRICT (erreur si pas de match)

### 5.4 - Recording Features
- [ ] Filtres d'enregistrement (ignorer certaines URLs)
- [ ] Hooks pré/post enregistrement
- [ ] Transformation des réponses (masquage de secrets)
- [ ] Recording conditionnel (selon headers, status, etc.)

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

### v0.4.0 (CLI) 🟡 - EN COURS
- ✅ CLI complet et testé
- ✅ Configuration avancée
- ✅ CI/CD configuré
- ⏳ Publication packages

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
- ✅ CLI : 8 commandes fonctionnelles
- ✅ CI/CD : workflows GitHub Actions configurés
- ⏳ Couverture de code > 80%
- ⏳ Performance : < 10ms overhead par requête

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

### Phase actuelle (4.2 - CI/CD & Publication)
1. Configurer les secrets GitHub pour la publication
2. Tester la publication sur les registres de packages
3. Créer la première release (v0.4.0)
4. Nettoyer les warnings Rust

### Prochaines phases
1. Optimiser la performance (Phase 4.3)
2. Benchmarks et métriques (Phase 4.3)
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
| **Semaine 8** | Phase 4.2 - CI/CD | 🟡 En cours |
| **Semaine 9** | Phase 4.3 - Production Ready | ⏳ À venir |
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

**Dernière mise à jour** : 2025-10-10
**Version actuelle** : v0.4.0-dev
**Prochaine milestone** : v0.4.0 (CLI & CI/CD)
