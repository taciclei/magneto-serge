# 🌐 Magneto-Serge Angular Client with Alcaeus

Client Angular professionnel pour l'API Hydra/JSON-LD de Magneto-Serge, utilisant **Alcaeus** pour exploiter pleinement les capacités hypermedia.

## ✨ Fonctionnalités

### 🎯 Navigation Hydra Complète
- **Découverte automatique** de l'API via JSON-LD
- **Navigation par liens** Hydra sans URLs codées en dur
- **Exécution d'opérations** découvertes dynamiquement
- **Breadcrumb de navigation** avec historique

### 🔄 Intégration Alcaeus
- Parsing JSON-LD complet avec types RDF
- Support des collections Hydra paginées
- Cache de ressources intelligent
- Événements de navigation en temps réel

### 📊 Interface Utilisateur
- Dashboard de statut du proxy en temps réel
- Gestion des cassettes avec actions Hydra
- Explorateur de ressources JSON-LD
- Logs des événements de navigation

## 📦 Installation

```bash
cd examples/angular-client
npm install
```

### Dépendances Principales

```json
{
  "alcaeus": "^2.0.0",           // Client Hydra/JSON-LD
  "@zazuko/env": "^2.0.0",       // Environnement RDF
  "@rdfjs/types": "^1.1.0",      // Types TypeScript pour RDF
  "@angular/core": "^19.0.0"     // Angular 19+
}
```

## 🚀 Démarrage Rapide

### 1. Démarrer l'API Magneto-Serge

```bash
# Dans le répertoire racine du projet
magneto api
```

### 2. Démarrer le client Angular

```bash
cd examples/angular-client
npm start
```

Ouvrir http://localhost:4200

## 🏗️ Architecture

### Structure du Projet

```
angular-client/
├── src/app/
│   ├── services/
│   │   ├── hydra-client.service.ts      # Service Alcaeus (bas niveau)
│   │   └── magneto-api.service.ts       # Service métier Magneto
│   ├── models/
│   │   └── hydra.models.ts              # Types TypeScript
│   ├── components/
│   │   └── hydra-explorer.component.ts  # Composant principal
│   └── app.component.ts
├── package.json
└── README.md
```

### Couches de l'Application

```
┌─────────────────────────────────────┐
│  HydraExplorerComponent             │  ← UI avec navigation Hydra
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  MagnetoApiService                  │  ← API métier typée
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  HydraClientService (Alcaeus)       │  ← Client Hydra générique
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  API Magneto-Serge (Hydra/JSON-LD)  │  ← Backend
└─────────────────────────────────────┘
```

## 💡 Exemples d'Utilisation

### Service Hydra Client de Bas Niveau

```typescript
import { HydraClientService } from './services/hydra-client.service';

export class MyComponent {
  constructor(private hydraClient: HydraClientService) {}

  ngOnInit() {
    // Configurer le client
    this.hydraClient.configure({
      baseUrl: 'http://localhost:8889',
      apiKey: 'your-api-key',
      cacheEnabled: true
    });

    // Charger une ressource
    this.hydraClient.loadResource('/proxy/status').subscribe(resource => {
      console.log('Resource:', resource);

      // Extraire les liens Hydra
      const links = this.hydraClient.extractHydraLinks(resource);
      console.log('Available links:', links);

      // Extraire les opérations
      const operations = this.hydraClient.extractOperations(resource);
      console.log('Available operations:', operations);
    });
  }
}
```

### Service Métier Magneto

```typescript
import { MagnetoApiService } from './services/magneto-api.service';

export class ProxyControlComponent {
  constructor(private magnetoApi: MagnetoApiService) {}

  startProxy() {
    this.magnetoApi.startProxy({
      mode: 'auto',
      cassette_name: 'test',
      port: 8888
    }).subscribe(result => {
      console.log('Proxy started:', result.message);

      // Navigation Hydra: actions suivantes découvertes automatiquement
      console.log('Next actions:', result.nextActions);
    });
  }

  loadStatus() {
    this.magnetoApi.getProxyStatus().subscribe(({ status, availableActions }) => {
      console.log('Status:', status);

      // Actions disponibles via liens Hydra
      console.log('Can perform:', availableActions);
      // Ex: ["Stop Proxy", "Get Statistics", "List Cassettes"]
    });
  }
}
```

### Navigation Hydra Dynamique

```typescript
// Découvrir l'API automatiquement
this.magnetoApi.discoverApi().subscribe(({ info, availableActions }) => {
  console.log('API:', info.title, info.version);
  console.log('Available:', availableActions);
  // ["Health Check", "Proxy Status", "Start Proxy", "List Cassettes"]
});

// Suivre un lien Hydra
this.magnetoApi.getProxyStatus().subscribe(({ resource }) => {
  // Suivre le lien "Stop Proxy" découvert dans la ressource
  this.magnetoApi.followLink(resource, 'Stop Proxy').subscribe(nextResource => {
    console.log('Navigated to:', nextResource);
  });
});
```

### Écouter les Événements de Navigation

