# 📚 Exemples matgto-serge

Ce dossier contient des exemples d'utilisation de matgto-serge.

## 🎯 Exemples Disponibles

### 1. `simple_record.rs` - Démarrage Basique
Démontre comment démarrer le proxy en mode record.

```bash
cargo run --example simple_record
```

**Ce que fait cet exemple :**
- Initialise une autorité de certificat (CA)
- Crée un proxy sur le port 8888
- Démarre l'enregistrement d'une cassette
- Affiche les instructions pour configurer votre client HTTP

### 2. `http_record_replay.rs` - Cycle Complet
Exemple complet montrant le cycle record → save → replay.

```bash
cargo run --example http_record_replay
```

**Ce que fait cet exemple :**
- ✅ Enregistre une requête HTTP simulée
- ✅ Sauvegarde dans une cassette JSON
- ✅ Charge la cassette
- ✅ Rejoue la requête depuis la cassette
- ✅ Affiche les statistiques

**Sortie attendue :**
```
🎬 matgto-serge - Exemple Record/Replay HTTP

📹 Phase 1: Enregistrement
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ✓ Requête : GET https://api.github.com/users/octocat
  ✓ Réponse : HTTP 200
  ✓ Interaction enregistrée
  ✓ Cassette sauvegardée : ./examples/cassettes/api-example.json

📼 Phase 2: Contenu de la cassette
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
{
  "version": "1.0",
  "name": "api-example",
  "recorded_at": "2025-10-10T...",
  "interactions": [...]
}

▶️  Phase 3: Replay depuis la cassette
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ✓ Cassette chargée : api-example.json
  ✓ Recherche interaction pour : GET https://api.github.com/users/octocat
  ✓ Interaction trouvée !
  ...

📊 Phase 4: Statistiques
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Nom : api-example
  Version : 1.0
  Interactions : 1
  Replays effectués : 1

✅ Exemple terminé avec succès !
```

## 🔧 Utilisation avec un Client HTTP Réel

### Configuration du Proxy

Une fois le proxy démarré, configurez votre client HTTP :

#### cURL
```bash
curl -x http://localhost:8888 https://api.github.com/users/octocat
```

#### Python (requests)
```python
import requests

proxies = {
    'http': 'http://localhost:8888',
    'https': 'http://localhost:8888',
}

response = requests.get(
    'https://api.github.com/users/octocat',
    proxies=proxies
)
```

#### JavaScript (axios)
```javascript
const axios = require('axios');

const response = await axios.get('https://api.github.com/users/octocat', {
  proxy: {
    host: 'localhost',
    port: 8888
  }
});
```

#### Java (OkHttp)
```java
import okhttp3.*;

Proxy proxy = new Proxy(
    Proxy.Type.HTTP,
    new InetSocketAddress("localhost", 8888)
);

OkHttpClient client = new OkHttpClient.Builder()
    .proxy(proxy)
    .build();

Request request = new Request.Builder()
    .url("https://api.github.com/users/octocat")
    .build();

Response response = client.newCall(request).execute();
```

## 📝 Cassettes Générées

Les cassettes sont sauvegardées au format JSON dans `./examples/cassettes/`.

### Exemple de Cassette

```json
{
  "version": "1.0",
  "name": "api-example",
  "recorded_at": "2025-10-10T12:00:00Z",
  "interactions": [
    {
      "type": "Http",
      "request": {
        "method": "GET",
        "url": "https://api.github.com/users/octocat",
        "headers": {
          "Accept": "application/json",
          "User-Agent": "matgto-serge/0.1.0"
        },
        "body": null
      },
      "response": {
        "status": 200,
        "headers": {
          "Content-Type": "application/json",
          "X-GitHub-Media-Type": "github.v3"
        },
        "body": "{\"login\":\"octocat\",...}"
      }
    }
  ]
}
```

## 🔐 Installation du Certificat (HTTPS)

Pour intercepter HTTPS, vous devez installer le certificat CA de matgto-serge.

