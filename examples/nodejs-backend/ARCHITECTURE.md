# 🏗️ Architecture de Production Recommandée

## Vue d'ensemble

Cette architecture sépare clairement les responsabilités entre le backend (qui gère Hydra/JSON-LD) et le frontend (qui consomme une API REST simplifiée).

## 📐 Diagramme d'Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                         CLIENT TIER                          │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Angular Application (Browser)                         │ │
│  │                                                         │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐  │ │
│  │  │  Components  │  │   Services   │  │  HttpClient │  │ │
│  │  │   (UI/UX)    │→ │  (Business)  │→ │   (HTTP)    │  │ │
│  │  └──────────────┘  └──────────────┘  └─────────────┘  │ │
│  └────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                            ↓ REST API (JSON simple)
┌──────────────────────────────────────────────────────────────┐
│                        BACKEND TIER                          │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Node.js/Express Backend (Port 3000)                   │ │
│  │                                                         │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐  │ │
│  │  │   Express    │  │   Alcaeus    │  │    Cache    │  │ │
│  │  │   Routes     │→ │  (Hydra)     │→ │  (Memory)   │  │ │
│  │  └──────────────┘  └──────────────┘  └─────────────┘  │ │
│  └────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                            ↓ Hydra/JSON-LD
┌──────────────────────────────────────────────────────────────┐
│                         API TIER                             │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Magneto-Serge API (Port 8889)                         │ │
│  │                                                         │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐  │ │
│  │  │    Axum      │  │    Proxy     │  │  Cassettes  │  │ │
│  │  │  (Rust API)  │→ │   Engine     │→ │  (Storage)  │  │ │
│  │  └──────────────┘  └──────────────┘  └─────────────┘  │ │
│  └────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                            ↓ HTTP/HTTPS/WebSocket
┌──────────────────────────────────────────────────────────────┐
│                      EXTERNAL SERVICES                       │
│                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │  HTTP APIs  │  │  WebSockets │  │  External Servers   │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
```

## 🎯 Séparation des Responsabilités

### Frontend Angular (Browser)

**Responsabilités:**
- Interface utilisateur (UI/UX)
- Logique métier côté client
- Validation des formulaires
- Gestion de l'état local
- Appels HTTP REST simples

**Technologies:**
- Angular 19+
- HttpClient (natif Angular)
- RxJS pour la réactivité
- TypeScript pour le typage

**Avantages:**
✅ Pas de dépendances RDF/Hydra
✅ Build rapide et léger
✅ Types TypeScript simples
✅ Pas de polyfills Node.js

### Backend Node.js (Server)

**Responsabilités:**
- Abstraction Hydra/JSON-LD
- Navigation dans l'API Magneto via Alcaeus
- Cache des ressources Hydra
- Transformation JSON-LD → JSON simple
- Gestion des opérations Hydra

**Technologies:**
- Node.js 18+
- Express 4
- Alcaeus 2.0
- Cache en mémoire (Map)

**Avantages:**
✅ Alcaeus fonctionne nativement
✅ Zéro polyfill nécessaire
✅ Performance optimale
✅ Cache partagé entre clients

### API Magneto-Serge (Rust)

**Responsabilités:**
- Proxy HTTP/HTTPS/WebSocket
- Enregistrement des interactions
- Replay des cassettes
- API Hydra/JSON-LD

**Technologies:**
- Rust (Axum)
- Hudsucker (proxy MITM)
- Tokio (async runtime)

## 📊 Flux de Données

### Exemple: Démarrage du Proxy

```
┌─────────────────────────────────────────────────────────────┐
│ 1. USER ACTION                                              │
└─────────────────────────────────────────────────────────────┘
    User clicks "Start Proxy" button
             ↓
┌─────────────────────────────────────────────────────────────┐
│ 2. ANGULAR COMPONENT                                        │
└─────────────────────────────────────────────────────────────┘
    startProxy() {
      this.magnetoService.startProxy({
        mode: 'auto',
        cassette_name: 'test'
      }).subscribe(...)
    }
             ↓
┌─────────────────────────────────────────────────────────────┐
│ 3. ANGULAR SERVICE (HttpClient)                             │
└─────────────────────────────────────────────────────────────┘
    POST http://localhost:3000/proxy/start
    Content-Type: application/json
    Body: {"mode": "auto", "cassette_name": "test"}
             ↓
