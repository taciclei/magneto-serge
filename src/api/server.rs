//! HTTP server for Magneto-Serge REST API
//!
//! Provides REST endpoints to control the proxy remotely.

use super::{
    ApiConfig, ApiResponse, CassetteInfo, HydraLink, HydraOperation, ProxyStats, ProxyStatus,
    StartProxyRequest, StopProxyRequest,
};
use crate::proxy::MagnetoProxy;
use crate::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use serde::Serialize;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// API Server state
pub struct ApiServer {
    /// Configuration
    config: ApiConfig,

    /// Proxy instance
    proxy: Arc<RwLock<Option<Arc<MagnetoProxy>>>>,

    /// Start time
    start_time: std::time::Instant,

    /// Statistics
    stats: Arc<RwLock<ProxyStats>>,
}

impl ApiServer {
    /// Create a new API server
    pub fn new(config: ApiConfig) -> Self {
        Self {
            config,
            proxy: Arc::new(RwLock::new(None)),
            start_time: std::time::Instant::now(),
            stats: Arc::new(RwLock::new(ProxyStats {
                total_requests: 0,
                total_responses: 0,
                requests_per_second: 0.0,
                avg_response_time_ms: 0.0,
                cache_hit_rate: 0.0,
                memory_mb: 0.0,
                metrics: std::collections::HashMap::new(),
            })),
        }
    }

