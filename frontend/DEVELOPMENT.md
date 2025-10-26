# Frontend Development Guide

## Vue d'ensemble

Le frontend Magnéto-Serge est une application Angular 17+ standalone avec Material Design et NgRx pour la gestion d'état. Il consomme l'API Hydra Hypermedia du backend.

**Stack Technique:**
- Angular 17.3 (Standalone Components)
- Angular Material 17.3
- NgRx 17.2 (Store + Effects)
- Alcaeus 1.1 (Client Hydra)
- RxJS 7.8
- TypeScript 5.4

## ⚠️ Problème Connu: Angular 17 + Vite Dev Server

Le nouveau builder `@angular-devkit/build-angular:application` avec Vite (Angular 17+) ne sert **pas correctement les fichiers JavaScript** en mode développement avec `ng serve`.

**Symptôme:** Page blanche dans le navigateur, main.js et polyfills.js retournent 404.

**Solution:** Utiliser le workflow build + http-server au lieu de `ng serve`.

## 🚀 Quick Start

### Option 1: Script Automatique (Recommandé)

Depuis le répertoire `frontend/`:

```bash
./dev-server.sh
```

Ce script:
1. Build l'application en mode développement
2. Active le watch mode (auto-rebuild sur changement)
3. Sert les fichiers avec http-server + proxy vers backend

**URLs:**
- Frontend: http://localhost:4201
- Backend (via proxy): http://localhost:4201/api/*

### Option 2: Commandes npm

```bash
# Build + serve (une seule commande)
npm run dev

# Ou étape par étape:
npm run build:dev          # Build en mode dev
npm run serve:built        # Servir les fichiers buildés

# Watch mode (rebuild automatique)
npm run watch              # Dans un terminal séparé
```

### Option 3: Commandes Manuelles

```bash
# Terminal 1: Backend API
cd ..
cargo run --example hydra_api_server --features hydra

# Terminal 2: Frontend build + watch
cd frontend
npx ng build --watch --configuration=development --output-path=dist/dev

# Terminal 3: Serveur HTTP
cd frontend/dist/dev/browser
npx http-server -p 4201 --proxy http://localhost:8889
```

## 📁 Structure du Projet

```
frontend/
├── src/
│   ├── app/
│   │   ├── core/              # Services globaux, guards, interceptors
│   │   │   ├── services/
│   │   │   │   └── alcaeus.service.ts    # Client Hydra API
│   │   │   └── models/        # Interfaces TypeScript
│   │   ├── features/          # Modules fonctionnels
│   │   │   └── cassettes/
│   │   │       ├── components/
│   │   │       │   ├── cassette-list/    # Liste des cassettes
│   │   │       │   └── cassette-detail/  # Détail d'une cassette
│   │   │       ├── store/     # NgRx state management
│   │   │       │   ├── cassette.actions.ts
│   │   │       │   ├── cassette.effects.ts
│   │   │       │   ├── cassette.reducer.ts
│   │   │       │   └── cassette.selectors.ts
│   │   │       └── cassettes.routes.ts
│   │   ├── shared/            # Composants réutilisables
│   │   ├── app.component.ts   # Composant racine
│   │   ├── app.config.ts      # Configuration standalone
│   │   └── app.routes.ts      # Routing
│   ├── environments/          # Configuration environnements
│   ├── styles.scss            # Styles globaux
│   ├── index.html
│   └── main.ts                # Point d'entrée
├── proxy.conf.json            # Configuration proxy dev
├── angular.json               # Configuration Angular CLI
├── tsconfig.json              # Configuration TypeScript
├── package.json               # Dépendances npm
└── dev-server.sh              # Script de développement
```

## 🔧 Configuration

### Proxy Configuration (`proxy.conf.json`)

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

Routes `/api/*` sont proxifiées vers le backend Hydra API.

### Environment Configuration

**Development** (`src/environments/environment.ts`):
```typescript
export const environment = {
  production: false,
  apiUrl: 'http://localhost:8889/api',
  hydraContext: 'http://localhost:8889/api'
};
```

**Production** (`src/environments/environment.prod.ts`):
```typescript
export const environment = {
  production: true,
  apiUrl: '/api',
  hydraContext: '/api'
};
```

## 🧪 Tests

```bash
# Unit tests (Jasmine + Karma)
npm test

# Tests avec coverage
npm test -- --code-coverage

# Tests en watch mode
npm test -- --watch
```

## 🏗️ Build

### Development Build

```bash
npm run build:dev
# Output: dist/dev/
# - Source maps: ✅
# - Minification: ❌
# - Optimisation: ❌
```

### Production Build

```bash
npm run build
# Output: dist/magneto-serge-ui/browser/
# - Source maps: ❌
# - Minification: ✅
# - Optimisation: ✅
# - Bundle budgets: 2MB max
```

