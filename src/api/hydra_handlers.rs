//! Handlers Hydra pour l'API hypermedia
//!
//! Ce module implémente les endpoints HTTP suivant les principes HATEOAS avec Hydra.
//! Chaque endpoint retourne des ressources auto-descriptives avec liens de navigation.

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::sync::Arc;

#[cfg(feature = "hydra")]
use crate::hydra::{
    ApiDocumentation, CassetteResource, HydraCollection, JsonLdContext, SupportedClass,
    TemplateResource,
};

use crate::api::CassetteManager;

/// État partagé pour les handlers Hydra
#[derive(Clone)]
pub struct HydraState {
    /// Gestionnaire de cassettes
    pub cassette_manager: Arc<CassetteManager>,
    /// URL de base pour l'API
    pub base_url: String,
}

impl HydraState {
    pub fn new(cassette_manager: Arc<CassetteManager>, base_url: impl Into<String>) -> Self {
        Self {
            cassette_manager,
            base_url: base_url.into(),
        }
    }
}

/// Content negotiation: extrait le type de contenu demandé depuis les headers Accept
fn get_content_type(headers: &HeaderMap) -> ContentType {
    if let Some(accept) = headers.get("accept") {
        if let Ok(accept_str) = accept.to_str() {
            if accept_str.contains("application/ld+json") {
                return ContentType::JsonLd;
            } else if accept_str.contains("text/turtle") {
                return ContentType::Turtle;
            } else if accept_str.contains("application/rdf+xml") {
                return ContentType::RdfXml;
            }
        }
    }
    ContentType::Json // Par défaut
}

/// Types de contenu supportés
#[derive(Debug, Clone, Copy, PartialEq)]
enum ContentType {
    Json,
    JsonLd,
    Turtle,
    RdfXml,
}

impl ContentType {
    fn mime_type(&self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::JsonLd => "application/ld+json",
            ContentType::Turtle => "text/turtle",
            ContentType::RdfXml => "application/rdf+xml",
        }
    }
}

/// GET /api
///
/// Point d'entrée principal de l'API Hydra.
/// Retourne l'ApiDocumentation décrivant toutes les ressources et opérations disponibles.
///
/// # Exemple de réponse
///
/// ```json
/// {
///   "@context": "http://www.w3.org/ns/hydra/context.jsonld",
///   "@id": "http://localhost:8889/api",
///   "@type": "hydra:ApiDocumentation",
///   "hydra:title": "Magneto-Serge Hypermedia API",
///   "hydra:description": "API for HTTP/WebSocket cassette recording and replay",
///   "hydra:entrypoint": "http://localhost:8889/api",
///   "hydra:supportedClass": [...]
/// }
/// ```
#[cfg(feature = "hydra")]
pub async fn api_entrypoint(State(state): State<HydraState>, headers: HeaderMap) -> Response {
    let content_type = get_content_type(&headers);

    let doc = ApiDocumentation {
        context: "http://www.w3.org/ns/hydra/context.jsonld".to_string(),
        id: format!("{}/api", state.base_url),
        type_: "hydra:ApiDocumentation".to_string(),
        title: "Magneto-Serge Hypermedia API".to_string(),
        description: Some(
            "REST API for HTTP/WebSocket cassette recording and replay with Hydra support"
                .to_string(),
        ),
        entrypoint: format!("{}/api", state.base_url),
        supported_classes: vec![
            SupportedClass::new(
                "Cassette",
                "Collection of recorded HTTP/WebSocket interactions",
            ),
            SupportedClass::new(
                "Interaction",
                "Single HTTP request/response or WebSocket message exchange",
            ),
            SupportedClass::new(
                "Template",
                "Handlebars template for dynamic response generation",
            ),
        ],
    };

    match content_type {
        ContentType::JsonLd | ContentType::Json => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, content_type.mime_type())],
            Json(doc),
        )
            .into_response(),
        ContentType::Turtle => {
            // TODO: Implémenter sérialisation Turtle
            (
                StatusCode::NOT_IMPLEMENTED,
                "Turtle format not yet implemented",
            )
                .into_response()
        }
        ContentType::RdfXml => {
            // TODO: Implémenter sérialisation RDF/XML
            (
                StatusCode::NOT_IMPLEMENTED,
                "RDF/XML format not yet implemented",
            )
                .into_response()
        }
    }
}

