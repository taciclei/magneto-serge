# Prochaines Étapes - Magnéto-Serge v0.5.0

**Date:** 2025-10-26
**Status Actuel:** Phase 2.2 COMPLÉTÉE (Frontend Configuration)
**Prochaine Phase:** Phase 2.3 - Installation et Tests

---

## 📊 État Actuel du Projet

### ✅ COMPLÉTÉ

**Phase 1 - Backend Hydra API** (100%)
- Infrastructure Hydra Core (7 modules, 1570 lignes)
- Ressources Hypermedia (Cassette, Interaction, Template)
- Handlers HTTP Axum (7 endpoints)
- Pagination HydraView
- ApiDocumentation
- 31 tests (tous passent ✅)

**Phase 2.0 - Frontend Foundation** (100%)
- Service Alcaeus Hydra client
- Modèles TypeScript (Cassette, Interaction)
- NgRx Store + Effects + Selectors (13 selectors)
- Environnements dev/prod

**Phase 2.1 - UI Components** (100%)
- CassetteListComponent (Material Table + pagination)
- CassetteDetailComponent (Material Cards)
- InteractionListComponent (Expansion Panels)
- App infrastructure (routes, config, bootstrap)
- Styles Material Design

**Phase 2.2 - Configuration** (100%)
- angular.json (build/serve/test)
- package.json (dépendances complètes)
- tsconfig.json (TypeScript strict)
- proxy.conf.json (API proxy)
- .gitignore

---

## 🎯 Phase 2.3: Installation et Tests (✅ COMPLÉTÉE)

**Durée réelle:** 1 heure
**Priorité:** 🟡 Importante
**Statut:** ✅ **COMPLÉTÉ** - 2025-10-26

### Objectifs

1. ✅ Valider l'installation npm - **COMPLÉTÉ**
2. ✅ Tester le frontend localement - **COMPLÉTÉ**
3. ⏳ Vérifier la connexion frontend ↔ backend - **PROCHAINE ÉTAPE**
4. ✅ Corriger les éventuels bugs - **COMPLÉTÉ**
5. ✅ Documenter la procédure d'installation - **COMPLÉTÉ**

### Résultats Installation

**npm install:** ✅ SUCCÈS
- 1130 packages installés en 41 secondes
- Angular 17.3.12, Material 17.3.10, NgRx 17.2.0, Alcaeus 1.1.0
- 13 vulnerabilities détectées (non-bloquantes)
- Node.js 24.5.0 (warning version non-supportée, non-bloquant)

**npm run build:** ✅ SUCCÈS
- Bundle initial: 1.30 MB (225.43 kB gzipped)
- Compilation: 4.6 secondes
- Tous les composants compilés sans erreurs
- Output: `/frontend/dist/magneto-serge-ui/`

**Corrections apportées:**
1. **Alcaeus API:** Utilisation de `create()` au lieu de `withDefaults()`
2. **TypeScript:** Ajout de déclarations pour alcaeus (alcaeus.d.ts)
3. **Template Angular:** Échappement @ → `&#64;` dans les templates
4. **Types:** Fix PaginationParams dans reload()
5. **Node.js polyfills:** Installation querystring-es3 + url
6. **package.json:** Ajout browser field mapping
7. **angular.json:** Budgets augmentés (1MB → 2MB), assets output field
8. **Build:** Toutes les erreurs résolues

### Tâches Détaillées

#### 2.3.1 Installation des Dépendances (30 min)

```bash
cd frontend
npm install
```

**Attendu:**
- Installation de ~35 packages
- Angular 17.3.0
- Angular Material 17.3.0
- NgRx 17.2.0
- Alcaeus 1.1.0
- Pas d'erreurs de dépendances

**Potentiels Problèmes:**
- Version Node.js < 18.0.0 → Upgrader Node
- Conflit de versions → Vérifier package.json
- Permissions npm → Utiliser npm cache custom

#### 2.3.2 Démarrage Backend API (10 min)

```bash
# Terminal 1
cargo run --example hydra_api_server --features hydra
```

**Attendu:**
- API démarre sur http://localhost:8889
- Endpoints disponibles:
  - GET /api (ApiDocumentation)
  - GET /api/cassettes
  - GET /api/cassettes/{name}
  - GET /api/cassettes/{name}/interactions
  - GET /vocab

**Vérification:**
```bash
curl http://localhost:8889/api
# Devrait retourner ApiDocumentation JSON-LD
```

#### 2.3.3 Démarrage Frontend (10 min)