┌─────────────────────────────────────────────────────────────┐
│ 4. NODE.JS BACKEND                                          │
└─────────────────────────────────────────────────────────────┘
    app.post('/proxy/start', async (req, res) => {
      // Charger la ressource Hydra
      const resource = await Alcaeus.loadResource(
        'http://localhost:8889/proxy/start'
      );

      // Trouver l'opération POST
      const operation = resource.operations.find(
        op => op.method === 'POST'
      );

      // Exécuter l'opération
      const result = await operation.invoke(req.body);

      // Simplifier et retourner
      res.json(simplify(result));
    });
             ↓
┌─────────────────────────────────────────────────────────────┐
│ 5. ALCAEUS (Hydra Client)                                   │
└─────────────────────────────────────────────────────────────┘
    // Charge la ressource JSON-LD
    GET http://localhost:8889/proxy/start
    Accept: application/ld+json

    // Parse JSON-LD et extrait les opérations Hydra
    // Exécute l'opération POST découverte
    POST http://localhost:8889/proxy/start
    Content-Type: application/json
    Body: {"mode": "auto", "cassette_name": "test"}
             ↓
┌─────────────────────────────────────────────────────────────┐
│ 6. MAGNETO-SERGE API (Rust)                                 │
└─────────────────────────────────────────────────────────────┘
    // Démarre le proxy
    proxy_manager.start(StartProxyRequest {
      mode: ProxyMode::Auto,
      cassette_name: "test",
      ...
    });

    // Retourne une réponse Hydra/JSON-LD
    Response {
      "@context": "...",
      "@type": "ProxyStarted",
      "success": true,
      "data": { "running": true, ... },
      "hydra:link": [ ... ]
    }
             ↓
┌─────────────────────────────────────────────────────────────┐
│ 7. RESPONSE FLOW (remonte la chaîne)                        │
└─────────────────────────────────────────────────────────────┘
    Magneto API → Alcaeus → Node.js Backend → Angular

    JSON-LD complexe    →    JSON simplifié
    {                        {
      "@context": "...",       "success": true,
      "@type": "...",          "message": "Proxy started",
      "hydra:link": [...]      "result": {
    }                            "running": true
                               }
                             }
```

## 🔄 Gestion du Cache

### Stratégie de Cache (Backend)

```javascript
// Cache en mémoire (Map)
const resourceCache = new Map();
const CACHE_TTL = 60000; // 1 minute

async function loadHydraResource(url) {
  // 1. Vérifier le cache
  const cached = resourceCache.get(url);
  if (cached && Date.now() - cached.timestamp < CACHE_TTL) {
    return cached.resource;  // Cache hit ✓
  }

  // 2. Cache miss: charger via Alcaeus
  const response = await Alcaeus.loadResource(url);

  // 3. Mettre en cache
  resourceCache.set(url, {
    resource: response.root,
    timestamp: Date.now()
  });

  return response.root;
}
```

### Bénéfices du Cache

- ✅ Réduit les requêtes à Magneto API
- ✅ Améliore les temps de réponse
- ✅ Partagé entre tous les clients Angular
- ✅ Invalidation automatique (TTL)
- ✅ Endpoint `/cache` pour gestion manuelle

## 🌐 Communication Inter-Tiers

### Format Angular → Backend

**REST API classique (JSON simple)**

```json
POST /proxy/start
{
  "mode": "auto",
  "cassette_name": "test",
  "port": 8888,
  "strict": false
}
```

### Format Backend → Magneto

**Hydra/JSON-LD (via Alcaeus)**

```json
POST http://localhost:8889/proxy/start
Accept: application/ld+json
{
  "mode": "auto",
  "cassette_name": "test",
  "port": 8888,
  "strict": false
}
```

### Format Magneto → Backend

**Réponse Hydra/JSON-LD**

```json
{
  "@context": "http://www.w3.org/ns/hydra/context.jsonld",
  "@type": "ProxyStarted",
  "success": true,
  "data": {
    "running": true,
    "mode": "auto",
    "port": 8888
  },
  "hydra:link": [
    {
      "@type": "hydra:Link",
      "hydra:target": "http://localhost:8889/proxy/status",
      "title": "Check Proxy Status"
    }
  ]
}
```

### Format Backend → Angular

**JSON simplifié**

```json
{
  "success": true,
  "message": "Proxy started",
  "result": {
    "running": true,
    "mode": "auto",
    "port": 8888
  },
  "nextActions": [
    {
      "title": "Check Proxy Status",
      "target": "http://localhost:8889/proxy/status"
    }
  ]
}
```

## 🚀 Déploiement Production

### Architecture Multi-Environnements

```
┌─────────────────────────────────────────────────────────────┐
│                         PRODUCTION                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌─────────────────┐   │
│  │   Angular    │  │   Node.js    │  │   Magneto-Serge │   │
│  │   (Nginx)    │→ │   Backend    │→ │   API (Rust)    │   │
│  │  Port 80/443 │  │   Port 3000  │  │   Port 8889     │   │
│  └──────────────┘  └──────────────┘  └─────────────────┘   │
│       CDN              Docker           Docker/Binary       │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                          STAGING                            │
├─────────────────────────────────────────────────────────────┤
│  Même architecture, URLs différentes                        │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                        DEVELOPMENT                          │
├─────────────────────────────────────────────────────────────┤
│  localhost:4200 → localhost:3000 → localhost:8889           │
└─────────────────────────────────────────────────────────────┘
```

### Configuration par Environnement

**Angular (environment.ts)**

```typescript
export const environment = {
  production: true,
  backendUrl: 'https://backend.example.com'
};
```

**Backend (.env)**

```bash
NODE_ENV=production
PORT=3000
MAGNETO_API_URL=http://magneto-api:8889
CACHE_TTL=300000  # 5 minutes
```

**Magneto (magneto.toml)**

```toml
[api]
host = "0.0.0.0"
port = 8889
auth_enabled = true
```

## 📈 Scalabilité

### Mise à l'échelle Horizontale

```
                    ┌─────────────┐
                    │   Nginx     │
                    │ Load Balancer│
                    └──────┬──────┘
                           │
          ┌────────────────┼────────────────┐
          ↓                ↓                ↓
    ┌──────────┐     ┌──────────┐     ┌──────────┐
    │ Backend  │     │ Backend  │     │ Backend  │
    │ Node.js  │     │ Node.js  │     │ Node.js  │
    │  :3001   │     │  :3002   │     │  :3003   │
    └────┬─────┘     └────┬─────┘     └────┬─────┘
         └────────────────┼────────────────┘
                          ↓
                  ┌───────────────┐
                  │  Magneto API  │
                  │   (Rust)      │
                  └───────────────┘
