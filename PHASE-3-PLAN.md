# Phase 3: Advanced Features & Interactivity - PLAN

**Date de début prévue:** 2025-10-27
**Durée estimée:** 3-4 semaines
**Statut:** 📋 Planification

---

## 📋 Vue d'ensemble

Phase 3 ajoute des fonctionnalités avancées à l'API Hydra et au frontend Angular pour transformer l'application en un outil de gestion complet et interactif.

### Objectifs Principaux

✅ **CRUD complet** - Créer, lire, mettre à jour, supprimer des cassettes
✅ **Recherche avancée** - Filtres, tri, recherche full-text
✅ **Mises à jour temps réel** - WebSocket pour notifications live
✅ **Documentation interactive** - OpenAPI/Swagger UI intégré

### Technologies Additionnelles

- **Backend**: Axum WebSocket, OpenAPI 3.0 (utoipa)
- **Frontend**: Angular Forms (Reactive), WebSocket client, RxJS operators avancés
- **Testing**: Tests E2E avec Playwright, tests WebSocket

---

## 🗺️ Roadmap Phase 3

### Phase 3.0: Actions CRUD sur Cassettes (Semaine 1)
**Durée:** 5-7 jours
**Priorité:** 🔴 Haute

**Backend:**
- [ ] Endpoint POST /api/cassettes (créer cassette vide ou depuis template)
- [ ] Endpoint DELETE /api/cassettes/:name (supprimer cassette)
- [ ] Endpoint PUT /api/cassettes/:name (mettre à jour métadonnées)
- [ ] Validation des noms de cassettes (regex, unicité)
- [ ] Gestion erreurs 409 Conflict, 404 Not Found

**Frontend:**
- [ ] Dialog Material pour créer cassette (nom, description, mode)
- [ ] Bouton "Nouvelle Cassette" dans toolbar
- [ ] Dialog de confirmation pour suppression
- [ ] Formulaire réactif avec validation temps réel
- [ ] Actions NgRx: CreateCassette, DeleteCassette, UpdateCassette
- [ ] Effets NgRx pour appels API asynchrones
- [ ] Toasts Material Snackbar pour feedback utilisateur

**Tests:**
- [ ] Tests unitaires backend (création, suppression, validation)
- [ ] Tests unitaires frontend (formulaires, dialogs)
- [ ] Tests d'intégration E2E (créer → afficher → supprimer)

**Livrables:**
- `PHASE-3.0-CRUD.md` - Documentation détaillée
- Backend handlers: `src/api/handlers/cassette_create.rs`, `cassette_delete.rs`
- Frontend components: `cassette-create-dialog/`, `cassette-delete-dialog/`
- Frontend actions: `cassette.actions.ts` (ajout CREATE, DELETE, UPDATE actions)

---

### Phase 3.1: Recherche et Filtres (Semaine 2)
**Durée:** 5-7 jours
**Priorité:** 🟡 Moyenne

**Backend:**
- [ ] Endpoint GET /api/cassettes?search=term (recherche full-text dans nom + description)
- [ ] Filtres par type: ?type=http|websocket
- [ ] Filtres par date: ?from=ISO8601&to=ISO8601
- [ ] Tri: ?sort=name|date&order=asc|desc
- [ ] Pagination améliorée avec hydra:search template

**Frontend:**
- [ ] Barre de recherche Material dans toolbar
- [ ] Chips Material pour filtres actifs
- [ ] Menu déroulant pour sélection type (HTTP/WebSocket/Tous)
- [ ] Date pickers Material pour plage de dates
- [ ] Boutons de tri (nom ↑↓, date ↑↓)
- [ ] Debounce sur input search (300ms) pour éviter trop d'appels API
- [ ] Effets NgRx pour synchroniser filtres avec URL query params
- [ ] Sélecteurs NgRx pour filtres actifs

**Tests:**
- [ ] Tests backend: recherche, filtres combinés, tri
- [ ] Tests frontend: input debounce, application filtres, URL sync
- [ ] Tests E2E: rechercher → filtrer → trier

**Livrables:**
- `PHASE-3.1-SEARCH.md` - Documentation détaillée
- Backend: `src/api/handlers/cassettes_search.rs`
- Frontend components: `cassette-search-bar/`, `cassette-filters/`
- Frontend: URL query params service

---

### Phase 3.2: WebSocket Live Updates (Semaine 3)
**Durée:** 5-7 jours
**Priorité:** 🟢 Basse (optionnelle)

**Backend:**
- [ ] WebSocket endpoint /ws (Axum WebSocket)
- [ ] Broadcast events: CassetteCreated, CassetteDeleted, CassetteUpdated
- [ ] Authentification WebSocket (JWT token dans query string)
- [ ] Keep-alive ping/pong toutes les 30s
- [ ] Gestion reconnexion automatique