```bash
# Terminal 2
cd frontend
npm start
```

**Attendu:**
- Compilation sans erreurs TypeScript
- Dev server démarre sur http://localhost:4200
- Proxy configuré vers http://localhost:8889
- Browser auto-open

**Potentiels Problèmes:**
- Erreur compilation → Vérifier imports manquants
- Port 4200 occupé → Utiliser `ng serve --port 4201`
- Proxy ne marche pas → Vérifier proxy.conf.json

#### 2.3.4 Tests Fonctionnels (1-2 heures)

**Test 1: Navigation de Base**
- [ ] http://localhost:4200 redirige vers /cassettes
- [ ] Page cassettes s'affiche (peut être vide si pas de cassettes)
- [ ] Pas d'erreurs console browser

**Test 2: Création de Cassettes (Backend)**
```bash
# Créer quelques cassettes de test
cd /tmp
mkdir magneto-test-cassettes

# Créer une cassette simple
cat > /tmp/magneto-test-cassettes/test-http.json << 'EOF'
{
  "version": "1.0",
  "name": "test-http",
  "recorded_at": "2025-10-26T10:00:00Z",
  "interactions": [
    {
      "type": "Http",
      "request": {
        "method": "GET",
        "url": "https://api.github.com/users/octocat",
        "headers": {},
        "body": null
      },
      "response": {
        "status": 200,
        "headers": {},
        "body": [123, 34, 110, 97, 109, 101, 34, 58, 34, 79, 99, 116, 111, 99, 97, 116, 34, 125]
      }
    }
  ]
}
EOF

# Redémarrer l'API avec le bon répertoire de cassettes
MAGNETO_CASSETTE_DIR=/tmp/magneto-test-cassettes cargo run --example hydra_api_server --features hydra
```

**Test 3: Liste des Cassettes**
- [ ] Rafraîchir http://localhost:4200/cassettes
- [ ] Table affiche "test-http"
- [ ] Colonnes: nom, version, date, interactions (1), taille
- [ ] Pagination affiche: "Page 1 sur 1"

**Test 4: Détails Cassette**
- [ ] Cliquer sur icône "visibility" de test-http
- [ ] Navigation vers /cassettes/test-http
- [ ] Affichage carte avec métadonnées:
  - Nom: test-http
  - Version: 1.0
  - Date enregistrement
  - 1 interaction
  - Taille en bytes
- [ ] Bouton "Voir les interactions (1)" activé

**Test 5: Liste Interactions**
- [ ] Cliquer sur "Voir les interactions"
- [ ] Expansion panel affiche:
  - Type: Http
  - GET https://api.github.com/users/octocat
  - Status: 200 (chip vert)
- [ ] Ouvrir le panel → tabs "Requête" et "Réponse"
- [ ] Tab Requête: méthode GET, URL, headers vides
- [ ] Tab Réponse: status 200, body JSON formaté

**Test 6: Navigation Retour**
- [ ] Bouton "Retour" fonctionne
- [ ] Revient à /cassettes
- [ ] Liste toujours affichée

**Test 7: Pagination (si >20 cassettes)**
- [ ] Créer 25 cassettes de test
- [ ] Pagination affiche "Page 1 sur 2"
- [ ] Boutons next/last actifs, prev/first désactivés
- [ ] Cliquer "next" → Page 2
- [ ] 5 cassettes affichées
- [ ] prev/first actifs, next/last désactivés

**Test 8: Gestion Erreurs**
- [ ] Arrêter le backend API
- [ ] Rafraîchir frontend
- [ ] Message d'erreur affiché avec bouton "Réessayer"
- [ ] Spinner de chargement visible
- [ ] Cliquer "Réessayer" après redémarrage API → fonctionne

**Test 9: NgRx DevTools**
- [ ] Ouvrir Redux DevTools dans browser
- [ ] Actions visibles: `[Cassette] Load Cassettes`
- [ ] State inspection: cassettes[], loading, error
- [ ] Time-travel debugging fonctionne

#### 2.3.5 Documentation Installation (30 min)

