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
