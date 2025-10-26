# Roadmap v0.5.0: Interface & Docker Management Evolution

**Version:** 0.5.0
**Date de début:** 2025-10-26
**Durée estimée:** 6-8 semaines
**Status:** 📋 Planning

---

## 🎯 Vision Globale

Transformer Magneto-Serge en une plateforme complète de gestion de cassettes avec:
- Interface web moderne et interactive
- API GraphQL temps réel
- Gestion Docker optimisée
- Édition avancée de templates
- Dashboard avec métriques

---

## 📊 Vue d'Ensemble des Phases

```
Phase 1: Backend GraphQL + WebSocket        [3 semaines] ████████░░
Phase 2: Interface Web Moderne              [3 semaines] ░░░░░░░░░░
Phase 3: Docker Stack Enrichie              [1 semaine]  ░░░░░░░░░░
Phase 4: Intégration & Tests E2E            [1 semaine]  ░░░░░░░░░░
```

**Total:** 8 semaines
**Effort:** ~320 heures
**Équipe:** 1-2 développeurs

---

# Phase 1: Backend GraphQL + WebSocket (3 semaines)

## Semaine 1: Infrastructure GraphQL

### 1.1 Setup async-graphql
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **1.1.1 Dépendances Cargo**
  - [ ] Ajouter `async-graphql = "7.0"`
  - [ ] Ajouter `async-graphql-axum = "7.0"`
  - [ ] Ajouter `async-graphql-poem = "7.0"` (optionnel)
  - [ ] Ajouter `tokio-stream = "0.1"` pour subscriptions
  - [ ] Ajouter `futures-util = "0.3"`
  - [ ] Mettre à jour Cargo.toml avec feature `graphql`

- [ ] **1.1.2 Structure de modules**
  - [ ] Créer `src/graphql/mod.rs`
  - [ ] Créer `src/graphql/schema.rs`
  - [ ] Créer `src/graphql/types.rs`
  - [ ] Créer `src/graphql/queries.rs`
  - [ ] Créer `src/graphql/mutations.rs`
  - [ ] Créer `src/graphql/subscriptions.rs`
  - [ ] Créer `src/graphql/context.rs`

- [ ] **1.1.3 Configuration de base**
  - [ ] Créer GraphQL schema root
  - [ ] Configurer Axum router pour `/graphql`
  - [ ] Ajouter GraphQL Playground à `/graphql/playground`
  - [ ] Configurer CORS pour frontend
  - [ ] Ajouter middleware de logging
  - [ ] Configurer max query depth (sécurité)
  - [ ] Ajouter rate limiting

### 1.2 Types GraphQL pour Cassettes
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **1.2.1 Types de base**
  - [ ] `CassetteType` (Object)
    - [ ] `id: ID!`
    - [ ] `name: String!`
    - [ ] `version: String!`
    - [ ] `recordedAt: DateTime!`
    - [ ] `interactions: [InteractionType!]!`
    - [ ] `metadata: CassetteMetadataType`
    - [ ] `stats: CassetteStatsType`

  - [ ] `InteractionType` (Object)
    - [ ] `id: ID!`
    - [ ] `kind: InteractionKind!` (enum: Http, WebSocket)
    - [ ] `request: HttpRequestType`
    - [ ] `response: HttpResponseType`
    - [ ] `websocketUrl: String`
    - [ ] `websocketMessages: [WebSocketMessageType!]`

  - [ ] `HttpRequestType` (Object)
    - [ ] `method: String!`
    - [ ] `url: String!`
    - [ ] `headers: JSON!`
    - [ ] `body: String`

  - [ ] `HttpResponseType` (Object)
    - [ ] `status: Int!`
    - [ ] `headers: JSON!`
    - [ ] `body: String`
    - [ ] `hasTemplates: Boolean!` (v0.4.0 feature!)

- [ ] **1.2.2 Types de métadonnées**
  - [ ] `CassetteMetadataType`
    - [ ] `createdAt: DateTime!`
    - [ ] `modifiedAt: DateTime!`
    - [ ] `sizeBytes: Int!`
    - [ ] `format: CassetteFormat!` (enum: Json, MessagePack)
    - [ ] `tags: [String!]`
    - [ ] `description: String`

  - [ ] `CassetteStatsType`
    - [ ] `interactionCount: Int!`
    - [ ] `httpCount: Int!`
    - [ ] `websocketCount: Int!`
    - [ ] `totalSizeBytes: Int!`

