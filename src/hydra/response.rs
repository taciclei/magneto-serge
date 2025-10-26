//! Hydra Response Builder
//!
//! Provides utilities for building Hydra-compliant HTTP responses.

use serde::{Deserialize, Serialize};

use super::{HydraOperation, JsonLdContext};

/// Hydra Response
///
/// A JSON-LD response with Hydra metadata.
///
/// # Example
///
/// ```rust
/// use magneto_serge::hydra::HydraResponse;
/// use serde_json::json;
///
/// let response = HydraResponse::new("/api/cassettes/test", "Cassette", json!({
///     "name": "test",
///     "version": "1.0"
/// }));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraResponse<T> {
    #[serde(rename = "@context")]
    pub context: serde_json::Value,

    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(flatten)]
    pub data: T,

    #[serde(rename = "hydra:operation", skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<HydraOperation>,
}

impl<T: Serialize> HydraResponse<T> {
    /// Create a new Hydra response
    ///
    /// # Arguments
    ///
    /// * `id` - Resource URI (e.g., "/api/cassettes/test")
    /// * `type_` - Resource type (e.g., "Cassette")
    /// * `data` - The actual resource data
    pub fn new(id: &str, type_: &str, data: T) -> Self {
        let context = JsonLdContext::default();

        Self {
            context: context.as_context_value(),
            id: id.to_string(),
            type_: type_.to_string(),
            data,
            operations: Vec::new(),
        }
    }

    /// Create a response with custom context
    pub fn with_context(id: &str, type_: &str, data: T, context: JsonLdContext) -> Self {
        Self {
            context: context.as_context_value(),
            id: id.to_string(),
            type_: type_.to_string(),
            data,
            operations: Vec::new(),
        }
    }

    /// Add operations to the response
    pub fn with_operations(mut self, operations: Vec<HydraOperation>) -> Self {
        self.operations = operations;
        self
    }

    /// Add a single operation
    pub fn with_operation(mut self, operation: HydraOperation) -> Self {
        self.operations.push(operation);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_hydra_response() {
        let data = json!({
            "name": "test",
            "version": "1.0"
        });

        let response = HydraResponse::new("/api/cassettes/test", "Cassette", data);

        assert_eq!(response.id, "/api/cassettes/test");
        assert_eq!(response.type_, "Cassette");
        assert!(response.operations.is_empty());
    }

    #[test]
    fn test_with_operations() {
        let data = json!({"name": "test"});
        let op = HydraOperation::get("Retrieve", "Cassette");

        let response =
            HydraResponse::new("/api/cassettes/test", "Cassette", data).with_operation(op);

        assert_eq!(response.operations.len(), 1);
    }
}
