//! Hydra Collections
//!
//! Provides support for paginated collections of resources.

use serde::{Deserialize, Serialize};

use super::JsonLdContext;

/// Hydra Collection
///
/// A paginated collection of resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraCollection<T> {
    #[serde(rename = "@context")]
    pub context: serde_json::Value,

    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:totalItems")]
    pub total_items: usize,

    #[serde(rename = "hydra:member")]
    pub members: Vec<T>,

    #[serde(rename = "hydra:view", skip_serializing_if = "Option::is_none")]
    pub view: Option<HydraView>,

    #[serde(rename = "hydra:search", skip_serializing_if = "Option::is_none")]
    pub search: Option<HydraSearch>,
}

impl<T: Serialize> HydraCollection<T> {
    /// Create a new collection
    pub fn new(id: &str, members: Vec<T>, total_items: usize) -> Self {
        let context = JsonLdContext::default();

        Self {
            context: context.as_context_value(),
            id: id.to_string(),
            type_: "hydra:Collection".to_string(),
            total_items,
            members,
            view: None,
            search: None,
        }
    }

    /// Add pagination view
    pub fn with_view(mut self, view: HydraView) -> Self {
        self.view = Some(view);
        self
    }

    /// Add search template
    pub fn with_search(mut self, search: HydraSearch) -> Self {
        self.search = Some(search);
        self
    }
}

/// Hydra View (Pagination)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraView {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@type")]
    pub type_: String,

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
    /// Create a pagination view
    pub fn paginate(base_url: &str, current_page: usize, total_pages: usize) -> Self {
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

/// Hydra Search (IRI Template)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraSearch {
    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:template")]
    pub template: String,

    #[serde(rename = "hydra:mapping")]
    pub mappings: Vec<HydraMapping>,
}

impl HydraSearch {
    /// Create a search template
    pub fn new(template: &str) -> Self {
        Self {
            type_: "hydra:IriTemplate".to_string(),
            template: template.to_string(),
            mappings: Vec::new(),
        }
    }

    /// Add a mapping
    pub fn with_mapping(mut self, mapping: HydraMapping) -> Self {
        self.mappings.push(mapping);
        self
    }
}

/// Hydra IRI Template Mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraMapping {
    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:variable")]
    pub variable: String,

    #[serde(rename = "hydra:property")]
    pub property: String,

    #[serde(rename = "hydra:required")]
    pub required: bool,
}

impl HydraMapping {
    /// Create a new mapping
    pub fn new(variable: &str, property: &str, required: bool) -> Self {
        Self {
            type_: "hydra:IriTemplateMapping".to_string(),
            variable: variable.to_string(),
            property: property.to_string(),
            required,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_hydra_collection() {
        let members = vec![json!({"name": "test1"}), json!({"name": "test2"})];

        let collection = HydraCollection::new("/api/cassettes", members, 2);

        assert_eq!(collection.total_items, 2);
        assert_eq!(collection.members.len(), 2);
    }

    #[test]
    fn test_hydra_view_pagination() {
        let view = HydraView::paginate("/api/cassettes", 2, 5);

        assert_eq!(view.first, "/api/cassettes?page=1");
        assert_eq!(view.previous, Some("/api/cassettes?page=1".to_string()));
        assert_eq!(view.next, Some("/api/cassettes?page=3".to_string()));
        assert_eq!(view.last, "/api/cassettes?page=5");
    }

    #[test]
    fn test_hydra_search() {
        let search = HydraSearch::new("/api/cassettes{?name,size}")
            .with_mapping(HydraMapping::new("name", "schema:name", false))
            .with_mapping(HydraMapping::new("size", "magneto:sizeBytes", false));

        assert_eq!(search.template, "/api/cassettes{?name,size}");
        assert_eq!(search.mappings.len(), 2);
    }
}
