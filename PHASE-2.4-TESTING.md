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

## Next Steps

1. ✅ Backend API running
2. ✅ Test cassettes created
3. ⏳ Start frontend (`npm start`)
4. ⏳ Test empty state
5. ⏳ Test API connection
6. ⏳ Add cassettes and test full flow
7. ⏳ Document findings

---

**Note:** Ce document sera mis à jour avec les résultats des tests frontend.