```

### Cache Distribué (Redis)

Pour production à grande échelle, remplacer le cache en mémoire par Redis :

```javascript
import Redis from 'ioredis';
const redis = new Redis();

async function loadHydraResource(url) {
  // Cache distribué partagé entre tous les backends
  const cached = await redis.get(`hydra:${url}`);
  if (cached) return JSON.parse(cached);

  const resource = await Alcaeus.loadResource(url);
  await redis.setex(`hydra:${url}`, 60, JSON.stringify(resource));

  return resource;
}
```

## 🔐 Sécurité

### CORS (Backend)

```javascript
app.use(cors({
  origin: ['http://localhost:4200', 'https://app.example.com'],
  credentials: true
}));
```

### Authentication

```javascript
// Backend: vérifier token
app.use((req, res, next) => {
  const token = req.headers.authorization;
  if (!verifyToken(token)) {
    return res.status(401).json({ error: 'Unauthorized' });
  }
  next();
});
```

### Rate Limiting

```javascript
import rateLimit from 'express-rate-limit';

app.use(rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100 // max 100 requêtes
}));
```

## 📊 Monitoring

### Métriques Backend

```javascript
let metrics = {
  totalRequests: 0,
  cacheHits: 0,
  cacheMisses: 0,
  avgResponseTime: 0
};

app.get('/metrics', (req, res) => {
  res.json({
    ...metrics,
    cacheHitRate: metrics.cacheHits / metrics.totalRequests,
    uptime: process.uptime()
  });
});
```

### Logs Structurés

```javascript
import winston from 'winston';

const logger = winston.createLogger({
  format: winston.format.json(),
  transports: [
    new winston.transports.File({ filename: 'error.log', level: 'error' }),
    new winston.transports.File({ filename: 'combined.log' })
  ]
});
```

## 🎯 Résumé des Avantages

| Aspect | Angular Direct + Alcaeus | Angular + Backend Node.js |
|--------|-------------------------|---------------------------|
| Complexité frontend | ⚠️ Élevée (types RDF) | ✅ Simple (JSON) |
| Build size | ⚠️ +100kb (polyfills) | ✅ Léger |
| Performance | ⚠️ Parsing client | ✅ Cache serveur |
| Maintenance | ⚠️ Dépendances RDF | ✅ API standard |
| Scalabilité | ❌ Cache par client | ✅ Cache partagé |
| TypeScript | ⚠️ Types complexes | ✅ Types simples |

---

**Cette architecture est la solution recommandée pour la production.**
