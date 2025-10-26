# Roadmap v0.5.0: Hypermedia API (Hydra/JSON-LD) + Angular

**Version:** 0.5.0
**Architecture:** HATEOAS + Hydra + JSON-LD
**Frontend:** Angular 17+ avec Heracles.ts / Alcaeus
**Date de début:** 2025-10-26
**Durée estimée:** 6-8 semaines
**Status:** 🚧 En cours - Phase 1 Week 2 Complétée
**Dernière mise à jour:** 2025-10-26

---

## 🎯 Vision: API Hypermedia Complète

Créer une **API RESTful auto-descriptive** suivant les principes HATEOAS avec:
- **Hydra**: Vocabulaire pour décrire les APIs RESTful
- **JSON-LD**: Format de données liées (Linked Data)
- **Angular + Alcaeus**: Frontend automatiquement découvrable
- **Auto-génération**: Documentation OpenAPI + Hydra spec

### Avantages de l'Hypermedia
✅ **API auto-découvrable** - Le client explore l'API via les liens
✅ **Évolutivité** - Ajout de fonctionnalités sans casser les clients
✅ **Documentation vivante** - L'API se documente elle-même
✅ **Découplage** - Frontend indépendant de la structure backend
✅ **Standards W3C** - JSON-LD, Hydra Core Vocabulary

---

## 📚 Stack Technique

### Backend (Rust)
- **Framework:** Axum (async HTTP)
- **Hydra:** Implémentation custom avec macros Rust
- **JSON-LD:** `serde_json` + context builder
- **Vocabulaire:** Hydra Core Vocabulary (W3C)
- **Documentation:** Auto-génération ApiDocumentation