/// GET /api/cassettes
///
/// Liste toutes les cassettes disponibles sous forme de HydraCollection paginée.
///
/// # Query Parameters
/// - `page` (optionnel): Numéro de page (défaut: 1)
/// - `limit` (optionnel): Nombre d'éléments par page (défaut: 20)
///
/// # Exemple de réponse
///
/// ```json
/// {
///   "@context": {...},
///   "@type": "hydra:Collection",
///   "hydra:totalItems": 42,
///   "hydra:member": [
///     {
///       "@id": "/api/cassettes/auth-test",
///       "@type": "Cassette",
///       "name": "auth-test",
///       "interactionCount": 5,
///       "_links": {...}
///     }
///   ],
///   "hydra:view": {...}
/// }
/// ```
#[cfg(feature = "hydra")]
pub async fn list_cassettes(State(state): State<HydraState>, headers: HeaderMap) -> Response {
    let content_type = get_content_type(&headers);

    // Charger toutes les cassettes
    let cassettes = match state.cassette_manager.list_cassettes() {
        Ok(metadata_list) => {
            let mut resources = Vec::new();
            for metadata in metadata_list {
                if let Ok(cassette) = state.cassette_manager.load_cassette(&metadata.name) {
                    let resource = CassetteResource::from_cassette(&cassette, &state.base_url);
                    resources.push(resource);
                }
            }
            resources
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to list cassettes: {}", e),
            )
                .into_response();
        }
    };

    let collection_id = format!("{}/api/cassettes", state.base_url);
    let total = cassettes.len();
    let collection = HydraCollection::new(&collection_id, cassettes, total);

    match content_type {
        ContentType::JsonLd | ContentType::Json => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, content_type.mime_type())],
            Json(collection),
        )
            .into_response(),
        _ => (
            StatusCode::NOT_IMPLEMENTED,
            "Only JSON-LD and JSON are currently supported",
        )
            .into_response(),
    }
}

/// GET /api/cassettes/{name}
///
/// Récupère une cassette spécifique avec ses métadonnées et liens Hydra.
///
/// # Path Parameters
/// - `name`: Nom de la cassette
///
/// # Exemple de réponse
///
/// ```json
/// {
///   "@id": "/api/cassettes/auth-test",
///   "@type": "Cassette",
///   "name": "auth-test",
///   "version": "1.0",
///   "recordedAt": "2025-10-26T10:00:00Z",
///   "interactionCount": 5,
///   "sizeBytes": 12345,
///   "_links": {
///     "self": {"href": "/api/cassettes/auth-test"},
///     "interactions": {"href": "/api/cassettes/auth-test/interactions"},
///     "edit": {"href": "/api/cassettes/auth-test"},
///     "delete": {"href": "/api/cassettes/auth-test"}
///   }
/// }
/// ```
#[cfg(feature = "hydra")]
pub async fn get_cassette(
    State(state): State<HydraState>,
    Path(name): Path<String>,
    headers: HeaderMap,
) -> Response {
    let content_type = get_content_type(&headers);

    let cassette = match state.cassette_manager.load_cassette(&name) {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                format!("Cassette '{}' not found", name),
            )
                .into_response();
        }
    };

    let resource = CassetteResource::from_cassette(&cassette, &state.base_url);

    match content_type {
        ContentType::JsonLd | ContentType::Json => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, content_type.mime_type())],
            Json(resource),
        )
            .into_response(),
        _ => (
            StatusCode::NOT_IMPLEMENTED,
            "Only JSON-LD and JSON are currently supported",
        )
            .into_response(),
    }
}

/// GET /api/templates
///
/// Liste tous les templates disponibles (helpers Handlebars intégrés).
///
/// # Exemple de réponse
///
/// ```json
/// {
///   "@context": {...},
///   "@type": "hydra:Collection",
///   "hydra:member": [
///     {
///       "@id": "/api/templates/env",
///       "name": "env",
///       "syntax": "{{ env \"VAR_NAME\" }}",
///       "description": "Environment variable substitution",
///       "_links": {...}
///     }
///   ]
/// }
/// ```
#[cfg(feature = "hydra")]
pub async fn list_templates(State(state): State<HydraState>, headers: HeaderMap) -> Response {
    let content_type = get_content_type(&headers);

    // Créer une ressource template avec tous les helpers
    let helpers = TemplateResource::built_in_helpers();

    // Convertir en collection simple pour l'instant
    let context = JsonLdContext::new(&state.base_url);

    let response = serde_json::json!({
        "@context": context.to_json(),
        "@type": "hydra:Collection",
        "hydra:totalItems": helpers.len(),
        "hydra:member": helpers.iter().map(|h| {
            serde_json::json!({
                "@id": format!("{}/api/templates/helpers/{}", state.base_url, h.name),
                "@type": "TemplateHelper",
                "name": h.name,
                "syntax": h.syntax,
                "description": h.description,
                "example": h.example,
                "outputExample": h.output_example,
            })
        }).collect::<Vec<_>>(),
    });

    match content_type {
        ContentType::JsonLd | ContentType::Json => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, content_type.mime_type())],
            Json(response),
        )
            .into_response(),
        _ => (
            StatusCode::NOT_IMPLEMENTED,
            "Only JSON-LD and JSON are currently supported",
        )
            .into_response(),
    }
}