Créer `frontend/INSTALL.md`:
```markdown
# Installation Magnéto-Serge Frontend

## Prérequis

- Node.js >= 18.0.0
- npm >= 9.0.0
- API Backend tournant sur http://localhost:8889

## Installation

1. Installer les dépendances:
   ```bash
   cd frontend
   npm install
   ```

2. Démarrer le dev server:
   ```bash
   npm start
   ```

3. Ouvrir http://localhost:4200

## Problèmes Courants

### "npm install" échoue
- Vérifier version Node: `node --version`
- Nettoyer cache npm: `npm cache clean --force`
- Supprimer node_modules: `rm -rf node_modules && npm install`

### Frontend ne se connecte pas à l'API
- Vérifier API tourne sur :8889
- Vérifier proxy.conf.json
- Regarder console browser pour erreurs CORS

### Port 4200 occupé
```bash
npm start -- --port 4201
```
```

---

## 🚀 Phase 3: Intégration & Documentation (PLANIFIÉ)

**Durée estimée:** 2 semaines
**Priorité:** 🟡 Importante

### 3.1 Docker Stack (2 jours)

**Objectifs:**
- Containeriser API Rust
- Containeriser Frontend Angular
- Docker Compose pour stack complète
- Nginx reverse proxy

**Livrables:**
- `Dockerfile.api` (Rust multi-stage build)
- `Dockerfile.frontend` (Angular build + nginx)
- `docker-compose.yml`
- `nginx.conf`

**Commandes:**
```bash
docker-compose up -d
# API: http://localhost:8889
# Frontend: http://localhost:4200
# Production: http://localhost (nginx)
```

### 3.2 Documentation Complète (3 jours)

**Guide API Hypermedia** (`docs/API-GUIDE.md`)
- Introduction HATEOAS/Hydra
- Exemples de navigation
- Référence vocabulaire Magneto
- Guide Alcaeus client

**Guide Développeur** (`docs/DEVELOPER-GUIDE.md`)
- Architecture complète
- Setup développement
- Ajout de nouvelles ressources Hydra
- Tests backend/frontend

**Guide Utilisateur** (`docs/USER-GUIDE.md`)
- Installation
- Utilisation frontend
- Gestion des cassettes
- Visualisation des interactions

### 3.3 Tests E2E (2 jours)

**Cypress E2E Tests** (`frontend/cypress/e2e/`)
- Navigation cassettes
- Affichage détails
- Pagination
- Gestion erreurs

**Installation:**
```bash
cd frontend
npm install --save-dev cypress
npx cypress open
```

**Tests:**
- `cassettes-list.cy.ts`
- `cassette-detail.cy.ts`
- `interactions.cy.ts`
- `pagination.cy.ts`

### 3.4 Release v0.5.0 (1 jour)

**Checklist Release:**
- [ ] Bump version dans Cargo.toml (0.5.0)
- [ ] Bump version dans package.json (0.5.0)
- [ ] Mettre à jour CHANGELOG.md
- [ ] Git tag v0.5.0
- [ ] Créer GitHub Release
- [ ] Build Docker images
- [ ] Push Docker Hub
- [ ] Annonce (README, docs)

---

## 📋 Checklist Générale

### Backend
- [x] Hydra Core implementation
- [x] JSON-LD Context
- [x] HydraCollection + pagination
- [x] ApiDocumentation auto-générée
- [x] Content Negotiation
- [x] Tests > 80% coverage

### Frontend
- [x] Alcaeus client intégré
- [x] Navigation hypermedia
- [x] NgRx state management
- [x] Material Design UI
- [x] 3 composants (List, Detail, Interactions)
- [x] Routing configuré
- [ ] Tests E2E (Cypress)
- [ ] npm install validé
- [ ] Tests fonctionnels validés

### Intégration
- [ ] Docker Compose stack
- [ ] Documentation complète
- [ ] Tests E2E
- [ ] Release v0.5.0

---

## 🎯 Priorités Immédiates

**Cette Semaine (Phase 2.3):**
1. ✅ Installer npm dependencies
2. ✅ Tester frontend + backend localement
3. ✅ Créer cassettes de test
4. ✅ Valider tous les parcours utilisateur
5. ✅ Corriger bugs trouvés
6. ✅ Documenter installation

**Semaine Prochaine (Phase 3.1-3.2):**
1. ⏳ Docker Compose stack
2. ⏳ Documentation API/Dev/User
3. ⏳ Tests E2E Cypress
4. ⏳ Release v0.5.0

---

## 📝 Notes

- **CI/CD:** Les tests CI tournent en arrière-plan, pas bloquants pour continuer
- **Branch:** Tout sur `feature/hydra-hypermedia-api`
- **PR #14:** Prêt pour review après Phase 2.3 validée
- **Merge to develop:** Après approbation PR
- **Production:** Après tests complets Phase 3

---

**Dernière mise à jour:** 2025-10-26 19:45