- [ ] **1.2.3 Enums**
  - [ ] `InteractionKind` (Http, WebSocket)
  - [ ] `CassetteFormat` (Json, MessagePack)
  - [ ] `SortBy` (Name, Size, Age, Interactions)
  - [ ] `SortOrder` (Asc, Desc)

### 1.3 Queries GraphQL
**Durée:** 1 jour
**Priorité:** 🟡 Important

- [ ] **1.3.1 Queries de lecture**
  - [ ] `cassette(name: String!): CassetteType`
    - [ ] Implémentation du resolver
    - [ ] Gestion erreur CassetteNotFound
    - [ ] Tests unitaires

  - [ ] `cassettes(filter: CassetteFilterInput, sort: SortInput): [CassetteType!]!`
    - [ ] Implémentation avec filtres
    - [ ] Tri multi-critères
    - [ ] Pagination (limit/offset)
    - [ ] Tests unitaires

  - [ ] `searchCassettes(query: String!): [CassetteType!]!`
    - [ ] Recherche full-text
    - [ ] Recherche dans nom/description/tags
    - [ ] Tests unitaires

  - [ ] `globalStats: GlobalStatsType`
    - [ ] Total cassettes
    - [ ] Total interactions
    - [ ] Taille totale
    - [ ] Tests unitaires

- [ ] **1.3.2 Types d'entrée (Input)**
  - [ ] `CassetteFilterInput`
    - [ ] `minAgeDays: Int`
    - [ ] `maxAgeDays: Int`
    - [ ] `minSizeBytes: Int`
    - [ ] `maxSizeBytes: Int`
    - [ ] `tags: [String!]`

  - [ ] `SortInput`
    - [ ] `sortBy: SortBy!`
    - [ ] `order: SortOrder!`

- [ ] **1.3.3 Documentation**
  - [ ] Ajouter descriptions GraphQL (docstrings)
  - [ ] Exemples de queries dans README
  - [ ] Créer fichier `graphql/examples.graphql`

## Semaine 2: Mutations & Templates

### 1.4 Mutations GraphQL
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **1.4.1 Mutations CRUD**
  - [ ] `createCassette(input: CreateCassetteInput!): CassetteType!`
    - [ ] Validation du nom (regex)
    - [ ] Création fichier JSON/MessagePack
    - [ ] Gestion erreurs (nom déjà existant)
    - [ ] Tests unitaires

  - [ ] `updateCassette(name: String!, input: UpdateCassetteInput!): CassetteType!`
    - [ ] Mise à jour métadonnées
    - [ ] Mise à jour interactions
    - [ ] Validation avant sauvegarde
    - [ ] Tests unitaires

  - [ ] `deleteCassette(name: String!): Boolean!`
    - [ ] Suppression sécurisée
    - [ ] Confirmation optionnelle
    - [ ] Tests unitaires

  - [ ] `duplicateCassette(name: String!, newName: String!): CassetteType!`
    - [ ] Copie complète
    - [ ] Nouveau timestamp
    - [ ] Tests unitaires

- [ ] **1.4.2 Mutations d'interactions**
  - [ ] `addInteraction(cassetteName: String!, interaction: InteractionInput!): InteractionType!`
  - [ ] `updateInteraction(cassetteName: String!, interactionId: ID!, interaction: InteractionInput!): InteractionType!`
  - [ ] `deleteInteraction(cassetteName: String!, interactionId: ID!): Boolean!`

- [ ] **1.4.3 Types d'entrée**
  - [ ] `CreateCassetteInput`
  - [ ] `UpdateCassetteInput`
  - [ ] `InteractionInput`

### 1.5 Templates GraphQL (v0.4.0 Integration)
**Durée:** 2 jours
**Priorité:** 🟡 Important

- [ ] **1.5.1 Queries templates**
  - [ ] `templateHelpers: [TemplateHelperType!]!`
    - [ ] Liste des helpers built-in (env, now, uuid...)
    - [ ] Documentation de chaque helper
    - [ ] Exemples d'utilisation

  - [ ] `validateTemplate(template: String!, sampleData: JSON): TemplateValidationResult!`
    - [ ] Validation syntaxe Handlebars
    - [ ] Test de rendu avec données exemple
    - [ ] Rapport d'erreurs détaillé