```typescript
export class NavLogComponent implements OnInit {
  constructor(private hydraClient: HydraClientService) {}

  ngOnInit() {
    this.hydraClient.navigationEvents.subscribe(event => {
      switch (event.type) {
        case HydraNavigationEvent.RESOURCE_LOADED:
          console.log('Resource loaded:', event.resource);
          break;

        case HydraNavigationEvent.LINK_FOLLOWED:
          console.log('Link followed:', event.link?.title);
          break;

        case HydraNavigationEvent.OPERATION_EXECUTED:
          console.log('Operation executed:', event.operation?.method);
          break;

        case HydraNavigationEvent.ERROR:
          console.error('Navigation error:', event.error);
          break;
      }
    });
  }
}
```

## 🎨 Composant Hydra Explorer

Le composant `HydraExplorerComponent` démontre toutes les capacités:

### Fonctionnalités

✅ **Découverte API automatique** - Charge `/` et extrait les liens Hydra
✅ **Navigation breadcrumb** - Historique des ressources visitées
✅ **Actions dynamiques** - Boutons générés depuis les liens Hydra
✅ **Statut en temps réel** - Affiche l'état du proxy
✅ **Contrôle du proxy** - Start/Stop via opérations Hydra
✅ **Gestion cassettes** - Liste, view, delete avec liens
✅ **Log d'événements** - Toutes les navigations Hydra
✅ **Vue JSON-LD brute** - Inspection des ressources

### Utilisation

```typescript
// app.component.ts
import { HydraExplorerComponent } from './components/hydra-explorer.component';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [HydraExplorerComponent],
  template: '<app-hydra-explorer></app-hydra-explorer>'
})
export class AppComponent {}
```

## 🔧 Configuration

### TypeScript Configuration

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ES2022",
    "moduleResolution": "bundler",
    "esModuleInterop": true,
    "skipLibCheck": true,
    "resolveJsonModule": true
  }
}
```

### Angular Configuration

```typescript
// app.config.ts
import { provideHttpClient } from '@angular/common/http';

export const appConfig: ApplicationConfig = {
  providers: [
    provideHttpClient(),
    // Services Hydra automatiquement injectés via providedIn: 'root'
  ]
};
```

## 🌟 Avantages de l'Approche Hydra

### Sans Hydra (Approche Classique)
```typescript
// URLs codées en dur
const response = await http.get('http://localhost:8889/proxy/start');

// Si l'API change d'URL, tout casse
if (response.running) {
  // Hardcodé: on sait qu'il faut aller à /proxy/stop
  await http.post('http://localhost:8889/proxy/stop');
}
```

### Avec Hydra (Approche Hypermedia)
```typescript
// Découverte automatique
const resource = await hydraClient.loadResource('/');
const links = hydraClient.extractHydraLinks(resource);

// Les URLs sont découvertes dynamiquement
const startLink = links.find(l => l.title === 'Start Proxy');
const nextResource = await hydraClient.loadResource(startLink.target);

// Les actions suivantes sont dans la réponse
const stopLink = nextResource['hydra:link'].find(l => l.title === 'Stop Proxy');
// L'API guide le client !
```

**Avantages:**
- ✅ URLs découvertes automatiquement (pas de hardcoding)
- ✅ API auto-documentée via les liens
- ✅ Changements d'API non-breaking (les liens s'adaptent)
- ✅ Client générique (fonctionne avec toute API Hydra)
- ✅ Navigation guidée (l'API dit ce qu'on peut faire)

## 📚 Ressources

### Documentation Hydra/JSON-LD
- [Hydra Core Vocabulary](https://www.hydra-cg.com/spec/latest/core/)
- [JSON-LD](https://json-ld.org/)
- [Alcaeus Documentation](https://alcaeus.hydra.how/)

### Documentation Angular
- [Angular Documentation](https://angular.dev/)
- [RxJS](https://rxjs.dev/)

### API Magneto-Serge
- [API Documentation](../../docs/API.md)
- [OpenAPI Spec](http://localhost:8889/openapi.json)

## 🐛 Troubleshooting

### Erreur CORS
Si vous obtenez des erreurs CORS:

```typescript
// L'API Magneto inclut déjà les headers CORS
// Vérifiez que l'API est démarrée:
magneto api --host 0.0.0.0
```

### Alcaeus Import Errors
```bash
# Si problèmes d'imports ES modules
npm install --save @zazuko/env @rdfjs/data-model
```

### TypeScript Errors avec Alcaeus
```typescript
// Utiliser les types RDF explicites
import { IResource } from 'alcaeus';

const resource: IResource = await client.loadResource('/');
```

## 🚀 Prochaines Étapes

- [ ] Tests unitaires avec Jasmine
- [ ] Tests E2E avec Playwright
- [ ] Optimisation du bundle (lazy loading)
- [ ] Mode offline avec Service Worker
- [ ] Support de la pagination Hydra
- [ ] Visualisation du graphe RDF

## 📄 Licence

Même licence que le projet principal Magneto-Serge (MIT OR Apache-2.0).

---

**Développé avec Angular 19 + Alcaeus 2.0 pour exploiter pleinement l'API Hydra de Magneto-Serge**
