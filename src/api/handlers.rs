//! REST API handlers for cassette management
//!
//! HTTP endpoints that expose CassetteManager functionality.
//! Built with Axum web framework for high-performance async HTTP.

use super::cassettes::{
    CassetteManager, CassetteMetadata, CassetteStats, GlobalStats, ValidationResult,
};
use crate::error::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state
#[derive(Clone)]
pub struct ApiState {
    pub manager: Arc<RwLock<CassetteManager>>,
}

impl ApiState {
    pub fn new(cassette_dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            manager: Arc::new(RwLock::new(CassetteManager::new(cassette_dir))),
        }
    }
}

/// API error response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub status: u16,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

impl From<crate::error::MatgtoError> for ApiError {
    fn from(err: crate::error::MatgtoError) -> Self {
        use crate::error::MatgtoError;

        match err {
            MatgtoError::CassetteNotFound { name } => ApiError {
                error: "cassette_not_found".to_string(),
                message: format!("Cassette '{}' not found", name),
                status: 404,
            },
            MatgtoError::Io(e) => ApiError {
                error: "io_error".to_string(),
                message: e.to_string(),
                status: 500,
            },
            MatgtoError::Serialization(e) => ApiError {
                error: "serialization_error".to_string(),
                message: e.to_string(),
                status: 500,
            },
            _ => ApiError {
                error: "internal_error".to_string(),
                message: err.to_string(),
                status: 500,
            },
        }
    }
}

/// Query parameters for listing cassettes
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    /// Sort by: name, size, age, interactions (default: name)
    #[serde(default)]
    pub sort_by: SortBy,

    /// Sort order: asc, desc (default: asc)
    #[serde(default)]
    pub order: SortOrder,

    /// Filter by minimum age in days
    pub min_age_days: Option<i64>,

    /// Filter by maximum age in days
    pub max_age_days: Option<i64>,

    /// Filter by minimum size in bytes
    pub min_size_bytes: Option<u64>,

    /// Filter by maximum size in bytes
    pub max_size_bytes: Option<u64>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortBy {
    #[default]
    Name,
    Size,
    Age,
    Interactions,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    Asc,
    Desc,
}

/// Response for list endpoint
#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub cassettes: Vec<CassetteMetadata>,
    pub total: usize,
}

/// Response for export endpoint
#[derive(Debug, Serialize)]
pub struct ExportResponse {
    pub format: String,
    pub size_bytes: usize,
    pub download_url: String,
}

// ============================================================
// ENDPOINT HANDLERS
// ============================================================

/// GET /cassettes
/// List all cassettes with optional filtering and sorting
pub async fn list_cassettes(
    State(state): State<ApiState>,
    Query(query): Query<ListQuery>,
) -> std::result::Result<Json<ListResponse>, ApiError> {
    let manager = state.manager.read().await;
    let mut cassettes = manager.list_cassettes().map_err(ApiError::from)?;

    // Apply filters
    if let Some(min_age) = query.min_age_days {
        cassettes.retain(|c| c.age_days >= min_age);
    }
    if let Some(max_age) = query.max_age_days {
        cassettes.retain(|c| c.age_days <= max_age);
    }
    if let Some(min_size) = query.min_size_bytes {
        cassettes.retain(|c| c.size_bytes >= min_size);
    }
    if let Some(max_size) = query.max_size_bytes {
        cassettes.retain(|c| c.size_bytes <= max_size);
    }

    // Sort
    match query.sort_by {
        SortBy::Name => cassettes.sort_by(|a, b| a.name.cmp(&b.name)),
        SortBy::Size => cassettes.sort_by_key(|c| c.size_bytes),
        SortBy::Age => cassettes.sort_by_key(|c| c.age_days),
        SortBy::Interactions => cassettes.sort_by_key(|c| c.interaction_count),
    }

    if matches!(query.order, SortOrder::Desc) {
        cassettes.reverse();
    }

    let total = cassettes.len();

    Ok(Json(ListResponse { cassettes, total }))
}

/// GET /cassettes/:name
/// Get metadata for a specific cassette
pub async fn get_cassette(
    State(state): State<ApiState>,
    Path(name): Path<String>,
) -> std::result::Result<Json<CassetteMetadata>, ApiError> {
    let manager = state.manager.read().await;
    let _cassette = manager.load_cassette(&name).map_err(ApiError::from)?;

    // Convert Cassette to metadata
    let cassettes = manager.list_cassettes().map_err(ApiError::from)?;
    let metadata = cassettes
        .into_iter()
        .find(|c| c.name == name)
        .ok_or_else(|| ApiError {
            error: "cassette_not_found".to_string(),
            message: format!("Cassette '{}' not found", name),
            status: 404,
        })?;

    Ok(Json(metadata))
}

