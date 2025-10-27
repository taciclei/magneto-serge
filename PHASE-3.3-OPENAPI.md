# Phase 3.3: Documentation OpenAPI / Swagger UI

**Date:** À définir
**Durée estimée:** 3-5 jours
**Statut:** 📋 Planification
**Priorité:** 🟡 Moyenne

---

## 📋 Objectif

Générer automatiquement une spécification OpenAPI 3.0 complète et l'exposer via Swagger UI pour une documentation interactive de l'API.

**Bénéfices:**
- Documentation toujours à jour (générée depuis le code)
- Interface interactive pour tester l'API
- Génération automatique de clients (via openapi-generator)
- Standard industriel (OpenAPI 3.0)

---

## 🎯 User Stories

### US-3.3.1: Accéder à la Spécification OpenAPI
**En tant que** développeur
**Je veux** accéder à la spécification OpenAPI en JSON
**Afin de** générer des clients ou valider l'API

**Critères d'acceptation:**
- [ ] GET /openapi.json retourne spec OpenAPI 3.0 valide
- [ ] Tous les endpoints documentés
- [ ] Schémas de requêtes et réponses inclus
- [ ] Exemples de payloads présents

### US-3.3.2: Utiliser Swagger UI
**En tant qu'** utilisateur
**Je veux** une interface Swagger UI interactive
**Afin de** tester l'API sans écrire de code

**Critères d'acceptation:**
- [ ] GET /swagger-ui affiche Swagger UI
- [ ] Tous les endpoints listés et testables
- [ ] "Try it out" fonctionne pour chaque endpoint
- [ ] Authentification supportée (si API key configurée)

### US-3.3.3: Accéder à Swagger UI depuis Frontend
**En tant qu'** utilisateur du frontend
**Je veux** un lien vers la documentation API
**Afin d'** accéder rapidement à Swagger UI

**Critères d'acceptation:**
- [ ] Menu navigation contient lien "API Docs"
- [ ] Click ouvre Swagger UI dans nouvel onglet
- [ ] Icône documentation visible

---

## 🏗️ Architecture Backend

### Nouveaux Endpoints

#### GET /openapi.json
**Retourne la spécification OpenAPI 3.0**

```http
GET /openapi.json HTTP/1.1
Host: localhost:8889
```

**Réponse 200 OK:**
```json
{
  "openapi": "3.0.3",
  "info": {
    "title": "Magnéto-Serge Hydra API",
    "version": "0.6.0",
    "description": "RESTful Hypermedia API for managing HTTP/WebSocket test cassettes",
    "license": {
      "name": "MIT OR Apache-2.0",
      "url": "https://github.com/taciclei/magneto-serge/blob/main/LICENSE"
    }
  },
  "servers": [
    {
      "url": "http://localhost:8889",
      "description": "Local development server"
    }
  ],
  "paths": {
    "/api": { ... },
    "/api/cassettes": { ... },
    "/api/cassettes/{name}": { ... },
    ...
  },
  "components": {
    "schemas": {
      "CassetteResource": { ... },
      "HydraCollection": { ... },
      ...
    }
  }
}
```

#### GET /swagger-ui
**Interface Swagger UI interactive**

```http
GET /swagger-ui HTTP/1.1
Host: localhost:8889
```

**Réponse 200 OK:**
```html
<!DOCTYPE html>
<html>
  <head>
    <title>Magnéto-Serge API</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist/swagger-ui.css">
  </head>
  <body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist/swagger-ui-bundle.js"></script>
    <script>
      SwaggerUIBundle({
        url: '/openapi.json',
        dom_id: '#swagger-ui'
      });
    </script>
  </body>
</html>
```

---

## 💻 Implémentation Backend

### Dépendances à Ajouter

**`Cargo.toml`:**

```toml
[dependencies]
utoipa = { version = "5.1", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "8.0", features = ["axum"] }
```

### Fichiers à Modifier

