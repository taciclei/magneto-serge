//! Hydra Error Responses
//!
//! Structured error responses following Hydra vocabulary.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

/// Hydra Error
///
/// A structured error response using Hydra vocabulary.
///
/// # Example
///
/// ```json
/// {
///   "@context": "http://www.w3.org/ns/hydra/context.jsonld",
///   "@type": "hydra:Error",
///   "hydra:title": "Not Found",
///   "hydra:description": "Cassette 'test' not found",
///   "hydra:statusCode": 404
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraError {
    #[serde(rename = "@context")]
    pub context: String,

    #[serde(rename = "@type")]
    pub type_: String,

    #[serde(rename = "hydra:title")]
    pub title: String,

    #[serde(rename = "hydra:description")]
    pub description: String,

    #[serde(rename = "hydra:statusCode")]
    pub status_code: u16,
}

impl HydraError {
    /// Create a new Hydra error
    pub fn new(status_code: u16, title: &str, description: &str) -> Self {
        Self {
            context: "http://www.w3.org/ns/hydra/context.jsonld".to_string(),
            type_: "hydra:Error".to_string(),
            title: title.to_string(),
            description: description.to_string(),
            status_code,
        }
    }

    /// Create a 404 Not Found error
    pub fn not_found(resource: &str) -> Self {
        Self::new(
            404,
            "Not Found",
            &format!("Resource '{}' not found", resource),
        )
    }

    /// Create a 400 Bad Request error
    pub fn bad_request(description: &str) -> Self {
        Self::new(400, "Bad Request", description)
    }

    /// Create a 422 Unprocessable Entity error
    pub fn validation_error(description: &str) -> Self {
        Self::new(422, "Validation Error", description)
    }

    /// Create a 500 Internal Server Error
    pub fn internal_error(description: &str) -> Self {
        Self::new(500, "Internal Server Error", description)
    }
}

impl IntoResponse for HydraError {
    fn into_response(self) -> Response {
        let status =
            StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status, Json(self)).into_response()
    }
}

impl From<crate::error::MatgtoError> for HydraError {
    fn from(err: crate::error::MatgtoError) -> Self {
        use crate::error::MatgtoError;

        match err {
            MatgtoError::CassetteNotFound { name } => {
                HydraError::not_found(&format!("Cassette '{}'", name))
            }
            MatgtoError::Io(e) => HydraError::internal_error(&format!("I/O error: {}", e)),
            MatgtoError::Serialization(e) => {
                HydraError::internal_error(&format!("Serialization error: {}", e))
            }
            _ => HydraError::internal_error(&err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error() {
        let error = HydraError::not_found("test");

        assert_eq!(error.status_code, 404);
        assert_eq!(error.title, "Not Found");
        assert!(error.description.contains("test"));
    }

    #[test]
    fn test_validation_error() {
        let error = HydraError::validation_error("Invalid name");

        assert_eq!(error.status_code, 422);
        assert_eq!(error.title, "Validation Error");
    }
}
