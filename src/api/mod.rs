//! REST API for controlling Magneto-Serge proxy
//!
//! This module provides an HTTP REST API to control the proxy remotely,
//! allowing you to start, stop, and monitor the proxy server via HTTP requests.

pub mod cassettes;
pub mod handlers;
pub mod openapi;
pub mod server;

pub use cassettes::CassetteManager;
pub use handlers::{build_router, build_router as create_router};
pub use openapi::{generate_openapi_spec, OpenApiSpec};
pub use server::ApiServer;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API server configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// API server host
    pub host: String,

    /// API server port
    pub port: u16,

    /// Proxy configuration
    pub proxy_port: u16,

    /// Cassette directory
    pub cassette_dir: String,

    /// Enable authentication
    pub auth_enabled: bool,

    /// API key for authentication
    pub api_key: Option<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8889,
            proxy_port: 8888,
            cassette_dir: "./cassettes".to_string(),
            auth_enabled: false,
            api_key: None,
        }
    }
}

/// Proxy status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyStatus {
    /// Is proxy running
    pub running: bool,

    /// Current mode
    pub mode: String,

    /// Proxy port
    pub port: u16,

    /// Current cassette name
    pub cassette: Option<String>,

    /// Number of interactions recorded/replayed
    pub interactions_count: usize,

    /// Uptime in seconds
    pub uptime_seconds: u64,
}

/// Start proxy request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartProxyRequest {
    /// Proxy mode (auto, record, replay, passthrough)
    pub mode: String,

    /// Cassette name
    pub cassette_name: String,

    /// Proxy port (optional, defaults to config)
    pub port: Option<u16>,

    /// Strict mode for replay
    #[serde(default)]
    pub strict: bool,
}

/// Stop proxy request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopProxyRequest {
    /// Force stop
    #[serde(default)]
    pub force: bool,
}

/// List cassettes response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CassetteInfo {
    /// Cassette name
    pub name: String,

    /// File size in bytes
    pub size_bytes: u64,

    /// Number of interactions
    pub interactions: usize,

    /// Created timestamp
    pub created_at: String,

    /// Format (json, msgpack, etc.)
    pub format: String,
}

/// Hydra operation for hypermedia controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraOperation {
    /// Operation type (e.g., "http://schema.org/UpdateAction")
    #[serde(rename = "@type")]
    pub operation_type: String,

    /// HTTP method
    pub method: String,

    /// Expected input (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expects: Option<String>,

    /// Returned output (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returns: Option<String>,
}

/// Hypermedia link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydraLink {
    /// Link relation type
    #[serde(rename = "@type")]
    pub link_type: String,

    /// Target URL
    #[serde(rename = "hydra:target")]
    pub target: String,

    /// Link title (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Supported operations
    #[serde(skip_serializing_if = "Option::is_none", rename = "hydra:operation")]
    pub operations: Option<Vec<HydraOperation>>,
}

/// API response wrapper with Hydra/JSON-LD support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// JSON-LD context
    #[serde(rename = "@context")]
    pub context: String,

    /// Resource type
    #[serde(rename = "@type")]
    pub resource_type: String,

    /// Success status
    pub success: bool,

    /// Response data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Timestamp
    pub timestamp: String,

    /// Hydra links for navigation
    #[serde(skip_serializing_if = "Option::is_none", rename = "hydra:link")]
    pub links: Option<Vec<HydraLink>>,
}

impl<T> ApiResponse<T> {
    /// Create success response with Hydra support
    pub fn success(data: T) -> Self {
        Self {
            context: "https://www.w3.org/ns/hydra/core".to_string(),
            resource_type: "hydra:Resource".to_string(),
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            links: None,
        }
    }

    /// Create success response with links
    pub fn success_with_links(data: T, links: Vec<HydraLink>) -> Self {
        Self {
            context: "https://www.w3.org/ns/hydra/core".to_string(),
            resource_type: "hydra:Resource".to_string(),
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            links: Some(links),
        }
    }

    /// Create error response
    pub fn error(message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            context: "https://www.w3.org/ns/hydra/core".to_string(),
            resource_type: "hydra:Error".to_string(),
            success: false,
            data: None,
            error: Some(message.into()),
            timestamp: chrono::Utc::now().to_rfc3339(),
            links: None,
        }
    }
}

/// Proxy statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyStats {
    /// Total requests processed
    pub total_requests: usize,

    /// Total responses sent
    pub total_responses: usize,

    /// Requests per second (last minute)
    pub requests_per_second: f64,

    /// Average response time (ms)
    pub avg_response_time_ms: f64,

    /// Cache hit rate (%)
    pub cache_hit_rate: f64,

    /// Memory usage (MB)
    pub memory_mb: f64,

    /// Additional metrics
    pub metrics: HashMap<String, f64>,
}