- [ ] **1.5.2 Mutations templates**
  - [ ] `applyTemplate(cassetteName: String!, interactionId: ID!, template: String!): InteractionType!`
    - [ ] Application template à une interaction
    - [ ] Preview avant application
    - [ ] Sauvegarde

  - [ ] `registerCustomHelper(name: String!, code: String!): Boolean!`
    - [ ] Enregistrement helper custom (sécurisé)
    - [ ] Validation du code
    - [ ] Tests de sécurité

- [ ] **1.5.3 Types**
  - [ ] `TemplateHelperType`
    - [ ] `name: String!`
    - [ ] `syntax: String!`
    - [ ] `description: String!`
    - [ ] `example: String!`

  - [ ] `TemplateValidationResult`
    - [ ] `valid: Boolean!`
    - [ ] `errors: [String!]`
    - [ ] `preview: String`

### 1.6 Subscriptions GraphQL (Temps Réel)
**Durée:** 2 jours
**Priorité:** 🟡 Important

- [ ] **1.6.1 Infrastructure WebSocket**
  - [ ] Configurer WebSocket dans Axum
  - [ ] Ajouter `graphql-ws` protocol
  - [ ] Gestion connexion/déconnexion
  - [ ] Heartbeat/ping-pong
  - [ ] Tests de reconnexion

- [ ] **1.6.2 Subscriptions**
  - [ ] `cassetteUpdated(name: String): CassetteType!`
    - [ ] Écoute modifications cassettes
    - [ ] Filtrage par nom (optionnel)
    - [ ] Tests temps réel

  - [ ] `proxyTraffic(cassetteName: String!): ProxyTrafficEvent!`
    - [ ] Flux temps réel du proxy
    - [ ] Requêtes/réponses live
    - [ ] Métriques (latency, taille)

  - [ ] `globalStatsUpdated: GlobalStatsType!`
    - [ ] Mise à jour stats globales
    - [ ] Intervalle configurable

- [ ] **1.6.3 Types d'événements**
  - [ ] `ProxyTrafficEvent`
    - [ ] `timestamp: DateTime!`
    - [ ] `request: HttpRequestType!`
    - [ ] `response: HttpResponseType`
    - [ ] `latencyMs: Int`
    - [ ] `matched: Boolean!`

## Semaine 3: Optimisations & Tests Backend

### 1.7 Cache & Performance
**Durée:** 2 jours
**Priorité:** 🟢 Souhaitable

- [ ] **1.7.1 Redis Integration**
  - [ ] Ajouter `redis = "0.24"`
  - [ ] Créer `src/cache/mod.rs`
  - [ ] Connexion pool Redis
  - [ ] Cache des métadonnées cassettes
  - [ ] Cache des stats globales
  - [ ] TTL configurable
  - [ ] Tests unitaires

- [ ] **1.7.2 DataLoader (N+1 queries)**
  - [ ] Ajouter `async-graphql-dataloader`
  - [ ] DataLoader pour interactions
  - [ ] Batching des requêtes
  - [ ] Tests de performance

- [ ] **1.7.3 Optimisations**
  - [ ] Lazy loading des interactions
  - [ ] Compression des réponses (gzip)
  - [ ] Pagination cursor-based
  - [ ] Index de recherche (optionnel: Meilisearch)

### 1.8 Tests Backend GraphQL
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **1.8.1 Tests unitaires**
  - [ ] Tests resolvers (queries)
  - [ ] Tests resolvers (mutations)
  - [ ] Tests resolvers (subscriptions)
  - [ ] Tests types GraphQL
  - [ ] Tests validation
  - [ ] Coverage > 80%

- [ ] **1.8.2 Tests d'intégration**
  - [ ] Tests E2E GraphQL queries
  - [ ] Tests E2E mutations
  - [ ] Tests WebSocket subscriptions
  - [ ] Tests avec cassettes réelles
  - [ ] Tests de performance (load testing)

- [ ] **1.8.3 Documentation**
  - [ ] Générer schema GraphQL (`schema.graphql`)
  - [ ] Documentation API (GraphQL Playground)
  - [ ] Exemples de queries/mutations
  - [ ] Guide de démarrage rapide

### 1.9 Migration REST → GraphQL
**Durée:** 1 jour
**Priorité:** 🟡 Important

