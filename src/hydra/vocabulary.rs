//! Hydra Vocabulary Types
//!
//! Core types from the Hydra vocabulary for building hypermedia APIs.

use serde::{Deserialize, Serialize};

/// Hydra Class
///
/// Represents a class of resources in the API.
///
/// # Example
///
/// ```json
/// {
///   "@id": "Cassette",
///   "@type": "hydra:Class",
///   "hydra:title": "Cassette",
///   "hydra:description": "A recorded HTTP/WebSocket cassette"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraClass {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:title")]
    pub title: String,

    #[serde(rename = "hydra:description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(
        rename = "hydra:supportedProperty",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_properties: Vec<HydraProperty>,

    #[serde(
        rename = "hydra:supportedOperation",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_operations: Vec<super::HydraOperation>,
}

impl HydraClass {
    /// Create a new Hydra class
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

    /// Set the description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Add a supported property
    pub fn with_property(mut self, property: HydraProperty) -> Self {
        self.supported_properties.push(property);
        self
    }

    /// Add a supported operation
    pub fn with_operation(mut self, operation: super::HydraOperation) -> Self {
        self.supported_operations.push(operation);
        self
    }
}

/// Hydra Property
///
/// Represents a property of a Hydra class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraProperty {
    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:property")]
    pub property: String,

    #[serde(rename = "hydra:title")]
    pub title: String,

    #[serde(rename = "hydra:description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "hydra:required")]
    pub required: bool,

    #[serde(rename = "hydra:readonly", skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,

    #[serde(rename = "hydra:writeonly", skip_serializing_if = "Option::is_none")]
    pub writeonly: Option<bool>,
}

impl HydraProperty {
    /// Create a new Hydra property
    pub fn new(property: &str, title: &str, required: bool) -> Self {
        Self {
            type_: "hydra:SupportedProperty".to_string(),
            property: property.to_string(),
            title: title.to_string(),
            description: None,
            required,
            readonly: None,
            writeonly: None,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
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

/// Hydra Link
///
/// Represents a hypermedia link to another resource.
///
/// # Example
///
/// ```json
/// {
///   "href": "/api/cassettes/test",
///   "title": "View cassette",
///   "templated": false
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraLink {
    pub href: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub templated: Option<bool>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

impl HydraLink {
    /// Create a new link
    pub fn new(href: &str) -> Self {
        Self {
            href: href.to_string(),
            title: None,
            templated: None,
            media_type: None,
        }
    }

    /// Set the title
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Mark as templated (URI template)
    pub fn templated(mut self) -> Self {
        self.templated = Some(true);
        self
    }

    /// Set the media type
    pub fn with_media_type(mut self, media_type: &str) -> Self {
        self.media_type = Some(media_type.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydra_class() {
        let class = HydraClass::new("Cassette", "Cassette Resource")
            .with_description("A recorded HTTP/WebSocket cassette");

        assert_eq!(class.id, "Cassette");
        assert_eq!(class.title, "Cassette Resource");
        assert_eq!(
            class.description,
            Some("A recorded HTTP/WebSocket cassette".to_string())
        );
    }

    #[test]
    fn test_hydra_property() {
        let prop = HydraProperty::new("name", "Cassette Name", true)
            .with_description("The name of the cassette");

        assert_eq!(prop.property, "name");
        assert_eq!(prop.title, "Cassette Name");
        assert!(prop.required);
    }

    #[test]
    fn test_hydra_link() {
        let link = HydraLink::new("/api/cassettes/test")
            .with_title("View cassette")
            .with_media_type("application/ld+json");

        assert_eq!(link.href, "/api/cassettes/test");
        assert_eq!(link.title, Some("View cassette".to_string()));
        assert_eq!(link.media_type, Some("application/ld+json".to_string()));
    }
}