    /// Start the API server
    pub async fn start(self) -> Result<()> {
        let addr: SocketAddr = format!("{}:{}", self.config.host, self.config.port)
            .parse()
            .map_err(|e| crate::MatgtoError::Config(format!("Invalid address: {}", e)))?;

        let server = Arc::new(self);

        let make_svc = make_service_fn(move |_conn| {
            let server = Arc::clone(&server);
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let server = Arc::clone(&server);
                    async move { server.handle_request(req).await }
                }))
            }
        });

        tracing::info!("🌐 API Server starting on http://{}", addr);

        Server::bind(&addr).serve(make_svc).await.map_err(|e| {
            crate::MatgtoError::ProxyStartFailed {
                reason: format!("Failed to start API server: {}", e),
            }
        })?;

        Ok(())
    }

    /// Handle incoming HTTP request
    async fn handle_request(
        &self,
        req: Request<Body>,
    ) -> std::result::Result<Response<Body>, Infallible> {
        // Check authentication
        if self.config.auth_enabled {
            if let Some(api_key) = &self.config.api_key {
                let auth_header = req
                    .headers()
                    .get("Authorization")
                    .and_then(|v| v.to_str().ok());

                if auth_header != Some(&format!("Bearer {}", api_key)) {
                    return Ok(Self::json_response(
                        StatusCode::UNAUTHORIZED,
                        ApiResponse::<()>::error("Unauthorized: Invalid or missing API key"),
                    ));
                }
            }
        }

        let method = req.method().clone();
        let path = req.uri().path().to_string();

        tracing::debug!("API Request: {} {}", method, path);

        // Route the request
        let response = match (method.as_str(), path.as_str()) {
            // API Root (entry point)
            ("GET", "/") => self.handle_api_root().await,

            // OpenAPI specification
            ("GET", "/openapi.json") => self.handle_openapi().await,

            // Health check
            ("GET", "/health") => self.handle_health().await,

            // Proxy control
            ("POST", "/proxy/start") => self.handle_start_proxy(req).await,
            ("POST", "/proxy/stop") => self.handle_stop_proxy(req).await,
            ("GET", "/proxy/status") => self.handle_proxy_status().await,
            ("GET", "/proxy/stats") => self.handle_proxy_stats().await,

            // Cassette management
            ("GET", "/cassettes") => self.handle_list_cassettes().await,
            ("GET", path) if path.starts_with("/cassettes/") => {
                let name = path.trim_start_matches("/cassettes/");
                self.handle_get_cassette(name).await
            }
            ("DELETE", path) if path.starts_with("/cassettes/") => {
                let name = path.trim_start_matches("/cassettes/");
                self.handle_delete_cassette(name).await
            }

            // 404 Not Found
            _ => Ok(Self::json_response(
                StatusCode::NOT_FOUND,
                ApiResponse::<()>::error(format!("Not found: {} {}", method, path)),
            )),
        };

        Ok(response.unwrap_or_else(|e| {
            tracing::error!("API error: {}", e);
            Self::json_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiResponse::<()>::error(format!("Internal error: {}", e)),
            )
        }))
    }

    /// Handle API root (entry point)
    async fn handle_api_root(&self) -> Result<Response<Body>> {
        let links = self.generate_api_links("/");

        Ok(Self::json_response(
            StatusCode::OK,
            ApiResponse::success_with_links(
                serde_json::json!({
                    "@id": format!("http://{}:{}/", self.config.host, self.config.port),
                    "title": "Magneto-Serge API",
                    "description": "REST API for controlling Magneto-Serge HTTP/WebSocket proxy",
                    "version": env!("CARGO_PKG_VERSION"),
                    "documentation": "https://github.com/taciclei/magneto-serge",
                    "openapi": format!("http://{}:{}/openapi.json", self.config.host, self.config.port),
                }),
                links,
            ),
        ))
    }

    /// Handle OpenAPI specification request
    async fn handle_openapi(&self) -> Result<Response<Body>> {
        let spec = super::openapi::generate_openapi_spec(&self.config.host, self.config.port);
        let json = serde_json::to_string_pretty(&spec).unwrap_or_else(|_| "{}".to_string());

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(Body::from(json))
            .unwrap())
    }

    /// Handle health check
    async fn handle_health(&self) -> Result<Response<Body>> {
        let links = self.generate_api_links("/health");
        Ok(Self::json_response(
            StatusCode::OK,
            ApiResponse::success_with_links(
                serde_json::json!({
                    "status": "healthy",
                    "uptime_seconds": self.start_time.elapsed().as_secs(),
                }),
                links,
            ),
        ))
    }

    /// Handle start proxy request
    async fn handle_start_proxy(&self, req: Request<Body>) -> Result<Response<Body>> {
        let body = hyper::body::to_bytes(req.into_body())
            .await
            .map_err(|e| crate::MatgtoError::Http(format!("Failed to read body: {}", e)))?;

        let start_req: StartProxyRequest =
            serde_json::from_slice(&body).map_err(crate::MatgtoError::Serialization)?;

        // Check if proxy is already running
        {
            let proxy_lock = self.proxy.read().await;
            if proxy_lock.is_some() {
                return Ok(Self::json_response(
                    StatusCode::CONFLICT,
                    ApiResponse::<()>::error("Proxy is already running"),
                ));
            }
        }

        // Create proxy
        let cassette_dir = PathBuf::from(&self.config.cassette_dir);
        let proxy = MagnetoProxy::new_internal(&cassette_dir).map_err(|e| {
            crate::MatgtoError::ProxyStartFailed {
                reason: format!("Failed to create proxy: {}", e),
            }
        })?;

        let proxy = Arc::new(proxy);
        let port = start_req.port.unwrap_or(self.config.proxy_port);

        proxy.set_port(port);

        // Set mode
        let success = match start_req.mode.as_str() {
            "auto" => {
                proxy.auto(&start_req.cassette_name);
                true
            }
            "record" => proxy.start_recording(start_req.cassette_name.clone()),
            "replay" => {
                if start_req.strict {
                    proxy.replay_strict(start_req.cassette_name.clone())
                } else {
                    proxy.replay(start_req.cassette_name.clone())
                }
            }
            "passthrough" => {
                proxy.passthrough();
                true
            }
            _ => {
                return Ok(Self::json_response(
                    StatusCode::BAD_REQUEST,
                    ApiResponse::<()>::error(format!("Invalid mode: {}", start_req.mode)),
                ));
            }
        };

        if !success {
            return Ok(Self::json_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiResponse::<()>::error("Failed to start proxy"),
            ));
        }

        // Store proxy
        {
            let mut proxy_lock = self.proxy.write().await;
            *proxy_lock = Some(Arc::clone(&proxy));
        }

        // Start proxy in background
        let proxy_clone = Arc::clone(&proxy);
        tokio::spawn(async move {
            if let Err(e) = proxy_clone.start().await {
                tracing::error!("Proxy error: {}", e);
            }
        });

        // Wait a bit for startup
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Create links to status and stop operations
        let base_url = format!("http://{}:{}", self.config.host, self.config.port);
        let links = vec![
            HydraLink {
                link_type: "hydra:Link".to_string(),
                target: format!("{}/proxy/status", base_url),
                title: Some("Check Proxy Status".to_string()),
                operations: Some(vec![HydraOperation {
                    operation_type: "http://schema.org/ViewAction".to_string(),
                    method: "GET".to_string(),
                    expects: None,
                    returns: Some("application/ld+json".to_string()),
                }]),
            },
            HydraLink {
                link_type: "hydra:Link".to_string(),
                target: format!("{}/proxy/stop", base_url),
                title: Some("Stop Proxy".to_string()),
                operations: Some(vec![HydraOperation {
                    operation_type: "http://schema.org/DeactivateAction".to_string(),
                    method: "POST".to_string(),
                    expects: Some("StopProxyRequest".to_string()),
                    returns: Some("application/ld+json".to_string()),
                }]),
            },
        ];

        Ok(Self::json_response(
            StatusCode::OK,
            ApiResponse::success_with_links(
                serde_json::json!({
                    "message": "Proxy started successfully",
                    "mode": start_req.mode,
                    "cassette": start_req.cassette_name,
                    "port": port,
                }),
                links,
            ),
        ))
    }

    /// Handle stop proxy request
    async fn handle_stop_proxy(&self, req: Request<Body>) -> Result<Response<Body>> {
        let body = hyper::body::to_bytes(req.into_body())
            .await
            .unwrap_or_else(|_| hyper::body::Bytes::from("{}"));

        let _stop_req: StopProxyRequest =
            serde_json::from_slice(&body).unwrap_or(StopProxyRequest { force: false });

        // Get and remove proxy
        let proxy = {
            let mut proxy_lock = self.proxy.write().await;
            proxy_lock.take()
        };

        if let Some(proxy) = proxy {
            proxy.shutdown();

            Ok(Self::json_response(
                StatusCode::OK,
                ApiResponse::success(serde_json::json!({
                    "message": "Proxy stopped successfully",
                })),
            ))
        } else {
            Ok(Self::json_response(
                StatusCode::NOT_FOUND,
                ApiResponse::<()>::error("Proxy is not running"),
            ))
        }
    }

    /// Handle proxy status request
    async fn handle_proxy_status(&self) -> Result<Response<Body>> {
        let proxy_lock = self.proxy.read().await;
        let links = self.generate_api_links("/proxy/status");

        if let Some(proxy) = proxy_lock.as_ref() {
            let status = ProxyStatus {
                running: true,
                mode: format!("{:?}", proxy.mode()),
                port: proxy.port(),
                cassette: proxy.current_cassette_name(),
                interactions_count: 0, // TODO: Get from player
                uptime_seconds: self.start_time.elapsed().as_secs(),
            };

            Ok(Self::json_response(
                StatusCode::OK,
                ApiResponse::success_with_links(status, links),
            ))
        } else {
            let status = ProxyStatus {
                running: false,
                mode: "None".to_string(),
                port: 0,
                cassette: None,
                interactions_count: 0,
                uptime_seconds: 0,
            };

            Ok(Self::json_response(
                StatusCode::OK,
                ApiResponse::success_with_links(status, links),
            ))
        }
    }

    /// Handle proxy stats request
    async fn handle_proxy_stats(&self) -> Result<Response<Body>> {
        let stats = self.stats.read().await;
        Ok(Self::json_response(
            StatusCode::OK,
            ApiResponse::success(stats.clone()),
        ))
    }

    /// Handle list cassettes request
    async fn handle_list_cassettes(&self) -> Result<Response<Body>> {
        let cassette_dir = PathBuf::from(&self.config.cassette_dir);

        if !cassette_dir.exists() {
            return Ok(Self::json_response(
                StatusCode::OK,
                ApiResponse::success(Vec::<CassetteInfo>::new()),
            ));
        }

        let mut cassettes = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&cassette_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(name) = entry.file_name().to_str() {
                            if name.ends_with(".json") || name.ends_with(".msgpack") {
                                let format = if name.ends_with(".json.gz") {
                                    "json.gz"
                                } else if name.ends_with(".msgpack.gz") {
                                    "msgpack.gz"
                                } else if name.ends_with(".json") {
                                    "json"
                                } else {
                                    "msgpack"
                                };

                                cassettes.push(CassetteInfo {
                                    name: name.to_string(),
                                    size_bytes: metadata.len(),
                                    interactions: 0, // TODO: Parse file to get count
                                    created_at: chrono::DateTime::<chrono::Utc>::from(
                                        metadata.created().unwrap_or(std::time::SystemTime::now()),
                                    )
                                    .to_rfc3339(),
                                    format: format.to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(Self::json_response(
            StatusCode::OK,
            ApiResponse::success(cassettes),
        ))
    }

    /// Handle get cassette request
    async fn handle_get_cassette(&self, name: &str) -> Result<Response<Body>> {
        let cassette_dir = PathBuf::from(&self.config.cassette_dir);
        let cassette_path = cassette_dir.join(name);

        if !cassette_path.exists() {
            return Ok(Self::json_response(
                StatusCode::NOT_FOUND,
                ApiResponse::<()>::error(format!("Cassette not found: {}", name)),
            ));
        }

        // Read cassette file
        let content = tokio::fs::read_to_string(&cassette_path)
            .await
            .map_err(crate::MatgtoError::Io)?;

        Ok(Self::json_response(
            StatusCode::OK,
            ApiResponse::success(serde_json::json!({
                "name": name,
                "content": content,
            })),
        ))
    }

    /// Handle delete cassette request
    async fn handle_delete_cassette(&self, name: &str) -> Result<Response<Body>> {
        let cassette_dir = PathBuf::from(&self.config.cassette_dir);
        let cassette_path = cassette_dir.join(name);

        if !cassette_path.exists() {
            return Ok(Self::json_response(
                StatusCode::NOT_FOUND,
                ApiResponse::<()>::error(format!("Cassette not found: {}", name)),
            ));
        }

        tokio::fs::remove_file(&cassette_path)
            .await
            .map_err(crate::MatgtoError::Io)?;

        Ok(Self::json_response(
            StatusCode::OK,
            ApiResponse::success(serde_json::json!({
                "message": format!("Cassette deleted: {}", name),
            })),
        ))
    }

    /// Create JSON response
    fn json_response<T: Serialize>(status: StatusCode, data: T) -> Response<Body> {
        let json = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());

        Response::builder()
            .status(status)
            .header("Content-Type", "application/ld+json")
            .header("Access-Control-Allow-Origin", "*")
            .body(Body::from(json))
            .unwrap()
    }

    /// Generate API navigation links
    fn generate_api_links(&self, current_path: &str) -> Vec<HydraLink> {
        let base_url = format!("http://{}:{}", self.config.host, self.config.port);
        let mut links = vec![];

        // Link to API root (entrypoint)
        if current_path != "/" {
            links.push(HydraLink {
                link_type: "hydra:Link".to_string(),
                target: format!("{}/", base_url),
                title: Some("API Root".to_string()),
                operations: None,
            });
        }

        // Link to health check
        if current_path != "/health" {
            links.push(HydraLink {
                link_type: "hydra:Link".to_string(),
                target: format!("{}/health", base_url),
                title: Some("Health Check".to_string()),
                operations: Some(vec![HydraOperation {
                    operation_type: "http://schema.org/CheckAction".to_string(),
                    method: "GET".to_string(),
                    expects: None,
                    returns: Some("application/ld+json".to_string()),
                }]),
            });
        }

        // Link to proxy status
        if current_path != "/proxy/status" {
            links.push(HydraLink {
                link_type: "hydra:Link".to_string(),
                target: format!("{}/proxy/status", base_url),
                title: Some("Proxy Status".to_string()),
                operations: Some(vec![HydraOperation {
                    operation_type: "http://schema.org/ViewAction".to_string(),
                    method: "GET".to_string(),
                    expects: None,
                    returns: Some("application/ld+json".to_string()),
                }]),
            });
        }

        // Link to proxy control operations
        if !current_path.starts_with("/proxy") {
            links.push(HydraLink {
                link_type: "hydra:Link".to_string(),
                target: format!("{}/proxy/start", base_url),
                title: Some("Start Proxy".to_string()),
                operations: Some(vec![HydraOperation {
                    operation_type: "http://schema.org/ActivateAction".to_string(),
                    method: "POST".to_string(),
                    expects: Some("StartProxyRequest".to_string()),
                    returns: Some("application/ld+json".to_string()),
                }]),
            });

            links.push(HydraLink {
                link_type: "hydra:Link".to_string(),
                target: format!("{}/proxy/stop", base_url),
                title: Some("Stop Proxy".to_string()),
                operations: Some(vec![HydraOperation {
                    operation_type: "http://schema.org/DeactivateAction".to_string(),
                    method: "POST".to_string(),
                    expects: Some("StopProxyRequest".to_string()),
                    returns: Some("application/ld+json".to_string()),
                }]),
            });

            links.push(HydraLink {
                link_type: "hydra:Link".to_string(),
                target: format!("{}/proxy/stats", base_url),
                title: Some("Proxy Statistics".to_string()),
                operations: Some(vec![HydraOperation {
                    operation_type: "http://schema.org/ViewAction".to_string(),
                    method: "GET".to_string(),
                    expects: None,
                    returns: Some("application/ld+json".to_string()),
                }]),
            });
        }

        // Link to cassettes collection
        if current_path != "/cassettes" {
            links.push(HydraLink {
                link_type: "hydra:Collection".to_string(),
                target: format!("{}/cassettes", base_url),
                title: Some("Cassettes Collection".to_string()),
                operations: Some(vec![HydraOperation {
                    operation_type: "http://schema.org/SearchAction".to_string(),
                    method: "GET".to_string(),
                    expects: None,
                    returns: Some("application/ld+json".to_string()),
                }]),
            });
        }

        links
    }
}