- [ ] **1.9.1 Compatibilité**
  - [ ] Garder anciens endpoints REST (backward compat)
  - [ ] Ajouter header `X-API-Version: graphql`
  - [ ] Documentation de migration
  - [ ] Changelog avec breaking changes

- [ ] **1.9.2 Endpoints REST → GraphQL mapping**
  - [ ] `GET /cassettes` → `query { cassettes { ... } }`
  - [ ] `GET /cassettes/:name` → `query { cassette(name: "...") { ... } }`
  - [ ] `POST /cassettes` → `mutation { createCassette(...) { ... } }`
  - [ ] `DELETE /cassettes/:name` → `mutation { deleteCassette(name: "...") }`
  - [ ] Documentation de mapping

---

# Phase 2: Interface Web Moderne (3 semaines)

## Semaine 4: Setup Frontend

### 2.1 Initialisation React + Vite
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **2.1.1 Création projet**
  - [ ] `npm create vite@latest magneto-ui -- --template react-ts`
  - [ ] Structure de dossiers:
    ```
    src/
    ├── components/     # Composants réutilisables
    ├── pages/          # Pages principales
    ├── graphql/        # Requêtes GraphQL
    ├── hooks/          # Custom hooks
    ├── services/       # Services API
    ├── stores/         # State management
    ├── types/          # TypeScript types
    └── utils/          # Utilitaires
    ```

- [ ] **2.1.2 Dépendances principales**
  - [ ] `@apollo/client` (GraphQL client)
  - [ ] `graphql` (GraphQL core)
  - [ ] `@tanstack/react-query` (data fetching)
  - [ ] `react-router-dom` (routing)
  - [ ] `zustand` (state management)
  - [ ] `tailwindcss` (styling)
  - [ ] `shadcn/ui` (composants UI)

- [ ] **2.1.3 Configuration**
  - [ ] Configuration Vite (`vite.config.ts`)
  - [ ] Configuration TailwindCSS
  - [ ] Configuration ESLint/Prettier
  - [ ] Configuration TypeScript strict
  - [ ] Variables d'environnement (`.env`)

### 2.2 Apollo Client & GraphQL Setup
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **2.2.1 Apollo Client**
  - [ ] Créer `src/graphql/client.ts`
  - [ ] Configuration du client Apollo
  - [ ] WebSocket link pour subscriptions
  - [ ] HTTP link pour queries/mutations
  - [ ] Cache configuration
  - [ ] Error handling global
  - [ ] Retry policy

- [ ] **2.2.2 Code Generation**
  - [ ] Installer `@graphql-codegen/cli`
  - [ ] Configuration `codegen.yml`
  - [ ] Générer types TypeScript depuis schema
  - [ ] Générer hooks Apollo
  - [ ] Script npm pour codegen
  - [ ] CI/CD integration

- [ ] **2.2.3 Queries & Mutations**
  - [ ] Créer `src/graphql/queries/cassettes.graphql`
  - [ ] Créer `src/graphql/mutations/cassettes.graphql`
  - [ ] Créer `src/graphql/subscriptions/cassettes.graphql`
  - [ ] Générer hooks custom

### 2.3 Composants de Base
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **2.3.1 Layout**
  - [ ] `AppLayout` (sidebar + header + content)
  - [ ] `Sidebar` (navigation)
  - [ ] `Header` (breadcrumb + actions)
  - [ ] `Footer` (status + version)

- [ ] **2.3.2 Composants UI (shadcn/ui)**
  - [ ] Button
  - [ ] Card
  - [ ] Table
  - [ ] Dialog/Modal
  - [ ] Input/Textarea
  - [ ] Select/Dropdown
  - [ ] Tabs
  - [ ] Toast (notifications)
  - [ ] Badge
  - [ ] Spinner/Loader

- [ ] **2.3.3 Composants métier**
  - [ ] `CassetteCard` (carte cassette)
  - [ ] `CassetteList` (liste avec filtres)
  - [ ] `InteractionViewer` (affichage interaction)
  - [ ] `StatsWidget` (widget de stats)
  - [ ] `TemplateEditor` (éditeur template)

## Semaine 5: Pages Principales

### 2.4 Dashboard Page
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **2.4.1 Layout Dashboard**
  - [ ] Grille responsive (3 colonnes)
  - [ ] Widgets réorganisables (react-grid-layout)
  - [ ] Sauvegarde préférences layout

