# Magnéto-Serge UI - Angular Frontend

Frontend Angular 17+ pour l'API Hydra Hypermedia de Magnéto-Serge (v0.5.0 Phase 2).

## 🎯 Fonctionnalités

- ✅ **Angular 17+** avec composants standalone
- ✅ **Alcaeus Client** pour navigation hypermedia Hydra
- ✅ **NgRx** pour gestion d'état (Store + Effects + Entity)
- ✅ **TypeScript strict** avec modèles typés
- ⏳ **Angular Material** UI (à installer)
- ⏳ **Composants** (cassette-list, cassette-detail, interaction-list)

## 📁 Structure

```
frontend/
├── src/
│   ├── app/
│   │   ├── core/                           # Services et modèles partagés
│   │   │   ├── services/
│   │   │   │   └── alcaeus.service.ts     ✅ Client Hydra
│   │   │   ├── models/
│   │   │   │   ├── cassette.model.ts      ✅ Modèle Cassette
│   │   │   │   └── interaction.model.ts   ✅ Modèle Interaction
│   │   │   └── interceptors/
│   │   │
│   │   ├── features/
│   │   │   └── cassettes/
│   │   │       ├── state/                  # NgRx State Management
│   │   │       │   ├── cassette.actions.ts     ✅ Actions
│   │   │       │   ├── cassette.reducer.ts     ✅ Reducer
│   │   │       │   ├── cassette.effects.ts     ✅ Effects
│   │   │       │   └── cassette.selectors.ts   ✅ Selectors
│   │   │       └── components/
│   │   │           ├── cassette-list/      ⏳ À implémenter
│   │   │           ├── cassette-detail/    ⏳ À implémenter
│   │   │           └── interaction-list/   ⏳ À implémenter
│   │   │
│   │   └── shared/                         # Composants réutilisables
│   │       ├── components/
│   │       │   └── pagination/             ⏳ Pagination Hydra
│   │       └── pipes/
│   │
│   ├── environments/
│   │   ├── environment.ts                  ✅ Config dev
│   │   └── environment.prod.ts             ✅ Config prod
│   │
│   └── assets/
│       └── styles/
│
└── package.json                            ✅ Dépendances définies
```

## 🚀 Installation

### Prérequis

```bash
node --version  # >= 18.0.0
npm --version   # >= 9.0.0
```

### Setup

```bash
cd frontend

# Installer les dépendances
npm install

# Installer Angular Material (interactif)
ng add @angular/material
```

## 📦 Dépendances Clés

Déjà configurées dans `package.json` :

- `@angular/core@^17.3.0` - Framework Angular
- `alcaeus@^1.0.0` - Client Hydra pour navigation hypermedia
- `@ngrx/store@^17.2.0` - State management
- `@ngrx/effects@^17.2.0` - Side effects
- `@ngrx/entity@^17.2.0` - Entity management
- `@ngrx/store-devtools@^17.2.0` - Redux DevTools

## 🏗️ Architecture

### Service Alcaeus

Le service `AlcaeusService` gère la communication avec l'API Hydra :

```typescript
// Charger une ressource
this.alcaeusService.loadResource('/cassettes')
  .subscribe(response => {
    const collection = response.root as CassetteCollection;
    // ...
  });

// Suivre un lien
this.alcaeusService.followLink(resource, 'interactions')
  .subscribe(response => {
    // ...
  });

// Navigation pagination
this.alcaeusService.nextPage(collection);
this.alcaeusService.previousPage(collection);
```

### NgRx State

State management avec actions, reducers, effects et selectors :

```typescript
// Dispatcher une action
this.store.dispatch(CassetteActions.loadCassettes({ params: { page: 1, limit: 20 } }));

// Sélectionner des données
this.cassettes$ = this.store.select(selectCassettes);
this.loading$ = this.store.select(selectLoading);
this.paginationInfo$ = this.store.select(selectPaginationInfo);
```

## 🧪 Développement

```bash
# Serveur de dev
ng serve
# http://localhost:4200

# Avec proxy vers l'API backend
ng serve --proxy-config proxy.conf.json
```

**proxy.conf.json** (à créer) :
```json
{
  "/api": {
    "target": "http://localhost:8889",
    "secure": false,
    "changeOrigin": true
  }
}
```

## 📝 Prochaines Étapes

### Phase 2.1 - Composants UI

- [ ] Installer Angular Material
- [ ] Créer `CassetteListComponent` avec table Material
- [ ] Créer `CassetteDetailComponent`
- [ ] Créer `InteractionListComponent`
- [ ] Créer `PaginationComponent` Hydra

### Phase 2.2 - Routing

- [ ] Configurer app.routes.ts
- [ ] Routes : `/`, `/cassettes`, `/cassettes/:name`
- [ ] Guards de navigation

### Phase 2.3 - Tests

- [ ] Tests unitaires (Jasmine)
- [ ] Tests E2E (Cypress)
- [ ] Coverage > 80%

## 📚 Documentation

- [Specs complètes](../docs/PHASE-2-ANGULAR-SPECS.md)
- [Roadmap Phase 2](../ROADMAP-v0.5.0-HYPERMEDIA-API.md)
- [Angular Docs](https://angular.io/docs)
- [Alcaeus Docs](https://alcaeus.hydra.how/)
- [NgRx Docs](https://ngrx.io/docs)

## 🔗 API Backend

L'API Hydra doit tourner sur `http://localhost:8889` :

```bash
# Depuis la racine du projet
cargo run --example hydra_api_server --features hydra
```

Endpoints disponibles :
- `GET /api` - Documentation
- `GET /api/cassettes?page=1&limit=20` - Cassettes paginées
- `GET /api/cassettes/:name` - Cassette individuelle
- `GET /api/cassettes/:name/interactions` - Interactions

## 🤝 Contribution

Voir [CONTRIBUTING.md](../CONTRIBUTING.md) à la racine du projet.

---

**Status:** 🚧 Phase 2.0 Foundation - Service Alcaeus + NgRx Store complétés

**Next:** Phase 2.1 - UI Components avec Angular Material