```
src/api/
├── mod.rs                    # Générer spec OpenAPI
├── routes.rs                 # Ajouter routes Swagger
└── handlers/                 # Annoter handlers avec utoipa
    ├── root.rs
    ├── cassettes.rs
    ├── cassette.rs
    ├── cassette_create.rs
    ├── cassette_delete.rs
    └── interactions.rs
```

### Code Backend - Génération Spec OpenAPI

**`src/api/mod.rs` (additions):**

```rust
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Magnéto-Serge Hydra API",
        version = "0.6.0",
        description = "RESTful Hypermedia API for managing HTTP/WebSocket test cassettes",
        license(
            name = "MIT OR Apache-2.0",
            url = "https://github.com/taciclei/magneto-serge/blob/main/LICENSE"
        ),
        contact(
            name = "Magnéto-Serge Team",
            email = "contact@taciclei.com"
        )
    ),
    servers(
        (url = "http://localhost:8889", description = "Local development"),
        (url = "https://api.magneto-serge.com", description = "Production (future)")
    ),
    paths(
        handlers::root::get_api_root,
        handlers::cassettes::get_cassettes,
        handlers::cassette::get_cassette,
        handlers::cassette_create::create_cassette,
        handlers::cassette_delete::delete_cassette,
        handlers::interactions::get_interactions,
    ),
    components(
        schemas(
            models::hydra::HydraCollection,
            models::cassette_resource::CassetteResource,
            models::interaction_resource::InteractionResource,
            handlers::cassette_create::CreateCassetteRequest,
        )
    ),
    tags(
        (name = "Cassettes", description = "Cassette management endpoints"),
        (name = "Interactions", description = "Interaction retrieval endpoints"),
        (name = "Hydra", description = "Hydra hypermedia endpoints")
    )
)]
pub struct ApiDoc;

pub fn openapi_spec() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
```

**`src/api/routes.rs` (additions):**

```rust
use utoipa_swagger_ui::SwaggerUi;
use crate::api::openapi_spec;

pub fn create_routes(state: Arc<ApiState>) -> Router {
    Router::new()
        // Routes API existantes
        .route("/api", get(handlers::root::get_api_root))
        .route("/api/cassettes", get(handlers::cassettes::get_cassettes))
        .route("/api/cassettes", post(handlers::cassette_create::create_cassette))
        // ...

        // OpenAPI endpoints (NOUVEAUX)
        .route("/openapi.json", get(serve_openapi_spec))
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", openapi_spec()))

        .with_state(state)
}

async fn serve_openapi_spec() -> Json<utoipa::openapi::OpenApi> {
    Json(openapi_spec())
}
```

### Code Backend - Annoter Handlers

**`src/api/handlers/cassette_create.rs` (additions):**

```rust
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCassetteRequest {
    /// Unique name for the cassette (alphanumeric, hyphens, underscores)
    #[schema(example = "my-api-test")]
    pub name: String,

    /// Optional description of the cassette purpose
    #[schema(example = "Test cassette for user service API")]
    pub description: Option<String>,

    /// Proxy mode: auto, record, or replay
    #[schema(example = "auto")]
    pub mode: Option<String>,
}

/// Create a new cassette
#[utoipa::path(
    post,
    path = "/api/cassettes",
    tag = "Cassettes",
    request_body = CreateCassetteRequest,
    responses(
        (status = 201, description = "Cassette created successfully", body = CassetteResource,
         example = json!({
             "@context": "https://www.w3.org/ns/hydra/core",
             "@type": "CassetteResource",
             "@id": "http://localhost:8889/api/cassettes/my-api-test",
             "name": "my-api-test",
             "description": "Test cassette for user service API",
             "recorded_at": "2025-10-27T10:00:00Z",
             "interactions_count": 0
         })),
        (status = 400, description = "Invalid request (bad cassette name)"),
        (status = 409, description = "Cassette already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_cassette(
    State(state): State<Arc<ApiState>>,
    Json(payload): Json<CreateCassetteRequest>,
) -> Result<(StatusCode, Json<CassetteResource>), (StatusCode, String)> {
    // ... implémentation existante ...
}
```

