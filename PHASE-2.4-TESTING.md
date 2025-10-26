# Phase 2.4: Testing & Runtime Validation

**Date:** 2025-10-26
**Status:** ✅ Backend Ready, Frontend Testing in Progress

---

## Backend API Status

✅ **API Server Running**
- URL: http://localhost:8889
- Port: 8889 (LISTEN confirmed with lsof)
- Example: hydra_api_server
- Features: hydra enabled

✅ **API Endpoints Verified**
```bash
# Root API Documentation
curl http://localhost:8889/api
# Returns: Hydra ApiDocumentation with entrypoint

# Cassettes Collection
curl http://localhost:8889/api/cassettes
# Returns: Empty HydraCollection (hydra:totalItems: 0)
```

## Test Cassettes Created

3 cassettes de test créées dans `/tmp/magneto-test-cassettes/`:

1. **github-api.json** (1.6K)
   - 2 HTTP interactions (GET user, GET repos)
   - GitHub API v3 format

2. **websocket-chat.json** (1.0K)
   - WebSocket interaction
   - 4 messages (2 sent, 2 received)
   - Echo pattern

3. **rest-api-test.json** (1.8K)
   - 3 HTTP interactions (POST, GET, DELETE)
   - CRUD operations
   - Status codes: 201, 200, 204

## Frontend Configuration

### Proxy Configuration
`frontend/proxy.conf.json`:
```json
{
  "/api": {
    "target": "http://localhost:8889",
    "secure": false,
    "changeOrigin": true,
    "logLevel": "debug"
  }
}
```

### Environment Configuration
`frontend/src/environments/environment.ts`:
```typescript
export const environment = {
  production: false,
  apiUrl: 'http://localhost:8889/api',
  hydraContext: 'http://localhost:8889/api'
};
```

## Testing Plan

### Test 1: Empty State (EN COURS)
**Objectif:** Valider l'affichage avec 0 cassettes

**Étapes:**
1. Backend API démarré ✅
2. Liste cassettes vide confirmée ✅
3. Démarrer frontend: `cd frontend && npm start`
4. Vérifier affichage liste vide
5. Vérifier message approprié
6. Vérifier que le loading spinner fonctionne

**Comportement attendu:**
- Table vide affichée
- Message "Aucune cassette disponible"
- Pagination: "Page 0 sur 0" ou désactivée
- Pas d'erreurs console

### Test 2: API Connection
**Objectif:** Vérifier la connexion frontend ↔ backend

**Étapes:**
1. Ouvrir DevTools → Network tab
2. Rafraîchir page
3. Vérifier requête: GET http://localhost:4200/api/cassettes
4. Vérifier proxy redirige vers http://localhost:8889/api/cassettes
5. Vérifier réponse 200 OK
6. Vérifier payload JSON-LD

### Test 3: NgRx State
**Objectif:** Vérifier NgRx Store fonctionne

**Étapes:**
1. Installer Redux DevTools Extension
2. Ouvrir Redux DevTools
3. Vérifier action: `[Cassette] Load Cassettes`
4. Vérifier state: `{ cassettes: [], totalItems: 0, loading: false }`
5. Tester time-travel debugging

### Test 4: Error Handling
**Objectif:** Tester gestion d'erreur quand API down

**Étapes:**
1. Arrêter backend API (Ctrl+C)
2. Rafraîchir frontend
3. Vérifier message d'erreur affiché
4. Vérifier bouton "Réessayer" présent
5. Redémarrer API
6. Cliquer "Réessayer"
7. Vérifier liste se charge

## Adding Cassettes (Future)

Pour tester avec des données réelles, il faut:

### Option 1: Copier cassettes dans le bon répertoire
Le serveur lit depuis `./cassettes` relatif au workspace root:
```bash
cp /tmp/magneto-test-cassettes/*.json cassettes/
```

### Option 2: Utiliser des vraies cassettes enregistrées
```bash
# Enregistrer une cassette avec magneto CLI
magneto record github-test
# Faire des requêtes HTTP/WebSocket
# Sauvegarder
magneto stop
# Les cassettes seront dans cassettes/
```

### Option 3: Modifier le code exemple
Éditer `examples/hydra_api_server.rs`:
```rust
let server_config = ApiServerConfig {
    cassette_dir: std::env::var("MAGNETO_CASSETTE_DIR")
        .unwrap_or_else(|_| "./cassettes".to_string()),
    // ...
};
```