- [ ] **2.4.2 Widgets Stats**
  - [ ] Widget "Total Cassettes"
    - [ ] Nombre total
    - [ ] Évolution (+/- depuis hier)
    - [ ] Icône + couleur

  - [ ] Widget "Total Interactions"
    - [ ] HTTP vs WebSocket
    - [ ] Graphique donut

  - [ ] Widget "Storage Used"
    - [ ] Taille totale
    - [ ] Répartition par cassette (top 5)
    - [ ] Barre de progression

  - [ ] Widget "Recent Activity"
    - [ ] Liste 10 dernières cassettes modifiées
    - [ ] Timeline visuelle

- [ ] **2.4.3 Graphiques (recharts)**
  - [ ] Graphique ligne: évolution cassettes/jour (7 jours)
  - [ ] Graphique barres: répartition HTTP/WebSocket
  - [ ] Heatmap: activité par jour/heure
  - [ ] Tous les graphiques temps réel (subscriptions)

- [ ] **2.4.4 Temps Réel**
  - [ ] Subscription `globalStatsUpdated`
  - [ ] Mise à jour auto des widgets
  - [ ] Indicateur "Live" quand actif
  - [ ] Gestion reconnexion WebSocket

### 2.5 Cassettes List Page
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **2.5.1 Table Cassettes**
  - [ ] Colonnes: Nom, Size, Age, Interactions, Actions
  - [ ] Tri multi-colonnes
  - [ ] Sélection multiple (checkbox)
  - [ ] Actions groupées (delete, export)
  - [ ] Pagination (server-side)
  - [ ] Responsive (mobile: liste cards)

- [ ] **2.5.2 Filtres**
  - [ ] Filtre par nom (recherche)
  - [ ] Filtre par taille (min/max)
  - [ ] Filtre par âge (date picker range)
  - [ ] Filtre par nombre d'interactions
  - [ ] Filtre par tags
  - [ ] Sauvegarde filtres (localStorage)
  - [ ] Reset filtres

- [ ] **2.5.3 Actions**
  - [ ] View (navigation vers détail)
  - [ ] Edit (modal édition rapide)
  - [ ] Duplicate
  - [ ] Delete (confirmation)
  - [ ] Export (JSON/MessagePack/HAR)
  - [ ] Download

- [ ] **2.5.4 Bulk Actions**
  - [ ] Sélection toutes
  - [ ] Delete multiple (confirmation)
  - [ ] Export multiple (zip)
  - [ ] Tag multiple

### 2.6 Cassette Detail Page
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **2.6.1 Header Cassette**
  - [ ] Nom éditable (inline)
  - [ ] Métadonnées (version, date, taille)
  - [ ] Tags éditables (input tags)
  - [ ] Actions: Save, Delete, Duplicate, Export

- [ ] **2.6.2 Tabs**
  - [ ] Tab "Overview"
    - [ ] Statistiques cassette
    - [ ] Graphique répartition HTTP/WS
    - [ ] Liste interactions (preview)

  - [ ] Tab "Interactions"
    - [ ] Liste complète interactions
    - [ ] Filtres (HTTP/WebSocket)
    - [ ] Recherche dans body/headers
    - [ ] Expand/collapse chaque interaction

  - [ ] Tab "Settings"
    - [ ] Mode cassette (Auto/Record/Replay)
    - [ ] Options matching
    - [ ] Options de filtrage

  - [ ] Tab "Raw JSON"
    - [ ] Éditeur Monaco (read-only)
    - [ ] Syntax highlighting
    - [ ] Copy to clipboard

- [ ] **2.6.3 Interaction Viewer**
  - [ ] Affichage Request
    - [ ] Method + URL
    - [ ] Headers (table)
    - [ ] Body (formatted JSON/text)

  - [ ] Affichage Response
    - [ ] Status code (coloré)
    - [ ] Headers (table)
    - [ ] Body (formatted)
    - [ ] Badge "Has Templates" si templates

  - [ ] Actions interaction
    - [ ] Edit
    - [ ] Delete
    - [ ] Duplicate
    - [ ] Apply Template

## Semaine 6: Éditeur de Templates

### 2.7 Template Editor
**Durée:** 3 jours
**Priorité:** 🟡 Important

- [ ] **2.7.1 Monaco Editor Integration**
  - [ ] Installer `@monaco-editor/react`
  - [ ] Configuration Monaco pour Handlebars
  - [ ] Syntax highlighting custom
  - [ ] Autocompletion helpers
  - [ ] Validation en temps réel
  - [ ] Thème dark/light