## 🔍 Debugging

### NgRx DevTools

Installer l'extension Chrome/Firefox: [Redux DevTools](https://github.com/reduxjs/redux-devtools)

Ouvrir DevTools → Redux → Observer le state et les actions.

### Browser Console

Logs de l'application Angular dans Console:
- Erreurs de réseau (API calls)
- Erreurs JavaScript
- Logs RxJS operators

### Network Tab

Vérifier les requêtes HTTP vers l'API:
- `/api/cassettes` → GET collection
- `/api/cassettes/:name` → GET ressource
- Vérifier headers `Accept: application/ld+json`

## 📦 Dépendances Importantes

### Alcaeus (Hydra Client)

**Issue connue:** Pas de déclarations TypeScript officielles.

**Solution:** `src/alcaeus.d.ts` fournit les types manuellement.

```typescript
import { create } from 'alcaeus';
import type { HydraResponse, Resource } from 'alcaeus';

const client = create({});
const response = await client.loadResource<CassetteResource>(url);
```

### Node.js Polyfills

**Issue:** `parse-link-header` (dépendance d'Alcaeus) utilise modules Node.js (`querystring`, `url`).

**Solution:** Browser polyfills dans `package.json`:

```json
{
  "devDependencies": {
    "querystring-es3": "^0.2.1",
    "url": "^0.11.4"
  },
  "browser": {
    "querystring": "querystring-es3",
    "url": "url"
  }
}
```

## 🎨 Styles

### Material Theme

Thème Material pré-construit: `indigo-pink.css`

Personnalisation dans `src/styles.scss`:

```scss
@use '@angular/material' as mat;

$custom-primary: mat.define-palette(mat.$indigo-palette);
$custom-accent: mat.define-palette(mat.$pink-palette);
$custom-theme: mat.define-light-theme((
  color: (
    primary: $custom-primary,
    accent: $custom-accent,
  )
));

@include mat.all-component-themes($custom-theme);
```

### Component Styles

Styles scopés par composant (encapsulation Shadow DOM):

```typescript
@Component({
  selector: 'app-cassette-list',
  styleUrls: ['./cassette-list.component.scss'],
  // Styles appliqués uniquement à ce composant
})
```

## 🚦 Linting & Formatting

```bash
# TypeScript linting
npx ng lint

# Format code
npx prettier --write "src/**/*.{ts,html,scss}"
```

## 📚 Ressources

### Documentation Officielle
- [Angular](https://angular.dev/)
- [Angular Material](https://material.angular.io/)
- [NgRx](https://ngrx.io/)
- [Hydra Core Vocabulary](https://www.hydra-cg.com/)
- [Alcaeus](https://github.com/wikibus/Alcaeus)

### Guides Internes
- `PHASE-2.4-TESTING.md` - Diagnostic issue Angular 17 + Vite
- `../CLAUDE.md` - Architecture globale du projet
- `../docs/ROADMAP.md` - Roadmap Magnéto-Serge

## 🐛 Troubleshooting

### Page blanche dans le navigateur

**Cause:** Vite dev server (ng serve) ne sert pas les fichiers JS.

**Solution:** Utiliser `./dev-server.sh` ou `npm run dev`.

### Erreur CORS lors des appels API

**Cause:** Backend ne configure pas CORS pour localhost:4201.

**Solution:** Le proxy `proxy.conf.json` contourne ce problème.

### Module 'alcaeus' not found

**Cause:** Dépendances npm non installées.

**Solution:**
```bash
npm install
```

### Build errors avec querystring/url

**Cause:** Polyfills manquants.

**Solution:** Vérifier `package.json` contient:
```json
{
  "devDependencies": {
    "querystring-es3": "^0.2.1",
    "url": "^0.11.4"
  }
}
```

## 🔄 Workflow Recommandé

1. **Démarrer la stack complète:**
   ```bash
   # Depuis le répertoire racine
   ./scripts/dev-stack.sh
   ```

2. **Développer:**
   - Modifier fichiers dans `src/`
   - Watch mode rebuild automatiquement
   - Rafraîchir navigateur (F5)

3. **Tester:**
   - Vérifier NgRx DevTools (state changes)
   - Vérifier Network tab (API calls)
   - Vérifier Console (erreurs)

4. **Commit:**
   ```bash
   git add .
   git commit -m "feat(frontend): ..."
   ```

## 📈 Futures Améliorations

- [ ] Migrer vers Angular 18+ quand Vite dev server est stable
- [ ] Ajouter tests E2E (Cypress/Playwright)
- [ ] Implémenter SSR (Server-Side Rendering)
- [ ] Optimiser bundle size (<1MB)
- [ ] Ajouter PWA support
- [ ] Internationalisation (i18n)

---

**Dernière mise à jour:** 2025-10-26
**Version:** 0.5.0
