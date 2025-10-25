# 🚀 Magneto-Serge Simple Angular Client

**Client Angular de production** qui utilise le backend Node.js/Express (pas d'Alcaeus côté client).

## 🎯 Architecture Production

Ce client démontre l'**architecture recommandée pour la production** :

```
┌─────────────────────────────┐
│  Angular Client (Browser)   │  ← HttpClient natif
│  Pas d'Alcaeus, pas de RDF  │     Types TypeScript simples
└─────────────────────────────┘
          ↓ REST API (JSON simple)
┌─────────────────────────────┐
│  Node.js Backend            │  ← Alcaeus natif
│  (Port 3000)                │     Cache partagé
└─────────────────────────────┘
          ↓ Hydra/JSON-LD
┌─────────────────────────────┐
│  Magneto-Serge API          │  ← API Rust
│  (Port 8889)                │
└─────────────────────────────┘
```

## ✨ Avantages vs Client Alcaeus Direct

| Aspect | Ce Client (Production) | angular-client (Démo) |
|--------|------------------------|----------------------|
| **Dépendances** | ✅ Angular standard | ⚠️ Angular + Alcaeus + RDF |
| **Build size** | ✅ ~50kb | ⚠️ ~150kb (polyfills) |
| **Types** | ✅ JSON simple | ⚠️ Types RDF complexes |
| **Performance** | ✅ Cache serveur | ⚠️ Parsing client |
| **Maintenance** | ✅ API standard | ⚠️ Dépendances RDF |
| **Production** | ✅ Recommandé | ⚠️ Démonstration |

## 📦 Installation

```bash
cd examples/angular-simple-client
npm install
```

### Dépendances

```json
{
  "@angular/core": "^19.0.0",
  "rxjs": "~7.8.0"
}
```

**Pas de dépendances Alcaeus, RDF ou polyfills Node.js !**

## 🚀 Démarrage

### 1. Démarrer l'API Magneto-Serge

```bash
# Dans le répertoire racine
magneto api
```

### 2. Démarrer le Backend Node.js

```bash
cd examples/nodejs-backend
npm install
npm start
# → Backend running on http://localhost:3000
```

### 3. Démarrer le Client Angular

```bash
cd examples/angular-simple-client
npm install
npm start
# → Angular running on http://localhost:4201
```

### 4. Ouvrir dans le Browser

Naviguer vers **http://localhost:4201**

## 🎨 Interface Utilisateur

### Fonctionnalités

✅ **Proxy Status Dashboard**
- État en temps réel (running/stopped)
- Mode actif (auto/record/replay/passthrough)
- Port et cassette actuelle
- Nombre d'interactions
- Uptime

✅ **Proxy Control Panel**
- Démarrer le proxy avec configuration
- Choisir le mode (auto, record, replay, passthrough)
- Spécifier le nom de cassette
- Port personnalisable
- Mode strict optionnel
- Arrêter le proxy

✅ **Cassettes Management**
- Liste toutes les cassettes enregistrées
- Affiche la taille, nombre d'interactions, date
- Supprimer des cassettes
- Rafraîchissement manuel

✅ **Backend Cache Info**
- Affiche le nombre de ressources en cache
- Bouton pour vider le cache backend

✅ **Error Handling**
- Notifications d'erreur élégantes
- Loading overlay pendant les opérations
- Messages de confirmation pour actions destructrices

## 🏗️ Structure du Code

```
angular-simple-client/
├── src/
│   ├── app/
│   │   ├── models/
│   │   │   └── magneto.models.ts     # Types TypeScript simples
│   │   │       - ApiResponse<T>
│   │   │       - ProxyStatus
│   │   │       - CassetteInfo
│   │   │       - StartProxyRequest
│   │   │
│   │   ├── services/
│   │   │   └── magneto.service.ts    # Service Angular
│   │   │       - getProxyStatus()
│   │   │       - startProxy()
│   │   │       - stopProxy()
│   │   │       - listCassettes()
│   │   │       - deleteCassette()
│   │   │
│   │   └── app.component.ts          # UI Component
│   │       - Dashboard complet
│   │       - Control panel
│   │       - Cassettes grid
│   │
│   ├── main.ts                       # Bootstrap
│   ├── index.html
│   └── styles.css
│
├── package.json
├── tsconfig.json
├── angular.json
└── README.md
```

## 💡 Utilisation du Service

### Exemple Simple

```typescript
import { Component } from '@angular/core';
import { MagnetoService } from './services/magneto.service';

@Component({
  selector: 'app-my-component',
  template: `
    <button (click)="startProxy()">Start Proxy</button>
    <div *ngIf="status">
      Status: {{ status.running ? 'Running' : 'Stopped' }}
    </div>
  `
})
export class MyComponent {
  status: any;

  constructor(private magnetoService: MagnetoService) {}

  startProxy() {
    this.magnetoService.startProxy({
      mode: 'auto',
      cassette_name: 'my-test',
      port: 8888
    }).subscribe(response => {
      console.log('Proxy started:', response);
      this.loadStatus();
    });
  }

  loadStatus() {
    this.magnetoService.getProxyStatus().subscribe(response => {
      this.status = response.status;
    });
  }
}
```

### API Disponibles

```typescript
// Découverte de l'API
this.magnetoService.discoverApi()

// Statut du proxy
this.magnetoService.getProxyStatus()

// Démarrer le proxy
this.magnetoService.startProxy({
  mode: 'auto',
  cassette_name: 'test',
  port: 8888,
  strict: false
})

// Arrêter le proxy
this.magnetoService.stopProxy()

// Lister les cassettes
this.magnetoService.listCassettes()

// Obtenir une cassette
this.magnetoService.getCassette('my-test')

// Supprimer une cassette
this.magnetoService.deleteCassette('my-test')

// Santé de l'API
this.magnetoService.checkHealth()

// Stats du cache backend
this.magnetoService.getCacheStats()

// Vider le cache backend
this.magnetoService.clearCache()
```

## 🔧 Configuration

### Changer l'URL du Backend

```typescript
// Dans un service ou component
constructor(private magnetoService: MagnetoService) {
  this.magnetoService.setBackendUrl('http://backend.example.com:3000');
}
```

### Variables d'Environnement (Angular)

```typescript
// src/environments/environment.ts
export const environment = {
  production: false,
  backendUrl: 'http://localhost:3000'
};

// src/environments/environment.prod.ts
export const environment = {
  production: true,
  backendUrl: 'https://api.example.com'
};

// Utilisation dans le service
import { environment } from '../../environments/environment';

@Injectable()
export class MagnetoService {
  private backendUrl = environment.backendUrl;
  ...
}
```

## 🎯 Workflow Complet

### Enregistrer une Session de Test

1. Ouvrir http://localhost:4201
2. Dans "Proxy Control":
   - Cassette Name: `my-api-test`
   - Mode: `Record`
   - Port: `8888`
   - Cliquer "Start Proxy"

3. Configurer votre application à tester :
```bash
export http_proxy=http://localhost:8888
export https_proxy=http://localhost:8888
```

4. Faire vos requêtes HTTP/HTTPS
```bash
curl http://httpbin.org/get
curl https://api.github.com/users/octocat
```

5. Arrêter le proxy: Cliquer "Stop Proxy"

6. Voir la cassette dans la section "Cassettes"

### Rejouer la Session

1. Changer le mode à `Replay`
2. Même nom de cassette: `my-api-test`
3. Cliquer "Start Proxy"
4. Relancer les mêmes requêtes → Réponses instantanées depuis la cassette !

## 🚀 Build Production

```bash
npm run build
```

Les fichiers sont générés dans `dist/magneto-simple-client/`

### Servir avec Nginx

```nginx
server {
    listen 80;
    server_name app.example.com;

    root /var/www/magneto-client;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    # Proxy vers le backend Node.js
    location /api/ {
        proxy_pass http://localhost:3000/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 📊 Comparaison: Simple vs Alcaeus

### Ce Client (Simple)

**Code TypeScript**:
```typescript
// Types simples
interface ProxyStatus {
  running: boolean;
  mode: string;
  port: number;
}

// HttpClient natif Angular
this.http.get<ProxyStatus>('/proxy/status')
  .subscribe(status => {
    console.log(status.running); // ✅ Simple !
  });
```

**Dependencies**:
- Angular core
- RxJS
- **Pas de dépendances RDF/Hydra**

**Build size**: ~50kb (gzip)

### Client avec Alcaeus

**Code TypeScript**:
```typescript
// Types RDF complexes
import { IResource } from 'alcaeus';
import { @rdfjs/types } from '@rdfjs/types';

// Parser JSON-LD
const response = await Alcaeus.loadResource('/proxy/status');
const resource: IResource = response.root;
const status = extractProperty(resource, 'data'); // ⚠️ Complexe
```

**Dependencies**:
- Angular core
- RxJS
- Alcaeus
- @zazuko/env
- @rdfjs/types
- Polyfills Node.js

**Build size**: ~150kb (gzip)

## 🐛 Troubleshooting

### Erreur "Cannot connect to backend"

Vérifiez que le backend Node.js est démarré :

```bash
cd examples/nodejs-backend
npm start
```

### Erreur CORS

Le backend Node.js inclut déjà CORS. Si problème persiste, vérifiez la config :

```javascript
// nodejs-backend/src/server.js
app.use(cors({
  origin: ['http://localhost:4201'], // Ajouter votre origine
  credentials: true
}));
```

### Port 4201 déjà utilisé

```bash
# Changer le port dans package.json
"start": "ng serve --port 4202"
```

## 📚 Ressources

- [Backend Node.js README](../nodejs-backend/README.md)
- [Backend Architecture](../nodejs-backend/ARCHITECTURE.md)
- [API Magneto Documentation](../../docs/API.md)
- [Angular Documentation](https://angular.dev/)

## 💡 Pourquoi cette Architecture ?

### Problème: Alcaeus dans le Browser

Alcaeus est conçu pour **Node.js**, pas le browser:
- Dépend de modules Node.js (`querystring`, `url`, `util`)
- Nécessite des polyfills (+100kb)
- Parsing JSON-LD côté client (lent)
- Types RDF complexes

### Solution: Backend Node.js

- ✅ Alcaeus fonctionne nativement
- ✅ Zéro polyfill nécessaire
- ✅ Cache serveur partagé
- ✅ API REST simple pour Angular
- ✅ Types TypeScript simples
- ✅ Performance optimale

Cette architecture sépare les responsabilités:
- **Backend**: Gère la complexité Hydra/JSON-LD
- **Frontend**: Interface utilisateur simple

## 📄 Licence

Même licence que Magneto-Serge (MIT OR Apache-2.0)

---

**Développé avec Angular 19 pour démontrer l'architecture de production recommandée**