### Frontend (Angular 17+)
- **Framework:** Angular 17 (standalone components)
- **Hypermedia Client:** **Alcaeus** (https://github.com/wikibus/Alcaeus)
  - Parsing automatique Hydra
  - Navigation via liens
  - Support JSON-LD
  - Cache intelligent
- **Alternative:** **Heracles.ts** (TypeScript Hydra client)
- **State Management:** NgRx + Hydra resources
- **UI:** Angular Material + TailwindCSS

### Outils & Standards
- **JSON-LD Context:** http://www.w3.org/ns/hydra/context.jsonld
- **Vocabulaire:** Hydra Core (http://www.w3.org/ns/hydra/core)
- **RDF:** Support optionnel (N-Triples, Turtle)
- **OpenAPI:** Génération depuis Hydra spec

---

# Phase 1: Backend Hypermedia API (3 semaines)

## ✅ Semaine 1: Infrastructure Hydra Core (COMPLÉTÉE)

### ✅ 1.1 Setup Hydra en Rust
**Durée:** 3 jours
**Priorité:** 🔴 Critique
**Status:** ✅ Complété le 2025-10-26

- [x] **1.1.1 Dépendances Cargo**
  - [x] Créer feature `hydra` dans Cargo.toml
  ```toml
  [dependencies]
  serde = { version = "1.0", features = ["derive"] }
  serde_json = { version = "1.0", features = ["preserve_order"] }
  axum = { version = "0.7", features = ["json"] }
  tower = "0.4"
  tower-http = { version = "0.5", features = ["cors", "trace"] }
  url = "2.5"

  [features]
  hydra = []
  ```

- [x] **1.1.2 Structure de modules**
  ```
  src/hydra/
  ├── mod.rs              # Module root ✅
  ├── context.rs          # JSON-LD Context ✅
  ├── vocabulary.rs       # Hydra vocabulary ✅
  ├── response.rs         # HydraResponse builder ✅
  ├── collection.rs       # HydraCollection (pagination) ✅
  ├── operation.rs        # HydraOperation (CRUD) ✅
  ├── documentation.rs    # ApiDocumentation generator ✅
  ├── error.rs            # HydraError ✅
  └── resources/          # Resource representations ✅
      ├── mod.rs
      ├── cassette.rs
      ├── interaction.rs
      └── template.rs
  ```

- [x] **1.1.3 Hydra Vocabulary Types**
  - [x] `HydraClass` (représente une classe de ressources)
  - [x] `HydraLink` (lien hypermedia)
  - [x] `HydraOperation` (opération CRUD)
  - [x] `HydraCollection` (collection paginée)
  - [x] `HydraView` (pagination view)
  - [x] `HydraError` (erreur structurée)
  - [x] `ApiDocumentation` (documentation auto-générée)

### ✅ 1.2 JSON-LD Context Builder
**Durée:** 2 jours
**Priorité:** 🔴 Critique
**Status:** ✅ Complété le 2025-10-26

- [x] **1.2.1 Context Generator**
  - [x] Créer `src/hydra/context.rs`
  ```rust
  pub struct JsonLdContext {
      base_url: String,
      vocab_url: String,
      mappings: HashMap<String, String>,
  }

  impl JsonLdContext {
      pub fn new(base_url: &str) -> Self {
          Self {
              base_url: base_url.to_string(),
              vocab_url: "http://www.w3.org/ns/hydra/core#".to_string(),
              mappings: Self::default_mappings(),
          }
      }

      fn default_mappings() -> HashMap<String, String> {
          let mut map = HashMap::new();
          map.insert("hydra".to_string(), "http://www.w3.org/ns/hydra/core#".to_string());
          map.insert("rdf".to_string(), "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string());
          map.insert("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string());
          map.insert("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string());
          map.insert("schema".to_string(), "http://schema.org/".to_string());
          // Magneto-specific
          map.insert("cassette".to_string(), format!("{}/vocab#Cassette", base_url));
          map.insert("interaction".to_string(), format!("{}/vocab#Interaction", base_url));
          map
      }

      pub fn to_json(&self) -> serde_json::Value {
          json!({
              "@context": self.mappings
          })
      }
  }
  ```

- [ ] **1.2.2 Vocabulaire Magneto**
  - [ ] Définir ontologie Magneto
  ```json
  {
    "@context": {
      "hydra": "http://www.w3.org/ns/hydra/core#",
      "magneto": "http://magneto-serge.dev/vocab#",
      "Cassette": "magneto:Cassette",
      "Interaction": "magneto:Interaction",
      "name": "schema:name",
      "version": "schema:version",
      "recordedAt": "schema:dateCreated",
      "interactions": {
        "@id": "magneto:interactions",
        "@type": "@id"
      }
    }
  }
  ```

- [ ] **1.2.3 Endpoint `/vocab`**
  - [ ] Servir vocabulaire Magneto
  - [ ] Support Content Negotiation (JSON-LD, Turtle, RDF/XML)
  - [ ] Cache header (immutable)

### 1.3 Hydra Response Builder
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **1.3.1 HydraResponse Struct**
  ```rust
  #[derive(Debug, Serialize)]
  pub struct HydraResponse<T> {
      #[serde(rename = "@context")]
      pub context: String,

      #[serde(rename = "@id")]
      pub id: String,

      #[serde(rename = "@type")]
      pub type_: String,

      #[serde(flatten)]
      pub data: T,

      #[serde(rename = "hydra:operation", skip_serializing_if = "Vec::is_empty")]
      pub operations: Vec<HydraOperation>,

      #[serde(rename = "hydra:view", skip_serializing_if = "Option::is_none")]
      pub view: Option<HydraView>,
  }

  impl<T> HydraResponse<T> {
      pub fn new(id: &str, type_: &str, data: T) -> Self {
          Self {
              context: "/vocab".to_string(),
              id: id.to_string(),
              type_: type_.to_string(),
              data,
              operations: Vec::new(),
              view: None,
          }
      }

      pub fn with_operations(mut self, ops: Vec<HydraOperation>) -> Self {
          self.operations = ops;
          self
      }
  }
  ```

- [ ] **1.3.2 HydraOperation**
  ```rust
  #[derive(Debug, Serialize)]
  pub struct HydraOperation {
      #[serde(rename = "@type")]
      pub type_: String, // "hydra:Operation"

      #[serde(rename = "hydra:method")]
      pub method: String, // "GET", "POST", "PUT", "DELETE"

      #[serde(rename = "hydra:expects", skip_serializing_if = "Option::is_none")]
      pub expects: Option<String>,

      #[serde(rename = "hydra:returns", skip_serializing_if = "Option::is_none")]
      pub returns: Option<String>,

      #[serde(rename = "hydra:title")]
      pub title: String,

      #[serde(rename = "hydra:description", skip_serializing_if = "Option::is_none")]
      pub description: Option<String>,
  }
  ```

- [ ] **1.3.3 Builder Macros**
  ```rust
  macro_rules! hydra_response {
      ($id:expr, $type:expr, $data:expr) => {
          HydraResponse::new($id, $type, $data)
      };
  }

  macro_rules! hydra_operation {
      (GET $title:expr => $returns:expr) => {
          HydraOperation {
              type_: "hydra:Operation".to_string(),
              method: "GET".to_string(),
              expects: None,
              returns: Some($returns.to_string()),
              title: $title.to_string(),
              description: None,
          }
      };
      (POST $title:expr, $expects:expr => $returns:expr) => {
          HydraOperation {
              type_: "hydra:Operation".to_string(),
              method: "POST".to_string(),
              expects: Some($expects.to_string()),
              returns: Some($returns.to_string()),
              title: $title.to_string(),
              description: None,
          }
      };
      // DELETE, PUT, PATCH variants...
  }
  ```

## ✅ Semaine 2: Ressources Hypermedia (COMPLÉTÉE)

### ✅ 1.4 Cassette Resource (Hydra)
**Durée:** 2 jours
**Priorité:** 🔴 Critique
**Status:** ✅ Complété le 2025-10-26

- [x] **1.4.1 CassetteResource Type**
  ```rust
  #[derive(Debug, Serialize, Deserialize)]
  pub struct CassetteResource {
      pub name: String,
      pub version: String,

      #[serde(rename = "recordedAt")]
      pub recorded_at: String,

      #[serde(rename = "interactionCount")]
      pub interaction_count: usize,

      #[serde(rename = "sizeBytes")]
      pub size_bytes: u64,

      // Hypermedia links
      #[serde(rename = "_links")]
      pub links: CassetteLinks,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct CassetteLinks {
      #[serde(rename = "self")]
      pub self_link: Link,

      pub interactions: Link,

      pub edit: Link,

      pub delete: Link,

      pub duplicate: Link,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct Link {
      pub href: String,

      #[serde(skip_serializing_if = "Option::is_none")]
      pub templated: Option<bool>,

      #[serde(skip_serializing_if = "Option::is_none")]
      pub title: Option<String>,
  }
  ```

- [ ] **1.4.2 Endpoints Cassette**
  - [ ] `GET /api/cassettes` (HydraCollection)
    ```json
    {
      "@context": "/vocab",
      "@id": "/api/cassettes",
      "@type": "hydra:Collection",
      "hydra:totalItems": 42,
      "hydra:member": [
        {
          "@id": "/api/cassettes/my-cassette",
          "@type": "Cassette",
          "name": "my-cassette",
          "version": "1.0",
          "recordedAt": "2025-10-26T10:00:00Z",
          "interactionCount": 5,
          "sizeBytes": 12345,
          "_links": {
            "self": { "href": "/api/cassettes/my-cassette" },
            "interactions": { "href": "/api/cassettes/my-cassette/interactions" },
            "edit": { "href": "/api/cassettes/my-cassette" },
            "delete": { "href": "/api/cassettes/my-cassette" }
          }
        }
      ],
      "hydra:view": {
        "@id": "/api/cassettes?page=1",
        "@type": "hydra:PartialCollectionView",
        "hydra:first": "/api/cassettes?page=1",
        "hydra:next": "/api/cassettes?page=2",
        "hydra:last": "/api/cassettes?page=10"
      },
      "hydra:search": {
        "@type": "hydra:IriTemplate",
        "hydra:template": "/api/cassettes{?name,minSize,maxSize}",
        "hydra:mapping": [
          {
            "@type": "hydra:IriTemplateMapping",
            "hydra:variable": "name",
            "hydra:property": "schema:name",
            "hydra:required": false
          }
        ]
      }
    }
    ```

  - [ ] `GET /api/cassettes/{name}` (Cassette unique)
  - [ ] `POST /api/cassettes` (Création)
  - [ ] `PUT /api/cassettes/{name}` (Mise à jour)
  - [ ] `DELETE /api/cassettes/{name}` (Suppression)

- [ ] **1.4.3 Operations Auto-Discovery**
  - [ ] Chaque ressource inclut `hydra:operation`
  - [ ] Client découvre automatiquement les actions possibles
  - [ ] Exemple:
  ```json
  {
    "@id": "/api/cassettes/test",
    "hydra:operation": [
      {
        "@type": "hydra:Operation",
        "hydra:method": "GET",
        "hydra:returns": "Cassette",
        "hydra:title": "Retrieve cassette"
      },
      {
        "@type": "hydra:Operation",
        "hydra:method": "PUT",
        "hydra:expects": "CassetteInput",
        "hydra:returns": "Cassette",
        "hydra:title": "Update cassette"
      },
      {
        "@type": "hydra:Operation",
        "hydra:method": "DELETE",
        "hydra:returns": "null",
        "hydra:title": "Delete cassette"
      }
    ]
  }
  ```

### ✅ 1.5 HydraCollection (Pagination)
**Durée:** 2 jours
**Priorité:** 🔴 Critique
**Status:** ✅ Complété le 2025-10-26 (implémenté dans src/hydra/collection.rs)

- [ ] **1.5.1 HydraCollection Struct**
  ```rust
  #[derive(Debug, Serialize)]
  pub struct HydraCollection<T> {
      #[serde(rename = "@context")]
      pub context: String,

      #[serde(rename = "@id")]
      pub id: String,

      #[serde(rename = "@type")]
      pub type_: String, // "hydra:Collection"

      #[serde(rename = "hydra:totalItems")]
      pub total_items: usize,

      #[serde(rename = "hydra:member")]
      pub members: Vec<T>,

      #[serde(rename = "hydra:view", skip_serializing_if = "Option::is_none")]
      pub view: Option<HydraView>,

      #[serde(rename = "hydra:search", skip_serializing_if = "Option::is_none")]
      pub search: Option<HydraSearch>,
  }
  ```

- [ ] **1.5.2 HydraView (Pagination)**
  ```rust
  #[derive(Debug, Serialize)]
  pub struct HydraView {
      #[serde(rename = "@id")]
      pub id: String, // Current page URL

      #[serde(rename = "@type")]
      pub type_: String, // "hydra:PartialCollectionView"

      #[serde(rename = "hydra:first")]
      pub first: String,

      #[serde(rename = "hydra:previous", skip_serializing_if = "Option::is_none")]
      pub previous: Option<String>,

      #[serde(rename = "hydra:next", skip_serializing_if = "Option::is_none")]
      pub next: Option<String>,

      #[serde(rename = "hydra:last")]
      pub last: String,
  }

  impl HydraView {
      pub fn paginate(
          base_url: &str,
          current_page: usize,
          total_pages: usize,
      ) -> Self {
          Self {
              id: format!("{}?page={}", base_url, current_page),
              type_: "hydra:PartialCollectionView".to_string(),
              first: format!("{}?page=1", base_url),
              previous: if current_page > 1 {
                  Some(format!("{}?page={}", base_url, current_page - 1))
              } else {
                  None
              },
              next: if current_page < total_pages {
                  Some(format!("{}?page={}", base_url, current_page + 1))
              } else {
                  None
              },
              last: format!("{}?page={}", base_url, total_pages),
          }
      }
  }
  ```

- [ ] **1.5.3 HydraSearch (IRI Template)**
  ```rust
  #[derive(Debug, Serialize)]
  pub struct HydraSearch {
      #[serde(rename = "@type")]
      pub type_: String, // "hydra:IriTemplate"

      #[serde(rename = "hydra:template")]
      pub template: String, // "/api/cassettes{?name,size}"

      #[serde(rename = "hydra:mapping")]
      pub mappings: Vec<HydraMapping>,
  }

  #[derive(Debug, Serialize)]
  pub struct HydraMapping {
      #[serde(rename = "@type")]
      pub type_: String, // "hydra:IriTemplateMapping"

      #[serde(rename = "hydra:variable")]
      pub variable: String, // "name"

      #[serde(rename = "hydra:property")]
      pub property: String, // "schema:name"

      #[serde(rename = "hydra:required")]
      pub required: bool,
  }
  ```

### ✅ 1.6 ApiDocumentation Generator
**Durée:** 2 jours
**Priorité:** 🟡 Important
**Status:** ✅ Complété le 2025-10-26 (implémenté dans src/hydra/documentation.rs)

- [ ] **1.6.1 Endpoint `/api` (Entrypoint)**
  ```rust
  pub async fn api_entrypoint() -> Json<ApiDocumentation> {
      Json(ApiDocumentation {
          context: "/vocab".to_string(),
          id: "/api".to_string(),
          type_: "hydra:ApiDocumentation".to_string(),
          title: "Magneto-Serge Hypermedia API".to_string(),
          description: Some("RESTful API for cassette management".to_string()),
          entrypoint: "/api".to_string(),
          supported_classes: vec![
              // Cassette class
              // Interaction class
              // etc.
          ],
      })
  }
  ```

- [ ] **1.6.2 SupportedClass**
  ```rust
  #[derive(Debug, Serialize)]
  pub struct SupportedClass {
      #[serde(rename = "@id")]
      pub id: String, // "Cassette"

      #[serde(rename = "@type")]
      pub type_: String, // "hydra:Class"

      #[serde(rename = "hydra:title")]
      pub title: String,

      #[serde(rename = "hydra:description", skip_serializing_if = "Option::is_none")]
      pub description: Option<String>,

      #[serde(rename = "hydra:supportedProperty")]
      pub supported_properties: Vec<SupportedProperty>,

      #[serde(rename = "hydra:supportedOperation")]
      pub supported_operations: Vec<HydraOperation>,
  }
  ```

- [ ] **1.6.3 Auto-génération depuis Rust**
  - [ ] Macro derive pour auto-générer SupportedClass
  ```rust
  #[derive(HydraClass)]
  #[hydra(title = "Cassette", description = "Recording of HTTP/WebSocket traffic")]
  pub struct Cassette {
      #[hydra(property = "schema:name", required = true)]
      pub name: String,

      #[hydra(property = "schema:version")]
      pub version: String,

      // ...
  }
  ```

## 🚧 Semaine 3: Intégration Axum + Endpoints HTTP (EN COURS)

### ✅ 1.7 Templates Resource
**Durée:** 2 jours
**Priorité:** 🟡 Important
**Status:** ✅ Complété le 2025-10-26 (implémenté dans src/hydra/resources/template.rs)

- [x] **1.7.1 TemplateResource**
  ```rust
  #[derive(Debug, Serialize, Deserialize)]
  pub struct TemplateResource {
      #[serde(rename = "@id")]
      pub id: String,

      #[serde(rename = "@type")]
      pub type_: String, // "Template"

      pub name: String,

      pub syntax: String, // Handlebars template

      pub description: Option<String>,

      pub category: String, // "auth", "webhook", "timestamp"

      #[serde(rename = "builtInHelpers")]
      pub built_in_helpers: Vec<TemplateHelper>,

      #[serde(rename = "_links")]
      pub links: TemplateLinks,
  }
  ```

- [ ] **1.7.2 Endpoints Templates**
  - [ ] `GET /api/templates` (Collection)
  - [ ] `GET /api/templates/{name}` (Template unique)
  - [ ] `POST /api/templates/validate` (Validation)
  - [ ] `POST /api/cassettes/{name}/interactions/{id}/apply-template`

- [ ] **1.7.3 Template Helpers Resource**
  ```json
  {
    "@context": "/vocab",
    "@id": "/api/templates/helpers",
    "@type": "hydra:Collection",
    "hydra:member": [
      {
        "@id": "/api/templates/helpers/env",
        "@type": "TemplateHelper",
        "name": "env",
        "syntax": "{{ env \"VAR_NAME\" }}",
        "description": "Environment variable substitution",
        "example": "{{ env \"API_KEY\" }}",
        "output": "sk-test-1234"
      }
    ]
  }
  ```

### 1.8 Error Handling (Hydra)
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **1.8.1 HydraError**
  ```rust
  #[derive(Debug, Serialize)]
  pub struct HydraError {
      #[serde(rename = "@context")]
      pub context: String,

      #[serde(rename = "@type")]
      pub type_: String, // "hydra:Error"

      #[serde(rename = "hydra:title")]
      pub title: String,

      #[serde(rename = "hydra:description")]
      pub description: String,

      #[serde(rename = "hydra:statusCode")]
      pub status_code: u16,
  }

  impl IntoResponse for HydraError {
      fn into_response(self) -> Response {
          let status = StatusCode::from_u16(self.status_code)
              .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
          (status, Json(self)).into_response()
      }
  }
  ```

- [ ] **1.8.2 Error Types**
  - [ ] `CassetteNotFound` → 404 Hydra Error
  - [ ] `ValidationError` → 422 Hydra Error
  - [ ] `UnauthorizedError` → 401 Hydra Error

### 1.9 Content Negotiation
**Durée:** 1 jour
**Priorité:** 🟢 Souhaitable

- [ ] **1.9.1 Support Multi-Format**
  - [ ] `Accept: application/ld+json` → JSON-LD (défaut)
  - [ ] `Accept: application/json` → JSON simple
  - [ ] `Accept: text/turtle` → Turtle (RDF)
  - [ ] `Accept: application/n-triples` → N-Triples

- [ ] **1.9.2 RDF Serialization (optionnel)**
  - [ ] Ajouter `sophia = "0.8"` (RDF toolkit)
  - [ ] Conversion JSON-LD → Turtle
  - [ ] Conversion JSON-LD → N-Triples

---

# Phase 2: Frontend Angular + Alcaeus (3 semaines)

## Semaine 4: Setup Angular

### 2.1 Projet Angular 17+
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **2.1.1 Création projet**
  ```bash
  ng new magneto-ui --standalone --routing --style=scss --ssr=false
  cd magneto-ui
  ```

- [ ] **2.1.2 Structure**
  ```
  src/
  ├── app/
  │   ├── core/                 # Services core
  │   │   ├── hypermedia/       # Alcaeus client
  │   │   ├── api/              # API service
  │   │   └── state/            # State management
  │   ├── features/             # Feature modules
  │   │   ├── cassettes/
  │   │   ├── templates/
  │   │   └── dashboard/
  │   ├── shared/               # Shared components
  │   │   ├── components/
  │   │   ├── directives/
  │   │   └── pipes/
  │   └── app.component.ts
  └── assets/
  ```

- [ ] **2.1.3 Dépendances**
  ```bash
  # Hypermedia client
  npm install @wikibus/alcaeus
  npm install @rdfjs/types

  # State management
  npm install @ngrx/store @ngrx/effects @ngrx/entity

  # UI
  npm install @angular/material
  npm install tailwindcss

  # HTTP
  npm install @angular/common/http
  ```

### 2.2 Alcaeus Client Setup
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **2.2.1 Alcaeus Service**
  ```typescript
  // src/app/core/hypermedia/alcaeus.service.ts
  import { Injectable } from '@angular/core';
  import Alcaeus from '@wikibus/alcaeus';
  import { environment } from '../../../environments/environment';

  @Injectable({ providedIn: 'root' })
  export class AlcaeusService {
    private client = Alcaeus.withDefaults();

    constructor() {
      // Configure base URL
      this.client.baseUri = environment.apiUrl;
    }

    async loadResource<T>(url: string): Promise<T> {
      const representation = await this.client.loadResource(url);
      return representation.root as unknown as T;
    }

    async followLink<T>(resource: any, rel: string): Promise<T> {
      const link = resource._links?.[rel];
      if (!link) {
        throw new Error(`Link ${rel} not found`);
      }
      return this.loadResource<T>(link.href);
    }

    async submitOperation(operation: any, data?: any): Promise<any> {
      return this.client.invokeOperation(operation, data);
    }
  }
  ```

- [ ] **2.2.2 Resource Models**
  ```typescript
  // src/app/core/models/cassette.model.ts
  export interface CassetteResource {
    '@id': string;
    '@type': string;
    name: string;
    version: string;
    recordedAt: string;
    interactionCount: number;
    sizeBytes: number;
    _links: {
      self: Link;
      interactions: Link;
      edit: Link;
      delete: Link;
    };
    'hydra:operation'?: HydraOperation[];
  }

  export interface Link {
    href: string;
    templated?: boolean;
    title?: string;
  }

  export interface HydraOperation {
    '@type': string;
    'hydra:method': string;
    'hydra:expects'?: string;
    'hydra:returns'?: string;
    'hydra:title': string;
  }
  ```

- [ ] **2.2.3 API Entrypoint Discovery**
  ```typescript
  // src/app/core/hypermedia/api-entrypoint.service.ts
  import { Injectable } from '@angular/core';
  import { AlcaeusService } from './alcaeus.service';

  @Injectable({ providedIn: 'root' })
  export class ApiEntrypointService {
    private entrypoint: any;

    constructor(private alcaeus: AlcaeusService) {}

    async loadEntrypoint(): Promise<void> {
      this.entrypoint = await this.alcaeus.loadResource('/api');
    }

    async getCassettesCollection(): Promise<any> {
      return this.alcaeus.followLink(this.entrypoint, 'cassettes');
    }

    async getTemplatesCollection(): Promise<any> {
      return this.alcaeus.followLink(this.entrypoint, 'templates');
    }
  }
  ```

### 2.3 NgRx State Management
**Durée:** 2 jours
**Priorité:** 🟡 Important

- [ ] **2.3.1 Store Structure**
  ```typescript
  // src/app/core/state/app.state.ts
  export interface AppState {
    cassettes: CassettesState;
    templates: TemplatesState;
    ui: UiState;
  }

  export interface CassettesState {
    entities: { [id: string]: CassetteResource };
    ids: string[];
    selectedId: string | null;
    loading: boolean;
    error: string | null;
  }
  ```

- [ ] **2.3.2 Actions**
  ```typescript
  // src/app/core/state/cassettes/cassettes.actions.ts
  import { createAction, props } from '@ngrx/store';

  export const loadCassettes = createAction('[Cassettes] Load');
  export const loadCassettesSuccess = createAction(
    '[Cassettes] Load Success',
    props<{ cassettes: CassetteResource[] }>()
  );
  export const loadCassettesFailure = createAction(
    '[Cassettes] Load Failure',
    props<{ error: string }>()
  );
  ```

- [ ] **2.3.3 Effects (Hypermedia-driven)**
  ```typescript
  // src/app/core/state/cassettes/cassettes.effects.ts
  import { Injectable } from '@angular/core';
  import { Actions, createEffect, ofType } from '@ngrx/effects';
  import { AlcaeusService } from '../../hypermedia/alcaeus.service';
  import * as CassettesActions from './cassettes.actions';
  import { catchError, map, switchMap } from 'rxjs/operators';
  import { of } from 'rxjs';

  @Injectable()
  export class CassettesEffects {
    loadCassettes$ = createEffect(() =>
      this.actions$.pipe(
        ofType(CassettesActions.loadCassettes),
        switchMap(() =>
          from(this.alcaeus.loadResource<HydraCollection>('/api/cassettes')).pipe(
            map(collection =>
              CassettesActions.loadCassettesSuccess({
                cassettes: collection['hydra:member']
              })
            ),
            catchError(error =>
              of(CassettesActions.loadCassettesFailure({ error: error.message }))
            )
          )
        )
      )
    );

    constructor(
      private actions$: Actions,
      private alcaeus: AlcaeusService
    ) {}
  }
  ```

## Semaine 5: Composants Hypermedia

### 2.4 Cassettes List Component
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **2.4.1 Component**
  ```typescript
  // src/app/features/cassettes/cassettes-list.component.ts
  import { Component, OnInit } from '@angular/core';
  import { Store } from '@ngrx/store';
  import { Observable } from 'rxjs';
  import { CassetteResource } from '../../core/models/cassette.model';
  import * as CassettesActions from '../../core/state/cassettes/cassettes.actions';
  import { selectAllCassettes } from '../../core/state/cassettes/cassettes.selectors';

  @Component({
    selector: 'app-cassettes-list',
    standalone: true,
    templateUrl: './cassettes-list.component.html'
  })
  export class CassettesListComponent implements OnInit {
    cassettes$: Observable<CassetteResource[]>;

    constructor(private store: Store) {
      this.cassettes$ = this.store.select(selectAllCassettes);
    }

    ngOnInit(): void {
      this.store.dispatch(CassettesActions.loadCassettes());
    }

    onDelete(cassette: CassetteResource): void {
      // Récupère l'opération DELETE depuis hydra:operation
      const deleteOp = cassette['hydra:operation']?.find(
        op => op['hydra:method'] === 'DELETE'
      );

      if (deleteOp) {
        this.store.dispatch(CassettesActions.deleteCassette({
          cassette,
          operation: deleteOp
        }));
      }
    }

    onEdit(cassette: CassetteResource): void {
      // Navigation via link hypermedia
      const editLink = cassette._links.edit;
      this.router.navigate([editLink.href]);
    }
  }
  ```

- [ ] **2.4.2 Template HTML**
  ```html
  <!-- cassettes-list.component.html -->
  <div class="cassettes-container">
    <h1>Cassettes</h1>

    <mat-table [dataSource]="cassettes$ | async">
      <!-- Colonnes -->
      <ng-container matColumnDef="name">
        <mat-header-cell *matHeaderCellDef>Name</mat-header-cell>
        <mat-cell *matCellDef="let cassette">{{ cassette.name }}</mat-cell>
      </ng-container>

      <!-- Actions dynamiques basées sur hydra:operation -->
      <ng-container matColumnDef="actions">
        <mat-header-cell *matHeaderCellDef>Actions</mat-header-cell>
        <mat-cell *matCellDef="let cassette">
          <button
            *ngFor="let op of cassette['hydra:operation']"
            [matTooltip]="op['hydra:title']"
            (click)="handleOperation(cassette, op)">
            {{ op['hydra:method'] }}
          </button>
        </mat-cell>
      </ng-container>
    </mat-table>
  </div>
  ```

- [ ] **2.4.3 Pagination Hypermedia**
  ```typescript
  // Pagination automatique via hydra:view
  export class CassettesListComponent {
    currentView: HydraView | null = null;

    async loadPage(url: string): Promise<void> {
      const collection = await this.alcaeus.loadResource<HydraCollection>(url);
      this.cassettes = collection['hydra:member'];
      this.currentView = collection['hydra:view'];
    }

    nextPage(): void {
      if (this.currentView?.['hydra:next']) {
        this.loadPage(this.currentView['hydra:next']);
      }
    }

    previousPage(): void {
      if (this.currentView?.['hydra:previous']) {
        this.loadPage(this.currentView['hydra:previous']);
      }
    }
  }
  ```

### 2.5 Dynamic Operations Handler
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **2.5.1 OperationsService**
  ```typescript
  // src/app/core/hypermedia/operations.service.ts
  import { Injectable } from '@angular/core';
  import { AlcaeusService } from './alcaeus.service';
  import { HydraOperation } from '../models/cassette.model';

  @Injectable({ providedIn: 'root' })
  export class OperationsService {
    constructor(private alcaeus: AlcaeusService) {}

    async invokeOperation(
      resource: any,
      operation: HydraOperation,
      payload?: any
    ): Promise<any> {
      const method = operation['hydra:method'];
      const resourceId = resource['@id'];

      switch (method) {
        case 'GET':
          return this.alcaeus.loadResource(resourceId);

        case 'POST':
        case 'PUT':
        case 'PATCH':
          return this.alcaeus.submitOperation(operation, payload);

        case 'DELETE':
          return this.delete(resourceId);

        default:
          throw new Error(`Unsupported method: ${method}`);
      }
    }

    private async delete(url: string): Promise<void> {
      await fetch(url, { method: 'DELETE' });
    }
  }
  ```

- [ ] **2.5.2 Dynamic Form Generator**
  ```typescript
  // Génération de formulaire basée sur hydra:expects
  export class DynamicFormService {
    generateForm(operation: HydraOperation): FormGroup {
      const expects = operation['hydra:expects'];

      // Récupère la classe attendue depuis ApiDocumentation
      const classDefinition = this.getClassDefinition(expects);

      // Génère FormGroup depuis supportedProperty
      const controls = {};
      for (const prop of classDefinition['hydra:supportedProperty']) {
        controls[prop['hydra:property']] = new FormControl('',
          prop['hydra:required'] ? Validators.required : []
        );
      }

      return new FormGroup(controls);
    }
  }
  ```

### 2.6 Template Editor Component
**Durée:** 2 jours
**Priorité:** 🟡 Important

- [ ] **2.6.1 Template Editor avec Monaco**
  ```typescript
  // src/app/features/templates/template-editor.component.ts
  import { Component, OnInit } from '@angular/core';
  import { AlcaeusService } from '../../core/hypermedia/alcaeus.service';

  @Component({
    selector: 'app-template-editor',
    standalone: true,
    template: `
      <div class="editor-container">
        <div class="helpers-sidebar">
          <h3>Template Helpers</h3>
          <div *ngFor="let helper of helpers">
            <button (click)="insertHelper(helper)">
              {{ helper.name }}
            </button>
            <p>{{ helper.description }}</p>
          </div>
        </div>

        <div class="editor">
          <ngx-monaco-editor
            [options]="editorOptions"
            [(ngModel)]="templateCode">
          </ngx-monaco-editor>
        </div>

        <div class="preview">
          <h3>Preview</h3>
          <pre>{{ preview }}</pre>
        </div>
      </div>
    `
  })
  export class TemplateEditorComponent implements OnInit {
    helpers: any[] = [];
    templateCode = '';
    preview = '';

    constructor(private alcaeus: AlcaeusService) {}

    async ngOnInit(): Promise<void> {
      // Charge les helpers via Hypermedia
      const helpersCollection = await this.alcaeus.loadResource('/api/templates/helpers');
      this.helpers = helpersCollection['hydra:member'];
    }

    insertHelper(helper: any): void {
      this.templateCode += helper.syntax;
    }

    async validateTemplate(): Promise<void> {
      // Utilise l'opération de validation
      const result = await this.alcaeus.submitOperation({
        '@type': 'hydra:Operation',
        'hydra:method': 'POST',
        // ...
      }, {
        template: this.templateCode
      });

      this.preview = result.preview;
    }
  }
  ```

## Semaine 6: Features Avancées

### 2.7 Search & Filters (Hydra IRI Template)
**Durée:** 2 jours
**Priorité:** 🟡 Important

- [ ] **2.7.1 Search Component**
  ```typescript
  // Utilise hydra:search pour construire l'URL de recherche
  export class SearchComponent {
    search Template: any;

    async ngOnInit(): Promise<void> {
      const collection = await this.alcaeus.loadResource('/api/cassettes');
      this.searchTemplate = collection['hydra:search'];
    }

    buildSearchUrl(params: any): string {
      // Parse le template: "/api/cassettes{?name,minSize,maxSize}"
      let url = this.searchTemplate['hydra:template'];

      for (const mapping of this.searchTemplate['hydra:mapping']) {
        const variable = mapping['hydra:variable'];
        const value = params[variable];

        if (value) {
          url = url.replace(`{?${variable}}`, `?${variable}=${value}`);
        }
      }

      return url;
    }

    async search(filters: any): Promise<void> {
      const url = this.buildSearchUrl(filters);
      const results = await this.alcaeus.loadResource(url);
      // Update state
    }
  }
  ```

### 2.8 Real-time Updates (SSE)
**Durée:** 2 jours
**Priorité:** 🟢 Souhaitable

- [ ] **2.8.1 Server-Sent Events**
  ```typescript
  // Backend: Endpoint SSE
  pub async fn cassette_events() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
      let stream = /* événements */;
      Sse::new(stream)
  }

  // Frontend: EventSource
  export class RealtimeService {
    private eventSource: EventSource;

    connect(): void {
      this.eventSource = new EventSource('/api/events');

      this.eventSource.addEventListener('cassette-updated', (event) => {
        const cassette = JSON.parse(event.data);
        this.store.dispatch(CassettesActions.cassetteUpdated({ cassette }));
      });
    }
  }
  ```

### 2.9 Tests Angular
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **2.9.1 Unit Tests**
  - [ ] Tests services Alcaeus
  - [ ] Tests composants
  - [ ] Tests state management

- [ ] **2.9.2 E2E Tests (Playwright)**
  - [ ] Test navigation hypermedia
  - [ ] Test opérations dynamiques
  - [ ] Test recherche via IRI Template

---

# Phase 3: Intégration & Documentation (2 semaines)

## Semaine 7-8: Finalisation

### 3.1 Docker Stack
**Durée:** 2 jours
**Priorité:** 🔴 Critique

- [ ] **3.1.1 docker-compose.yml**
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
        - API_BASE_URL=http://localhost:8889
      volumes:
        - ./cassettes:/app/cassettes

    magneto-ui:
      build:
        context: ./magneto-ui
      ports:
        - "4201:80"
      environment:
        - API_URL=http://localhost:8889/api
      depends_on:
        - magneto-api

    nginx:
      image: nginx:alpine
      ports:
        - "80:80"
      volumes:
        - ./nginx.conf:/etc/nginx/nginx.conf
      depends_on:
        - magneto-api
        - magneto-ui
  ```

### 3.2 Documentation Complète
**Durée:** 3 jours
**Priorité:** 🔴 Critique

- [ ] **3.2.1 Guide API Hypermedia**
  - [ ] Introduction HATEOAS/Hydra
  - [ ] Exemples navigation
  - [ ] Guide Alcaeus
  - [ ] Référence vocabulaire

- [ ] **3.2.2 Guide Développeur**
  - [ ] Setup développement
  - [ ] Architecture Angular
  - [ ] Tests

### 3.3 Release v0.5.0
**Durée:** 1 jour
**Priorité:** 🔴 Critique

- [ ] **3.3.1 Publication**
  - [ ] Bump version
  - [ ] CHANGELOG
  - [ ] Release GitHub
  - [ ] Docker Hub
  - [ ] Annonce

---

# 📊 Checklist Complète

## Backend Hypermedia API
- [ ] Hydra Core implementation
- [ ] JSON-LD Context
- [ ] HydraCollection avec pagination
- [ ] ApiDocumentation auto-générée
- [ ] Content Negotiation
- [ ] Tests > 80% coverage

## Frontend Angular
- [ ] Alcaeus client intégré
- [ ] Navigation hypermedia
- [ ] Opérations dynamiques
- [ ] Search via IRI Template
- [ ] Template editor
- [ ] Tests E2E

## Docker & Déploiement
- [ ] Docker Compose stack
- [ ] Documentation déploiement
- [ ] Monitoring
- [ ] Backup

## Documentation
- [ ] Guide utilisateur
- [ ] Référence API
- [ ] Guide développeur
- [ ] Exemples

---

**Ready to start!** 🚀

Voulez-vous que je commence par Phase 1 (Backend Hydra) ? 🎯