**Frontend:**
- [ ] Service WebSocket Angular avec RxJS
- [ ] Reconnexion automatique avec backoff exponentiel
- [ ] Effets NgRx pour écouter événements WebSocket
- [ ] Mise à jour automatique de la liste cassettes
- [ ] Notifications Material Snackbar pour événements:
  - "Nouvelle cassette: {name}"
  - "Cassette supprimée: {name}"
  - "Cassette mise à jour: {name}"
- [ ] Indicateur de connexion WebSocket dans UI (badge "Live")

**Tests:**
- [ ] Tests backend: connexion WebSocket, broadcast, reconnexion
- [ ] Tests frontend: WebSocket service, reconnexion, événements
- [ ] Tests E2E: créer cassette dans tab 1 → voir apparaître dans tab 2

**Livrables:**
- `PHASE-3.2-REALTIME.md` - Documentation détaillée
- Backend: `src/api/websocket.rs`, `src/api/events.rs`
- Frontend service: `websocket.service.ts`
- Frontend effects: `websocket.effects.ts`

---

### Phase 3.3: Documentation OpenAPI (Semaine 4)
**Durée:** 3-5 jours
**Priorité:** 🟡 Moyenne

**Backend:**
- [ ] Intégrer crate `utoipa` pour génération OpenAPI 3.0
- [ ] Annoter tous les handlers avec `#[utoipa::path]`
- [ ] Définir schemas avec `#[derive(ToSchema)]`
- [ ] Endpoint GET /openapi.json (spec OpenAPI complète)
- [ ] Intégrer Swagger UI (via utoipa-swagger-ui)
- [ ] Endpoint GET /swagger-ui (interface interactive)
- [ ] Exemples de requêtes/réponses dans spec

**Frontend:**
- [ ] Lien vers Swagger UI dans menu navigation
- [ ] Page "API Documentation" avec embedded Swagger UI
- [ ] Bouton "Voir API Docs" dans toolbar

**Tests:**
- [ ] Validation OpenAPI spec (via openapi-validator)
- [ ] Tests que tous les endpoints sont documentés
- [ ] Tests exemples dans spec sont valides

**Livrables:**
- `PHASE-3.3-OPENAPI.md` - Documentation détaillée
- Backend: spec OpenAPI complète à `/openapi.json`
- Backend: Swagger UI à `/swagger-ui`
- Frontend: page API documentation

---

## 📊 Métriques de Succès

### Backend
- [ ] 15+ nouveaux endpoints documentés
- [ ] 100% couverture OpenAPI spec
- [ ] WebSocket supporte 100+ clients simultanés
- [ ] Latence recherche < 50ms pour 10k cassettes

### Frontend
- [ ] Formulaire de création validé en temps réel
- [ ] Recherche avec debounce fonctionnelle
- [ ] Mises à jour WebSocket < 100ms de latence
- [ ] 0 erreur console en production

### Tests
- [ ] 50+ nouveaux tests unitaires (backend + frontend)
- [ ] 10+ tests E2E (création, recherche, suppression, WebSocket)
- [ ] 95%+ code coverage sur nouveaux modules

---

## 🏗️ Architecture Changes

### Backend - Nouvelles Routes

```rust
// CRUD
POST   /api/cassettes          // Créer cassette
PUT    /api/cassettes/:name    // Mettre à jour métadonnées
DELETE /api/cassettes/:name    // Supprimer cassette

// Recherche
GET    /api/cassettes?search=term&type=http&sort=date&order=desc

// WebSocket
GET    /ws                     // WebSocket live updates

// Documentation
GET    /openapi.json           // Spec OpenAPI 3.0
GET    /swagger-ui             // Interface Swagger UI
```

### Frontend - Nouveaux Composants

```
frontend/src/app/features/cassettes/components/
├── cassette-create-dialog/       # Dialog création
├── cassette-delete-dialog/       # Dialog confirmation suppression
├── cassette-search-bar/          # Barre de recherche
├── cassette-filters/             # Filtres type, date, tri
└── api-documentation/            # Page docs Swagger UI

frontend/src/app/core/services/
├── websocket.service.ts          # Service WebSocket
└── search.service.ts             # Service recherche

frontend/src/app/features/cassettes/store/
├── cassette.actions.ts           # + CREATE, DELETE, UPDATE, SEARCH actions
├── cassette.effects.ts           # + WebSocket effects
└── cassette.selectors.ts         # + Selectors filtres/recherche
```

### NgRx State Extensions

```typescript
interface CassetteState {
  cassettes: Cassette[];
  totalItems: number;
  loading: boolean;
  error: string | null;

  // Phase 3 additions
  searchTerm: string;              // Terme de recherche actuel
  filters: {
    type: 'all' | 'http' | 'websocket';
    dateFrom: string | null;
    dateTo: string | null;
  };
  sortBy: 'name' | 'date';
  sortOrder: 'asc' | 'desc';
  websocketConnected: boolean;     // Statut connexion WebSocket
  websocketReconnecting: boolean;  // En cours de reconnexion
}
```

