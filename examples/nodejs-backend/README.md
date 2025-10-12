# 🚀 Magneto-Serge Hydra Backend (Node.js/Express)

**Backend de production recommandé** pour consommer l'API Hydra/JSON-LD de Magneto-Serge via Alcaeus.

## 🎯 Pourquoi ce backend ?

### Problème avec Alcaeus dans le Browser

Alcaeus est un client Hydra/JSON-LD conçu pour **Node.js**. L'utiliser directement dans Angular pose plusieurs problèmes :

❌ Dépendances Node.js natives (`querystring`, `url`, `util`)
❌ Polyfills nécessaires (+100kb)
❌ Problèmes d'import ESM/TypeScript
❌ Performance sous-optimale
❌ Build complexe

### Solution : Backend Node.js

✅ Alcaeus fonctionne nativement
✅ Zéro polyfill nécessaire
✅ Performance optimale
✅ Cache partagé entre tous les clients
✅ API REST simplifiée pour Angular
✅ Build simple et rapide

## 📦 Installation

```bash
cd examples/nodejs-backend
npm install
```

## 🚀 Démarrage

```bash
# Démarrer Magneto-Serge API
magneto api

# Dans un autre terminal, démarrer le backend
npm start

# Ou avec auto-reload (Node.js 18+)
npm run dev
```

Le serveur démarre sur **http://localhost:3000**

## 🏗️ Architecture

```
┌─────────────────────────────────────┐
│  Angular Client                     │  ← HttpClient (pas Alcaeus)
│  (HttpClient natif)                 │
└─────────────────────────────────────┘
           ↓ HTTP REST
┌─────────────────────────────────────┐
│  Node.js/Express Backend            │  ← Ce serveur
│  (src/server.js)                    │
└─────────────────────────────────────┘
           ↓ Alcaeus
┌─────────────────────────────────────┐
│  Alcaeus (Hydra/JSON-LD)            │  ← Client Hydra officiel
└─────────────────────────────────────┘
           ↓ HTTP + JSON-LD
┌─────────────────────────────────────┐
│  Magneto-Serge API                  │  ← API Hydra/JSON-LD
│  (localhost:8889)                   │
└─────────────────────────────────────┘
```

## 📡 API Endpoints

### Découverte

