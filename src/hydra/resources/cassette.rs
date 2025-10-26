//! Cassette Hydra Resource
//!
//! Hypermedia representation of cassettes with links and operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cassette::Cassette;
use crate::hydra::{HydraLink, HydraOperation};

/// Cassette Resource
///
/// Hypermedia representation of a cassette with Hydra links and operations.
///
/// # Example JSON-LD
///
/// ```json
/// {
///   "@context": "/vocab",
///   "@id": "/api/cassettes/test",
///   "@type": "Cassette",
///   "name": "test",
///   "version": "1.0",
///   "recordedAt": "2025-10-26T10:00:00Z",
///   "interactionCount": 5,
///   "sizeBytes": 12345,
///   "_links": {
///     "self": { "href": "/api/cassettes/test" },
///     "interactions": { "href": "/api/cassettes/test/interactions" },
///     "edit": { "href": "/api/cassettes/test" },
///     "delete": { "href": "/api/cassettes/test" }
///   },
///   "hydra:operation": [...]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CassetteResource {
    /// Cassette name
    pub name: String,

    /// Cassette version
    pub version: String,

    /// Recording timestamp (ISO 8601)
    #[serde(rename = "recordedAt")]
    pub recorded_at: String,

    /// Number of interactions
    #[serde(rename = "interactionCount")]
    pub interaction_count: usize,

    /// Total size in bytes
    #[serde(rename = "sizeBytes")]
    pub size_bytes: u64,

    /// Cassette format (Json or MessagePack)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Hypermedia links
    #[serde(rename = "_links")]
    pub links: CassetteLinks,

    /// Metadata (custom properties)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl CassetteResource {
    /// Create a new cassette resource from a Cassette
    ///
    /// # Arguments
    ///
    /// * `cassette` - The cassette to convert
    /// * `base_url` - Base URL for generating links (e.g., "http://localhost:8889")
    pub fn from_cassette(cassette: &Cassette, base_url: &str) -> Self {
        let interaction_count = cassette.interactions.len();

        // Calculate approximate size (serialized JSON size)
        let size_bytes = serde_json::to_string(cassette)
            .map(|s| s.len() as u64)
            .unwrap_or(0);

        let cassette_url = format!("{}/api/cassettes/{}", base_url, cassette.name);

        Self {
            name: cassette.name.clone(),
            version: cassette.version.clone(),
            recorded_at: cassette.recorded_at.to_rfc3339(),
            interaction_count,
            size_bytes,
            format: Some("Json".to_string()),
            tags: None,
            description: None,
            links: CassetteLinks::new(&cassette_url, &cassette.name),
            metadata: None,
        }
    }

    /// Get all available operations for this cassette
    pub fn operations() -> Vec<HydraOperation> {
        vec![
            HydraOperation::get("Retrieve cassette", "Cassette")
                .with_description("Get the full cassette with all interactions"),
            HydraOperation::put("Update cassette", "CassetteInput", "Cassette")
                .with_description("Update cassette metadata"),
            HydraOperation::delete("Delete cassette")
                .with_description("Permanently delete this cassette"),
        ]
    }

    /// Get collection-level operations (for /api/cassettes)
    pub fn collection_operations() -> Vec<HydraOperation> {
        vec![
            HydraOperation::get("List cassettes", "Collection")
                .with_description("Retrieve all cassettes with pagination"),
            HydraOperation::post("Create cassette", "CassetteInput", "Cassette")
                .with_description("Create a new cassette"),
        ]
    }
}

/// Cassette Links
///
/// Hypermedia links for cassette navigation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CassetteLinks {
    /// Self link
    #[serde(rename = "self")]
    pub self_link: HydraLink,

    /// Link to interactions collection
    pub interactions: HydraLink,

    /// Edit link (same as self, but semantic)
    pub edit: HydraLink,

    /// Delete link
    pub delete: HydraLink,

    /// Duplicate link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<HydraLink>,

    /// Export link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<HydraLink>,
}

impl CassetteLinks {
    /// Create links for a cassette
    ///
    /// # Arguments
    ///
    /// * `cassette_url` - Full URL to the cassette (e.g., "/api/cassettes/test")
    /// * `cassette_name` - Name of the cassette
    pub fn new(cassette_url: &str, cassette_name: &str) -> Self {
        Self {
            self_link: HydraLink::new(cassette_url).with_title("Self"),
            interactions: HydraLink::new(&format!("{}/interactions", cassette_url))
                .with_title("Interactions"),
            edit: HydraLink::new(cassette_url).with_title("Edit cassette"),
            delete: HydraLink::new(cassette_url).with_title("Delete cassette"),
            duplicate: Some(
                HydraLink::new(&format!("{}/duplicate", cassette_url))
                    .with_title(&format!("Duplicate {}", cassette_name)),
            ),
            export: Some(
                HydraLink::new(&format!("{}/export", cassette_url)).with_title("Export cassette"),
            ),
        }
    }
}

/// Cassette Input (for POST/PUT)
///
/// Request body for creating or updating cassettes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CassetteInput {
    /// Cassette name (required for POST)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Cassette version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cassette::{HttpRequest, HttpResponse, Interaction, InteractionKind};
    use std::collections::HashMap;

    fn create_test_cassette() -> Cassette {
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/test".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(b"{\"test\":true}".to_vec()),
        };

        let interaction = Interaction {
            kind: InteractionKind::Http { request, response },
            recorded_at: chrono::Utc::now(),
            response_time_ms: Some(100),
        };

        Cassette {
            name: "test".to_string(),
            version: "1.0".to_string(),
            recorded_at: chrono::Utc::now(),
            interactions: vec![interaction],
            cookies: Some(vec![]),
        }
    }

    #[test]
    fn test_cassette_resource_from_cassette() {
        let cassette = create_test_cassette();
        let resource = CassetteResource::from_cassette(&cassette, "http://localhost:8889");

        assert_eq!(resource.name, "test");
        assert_eq!(resource.version, "1.0");
        assert_eq!(resource.interaction_count, 1);
        assert!(resource.size_bytes > 0);
    }

    #[test]
    fn test_cassette_links() {
        let links = CassetteLinks::new("/api/cassettes/test", "test");

        assert_eq!(links.self_link.href, "/api/cassettes/test");
        assert_eq!(links.interactions.href, "/api/cassettes/test/interactions");
        assert!(links.duplicate.is_some());
        assert!(links.export.is_some());
    }

    #[test]
    fn test_cassette_operations() {
        let ops = CassetteResource::operations();

        assert_eq!(ops.len(), 3); // GET, PUT, DELETE
        assert!(ops.iter().any(|op| op.method == "GET"));
        assert!(ops.iter().any(|op| op.method == "PUT"));
        assert!(ops.iter().any(|op| op.method == "DELETE"));
    }

    #[test]
    fn test_collection_operations() {
        let ops = CassetteResource::collection_operations();

        assert_eq!(ops.len(), 2); // GET (list), POST (create)
        assert!(ops.iter().any(|op| op.method == "GET"));
        assert!(ops.iter().any(|op| op.method == "POST"));
    }
}
