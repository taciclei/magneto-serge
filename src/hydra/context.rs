//! JSON-LD Context Builder
//!
//! Provides utilities for creating and managing JSON-LD contexts for Hydra APIs.

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use super::{
    HYDRA_NAMESPACE, MAGNETO_NAMESPACE, RDFS_NAMESPACE, RDF_NAMESPACE, SCHEMA_NAMESPACE,
    XSD_NAMESPACE,
};

/// JSON-LD Context
///
/// Defines the vocabulary and namespace mappings for JSON-LD documents.
///
/// # Example
///
/// ```rust
/// use magneto_serge::hydra::JsonLdContext;
///
/// let context = JsonLdContext::new("http://localhost:8889");
/// let json = context.to_json();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonLdContext {
    /// Base URL for the API
    #[serde(skip)]
    pub base_url: String,

    /// Vocabulary URL (default Hydra namespace)
    #[serde(skip)]
    pub vocab_url: String,

    /// Namespace mappings
    #[serde(flatten)]
    pub mappings: HashMap<String, serde_json::Value>,
}

impl JsonLdContext {
    /// Create a new JSON-LD context with default Hydra mappings
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API (e.g., "http://localhost:8889")
    ///
    /// # Example
    ///
    /// ```rust
    /// use magneto_serge::hydra::JsonLdContext;
    /// let context = JsonLdContext::new("http://localhost:8889");
    /// ```
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            vocab_url: HYDRA_NAMESPACE.to_string(),
            mappings: Self::default_mappings(base_url),
        }
    }

    /// Create default namespace mappings
    fn default_mappings(_base_url: &str) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();

        // Standard vocabularies
        map.insert("hydra".to_string(), json!(HYDRA_NAMESPACE));
        map.insert("rdf".to_string(), json!(RDF_NAMESPACE));
        map.insert("rdfs".to_string(), json!(RDFS_NAMESPACE));
        map.insert("xsd".to_string(), json!(XSD_NAMESPACE));
        map.insert("schema".to_string(), json!(SCHEMA_NAMESPACE));

        // Magneto-Serge specific vocabulary
        map.insert("magneto".to_string(), json!(MAGNETO_NAMESPACE));

        // Resource types
        map.insert(
            "Cassette".to_string(),
            json!({
                "@id": "magneto:Cassette",
                "@type": "@id"
            }),
        );

        map.insert(
            "Interaction".to_string(),
            json!({
                "@id": "magneto:Interaction",
                "@type": "@id"
            }),
        );

        map.insert(
            "HttpRequest".to_string(),
            json!({
                "@id": "magneto:HttpRequest",
                "@type": "@id"
            }),
        );

        map.insert(
            "HttpResponse".to_string(),
            json!({
                "@id": "magneto:HttpResponse",
                "@type": "@id"
            }),
        );

        map.insert(
            "WebSocketMessage".to_string(),
            json!({
                "@id": "magneto:WebSocketMessage",
                "@type": "@id"
            }),
        );

        // Properties with schema.org mappings
        map.insert("name".to_string(), json!("schema:name"));
        map.insert("version".to_string(), json!("schema:version"));
        map.insert("description".to_string(), json!("schema:description"));
        map.insert(
            "recordedAt".to_string(),
            json!({
                "@id": "schema:dateCreated",
                "@type": "xsd:dateTime"
            }),
        );

        // Magneto-specific properties
        map.insert(
            "interactions".to_string(),
            json!({
                "@id": "magneto:interactions",
                "@type": "@id",
                "@container": "@list"
            }),
        );

        map.insert(
            "interactionCount".to_string(),
            json!({
                "@id": "magneto:interactionCount",
                "@type": "xsd:integer"
            }),
        );

        map.insert(
            "sizeBytes".to_string(),
            json!({
                "@id": "magneto:sizeBytes",
                "@type": "xsd:integer"
            }),
        );

        map.insert("method".to_string(), json!("magneto:method"));
        map.insert("url".to_string(), json!("schema:url"));
        map.insert("headers".to_string(), json!("magneto:headers"));
        map.insert("body".to_string(), json!("magneto:body"));
        map.insert("status".to_string(), json!("magneto:statusCode"));

        // Template-related (v0.4.0)
        map.insert("hasTemplates".to_string(), json!("magneto:hasTemplates"));
        map.insert("template".to_string(), json!("magneto:template"));

        map
    }

    /// Add a custom mapping to the context
    ///
    /// # Example
    ///
    /// ```rust
    /// use magneto_serge::hydra::JsonLdContext;
    /// let mut context = JsonLdContext::new("http://localhost:8889");
    /// context.add_mapping("customProp", "http://example.com/vocab#custom");
    /// ```
    pub fn add_mapping(&mut self, key: &str, value: impl Into<serde_json::Value>) {
        self.mappings.insert(key.to_string(), value.into());
    }

    /// Convert context to JSON value for serialization
    ///
    /// # Returns
    ///
    /// A JSON object with "@context" key containing all mappings
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "@context": self.mappings
        })
    }

    /// Get the context mappings as a JSON-LD @context value
    ///
    /// This is useful for embedding in Hydra responses
    pub fn as_context_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.mappings).unwrap_or_else(|_| json!({}))
    }

    /// Create a scoped context for a specific resource type
    ///
    /// # Arguments
    ///
    /// * `resource_type` - The type of resource (e.g., "Cassette")
    ///
    /// # Returns
    ///
    /// A new context with additional type-specific mappings
    pub fn for_resource(&self, resource_type: &str) -> Self {
        let mut context = self.clone();

        match resource_type {
            "Cassette" => {
                // Add cassette-specific mappings if needed
                context.add_mapping(
                    "format",
                    json!({
                        "@id": "magneto:format",
                        "@type": "@vocab"
                    }),
                );
            }
            "Interaction" => {
                // Add interaction-specific mappings
                context.add_mapping("kind", json!("magneto:interactionKind"));
            }
            _ => {}
        }

        context
    }
}

impl Default for JsonLdContext {
    fn default() -> Self {
        Self::new("http://localhost:8889")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_context() {
        let context = JsonLdContext::new("http://localhost:8889");

        assert_eq!(context.base_url, "http://localhost:8889");
        assert_eq!(context.vocab_url, HYDRA_NAMESPACE);
        assert!(context.mappings.contains_key("hydra"));
        assert!(context.mappings.contains_key("magneto"));
    }

    #[test]
    fn test_add_custom_mapping() {
        let mut context = JsonLdContext::new("http://localhost:8889");
        context.add_mapping("customProp", "http://example.com/custom");

        assert_eq!(
            context.mappings.get("customProp"),
            Some(&json!("http://example.com/custom"))
        );
    }

    #[test]
    fn test_to_json() {
        let context = JsonLdContext::new("http://localhost:8889");
        let json = context.to_json();

        assert!(json.is_object());
        assert!(json.get("@context").is_some());
    }

    #[test]
    fn test_resource_specific_context() {
        let context = JsonLdContext::new("http://localhost:8889");
        let cassette_context = context.for_resource("Cassette");

        assert!(cassette_context.mappings.contains_key("format"));
    }
}