/// GET /cassettes/:name/stats
/// Get detailed statistics for a cassette
pub async fn get_cassette_stats(
    State(state): State<ApiState>,
    Path(name): Path<String>,
) -> std::result::Result<Json<CassetteStats>, ApiError> {
    let manager = state.manager.read().await;
    let stats = manager.get_cassette_stats(&name).map_err(ApiError::from)?;

    Ok(Json(stats))
}

/// GET /cassettes/:name/validate
/// Validate a cassette's integrity and format
pub async fn validate_cassette(
    State(state): State<ApiState>,
    Path(name): Path<String>,
) -> std::result::Result<Json<ValidationResult>, ApiError> {
    let manager = state.manager.read().await;
    let result = manager.validate_cassette(&name).map_err(ApiError::from)?;

    Ok(Json(result))
}

/// DELETE /cassettes/:name
/// Delete a cassette
pub async fn delete_cassette(
    State(state): State<ApiState>,
    Path(name): Path<String>,
) -> std::result::Result<StatusCode, ApiError> {
    let manager = state.manager.write().await;
    manager.delete_cassette(&name).map_err(ApiError::from)?;

    Ok(StatusCode::NO_CONTENT)
}

/// GET /cassettes/stats
/// Get global statistics across all cassettes
pub async fn get_global_stats(
    State(state): State<ApiState>,
) -> std::result::Result<Json<GlobalStats>, ApiError> {
    let manager = state.manager.read().await;
    let stats = manager.global_stats().map_err(ApiError::from)?;

    Ok(Json(stats))
}

/// POST /cassettes/:name/export
/// Export a cassette in different formats
#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    /// Format: json, msgpack, yaml, har (default: json)
    #[serde(default)]
    pub format: ExportFormat,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    #[default]
    Json,
    Msgpack,
    Yaml,
    Har, // HTTP Archive format (for browser tools)
}

pub async fn export_cassette(
    State(state): State<ApiState>,
    Path(name): Path<String>,
    Query(query): Query<ExportQuery>,
) -> std::result::Result<Json<ExportResponse>, ApiError> {
    let manager = state.manager.read().await;
    let _cassette = manager.load_cassette(&name).map_err(ApiError::from)?;

    // TODO: Implement actual export logic
    // For now, return metadata about what would be exported

    let format_str = match query.format {
        ExportFormat::Json => "json",
        ExportFormat::Msgpack => "msgpack",
        ExportFormat::Yaml => "yaml",
        ExportFormat::Har => "har",
    };

    Ok(Json(ExportResponse {
        format: format_str.to_string(),
        size_bytes: 0, // TODO: Calculate actual size
        download_url: format!(
            "/downloads/{}_{}.{}",
            name,
            chrono::Utc::now().timestamp(),
            format_str
        ),
    }))
}

/// GET /health
/// Health check endpoint
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0, // TODO: Track actual uptime
    })
}

// ============================================================
// ROUTER BUILDER
// ============================================================

/// Build the API router with all endpoints
pub fn build_router(state: ApiState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health))
        // Cassette management
        .route("/cassettes", get(list_cassettes))
        .route("/cassettes/stats", get(get_global_stats))
        .route("/cassettes/:name", get(get_cassette))
        .route("/cassettes/:name", delete(delete_cassette))
        .route("/cassettes/:name/stats", get(get_cassette_stats))
        .route("/cassettes/:name/validate", get(validate_cassette))
        .route("/cassettes/:name/export", post(export_cassette))
        // Shared state
        .with_state(state)
}

/// Start the API server
pub async fn start_server(
    host: &str,
    port: u16,
    cassette_dir: impl Into<std::path::PathBuf>,
) -> Result<()> {
    let state = ApiState::new(cassette_dir);
    let app = build_router(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("API server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_conversion() {
        let err = crate::error::MatgtoError::CassetteNotFound {
            name: "test".to_string(),
        };

        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, 404);
        assert_eq!(api_err.error, "cassette_not_found");
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let response = health().await;
        assert_eq!(response.0.status, "healthy");
    }
}