**`src/api/handlers/cassette_delete.rs` (additions):**

```rust
/// Delete an existing cassette
#[utoipa::path(
    delete,
    path = "/api/cassettes/{name}",
    tag = "Cassettes",
    params(
        ("name" = String, Path, description = "Cassette name", example = "my-api-test")
    ),
    responses(
        (status = 204, description = "Cassette deleted successfully"),
        (status = 404, description = "Cassette not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_cassette(
    State(state): State<Arc<ApiState>>,
    Path(name): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    // ... implémentation existante ...
}
```

**`src/api/models/cassette_resource.rs` (additions):**

```rust
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CassetteResource {
    /// JSON-LD context
    #[serde(rename = "@context")]
    #[schema(example = "https://www.w3.org/ns/hydra/core")]
    pub context: String,

    /// Resource type
    #[serde(rename = "@type")]
    #[schema(example = "CassetteResource")]
    pub type_: String,

    /// Resource unique identifier (IRI)
    #[serde(rename = "@id")]
    #[schema(example = "http://localhost:8889/api/cassettes/my-cassette")]
    pub id: String,

    /// Cassette name
    #[schema(example = "my-cassette")]
    pub name: String,

    /// Cassette description
    #[schema(example = "Test cassette for API")]
    pub description: Option<String>,

    /// Timestamp when cassette was recorded
    #[schema(example = "2025-10-27T10:00:00Z")]
    pub recorded_at: String,

    /// Number of interactions in cassette
    #[schema(example = 5)]
    pub interactions_count: usize,

    /// Hydra links for navigation
    #[serde(rename = "hydra:link")]
    pub links: Vec<HydraLink>,
}
```

---

## 🅰️ Implémentation Frontend

### Code Frontend - Lien Menu Navigation

**`app.component.html` (modifications):**

```html
<mat-toolbar color="primary">
  <span>Magnéto-Serge</span>

  <span class="spacer"></span>

  <button mat-button routerLink="/cassettes">
    <mat-icon>video_library</mat-icon>
    Cassettes
  </button>

  <!-- NOUVEAU -->
  <button mat-button (click)="openApiDocs()">
    <mat-icon>description</mat-icon>
    API Docs
  </button>

  <button mat-icon-button [matMenuTriggerFor]="menu">
    <mat-icon>more_vert</mat-icon>
  </button>

  <mat-menu #menu="matMenu">
    <a mat-menu-item href="http://localhost:8889/swagger-ui" target="_blank">
      <mat-icon>api</mat-icon>
      <span>Swagger UI</span>
    </a>
    <a mat-menu-item href="http://localhost:8889/openapi.json" target="_blank">
      <mat-icon>code</mat-icon>
      <span>OpenAPI Spec (JSON)</span>
    </a>
  </mat-menu>
</mat-toolbar>
```

**`app.component.ts` (additions):**

```typescript
export class AppComponent {
  openApiDocs(): void {
    window.open('http://localhost:8889/swagger-ui', '_blank');
  }
}
```

---

## 🧪 Tests

### Tests Backend

**`tests/api/openapi_test.rs`:**

