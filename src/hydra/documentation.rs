//! Hydra API Documentation
//!
//! Auto-generated API documentation using Hydra vocabulary.

use serde::{Deserialize, Serialize};

use super::{HydraOperation, HydraProperty};

/// API Documentation
///
/// The entry point for a Hydra API, describing all available resources and operations.
///
/// # Example
///
/// ```rust
/// use magneto_serge::hydra::ApiDocumentation;
///
/// let api_doc = ApiDocumentation::new(
///     "/api",
///     "Magneto-Serge Hypermedia API",
///     "RESTful API for cassette management"
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDocumentation {
    #[serde(rename = "@context")]
    pub context: String,

    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:title")]
    pub title: String,

    #[serde(rename = "hydra:description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "hydra:entrypoint")]
    pub entrypoint: String,

    #[serde(rename = "hydra:supportedClass")]
    pub supported_classes: Vec<SupportedClass>,
}

impl ApiDocumentation {
    /// Create new API documentation
    pub fn new(entrypoint: &str, title: &str, description: &str) -> Self {
        Self {
            context: "http://www.w3.org/ns/hydra/context.jsonld".to_string(),
            id: "/api".to_string(),
            type_: "hydra:ApiDocumentation".to_string(),
            title: title.to_string(),
            description: Some(description.to_string()),
            entrypoint: entrypoint.to_string(),
            supported_classes: Vec::new(),
        }
    }

    /// Add a supported class
    pub fn with_class(mut self, class: SupportedClass) -> Self {
        self.supported_classes.push(class);
        self
    }
}

/// Supported Class
///
/// Describes a resource type supported by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedClass {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:title")]
    pub title: String,

    #[serde(rename = "hydra:description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "hydra:supportedProperty")]
    pub supported_properties: Vec<SupportedProperty>,

    #[serde(rename = "hydra:supportedOperation")]
    pub supported_operations: Vec<HydraOperation>,
}

impl SupportedClass {
    /// Create a new supported class
    pub fn new(id: &str, title: &str) -> Self {
        Self {
            id: id.to_string(),
            type_: "hydra:Class".to_string(),
            title: title.to_string(),
            description: None,
            supported_properties: Vec::new(),
            supported_operations: Vec::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Add a property
    pub fn with_property(mut self, property: SupportedProperty) -> Self {
        self.supported_properties.push(property);
        self
    }

    /// Add an operation
    pub fn with_operation(mut self, operation: HydraOperation) -> Self {
        self.supported_operations.push(operation);
        self
    }
}

/// Supported Property
///
/// Describes a property of a resource class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedProperty {
    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:property")]
    pub property: HydraProperty,

    #[serde(rename = "hydra:required")]
    pub required: bool,

    #[serde(rename = "hydra:readonly", skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,

    #[serde(rename = "hydra:writeonly", skip_serializing_if = "Option::is_none")]
    pub writeonly: Option<bool>,
}

impl SupportedProperty {
    /// Create a new supported property
    pub fn new(property: HydraProperty, required: bool) -> Self {
        Self {
            type_: "hydra:SupportedProperty".to_string(),
            property,
            required,
            readonly: None,
            writeonly: None,
        }
    }

    /// Mark as readonly
    pub fn readonly(mut self) -> Self {
        self.readonly = Some(true);
        self
    }

    /// Mark as writeonly
    pub fn writeonly(mut self) -> Self {
        self.writeonly = Some(true);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_documentation() {
        let api_doc = ApiDocumentation::new("/api", "Test API", "A test API");

        assert_eq!(api_doc.title, "Test API");
        assert_eq!(api_doc.entrypoint, "/api");
    }

    #[test]
    fn test_supported_class() {
        let class = SupportedClass::new("Cassette", "Cassette Resource")
            .with_description("A recorded cassette");

        assert_eq!(class.id, "Cassette");
        assert_eq!(class.title, "Cassette Resource");
    }
}