## Issue Rencontré: ng serve (Vite) Ne Sert Pas les Fichiers JS

### Symptôme
- `ng serve` démarre correctement et compile l'application
- Le HTML est servi (http://localhost:4201/)
- **MAIS** les fichiers `main.js` et `polyfills.js` retournent 404
- Résultat: page blanche dans le navigateur

### Diagnostic
```bash
# HTML OK
curl http://localhost:4201/
# -> Retourne HTML avec <script src="main.js">

# JavaScript 404
curl -I http://localhost:4201/main.js
# -> HTTP/1.1 404 Not Found

# CSS OK
curl -I http://localhost:4201/styles.css
# -> HTTP/1.1 200 OK
```

### Cause Racine
**Angular 17+ avec `@angular-devkit/build-angular:application` et Vite**

Le nouveau builder `application` utilise Vite pour le dev server. En mode développement, Vite ne sert pas les fichiers compilés comme `main.js` à la racine - ils doivent être chargés via le système de modules Vite.

Problème: Angular transforme l'index.html et injecte:
```html
<script src="polyfills.js" type="module"></script>
<script src="main.js" type="module"></script>
```

Mais ces fichiers n'existent pas à ces chemins dans le dev server Vite.

**Note additionnelle:** Node.js 24.5.0 est non supporté par Angular 17 (version LTS recommandée: 18.x ou 20.x)

### Solution (Workaround)

#### Option 1: Build + http-server (UTILISÉ)
```bash
# 1. Build en mode développement
cd frontend
npx ng build --configuration=development --output-path=dist/dev

# 2. Servir avec http-server + proxy
cd dist/dev/browser
npx http-server -p 4201 --proxy http://localhost:8889
```

**Avantages:**
- ✅ Fonctionne immédiatement
- ✅ Proxy vers backend configuré
- ✅ Fichiers JS/CSS servis correctement
- ✅ Pas besoin de modification de code

**Inconvénients:**
- ⚠️ Pas de hot reload (il faut rebuild manuellement)
- ⚠️ Temps de build plus long (~3 secondes)

#### Option 2: Downgrade vers browser builder
Changer dans `angular.json`:
```json
{
  "architect": {
    "build": {
      "builder": "@angular-devkit/build-angular:browser",
      // ...
    }
  }
}
```

**Problème:** Nécessite migration complète (Angular 17+ recommande `application`)

#### Option 3: Patcher Vite config
Créer `vite.config.ts` pour customiser le comportement Vite.

**Problème:** Angular 17 ne supporte pas encore bien Vite customization

### Résultat Final

✅ **Stack Fonctionnelle:**
- **Backend:** http://localhost:8889/api (Hydra API Server)
- **Frontend:** http://localhost:4201/ (http-server avec proxy)

✅ **Vérifications:**
```bash
# HTML
curl http://localhost:4201/ # -> 200 OK

# JavaScript
curl -I http://localhost:4201/main.js # -> 200 OK (3.0 MB)
curl -I http://localhost:4201/polyfills.js # -> 200 OK (90 KB)

# CSS
curl -I http://localhost:4201/styles.css # -> 200 OK (186 KB)

# API via proxy
curl http://localhost:4201/api/cassettes
# -> Proxy vers http://localhost:8889/api/cassettes
```

## Next Steps

1. ✅ Backend API running
2. ✅ Test cassettes created
3. ✅ Frontend built and served (workaround pour Angular 17 + Vite)
4. ⏳ Tester dans le navigateur: http://localhost:4201/
5. ⏳ Vérifier l'affichage de la liste vide de cassettes
6. ⏳ Vérifier NgRx DevTools
7. ⏳ Copier cassettes vers `./cassettes` pour test avec données

### Commandes Rapides

```bash
# Terminal 1: Backend API
cargo run --example hydra_api_server --features hydra

# Terminal 2: Frontend (build + serve)
cd frontend
npx ng build --configuration=development --output-path=dist/dev && \
cd dist/dev/browser && \
npx http-server -p 4201 --proxy http://localhost:8889
```

---

**Note:** Une fois Angular 17 + Vite mieux stabilisé, `ng serve` devrait fonctionner directement.