- [ ] **2.7.2 Template Editor Component**
  - [ ] Layout: éditeur + preview (split horizontal)
  - [ ] Éditeur Monaco (gauche)
  - [ ] Preview rendu (droite)
  - [ ] Barre d'outils:
    - [ ] Insert Helper (dropdown)
    - [ ] Validate Template
    - [ ] Format (prettify)
    - [ ] Full Screen
    - [ ] Theme toggle

- [ ] **2.7.3 Helpers Palette**
  - [ ] Liste helpers built-in
  - [ ] Documentation inline
  - [ ] Click to insert
  - [ ] Exemples d'utilisation
  - [ ] Custom helpers (si configurés)

- [ ] **2.7.4 Live Preview**
  - [ ] Saisie données exemple (JSON)
  - [ ] Rendu temps réel (debounce 300ms)
  - [ ] Affichage erreurs
  - [ ] Highlight variables utilisées
  - [ ] Copy preview to clipboard

- [ ] **2.7.5 Template Library**
  - [ ] Bibliothèque templates pré-définis
  - [ ] Catégories: Auth, Webhooks, Timestamps, UUIDs
  - [ ] Import template
  - [ ] Save to library
  - [ ] Share (export JSON)

### 2.8 Template Application Workflow
**Durée:** 1 jour
**Priorité:** 🟡 Important

- [ ] **2.8.1 Apply Template Dialog**
  - [ ] Sélection interaction
  - [ ] Sélection template (library ou custom)
  - [ ] Preview avant/après
  - [ ] Confirmation
  - [ ] Mutation GraphQL `applyTemplate`

- [ ] **2.8.2 Batch Template Application**
  - [ ] Sélection multiple interactions
  - [ ] Application template à toutes
  - [ ] Progress bar
  - [ ] Rapport succès/erreurs

---

# Phase 3: Docker Stack Enrichie (1 semaine)

## Semaine 7: Infrastructure Docker

### 3.1 Docker Compose Stack Complète
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **3.1.1 Services**
  - [ ] `magneto-api` (GraphQL API - Rust)
    - [ ] Dockerfile multi-stage optimisé
    - [ ] Health check GraphQL
    - [ ] Logs structurés JSON
    - [ ] Métriques Prometheus

  - [ ] `magneto-ui` (Interface React - Vite)
    - [ ] Dockerfile Nginx
    - [ ] Build assets static
    - [ ] Gzip compression
    - [ ] Cache headers

  - [ ] `magneto-proxy` (Proxy HTTP/WS)
    - [ ] Mode standalone
    - [ ] Configuration via env vars
    - [ ] Auto-restart

  - [ ] `redis` (Cache + PubSub)
    - [ ] Redis 7 Alpine
    - [ ] Persistence AOF
    - [ ] Max memory policy

  - [ ] `postgres` (Metadata - optionnel)
    - [ ] PostgreSQL 16 Alpine
    - [ ] Init schema SQL
    - [ ] Backup automated

- [ ] **3.1.2 docker-compose.yml**
  ```yaml
  version: '3.9'

  services:
    magneto-api:
      build:
        context: .
        dockerfile: Dockerfile.api
      ports:
        - "8889:8889"
      environment:
        - RUST_LOG=info
        - REDIS_URL=redis://redis:6379
        - DATABASE_URL=postgres://...
      depends_on:
        - redis
        - postgres
      healthcheck:
        test: ["CMD", "curl", "-f", "http://localhost:8889/health"]
      volumes:
        - ./cassettes:/app/cassettes
      networks:
        - magneto-net

    magneto-ui:
      build:
        context: ./magneto-ui
        dockerfile: Dockerfile
      ports:
        - "4201:80"
      environment:
        - VITE_API_URL=http://localhost:8889/graphql
        - VITE_WS_URL=ws://localhost:8889/graphql
      depends_on:
        - magneto-api
      networks:
        - magneto-net

    redis:
      image: redis:7-alpine
      ports:
        - "6379:6379"
      volumes:
        - redis-data:/data
      command: redis-server --appendonly yes
      networks:
        - magneto-net

    postgres:
      image: postgres:16-alpine
      environment:
        POSTGRES_DB: magneto
        POSTGRES_USER: magneto
        POSTGRES_PASSWORD: secret
      volumes:
        - postgres-data:/var/lib/postgresql/data
        - ./scripts/init-db.sql:/docker-entrypoint-initdb.d/init.sql
      networks:
        - magneto-net

  volumes:
    redis-data:
    postgres-data:

  networks:
    magneto-net:
      driver: bridge
  ```