### macOS
```bash
# Le certificat est généré dans .matgto/certs/ca.pem
sudo security add-trusted-cert -d -r trustRoot \
  -k /Library/Keychains/System.keychain \
  .matgto/certs/ca.pem
```

### Linux (Ubuntu/Debian)
```bash
sudo cp .matgto/certs/ca.pem /usr/local/share/ca-certificates/matgto-ca.crt
sudo update-ca-certificates
```

### Windows (PowerShell Admin)
```powershell
Import-Certificate -FilePath .matgto\certs\ca.pem `
  -CertStoreLocation Cert:\LocalMachine\Root
```

## 🧪 Tests

Pour tester les exemples sans les exécuter :

```bash
# Vérifier la compilation
cargo check --examples

# Compiler tous les exemples
cargo build --examples

# Exécuter les tests
cargo test
```

## 🌐 Exemples API REST

Magneto-Serge fournit une API REST complète avec support Hydra/JSON-LD et OpenAPI 3.0 pour contrôler le proxy à distance.

### Démarrage du Serveur API

```bash
# Démarrer le serveur API
magneto api

# Avec authentification
magneto api --auth --api-key votre-clé-secrète

# Avec host/port personnalisés
magneto api --host 0.0.0.0 --port 8889
```

### Exemples de Clients API

Trois exemples complets sont fournis :

#### Python (`api_client.py`)
```bash
./api_client.py
```

Démontre :
- Démarrage/arrêt du proxy via l'API
- Vérification du statut
- Liste des cassettes
- Navigation hypermedia avec liens Hydra

#### JavaScript/Node.js (`api_client.js`)
```bash
./api_client.js
```

Utilise `fetch` natif de Node.js 18+ pour interagir avec l'API.

#### Bash/curl (`api_client.sh`)
```bash
./api_client.sh
```

Exemple simple utilisant `curl` et `jq` pour tester l'API.

### Endpoints Principaux

- `GET /` - Racine de l'API avec liens Hydra
- `GET /openapi.json` - Spécification OpenAPI 3.0
- `GET /health` - Vérification de santé
- `POST /proxy/start` - Démarrer le proxy
- `POST /proxy/stop` - Arrêter le proxy
- `GET /proxy/status` - Statut du proxy
- `GET /cassettes` - Liste des cassettes
- `DELETE /cassettes/{name}` - Supprimer une cassette

### Exemple d'Utilisation avec curl

```bash
# Démarrer le proxy en mode auto
curl -X POST http://localhost:8889/proxy/start \
  -H "Content-Type: application/json" \
  -d '{
    "mode": "auto",
    "cassette_name": "my-test",
    "port": 8888
  }'

# Vérifier le statut
curl http://localhost:8889/proxy/status

# Arrêter le proxy
curl -X POST http://localhost:8889/proxy/stop
```

### Hypermedia (HATEOAS)

L'API suit les principes HATEOAS avec Hydra/JSON-LD. Chaque réponse inclut des liens vers les ressources liées :

```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": { ... },
  "hydra:link": [
    {
      "@type": "hydra:Link",
      "hydra:target": "http://localhost:8889/proxy/status",
      "title": "Check Proxy Status"
    }
  ]
}
```

### Documentation Complète

Voir [docs/API.md](../docs/API.md) pour la documentation complète de l'API.

## 🚀 Prochains Exemples (À venir)

- [ ] `websocket_record.rs` - Enregistrement WebSocket
- [ ] `auto_mode.rs` - Démonstration du mode Auto
- [ ] `custom_matching.rs` - Matching personnalisé
- [ ] `multi_cassettes.rs` - Gestion de plusieurs cassettes
- [ ] `java_integration/` - Exemple d'utilisation depuis Java
- [ ] `javascript_integration/` - Exemple Node.js
- [ ] `python_integration/` - Exemple Python avec pytest

## 📚 Documentation

Pour plus d'informations :
- [README.md](../README.md) - Documentation principale
- [ROADMAP.md](../docs/ROADMAP.md) - Feuille de route
- [ARCHITECTURE.md](../docs/ARCHITECTURE.md) - Architecture technique

## 💬 Support

Questions ? Ouvrez une issue sur GitHub !