---

## 🧪 Testing Strategy

### Tests Unitaires Backend
```bash
# CRUD handlers
cargo test cassette_create
cargo test cassette_delete
cargo test cassette_update

# Recherche
cargo test search_by_name
cargo test filter_by_type
cargo test sort_by_date

# WebSocket
cargo test websocket_connect
cargo test websocket_broadcast
cargo test websocket_reconnect
```

### Tests Unitaires Frontend
```bash
# Composants
ng test --include='**/cassette-create-dialog.component.spec.ts'
ng test --include='**/cassette-search-bar.component.spec.ts'

# Services
ng test --include='**/websocket.service.spec.ts'

# NgRx
ng test --include='**/cassette.effects.spec.ts'
```

### Tests E2E
```bash
# Playwright
npx playwright test cassette-crud.spec.ts
npx playwright test cassette-search.spec.ts
npx playwright test websocket-live-updates.spec.ts
```

---

## 📅 Timeline Détaillé

### Semaine 1 (Phase 3.0 - CRUD)
- **Jour 1-2**: Backend endpoints (POST, DELETE, PUT)
- **Jour 3-4**: Frontend dialogs et formulaires
- **Jour 5**: NgRx actions et effets
- **Jour 6-7**: Tests et documentation

### Semaine 2 (Phase 3.1 - Recherche)
- **Jour 1-2**: Backend recherche et filtres
- **Jour 3-4**: Frontend barre de recherche et filtres
- **Jour 5**: URL query params sync
- **Jour 6-7**: Tests et optimisations

### Semaine 3 (Phase 3.2 - WebSocket)
- **Jour 1-2**: Backend WebSocket endpoint
- **Jour 3-4**: Frontend WebSocket service
- **Jour 5**: Notifications et UI updates
- **Jour 6-7**: Tests WebSocket et stabilisation

### Semaine 4 (Phase 3.3 - OpenAPI)
- **Jour 1-2**: Intégration utoipa et annotations
- **Jour 3-4**: Swagger UI et documentation
- **Jour 5**: Tests et validation spec

---

## 🚀 Commandes de Développement

### Démarrer Phase 3.0 (CRUD)
```bash
# Créer branche feature
git checkout develop
git pull origin develop
git checkout -b feature/phase-3.0-crud

# Démarrer backend
cargo run --example hydra_api_server --features hydra

# Démarrer frontend
cd frontend
./dev-server.sh
```

### Tests Phase 3
```bash
# Backend
cargo test --features hydra crud
cargo test --features hydra search
cargo test --features hydra websocket

# Frontend
cd frontend
npm test
npm run e2e
```

---

## 📚 Documentation à Créer

1. **PHASE-3.0-CRUD.md** - Guide CRUD complet
2. **PHASE-3.1-SEARCH.md** - Guide recherche et filtres
3. **PHASE-3.2-REALTIME.md** - Guide WebSocket live updates
4. **PHASE-3.3-OPENAPI.md** - Guide OpenAPI/Swagger
5. **PHASE-3-COMPLETE.md** - Résumé Phase 3 (après achèvement)

---

## 🎯 Critères d'Achèvement Phase 3

### Phase 3.0 Complete
- [x] Cassettes peuvent être créées via UI
- [x] Cassettes peuvent être supprimées via UI
- [x] Validation formulaires fonctionne
- [x] Toasts de confirmation affichés
- [x] Tests CRUD passent

### Phase 3.1 Complete
- [x] Recherche full-text fonctionne
- [x] Filtres par type fonctionnent
- [x] Tri par nom/date fonctionne
- [x] URL query params synchronisés
- [x] Tests recherche passent

### Phase 3.2 Complete
- [x] WebSocket se connecte automatiquement
- [x] Événements live reçus et affichés
- [x] Reconnexion automatique fonctionne
- [x] Notifications snackbar affichées
- [x] Tests WebSocket passent

### Phase 3.3 Complete
- [x] Spec OpenAPI 3.0 générée
- [x] Swagger UI accessible à /swagger-ui
- [x] Tous les endpoints documentés
- [x] Exemples de requêtes valides
- [x] Tests validation spec passent

---

## 🔗 Liens Utiles

- **Phase 2 Complete**: [PHASE-2-COMPLETE.md](PHASE-2-COMPLETE.md)
- **Architecture Backend**: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- **Frontend Guide**: [frontend/DEVELOPMENT.md](frontend/DEVELOPMENT.md)
- **Hydra Spec**: https://www.hydra-cg.com/spec/latest/core/
- **OpenAPI 3.0 Spec**: https://spec.openapis.org/oas/v3.0.3
- **utoipa Docs**: https://docs.rs/utoipa/latest/utoipa/

---

**Auteur:** Claude Code + Équipe Magnéto-Serge
**Date:** 2025-10-26
**Version:** 0.7.0 (prévue)