```rust
#[tokio::test]
async fn test_openapi_spec() {
    let app = test_app().await;

    let response = app.get("/openapi.json").await;
    assert_eq!(response.status(), StatusCode::OK);

    let spec: serde_json::Value = response.json().await;

    // Vérifier version OpenAPI
    assert_eq!(spec["openapi"], "3.0.3");

    // Vérifier info
    assert_eq!(spec["info"]["title"], "Magnéto-Serge Hydra API");
    assert_eq!(spec["info"]["version"], "0.6.0");

    // Vérifier que tous les paths sont présents
    assert!(spec["paths"]["/api"].is_object());
    assert!(spec["paths"]["/api/cassettes"].is_object());
    assert!(spec["paths"]["/api/cassettes/{name}"].is_object());
}

#[tokio::test]
async fn test_swagger_ui_accessible() {
    let app = test_app().await;

    let response = app.get("/swagger-ui/").await;
    assert_eq!(response.status(), StatusCode::OK);

    let html = response.text().await;
    assert!(html.contains("swagger-ui"));
    assert!(html.contains("/openapi.json"));
}

#[tokio::test]
async fn test_all_endpoints_documented() {
    let spec = openapi_spec();

    // Vérifier que tous les endpoints critiques sont documentés
    let paths = spec.paths.paths;

    assert!(paths.contains_key("/api"));
    assert!(paths.contains_key("/api/cassettes"));
    assert!(paths.contains_key("/api/cassettes/{name}"));
    assert!(paths.contains_key("/api/cassettes/{name}/interactions"));

    // Vérifier méthodes HTTP
    let cassettes_path = paths.get("/api/cassettes").unwrap();
    assert!(cassettes_path.get.is_some()); // GET
    assert!(cassettes_path.post.is_some()); // POST (Phase 3.0)

    let cassette_path = paths.get("/api/cassettes/{name}").unwrap();
    assert!(cassette_path.get.is_some()); // GET
    assert!(cassette_path.delete.is_some()); // DELETE (Phase 3.0)
}
```

### Validation Spec OpenAPI

```bash
# Installer openapi-generator
npm install -g @openapitools/openapi-generator-cli

# Valider spec
openapi-generator-cli validate -i http://localhost:8889/openapi.json

# Générer client TypeScript (exemple)
openapi-generator-cli generate \
  -i http://localhost:8889/openapi.json \
  -g typescript-axios \
  -o ./generated-client
```

---

## ✅ Critères d'Achèvement

### Backend
- [ ] GET /openapi.json retourne spec OpenAPI 3.0 valide
- [ ] GET /swagger-ui affiche interface Swagger UI
- [ ] Tous les endpoints annotés avec #[utoipa::path]
- [ ] Tous les schemas annotés avec #[derive(ToSchema)]
- [ ] Exemples de requêtes/réponses présents
- [ ] Validation spec passe (openapi-generator validate)
- [ ] Tests OpenAPI passent (5+ tests)

### Frontend
- [ ] Lien "API Docs" dans menu navigation
- [ ] Click ouvre Swagger UI dans nouvel onglet
- [ ] Menu déroulant avec liens OpenAPI JSON et Swagger UI

### Documentation
- [ ] README.md mis à jour avec lien Swagger UI
- [ ] Guide d'utilisation Swagger UI créé
- [ ] Exemples de génération de clients documentés

---

## 📚 Ressources

- **utoipa Docs**: https://docs.rs/utoipa/latest/utoipa/
- **OpenAPI 3.0 Spec**: https://spec.openapis.org/oas/v3.0.3
- **Swagger UI**: https://swagger.io/tools/swagger-ui/
- **OpenAPI Generator**: https://openapi-generator.tech/

---

## 🎯 Bonus: Génération de Clients

Une fois la spec OpenAPI disponible, générer des clients pour différents langages:

```bash
# TypeScript (Axios)
openapi-generator-cli generate \
  -i http://localhost:8889/openapi.json \
  -g typescript-axios \
  -o ./clients/typescript

# Python
openapi-generator-cli generate \
  -i http://localhost:8889/openapi.json \
  -g python \
  -o ./clients/python

# Java
openapi-generator-cli generate \
  -i http://localhost:8889/openapi.json \
  -g java \
  -o ./clients/java

# Go
openapi-generator-cli generate \
  -i http://localhost:8889/openapi.json \
  -g go \
  -o ./clients/go
```

---

**Auteur:** Claude Code + Équipe Magnéto-Serge
**Date:** 2025-10-26
