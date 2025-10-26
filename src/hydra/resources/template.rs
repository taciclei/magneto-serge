//! Template Hydra Resource
//!
//! Hypermedia representation of Handlebars templates (v0.4.0 integration).

use serde::{Deserialize, Serialize};

use crate::hydra::{HydraLink, HydraOperation};

/// Template Resource
///
/// Represents a Handlebars template with built-in helpers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateResource {
    #[serde(rename = "@id")]
    pub id: String,

    pub name: String,
    pub syntax: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub category: String,

    #[serde(rename = "builtInHelpers")]
    pub built_in_helpers: Vec<TemplateHelper>,

    #[serde(rename = "_links")]
    pub links: TemplateLinks,
}

impl TemplateResource {
    /// Create built-in helpers list
    pub fn built_in_helpers() -> Vec<TemplateHelper> {
        vec![
            TemplateHelper {
                name: "env".to_string(),
                syntax: "{{ env \"VAR_NAME\" }}".to_string(),
                description: "Environment variable substitution".to_string(),
                example: "{{ env \"API_KEY\" }}".to_string(),
                output_example: Some("sk-test-1234567890".to_string()),
            },
            TemplateHelper {
                name: "now".to_string(),
                syntax: "{{ now }}".to_string(),
                description: "Current timestamp in ISO 8601 format".to_string(),
                example: "{{ now }}".to_string(),
                output_example: Some("2025-10-26T10:30:45Z".to_string()),
            },
            TemplateHelper {
                name: "now_timestamp".to_string(),
                syntax: "{{ now_timestamp }}".to_string(),
                description: "Current Unix epoch timestamp".to_string(),
                example: "{{ now_timestamp }}".to_string(),
                output_example: Some("1729940445".to_string()),
            },
            TemplateHelper {
                name: "uuid".to_string(),
                syntax: "{{ uuid }}".to_string(),
                description: "Generate UUID v4".to_string(),
                example: "{{ uuid }}".to_string(),
                output_example: Some("a1b2c3d4-e5f6-4789-a0b1-c2d3e4f5g6h7".to_string()),
            },
            TemplateHelper {
                name: "request.method".to_string(),
                syntax: "{{ request.method }}".to_string(),
                description: "HTTP request method".to_string(),
                example: "{{ request.method }}".to_string(),
                output_example: Some("POST".to_string()),
            },
            TemplateHelper {
                name: "request.url".to_string(),
                syntax: "{{ request.url }}".to_string(),
                description: "HTTP request URL".to_string(),
                example: "{{ request.url }}".to_string(),
                output_example: Some("https://api.example.com/users".to_string()),
            },
            TemplateHelper {
                name: "request.headers".to_string(),
                syntax: "{{ request.headers.header-name }}".to_string(),
                description: "Access request header value".to_string(),
                example: "{{ request.headers.x-user-id }}".to_string(),
                output_example: Some("user-12345".to_string()),
            },
        ]
    }

    /// Get operations for templates
    pub fn operations() -> Vec<HydraOperation> {
        vec![
            HydraOperation::post(
                "Validate template",
                "TemplateValidationInput",
                "TemplateValidationResult",
            )
            .with_description("Validate template syntax and test rendering"),
            HydraOperation::post("Apply template", "ApplyTemplateInput", "Interaction")
                .with_description("Apply template to an interaction response"),
        ]
    }
}

/// Template Helper
///
/// Describes a template helper function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateHelper {
    pub name: String,
    pub syntax: String,
    pub description: String,
    pub example: String,

    #[serde(rename = "outputExample", skip_serializing_if = "Option::is_none")]
    pub output_example: Option<String>,
}

/// Template Links
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLinks {
    #[serde(rename = "self")]
    pub self_link: HydraLink,

    pub validate: HydraLink,

    #[serde(rename = "applyTo")]
    pub apply_to: HydraLink,
}

impl TemplateLinks {
    pub fn new(template_url: &str) -> Self {
        Self {
            self_link: HydraLink::new(template_url),
            validate: HydraLink::new(&format!("{}/validate", template_url))
                .with_title("Validate template"),
            apply_to: HydraLink::new(&format!("{}/apply", template_url))
                .with_title("Apply template"),
        }
    }
}

/// Template Validation Input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateValidationInput {
    pub template: String,

    #[serde(rename = "sampleData", skip_serializing_if = "Option::is_none")]
    pub sample_data: Option<serde_json::Value>,
}

/// Template Validation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateValidationResult {
    pub valid: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
}

/// Apply Template Input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyTemplateInput {
    #[serde(rename = "cassetteName")]
    pub cassette_name: String,

    #[serde(rename = "interactionId")]
    pub interaction_id: usize,

    pub template: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_built_in_helpers() {
        let helpers = TemplateResource::built_in_helpers();

        assert_eq!(helpers.len(), 7); // env, now, now_timestamp, uuid, method, url, headers
        assert!(helpers.iter().any(|h| h.name == "env"));
        assert!(helpers.iter().any(|h| h.name == "now"));
        assert!(helpers.iter().any(|h| h.name == "uuid"));
    }

    #[test]
    fn test_template_validation_input() {
        let input = TemplateValidationInput {
            template: "{{ env \"API_KEY\" }}".to_string(),
            sample_data: Some(serde_json::json!({"API_KEY": "test-key"})),
        };

        assert_eq!(input.template, "{{ env \"API_KEY\" }}");
        assert!(input.sample_data.is_some());
    }

    #[test]
    fn test_template_links() {
        let links = TemplateLinks::new("/api/templates/auth");

        assert_eq!(links.self_link.href, "/api/templates/auth");
        assert_eq!(links.validate.href, "/api/templates/auth/validate");
        assert_eq!(links.apply_to.href, "/api/templates/auth/apply");
    }
}
