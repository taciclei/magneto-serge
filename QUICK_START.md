# 🚀 Guide de Démarrage Rapide - Magneto-Serge

Guide complet pour démarrer rapidement avec Magneto-Serge et son écosystème.

## 📋 Table des Matières

1. [Installation](#installation)
2. [Démarrage Rapide (CLI)](#démarrage-rapide-cli)
3. [Architecture Complète (API + Frontend)](#architecture-complète)
4. [Cas d'Usage](#cas-dusage)
5. [Troubleshooting](#troubleshooting)

---

## 🔧 Installation

### Prérequis

- **Rust 1.75+** - [Installer Rust](https://rustup.rs/)
- **Node.js 18+** (optionnel, pour l'API et les clients)
- **Angular CLI 19+** (optionnel, pour les clients frontend)

### Installation de Magneto-Serge

```bash
# Cloner le repository
git clone https://github.com/votre-username/magneto-serge.git
cd magneto-serge

# Compiler (binaire + CLI)
cargo build --release

# Installer le CLI (optionnel)
cargo install --path .

# Vérifier l'installation
magneto --version
```

---

## ⚡ Démarrage Rapide (CLI)

### Scénario 1: Enregistrer des Requêtes HTTP

```bash
# 1. Démarrer le proxy en mode record
magneto record my-test --port 8888

# 2. Configurer votre client HTTP (nouveau terminal)
export http_proxy=http://localhost:8888
export https_proxy=http://localhost:8888

# 3. Faire des requêtes
curl http://httpbin.org/get
curl https://api.github.com/users/octocat

# 4. Arrêter le proxy (Ctrl+C)
# Cassette sauvegardée dans: ~/.magneto/cassettes/my-test.json
```

### Scénario 2: Rejouer des Requêtes

```bash
# 1. Démarrer en mode replay
magneto replay my-test --port 8888

# 2. Relancer les mêmes requêtes
curl http://httpbin.org/get
# → Réponse instantanée depuis la cassette !
```

### Scénario 3: Mode Auto (Intelligent)

```bash
# Mode auto: enregistre si cassette manquante, sinon replay
magneto auto my-test --port 8888

# Première fois: enregistre
curl http://httpbin.org/get

# Fois suivantes: replay depuis la cassette
curl http://httpbin.org/get  # ⚡ Instantané !
```

### Commandes CLI Utiles

```bash
# Lister les cassettes
magneto list

# Voir les détails d'une cassette
magneto inspect my-test

# Supprimer une cassette
magneto delete my-test

# Initialiser la configuration
magneto init

# Afficher la version
magneto version

# Aide
magneto --help
```

---

## 🌐 Architecture Complète

Pour des cas d'usage avancés, utilisez l'**API REST** avec l'**interface web**.

### Architecture Recommandée

```
┌─────────────────────────────┐
│  Angular Client (Browser)   │  ← Interface web moderne
│  Port 4201                  │
└─────────────────────────────┘
          ↓ REST API
┌─────────────────────────────┐
│  Node.js Backend            │  ← Wrapper Alcaeus
│  Port 3000                  │     (Cache + Hydra)
└─────────────────────────────┘
          ↓ Hydra/JSON-LD
┌─────────────────────────────┐
│  Magneto-Serge API (Rust)   │  ← API REST officielle
│  Port 8889                  │     (Control du proxy)
└─────────────────────────────┘
          ↓ Proxy
┌─────────────────────────────┐
│  HTTP/HTTPS/WebSocket Proxy │  ← Proxy MITM
│  Port 8888                  │     (Enregistrement)
└─────────────────────────────┘
```

### Démarrage de la Stack Complète

#### Étape 1: API Magneto-Serge

```bash
# Terminal 1
magneto api

# Ou avec options
magneto api --host 0.0.0.0 --port 8889 --proxy-port 8888
```

**Résultat**:
```
╔════════════════════════════════════════════╗
║  🌐 Magneto-Serge API Server               ║
╚════════════════════════════════════════════╝

✓ Server running on http://127.0.0.1:8889
✓ OpenAPI docs: http://127.0.0.1:8889/openapi.json
✓ Health check: http://127.0.0.1:8889/health
✓ Proxy control: http://127.0.0.1:8889/proxy/*
```

#### Étape 2: Backend Node.js (Optionnel mais Recommandé)

```bash
# Terminal 2
cd examples/nodejs-backend
npm install
npm start
```

**Résultat**:
```
╔════════════════════════════════════════════════════════════╗
║  🌐 Magneto-Serge Hydra Backend (Node.js/Express)         ║
╚════════════════════════════════════════════════════════════╝

✓ Server running on http://localhost:3000
✓ Magneto API: http://localhost:8889
✓ Using Alcaeus for Hydra/JSON-LD navigation
✓ Resource caching enabled (TTL: 60000ms)
```

#### Étape 3: Client Angular

```bash
# Terminal 3
cd examples/angular-simple-client
npm install
npm start
```

**Résultat**:
```
✔ Browser application bundle generation complete.
Local:   http://localhost:4201/
```

#### Étape 4: Ouvrir le Browser

Naviguer vers **http://localhost:4201**

---

## 🎯 Cas d'Usage

### Cas 1: Tests d'Intégration Rapides

**Problème**: Tests d'intégration lents car ils appellent des APIs externes.

**Solution**: Mode auto avec Magneto

```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_github_api() {
    // Démarrer proxy en mode auto
    let proxy = MatgtoProxy::builder()
        .with_mode(ProxyMode::Auto)
        .with_cassette("github-user")
        .with_port(8888)
        .build()
        .await
        .unwrap();

    proxy.start().await.unwrap();

    // Première fois: vraie requête (enregistrée)
    // Fois suivantes: replay instantané !
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::http("http://localhost:8888").unwrap())
        .build()
        .unwrap();

    let response = client
        .get("https://api.github.com/users/octocat")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    // ⚡ Exécution instantanée après le premier run !
}
```

### Cas 2: Développement Frontend sans Backend

**Problème**: Backend pas prêt, frontend bloqué.

**Solution**: Enregistrer des réponses mockées

```bash
# 1. Enregistrer des mocks
magneto record frontend-dev --port 8888

# 2. Faire des requêtes manuelles pour créer les mocks
curl http://localhost:8888/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe"}'

# 3. Mode replay pour le frontend
magneto replay frontend-dev --port 8888

# 4. Le frontend peut maintenant travailler avec des réponses consistantes
```

### Cas 3: Tests de Régression API

**Problème**: Détecter les changements breaking dans une API tierce.

**Solution**: Strict mode + replay

```bash
# Enregistrer l'état actuel de l'API
magneto record api-v1 --port 8888
# ... faire des requêtes ...

# Plus tard, vérifier si l'API a changé
magneto replay api-v1 --port 8888 --strict

# En mode strict, toute différence génère une erreur
```

### Cas 4: Performance Testing

**Problème**: Mesurer la performance de l'app sans la latence réseau.

**Solution**: Mode replay

```bash
# 1. Enregistrer une session typique
magneto record perf-test --port 8888

# 2. Exécuter les tests de performance en mode replay
magneto replay perf-test --port 8888

# La latence réseau est éliminée, les tests mesurent
# uniquement la performance de votre application
```

### Cas 5: Démo Offline

**Problème**: Faire une démo sans connexion internet.

**Solution**: Enregistrer à l'avance

```bash
# Avant la démo (avec connexion)
magneto record demo --port 8888
# Faire toutes les requêtes de la démo

# Pendant la démo (sans connexion)
magneto replay demo --port 8888
# Tout fonctionne offline !
```

---

## 🔐 Installation du Certificat (HTTPS)

Pour intercepter HTTPS, installez le certificat CA :

### macOS

```bash
# Le certificat est généré dans ~/.magneto/certs/ca.pem
sudo security add-trusted-cert -d -r trustRoot \
  -k /Library/Keychains/System.keychain \
  ~/.magneto/certs/ca.pem
```

### Linux (Ubuntu/Debian)

```bash
sudo cp ~/.magneto/certs/ca.pem /usr/local/share/ca-certificates/magneto-ca.crt
sudo update-ca-certificates
```

### Windows (PowerShell Admin)

```powershell
Import-Certificate -FilePath $env:USERPROFILE\.magneto\certs\ca.pem `
  -CertStoreLocation Cert:\LocalMachine\Root
```

---

## 🐛 Troubleshooting

### Problème: "Port already in use"

```bash
# Trouver le processus
lsof -i :8888

# Ou changer le port
magneto record my-test --port 9999
```

### Problème: "Certificate verification failed"

```bash
# Réinstaller le certificat
magneto init --reset-cert
```

### Problème: "Cannot connect to API"

```bash
# Vérifier que l'API est démarrée
magneto api

# Vérifier le statut
curl http://localhost:8889/health
```

### Problème: "No matching interaction found"

**Cause**: La requête ne correspond pas exactement à celle enregistrée.

**Solution**:

```bash
# 1. Vérifier la cassette
magneto inspect my-test

# 2. Ré-enregistrer
magneto record my-test --port 8888 --force

# 3. Ou utiliser le mode auto
magneto auto my-test --port 8888
```

### Problème: "Backend connection refused" (Angular)

```bash
# Vérifier que le backend Node.js est démarré
cd examples/nodejs-backend
npm start

# Vérifier l'URL dans le service Angular
# src/app/services/magneto.service.ts
private backendUrl = 'http://localhost:3000';
```

---

## 📚 Prochaines Étapes

### Documentation Complète

- [README.md](README.md) - Documentation principale
- [docs/API.md](docs/API.md) - Documentation de l'API REST
- [examples/nodejs-backend/ARCHITECTURE.md](examples/nodejs-backend/ARCHITECTURE.md) - Architecture détaillée

### Exemples

- [examples/README.md](examples/README.md) - Liste de tous les exemples
- [examples/api_client.py](examples/api_client.py) - Client Python
- [examples/api_client.js](examples/api_client.js) - Client JavaScript
- [examples/api_client.sh](examples/api_client.sh) - Client Bash

### Clients Frontend

- **Angular Simplifié** (Production): `examples/angular-simple-client/`
- **Angular avec Alcaeus** (Démo): `examples/angular-client/`
- **Backend Node.js**: `examples/nodejs-backend/`

---

## 🎉 Vous êtes prêt !

Maintenant vous pouvez :

✅ Enregistrer et rejouer des requêtes HTTP/HTTPS
✅ Accélérer vos tests d'intégration
✅ Développer offline
✅ Contrôler le proxy via API REST
✅ Utiliser une interface web moderne

**Questions ?** Ouvrez une issue sur GitHub !

---

**Développé avec ❤️ en Rust pour des performances maximales**
