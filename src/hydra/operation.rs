//! Hydra Operations
//!
//! Represents operations that can be performed on resources.

use serde::{Deserialize, Serialize};

/// HTTP Methods supported by Hydra
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
        }
    }
}

/// Hydra Operation
///
/// Represents an operation that can be performed on a resource.
///
/// # Example
///
/// ```json
/// {
///   "@type": "hydra:Operation",
///   "hydra:method": "GET",
///   "hydra:returns": "Cassette",
///   "hydra:title": "Retrieve cassette"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraOperation {
    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:method")]
    pub method: String,

    #[serde(rename = "hydra:title")]
    pub title: String,

    #[serde(rename = "hydra:description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "hydra:expects", skip_serializing_if = "Option::is_none")]
    pub expects: Option<String>,

    #[serde(rename = "hydra:returns", skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,

    #[serde(rename = "hydra:statusCodes", skip_serializing_if = "Vec::is_empty")]
    pub status_codes: Vec<HydraStatusCode>,
}

impl HydraOperation {
    /// Create a new GET operation
    pub fn get(title: &str, returns: &str) -> Self {
        Self {
            type_: "hydra:Operation".to_string(),
            method: "GET".to_string(),
            title: title.to_string(),
            description: None,
            expects: None,
            returns: Some(returns.to_string()),
            status_codes: vec![
                HydraStatusCode::new(200, "Success"),
                HydraStatusCode::new(404, "Not Found"),
            ],
        }
    }

    /// Create a new POST operation
    pub fn post(title: &str, expects: &str, returns: &str) -> Self {
        Self {
            type_: "hydra:Operation".to_string(),
            method: "POST".to_string(),
            title: title.to_string(),
            description: None,
            expects: Some(expects.to_string()),
            returns: Some(returns.to_string()),
            status_codes: vec![
                HydraStatusCode::new(201, "Created"),
                HydraStatusCode::new(400, "Bad Request"),
            ],
        }
    }

    /// Create a new PUT operation
    pub fn put(title: &str, expects: &str, returns: &str) -> Self {
        Self {
            type_: "hydra:Operation".to_string(),
            method: "PUT".to_string(),
            title: title.to_string(),
            description: None,
            expects: Some(expects.to_string()),
            returns: Some(returns.to_string()),
            status_codes: vec![
                HydraStatusCode::new(200, "Updated"),
                HydraStatusCode::new(400, "Bad Request"),
                HydraStatusCode::new(404, "Not Found"),
            ],
        }
    }

    /// Create a new DELETE operation
    pub fn delete(title: &str) -> Self {
        Self {
            type_: "hydra:Operation".to_string(),
            method: "DELETE".to_string(),
            title: title.to_string(),
            description: None,
            expects: None,
            returns: None,
            status_codes: vec![
                HydraStatusCode::new(204, "Deleted"),
                HydraStatusCode::new(404, "Not Found"),
            ],
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Add a status code
    pub fn with_status_code(mut self, code: u16, description: &str) -> Self {
        self.status_codes
            .push(HydraStatusCode::new(code, description));
        self
    }
}

/// Hydra Status Code
///
/// Represents a possible HTTP status code for an operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraStatusCode {
    #[serde(rename = "hydra:statusCode")]
    pub code: u16,

    #[serde(rename = "hydra:description")]
    pub description: String,
}

impl HydraStatusCode {
    /// Create a new status code description
    pub fn new(code: u16, description: &str) -> Self {
        Self {
            code,
            description: description.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_operation() {
        let op = HydraOperation::get("Retrieve cassette", "Cassette");

        assert_eq!(op.method, "GET");
        assert_eq!(op.title, "Retrieve cassette");
        assert_eq!(op.returns, Some("Cassette".to_string()));
        assert!(op.expects.is_none());
    }

    #[test]
    fn test_post_operation() {
        let op = HydraOperation::post("Create cassette", "CassetteInput", "Cassette");

        assert_eq!(op.method, "POST");
        assert_eq!(op.expects, Some("CassetteInput".to_string()));
        assert_eq!(op.returns, Some("Cassette".to_string()));
    }

    #[test]
    fn test_delete_operation() {
        let op = HydraOperation::delete("Delete cassette");

        assert_eq!(op.method, "DELETE");
        assert!(op.expects.is_none());
        assert!(op.returns.is_none());
    }

    #[test]
    fn test_custom_status_codes() {
        let op =
            HydraOperation::get("Retrieve cassette", "Cassette").with_status_code(403, "Forbidden");

        assert_eq!(op.status_codes.len(), 3); // 200, 404, 403
    }
}