/// GET /vocab
///
/// Retourne le vocabulaire Magneto-Serge (ontologie RDF).
///
/// # Exemple de réponse
///
/// ```json
/// {
///   "@context": "http://www.w3.org/ns/rdfs#",
///   "@graph": [
///     {
///       "@id": "magneto:Cassette",
///       "@type": "rdfs:Class",
///       "rdfs:label": "Cassette",
///       "rdfs:comment": "A collection of recorded HTTP/WebSocket interactions"
///     }
///   ]
/// }
/// ```
#[cfg(feature = "hydra")]
pub async fn vocabulary(State(state): State<HydraState>, headers: HeaderMap) -> Response {
    let content_type = get_content_type(&headers);

    let vocab = serde_json::json!({
        "@context": {
            "rdfs": "http://www.w3.org/2000/01/rdf-schema#",
            "schema": "http://schema.org/",
            "magneto": format!("{}/vocab#", state.base_url),
        },
        "@graph": [
            {
                "@id": "magneto:Cassette",
                "@type": "rdfs:Class",
                "rdfs:label": "Cassette",
                "rdfs:comment": "A collection of recorded HTTP/WebSocket interactions for deterministic replay",
                "rdfs:subClassOf": "schema:DataCatalog",
            },
            {
                "@id": "magneto:Interaction",
                "@type": "rdfs:Class",
                "rdfs:label": "Interaction",
                "rdfs:comment": "A single HTTP request/response or WebSocket message exchange",
                "rdfs:subClassOf": "schema:Action",
            },
            {
                "@id": "magneto:Template",
                "@type": "rdfs:Class",
                "rdfs:label": "Template",
                "rdfs:comment": "Handlebars template for dynamic response generation",
                "rdfs:subClassOf": "schema:CreativeWork",
            },
            {
                "@id": "magneto:TemplateHelper",
                "@type": "rdfs:Class",
                "rdfs:label": "TemplateHelper",
                "rdfs:comment": "Built-in Handlebars helper function (env, now, uuid, etc.)",
            },
            {
                "@id": "magneto:interactionCount",
                "@type": "rdf:Property",
                "rdfs:label": "interaction count",
                "rdfs:domain": "magneto:Cassette",
                "rdfs:range": "xsd:integer",
            },
            {
                "@id": "magneto:recordedAt",
                "@type": "rdf:Property",
                "rdfs:label": "recorded at",
                "rdfs:domain": ["magneto:Cassette", "magneto:Interaction"],
                "rdfs:range": "xsd:dateTime",
            },
        ]
    });

    match content_type {
        ContentType::JsonLd | ContentType::Json => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, content_type.mime_type())],
            Json(vocab),
        )
            .into_response(),
        ContentType::Turtle => {
            // TODO: Implémenter sérialisation Turtle pour le vocabulaire
            (
                StatusCode::NOT_IMPLEMENTED,
                "Turtle format not yet implemented",
            )
                .into_response()
        }
        _ => (
            StatusCode::NOT_IMPLEMENTED,
            "Only JSON-LD and JSON are currently supported",
        )
            .into_response(),
    }
}

/// Construit le routeur Axum pour les endpoints Hydra
///
/// # Exemple
///
/// ```rust,no_run
/// use magneto_serge::api::{CassetteManager, hydra_handlers::{HydraState, build_hydra_router}};
/// use std::sync::Arc;
///
/// let manager = Arc::new(CassetteManager::new("./cassettes"));
/// let state = HydraState::new(manager, "http://localhost:8889");
/// let router = build_hydra_router(state);
/// ```
#[cfg(feature = "hydra")]
pub fn build_hydra_router(state: HydraState) -> Router {
    Router::new()
        // API Documentation entrypoint
        .route("/api", get(api_entrypoint))
        // Cassette endpoints
        .route("/api/cassettes", get(list_cassettes))
        .route("/api/cassettes/:name", get(get_cassette))
        // Template endpoints
        .route("/api/templates", get(list_templates))
        // Vocabulary endpoint
        .route("/vocab", get(vocabulary))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_detection() {
        let mut headers = HeaderMap::new();
        headers.insert("accept", "application/ld+json".parse().unwrap());
        assert_eq!(get_content_type(&headers), ContentType::JsonLd);

        headers.insert("accept", "text/turtle".parse().unwrap());
        assert_eq!(get_content_type(&headers), ContentType::Turtle);

        headers.insert("accept", "application/json".parse().unwrap());
        assert_eq!(get_content_type(&headers), ContentType::Json);
    }

    #[test]
    fn test_content_type_mime() {
        assert_eq!(ContentType::Json.mime_type(), "application/json");
        assert_eq!(ContentType::JsonLd.mime_type(), "application/ld+json");
        assert_eq!(ContentType::Turtle.mime_type(), "text/turtle");
        assert_eq!(ContentType::RdfXml.mime_type(), "application/rdf+xml");
    }
}