- [ ] **3.1.3 Dockerfiles**
  - [ ] `Dockerfile.api` (multi-stage Rust)
    - [ ] Stage 1: Builder (cargo build --release)
    - [ ] Stage 2: Runtime (alpine + binary)
    - [ ] Optimisations (strip, lto)
    - [ ] Taille < 50MB

  - [ ] `Dockerfile.ui` (Nginx)
    - [ ] Stage 1: Builder (npm run build)
    - [ ] Stage 2: Nginx Alpine
    - [ ] Gzip compression
    - [ ] Taille < 20MB

### 3.2 Orchestration & Monitoring
**Durée:** 2 jours
**Priorité:** 🟡 Important

- [ ] **3.2.1 Health Checks**
  - [ ] Health endpoint GraphQL (`/health`)
  - [ ] Health check Redis
  - [ ] Health check Postgres
  - [ ] Dashboard health (UI)

- [ ] **3.2.2 Logging**
  - [ ] Logs centralisés (Loki optionnel)
  - [ ] Format JSON structuré
  - [ ] Rotation logs
  - [ ] Filtrage niveau log

- [ ] **3.2.3 Métriques (Prometheus - optionnel)**
  - [ ] Metrics endpoint (`/metrics`)
  - [ ] Métriques GraphQL (queries/mutations/subscriptions)
  - [ ] Métriques proxy (requests/responses)
  - [ ] Métriques cache (hits/misses)
  - [ ] Dashboard Grafana (JSON)

- [ ] **3.2.4 Backup & Restore**
  - [ ] Script backup cassettes
  - [ ] Script backup Redis
  - [ ] Script backup Postgres
  - [ ] Restore procedure
  - [ ] Automated daily backup (cron)

### 3.3 Production Readiness
**Durée:** 1 jour
**Priorité:** 🟢 Souhaitable

- [ ] **3.3.1 Sécurité**
  - [ ] Secrets management (Docker secrets)
  - [ ] Non-root user dans containers
  - [ ] Read-only filesystem (où possible)
  - [ ] Network isolation
  - [ ] Rate limiting
  - [ ] HTTPS/TLS (Nginx reverse proxy)

- [ ] **3.3.2 Performance**
  - [ ] Resource limits (CPU/Memory)
  - [ ] Restart policy
  - [ ] Connection pooling
  - [ ] Cache tuning

- [ ] **3.3.3 Documentation**
  - [ ] `docs/DOCKER.md` (guide déploiement)
  - [ ] `docker-compose.prod.yml` (production)
  - [ ] `docker-compose.dev.yml` (développement)
  - [ ] Environment variables reference
  - [ ] Troubleshooting guide

---

# Phase 4: Intégration & Tests E2E (1 semaine)

## Semaine 8: Tests & Finalisation

### 4.1 Tests End-to-End
**Durée:** 3 jours
**Priorité:** 🔴 Critique

- [ ] **4.1.1 Setup Playwright**
  - [ ] Installer `@playwright/test`
  - [ ] Configuration `playwright.config.ts`
  - [ ] Scripts npm pour tests E2E
  - [ ] CI/CD integration

- [ ] **4.1.2 Tests utilisateur**
  - [ ] Test: Créer cassette via UI
  - [ ] Test: Éditer cassette
  - [ ] Test: Appliquer template
  - [ ] Test: Supprimer cassette
  - [ ] Test: Filtrer/trier cassettes
  - [ ] Test: Recherche cassettes
  - [ ] Test: Temps réel (subscriptions)
  - [ ] Test: Export cassettes

- [ ] **4.1.3 Tests d'intégration stack**
  - [ ] Test: API GraphQL + Redis cache
  - [ ] Test: WebSocket reconnection
  - [ ] Test: Performance (load test)
  - [ ] Test: Failover Redis
  - [ ] Test: Backup/restore