**GET /**
Découvre l'API et retourne les endpoints disponibles

```bash
curl http://localhost:3000/
```

```json
{
  "success": true,
  "api": {
    "name": "Magneto-Serge Hydra Backend",
    "version": "1.0.0"
  },
  "magnetoRoot": { ... },
  "availableEndpoints": [...]
}
```

### Proxy

**GET /proxy/status**
Obtient le statut du proxy

```bash
curl http://localhost:3000/proxy/status
```

```json
{
  "success": true,
  "status": {
    "running": false,
    "mode": "auto",
    "port": 8888
  },
  "nextActions": [
    {
      "title": "Start Proxy",
      "target": "http://localhost:8889/proxy/start"
    }
  ]
}
```

**POST /proxy/start**
Démarre le proxy

```bash
curl -X POST http://localhost:3000/proxy/start \
  -H "Content-Type: application/json" \
  -d '{
    "mode": "auto",
    "cassette_name": "test",
    "port": 8888,
    "strict": false
  }'
```

**POST /proxy/stop**
Arrête le proxy

```bash
curl -X POST http://localhost:3000/proxy/stop
```

### Cassettes

**GET /cassettes**
Liste toutes les cassettes

```bash
curl http://localhost:3000/cassettes
```

**GET /cassettes/:name**
Obtient une cassette spécifique

```bash
curl http://localhost:3000/cassettes/my-test
```

**DELETE /cassettes/:name**
Supprime une cassette

```bash
curl -X DELETE http://localhost:3000/cassettes/my-test
```

### Santé

**GET /health**
Vérifie la santé de l'API

```bash
curl http://localhost:3000/health
```

### Cache

**GET /cache/stats**
Statistiques du cache de ressources

```bash
curl http://localhost:3000/cache/stats
```

**DELETE /cache**
Vide le cache

```bash
curl -X DELETE http://localhost:3000/cache
```

## 🔧 Configuration

### Variables d'environnement

```bash
# Port du backend (défaut: 3000)
PORT=3000

# URL de l'API Magneto-Serge (défaut: http://localhost:8889)
MAGNETO_API_URL=http://localhost:8889

# Démarrer avec variables
PORT=4000 MAGNETO_API_URL=http://api.example.com npm start
```

### Cache

Le backend cache les ressources Hydra pour améliorer les performances :
- **TTL**: 60 secondes (configurable dans `src/server.js`)
- **Stockage**: En mémoire (Map)
- **Invalidation**: Automatique après TTL ou via `DELETE /cache`

## 💻 Utilisation avec Angular

### Client Angular simplifié

```typescript
// magneto.service.ts
import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class MagnetoService {
  private baseUrl = 'http://localhost:3000';

  constructor(private http: HttpClient) {}

  // Plus besoin d'Alcaeus !
  getProxyStatus(): Observable<any> {
    return this.http.get(`${this.baseUrl}/proxy/status`);
  }

  startProxy(config: any): Observable<any> {
    return this.http.post(`${this.baseUrl}/proxy/start`, config);
  }

  listCassettes(): Observable<any> {
    return this.http.get(`${this.baseUrl}/cassettes`);
  }
}
```

### Avantages

✅ **Pas d'Alcaeus dans Angular** - Plus simple
✅ **HttpClient natif** - API Angular standard
✅ **Pas de polyfills** - Build rapide
✅ **Types TypeScript simples** - Pas de types RDF
✅ **Performance** - Cache côté serveur

## 📊 Fonctionnalités Hydra

Le backend gère automatiquement :

- ✅ **Navigation Hydra** - Suit les liens dynamiquement
- ✅ **Opérations Hydra** - Exécute POST/DELETE/PUT via Alcaeus
- ✅ **JSON-LD** - Parse et simplifie pour les clients
- ✅ **Collections paginées** - Support pagination Hydra
- ✅ **Cache intelligent** - Réduit les requêtes à Magneto

## 🔍 Exemple Complet

### 1. Démarrer tout

```bash
# Terminal 1: API Magneto
magneto api

# Terminal 2: Backend Node.js
cd examples/nodejs-backend
npm start

# Terminal 3: Tester
curl http://localhost:3000/
```

### 2. Workflow complet

```bash
# 1. Vérifier le statut
curl http://localhost:3000/proxy/status

# 2. Démarrer le proxy
curl -X POST http://localhost:3000/proxy/start \
  -H "Content-Type: application/json" \
  -d '{"mode": "record", "cassette_name": "demo", "port": 8888}'

# 3. Configurer le proxy système
export http_proxy=http://localhost:8888
export https_proxy=http://localhost:8888

# 4. Faire une requête via le proxy
curl http://httpbin.org/get

# 5. Arrêter le proxy
curl -X POST http://localhost:3000/proxy/stop

# 6. Lister les cassettes
curl http://localhost:3000/cassettes

# 7. Voir la cassette enregistrée
curl http://localhost:3000/cassettes/demo
```

## 🎯 Comparaison: Direct vs Backend

### Approche 1: Angular + Alcaeus (direct)

```typescript
// ❌ Complexe
import Alcaeus from 'alcaeus';  // Polyfills nécessaires
import { IResource } from 'alcaeus';

const response = await Alcaeus.loadResource('http://...');
const resource: IResource = response.root;
// Parsing JSON-LD complexe...
```

### Approche 2: Angular + Backend Node.js (recommandé)

```typescript
// ✅ Simple
import { HttpClient } from '@angular/common/http';

this.http.get('http://localhost:3000/proxy/status')
  .subscribe(data => {
    console.log(data.status);  // JSON simple !
  });
```

## 🚀 Déploiement

### Docker

```dockerfile
FROM node:20-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --production
COPY src ./src
EXPOSE 3000
CMD ["node", "src/server.js"]
```

```bash
docker build -t magneto-backend .
docker run -p 3000:3000 -e MAGNETO_API_URL=http://host.docker.internal:8889 magneto-backend
```

### PM2 (Production)

```bash
npm install -g pm2
pm2 start src/server.js --name magneto-backend
pm2 save
pm2 startup
```

## 📚 Ressources

- [Alcaeus Documentation](https://alcaeus.hydra.how/)
- [Hydra Core Vocabulary](https://www.hydra-cg.com/spec/latest/core/)
- [Express Documentation](https://expressjs.com/)

## 🐛 Troubleshooting

### Erreur "Cannot find module 'alcaeus'"

```bash
npm install
```

### Port 3000 déjà utilisé

```bash
PORT=4000 npm start
```

### Connexion refusée à Magneto

Vérifiez que l'API Magneto-Serge est démarrée :

```bash
magneto api
```

## 📄 Licence

Même licence que Magneto-Serge (MIT OR Apache-2.0)

---

**Développé avec Node.js + Express + Alcaeus pour une architecture de production optimale**