### 4.2 Documentation Complète
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **4.2.1 Documentation utilisateur**
  - [ ] `docs/USER-GUIDE.md`
    - [ ] Guide de démarrage rapide
    - [ ] Guide complet de l'interface
    - [ ] Guide des templates
    - [ ] FAQ

  - [ ] `docs/GRAPHQL-API.md`
    - [ ] Introduction GraphQL
    - [ ] Exemples queries/mutations
    - [ ] Subscriptions guide
    - [ ] Playground usage

- [ ] **4.2.2 Documentation développeur**
  - [ ] `docs/ARCHITECTURE-v0.5.0.md`
    - [ ] Diagramme architecture
    - [ ] Stack technique
    - [ ] Flow de données

  - [ ] `docs/CONTRIBUTING.md`
    - [ ] Setup développement
    - [ ] Guidelines code
    - [ ] Process PR

  - [ ] `docs/DEPLOYMENT.md`
    - [ ] Déploiement Docker
    - [ ] Déploiement Kubernetes (optionnel)
    - [ ] Configuration production

- [ ] **4.2.3 Vidéos/Screenshots**
  - [ ] Screenshots Dashboard
  - [ ] Screenshots Cassette Editor
  - [ ] Screenshots Template Editor
  - [ ] GIF animations features
  - [ ] Vidéo démo (3-5 min)

### 4.3 Release v0.5.0
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **4.3.1 Préparation release**
  - [ ] Bump version 0.5.0
  - [ ] Update CHANGELOG.md
  - [ ] Update README.md
  - [ ] Tag git v0.5.0
  - [ ] Build Docker images
  - [ ] Push Docker Hub

- [ ] **4.3.2 Publication**
  - [ ] Publish Rust crate (crates.io)
  - [ ] Publish npm package (UI)
  - [ ] GitHub Release notes
  - [ ] Announcement blog post
  - [ ] Tweet/social media

- [ ] **4.3.3 Hotfix preparedness**
  - [ ] Hotfix branch ready
  - [ ] Rollback procedure documented
  - [ ] Monitoring alerts configured

---

# 📊 Métriques de Succès

## KPIs Phase 1 (Backend GraphQL)
- [ ] API GraphQL opérationnelle
- [ ] 100% queries/mutations implémentées
- [ ] Subscriptions temps réel fonctionnelles
- [ ] Tests > 80% coverage
- [ ] Latency < 50ms (p95)
- [ ] Documentation complète

## KPIs Phase 2 (Interface Web)
- [ ] Dashboard responsive (desktop/mobile)
- [ ] Templates editor fonctionnel
- [ ] Live preview temps réel
- [ ] Lighthouse score > 90
- [ ] Accessibilité (WCAG AA)
- [ ] Cross-browser compatible

## KPIs Phase 3 (Docker)
- [ ] Stack complète déployable en 1 commande
- [ ] Images < 100MB total
- [ ] Startup time < 30s
- [ ] Health checks 100% OK
- [ ] Backup automatisé
- [ ] Documentation déploiement

## KPIs Phase 4 (Tests & Release)
- [ ] Tests E2E > 90% coverage
- [ ] Zero regression bugs
- [ ] Documentation complète
- [ ] Release notes publiées
- [ ] Community feedback > 4/5

---

# 🎯 Priorités

## Must Have (v0.5.0)
✅ GraphQL API complète
✅ Dashboard avec stats temps réel
✅ Liste cassettes avec filtres
✅ Template editor de base
✅ Docker Compose stack

## Should Have (v0.5.0)
⚠️ Redis cache
⚠️ Live preview templates
⚠️ Bulk actions
⚠️ Export multi-format
⚠️ Monitoring Prometheus

## Could Have (v0.5.1+)
❓ PostgreSQL metadata
❓ Template library
❓ Grafana dashboards
❓ Kubernetes operator
❓ Multi-tenant

---

# 🚀 Quick Start (Post v0.5.0)

```bash
# Clone repository
git clone https://github.com/taciclei/magneto-serge.git
cd magneto-serge

# Checkout v0.5.0
git checkout v0.5.0

# Start full stack
docker-compose up -d

# Access services
# - UI: http://localhost:4201
# - GraphQL Playground: http://localhost:8889/graphql
# - API Health: http://localhost:8889/health
```

---

**Roadmap créée le:** 2025-10-26
**Dernière mise à jour:** 2025-10-26
**Version:** 1.0
**Status:** 📋 Planning Phase

🤖 Generated with [Claude Code](https://claude.com/claude-code)
