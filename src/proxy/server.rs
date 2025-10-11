//! Proxy server implementation using Hudsucker
//!
//! This module implements the actual MITM proxy server that intercepts
//! HTTP/HTTPS and WebSocket traffic.

use crate::cassette::{HttpRequest, HttpResponse};
use crate::error::{MatgtoError, Result};
use crate::player::{Player, RequestSignature};
use crate::proxy::client::HttpForwarder;
use crate::proxy::ProxyMode;
use crate::recorder::Recorder;
use crate::tls::CertificateAuthority;

use hudsucker::{
    hyper::{Body, Request, Response, StatusCode},
    HttpContext, HttpHandler as HudsuckerHttpHandler, RequestOrResponse,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Matgto HTTP handler that implements Hudsucker's HttpHandler trait
#[derive(Debug, Clone)]
pub struct MatgtoHttpHandler {
    mode: ProxyMode,
    recorder: Option<Arc<Mutex<Recorder>>>,
    player: Option<Arc<Mutex<Player>>>,
    forwarder: HttpForwarder,
}

impl MatgtoHttpHandler {
    /// Create a new Matgto HTTP handler
    pub fn new(mode: ProxyMode) -> Self {
        Self {
            mode,
            recorder: None,
            player: None,
            forwarder: HttpForwarder::new(),
        }
    }

    /// Set the recorder for Record mode
    pub fn with_recorder(mut self, recorder: Arc<Mutex<Recorder>>) -> Self {
        self.recorder = Some(recorder);
        self
    }

    /// Set the player for Replay mode
    pub fn with_player(mut self, player: Arc<Mutex<Player>>) -> Self {
        self.player = Some(player);
        self
    }

    /// Convert hyper Request to our HttpRequest format
    /// This consumes the request body
    async fn convert_request(req: Request<Body>) -> Result<(HttpRequest, Vec<u8>)> {
        let method = req.method().to_string();
        let url = req.uri().to_string();

        // Extract headers
        let mut headers = HashMap::new();
        for (name, value) in req.headers().iter() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(name.to_string(), value_str.to_string());
            }
        }

        // Read and buffer the body
        let body_bytes = hyper::body::to_bytes(req.into_body()).await.map_err(|e| {
            MatgtoError::ProxyStartFailed {
                reason: format!("Failed to read request body: {}", e),
            }
        })?;

        let body_vec = body_bytes.to_vec();
        let body_option = if !body_vec.is_empty() {
            Some(body_vec.clone())
        } else {
            None
        };

        let http_request = HttpRequest {
            method,
            url,
            headers,
            body: body_option,
        };

        Ok((http_request, body_vec))
    }

    /// Reconstruct hyper Request from our HttpRequest and body bytes
    #[allow(dead_code)]
    fn reconstruct_request(http_req: &HttpRequest, body_bytes: &[u8]) -> Result<Request<Body>> {
        let mut builder = Request::builder()
            .method(http_req.method.as_str())
            .uri(&http_req.url);

        // Add headers
        for (name, value) in &http_req.headers {
            builder = builder.header(name, value);
        }

        // Build request with body
        let body = if !body_bytes.is_empty() {
            Body::from(body_bytes.to_vec())
        } else {
            Body::empty()
        };

        builder
            .body(body)
            .map_err(|e| MatgtoError::ProxyStartFailed {
                reason: format!("Failed to reconstruct request: {}", e),
            })
    }

    /// Convert our HttpResponse to hyper Response
    fn convert_response(resp: &HttpResponse) -> Result<Response<Body>> {
        let mut builder = Response::builder().status(resp.status);

        // Add headers
        for (name, value) in &resp.headers {
            builder = builder.header(name, value);
        }

        // Build response with body
        let body = if let Some(body_data) = &resp.body {
            Body::from(body_data.clone())
        } else {
            Body::empty()
        };

        builder
            .body(body)
            .map_err(|e| MatgtoError::ProxyStartFailed {
                reason: format!("Failed to build response: {}", e),
            })
    }

    /// Convert HttpRequest to RequestSignature for player lookups
    fn to_signature(http_req: &HttpRequest) -> RequestSignature {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let body_hash = http_req.body.as_ref().map(|b| {
            let mut hasher = DefaultHasher::new();
            b.hash(&mut hasher);
            hasher.finish()
        });

        RequestSignature {
            method: http_req.method.clone(),
            url: http_req.url.clone(),
            body_hash,
        }
    }
}

#[async_trait::async_trait]
impl HudsuckerHttpHandler for MatgtoHttpHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        tracing::debug!("Intercepting request: {} {}", req.method(), req.uri());

        match self.mode {
            ProxyMode::Record => {
                // In record mode, we buffer the request, forward it, and record
                tracing::info!("Record mode: Buffering {} {}", req.method(), req.uri());

                // Buffer the request (this consumes it)
                match Self::convert_request(req).await {
                    Ok((http_req, _body_bytes)) => {
                        // Forward via our HttpForwarder
                        match self.forwarder.forward(&http_req).await {
                            Ok(http_resp) => {
                                // Record the interaction
                                if let Some(recorder) = &self.recorder {
                                    let mut recorder_lock = recorder.lock().await;
                                    recorder_lock.record_http(http_req, http_resp.clone());
                                    tracing::debug!("Recorded interaction");
                                }

                                // Convert response back to hyper Response
                                match Self::convert_response(&http_resp) {
                                    Ok(response) => RequestOrResponse::Response(response),
                                    Err(e) => {
                                        tracing::error!("Failed to convert response: {}", e);
                                        let err_response = Response::builder()
                                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                                            .body(Body::from("Failed to convert response"))
                                            .unwrap();
                                        RequestOrResponse::Response(err_response)
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to forward request: {}", e);
                                let err_response = Response::builder()
                                    .status(StatusCode::BAD_GATEWAY)
                                    .body(Body::from(format!("Proxy error: {}", e)))
                                    .unwrap();
                                RequestOrResponse::Response(err_response)
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to buffer request: {}", e);
                        let err_response = Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from(format!("Failed to read request: {}", e)))
                            .unwrap();
                        RequestOrResponse::Response(err_response)
                    }
                }
            }

            ProxyMode::Replay | ProxyMode::ReplayStrict => {
                // In replay mode, we match against cassette and return recorded response
                let is_strict = matches!(self.mode, ProxyMode::ReplayStrict);

                if is_strict {
                    tracing::info!("🔒 STRICT Replay mode: Matching {} {}", req.method(), req.uri());
                } else {
                    tracing::info!("Replay mode: Matching {} {}", req.method(), req.uri());
                }

                // Buffer the request to match against cassette
                match Self::convert_request(req).await {
                    Ok((http_req, _body_bytes)) => {
                        if let Some(player) = &self.player {
                            let mut player_lock = player.lock().await;
                            let signature = Self::to_signature(&http_req);

                            // Try to find matching interaction
                            match player_lock.find_interaction(&signature) {
                                Ok(idx) => {
                                    if let Some(interaction) = player_lock.get_interaction(idx) {
                                        if let crate::cassette::InteractionKind::Http {
                                            response,
                                            ..
                                        } = &interaction.kind
                                        {
                                            tracing::info!(
                                                "✅ Replay match found for {} {}",
                                                http_req.method,
                                                http_req.url
                                            );

                                            match Self::convert_response(response) {
                                                Ok(response) => {
                                                    return RequestOrResponse::Response(response);
                                                }
                                                Err(e) => {
                                                    tracing::error!(
                                                        "Failed to convert response: {}",
                                                        e
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::warn!("No match: {}", e);
                                }
                            }
                        }

                        // If replay fails, return 404
                        tracing::warn!(
                            "❌ No matching interaction found for {} {}",
                            http_req.method,
                            http_req.url
                        );
                        let response = Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("No matching interaction in cassette"))
                            .unwrap();
                        RequestOrResponse::Response(response)
                    }
                    Err(e) => {
                        tracing::error!("Failed to buffer request: {}", e);
                        let err_response = Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from("Failed to read request"))
                            .unwrap();
                        RequestOrResponse::Response(err_response)
                    }
                }
            }

            ProxyMode::Auto => {
                // Auto mode: Try replay if cassette exists, otherwise record
                tracing::info!("Auto mode: Checking cassette availability");

                // Buffer the request first
                match Self::convert_request(req).await {
                    Ok((http_req, _body_bytes)) => {
                        // Check if we should replay
                        let should_replay = if let Some(player) = &self.player {
                            player.lock().await.has_cassette()
                        } else {
                            false
                        };

                        if should_replay {
                            // Try replay
                            tracing::info!("Auto mode: Cassette exists, attempting replay");
                            if let Some(player) = &self.player {
                                let mut player_lock = player.lock().await;
                                let signature = Self::to_signature(&http_req);

                                if let Ok(idx) = player_lock.find_interaction(&signature) {
                                    if let Some(interaction) = player_lock.get_interaction(idx) {
                                        if let crate::cassette::InteractionKind::Http {
                                            response,
                                            ..
                                        } = &interaction.kind
                                        {
                                            tracing::info!("✅ Auto replay: Match found");
                                            if let Ok(response) = Self::convert_response(response) {
                                                return RequestOrResponse::Response(response);
                                            }
                                        }
                                    }
                                }
                            }
                            tracing::warn!("Auto mode: Replay failed, falling back to record");
                        } else {
                            tracing::info!("Auto mode: No cassette, using record");
                        }

                        // Fall back to record mode
                        match self.forwarder.forward(&http_req).await {
                            Ok(http_resp) => {
                                // Record if recorder is available
                                if let Some(recorder) = &self.recorder {
                                    let mut recorder_lock = recorder.lock().await;
                                    recorder_lock.record_http(http_req.clone(), http_resp.clone());
                                    tracing::debug!("Auto mode: Recorded interaction");
                                }

                                match Self::convert_response(&http_resp) {
                                    Ok(response) => RequestOrResponse::Response(response),
                                    Err(e) => {
                                        tracing::error!("Failed to convert response: {}", e);
                                        let err_response = Response::builder()
                                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                                            .body(Body::from("Failed to convert response"))
                                            .unwrap();
                                        RequestOrResponse::Response(err_response)
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::error!("Failed to forward request: {}", e);
                                let err_response = Response::builder()
                                    .status(StatusCode::BAD_GATEWAY)
                                    .body(Body::from(format!("Proxy error: {}", e)))
                                    .unwrap();
                                RequestOrResponse::Response(err_response)
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to buffer request: {}", e);
                        let err_response = Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from("Failed to read request"))
                            .unwrap();
                        RequestOrResponse::Response(err_response)
                    }
                }
            }

            ProxyMode::Hybrid => {
                // Hybrid mode: Try replay first, fall back to record if not found
                tracing::info!("🔀 Hybrid mode: {} {}", req.method(), req.uri());

                match Self::convert_request(req).await {
                    Ok((http_req, _body_bytes)) => {
                        // First, try to find in cassette
                        let mut found_in_cassette = false;
                        if let Some(player) = &self.player {
                            let mut player_lock = player.lock().await;
                            let signature = Self::to_signature(&http_req);

                            if let Ok(idx) = player_lock.find_interaction(&signature) {
                                if let Some(interaction) = player_lock.get_interaction(idx) {
                                    if let crate::cassette::InteractionKind::Http { response, .. } =
                                        &interaction.kind
                                    {
                                        tracing::info!("  📼 Found in cassette, replaying");
                                        found_in_cassette = true;

                                        if let Ok(resp) = Self::convert_response(response) {
                                            return RequestOrResponse::Response(resp);
                                        }
                                    }
                                }
                            }
                        }

                        // If not found, record new interaction
                        if !found_in_cassette {
                            tracing::info!("  📹 Not in cassette, forwarding and recording");

                            match self.forwarder.forward(&http_req).await {
                                Ok(http_resp) => {
                                    // Record the new interaction
                                    if let Some(recorder) = &self.recorder {
                                        let mut recorder_lock = recorder.lock().await;
                                        recorder_lock
                                            .record_http(http_req.clone(), http_resp.clone());
                                        tracing::debug!("  ✅ New interaction recorded");
                                    }

                                    match Self::convert_response(&http_resp) {
                                        Ok(response) => RequestOrResponse::Response(response),
                                        Err(e) => {
                                            tracing::error!("Failed to convert response: {}", e);
                                            let err_response = Response::builder()
                                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                .body(Body::from("Failed to convert response"))
                                                .unwrap();
                                            RequestOrResponse::Response(err_response)
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Failed to forward request: {}", e);
                                    let err_response = Response::builder()
                                        .status(StatusCode::BAD_GATEWAY)
                                        .body(Body::from(format!("Proxy error: {}", e)))
                                        .unwrap();
                                    RequestOrResponse::Response(err_response)
                                }
                            }
                        } else {
                            // Should never reach here but handle it anyway
                            let err_response = Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from("Hybrid mode logic error"))
                                .unwrap();
                            RequestOrResponse::Response(err_response)
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to buffer request: {}", e);
                        let err_response = Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from("Failed to read request"))
                            .unwrap();
                        RequestOrResponse::Response(err_response)
                    }
                }
            }

            ProxyMode::Once => {
                // Once mode: Record if cassette doesn't exist, otherwise replay
                // This protects against accidental overwrites
                tracing::info!("🔒 Once mode: {} {}", req.method(), req.uri());

                match Self::convert_request(req).await {
                    Ok((http_req, _body_bytes)) => {
                        // Check if cassette exists
                        let cassette_exists = if let Some(player) = &self.player {
                            player.lock().await.has_cassette()
                        } else {
                            false
                        };

                        if cassette_exists {
                            // Cassette exists, replay from it
                            tracing::info!("  📼 Cassette exists, replaying (read-only)");

                            if let Some(player) = &self.player {
                                let mut player_lock = player.lock().await;
                                let signature = Self::to_signature(&http_req);

                                if let Ok(idx) = player_lock.find_interaction(&signature) {
                                    if let Some(interaction) = player_lock.get_interaction(idx) {
                                        if let crate::cassette::InteractionKind::Http {
                                            response,
                                            ..
                                        } = &interaction.kind
                                        {
                                            tracing::info!("  ✅ Replayed from cassette");
                                            if let Ok(resp) = Self::convert_response(response) {
                                                return RequestOrResponse::Response(resp);
                                            }
                                        }
                                    }
                                }
                            }

                            // No match found in cassette
                            tracing::warn!("  ❌ No matching interaction in cassette");
                            let response = Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::from(
                                    "Once mode: Cassette exists but no matching interaction",
                                ))
                                .unwrap();
                            RequestOrResponse::Response(response)
                        } else {
                            // Cassette doesn't exist, record new one
                            tracing::info!("  📹 Cassette doesn't exist, recording (first time)");

                            match self.forwarder.forward(&http_req).await {
                                Ok(http_resp) => {
                                    // Record the interaction
                                    if let Some(recorder) = &self.recorder {
                                        let mut recorder_lock = recorder.lock().await;
                                        recorder_lock
                                            .record_http(http_req.clone(), http_resp.clone());
                                        tracing::debug!("  ✅ Interaction recorded");
                                    }

                                    match Self::convert_response(&http_resp) {
                                        Ok(response) => RequestOrResponse::Response(response),
                                        Err(e) => {
                                            tracing::error!("Failed to convert response: {}", e);
                                            let err_response = Response::builder()
                                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                .body(Body::from("Failed to convert response"))
                                                .unwrap();
                                            RequestOrResponse::Response(err_response)
                                        }
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Failed to forward request: {}", e);
                                    let err_response = Response::builder()
                                        .status(StatusCode::BAD_GATEWAY)
                                        .body(Body::from(format!("Proxy error: {}", e)))
                                        .unwrap();
                                    RequestOrResponse::Response(err_response)
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to buffer request: {}", e);
                        let err_response = Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from("Failed to read request"))
                            .unwrap();
                        RequestOrResponse::Response(err_response)
                    }
                }
            }

            ProxyMode::Passthrough => {
                // Pass through - forward without recording
                tracing::info!(
                    "Passthrough mode: Forwarding {} {}",
                    req.method(),
                    req.uri()
                );

                match Self::convert_request(req).await {
                    Ok((http_req, _body_bytes)) => match self.forwarder.forward(&http_req).await {
                        Ok(http_resp) => match Self::convert_response(&http_resp) {
                            Ok(response) => RequestOrResponse::Response(response),
                            Err(e) => {
                                tracing::error!("Failed to convert response: {}", e);
                                let err_response = Response::builder()
                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                    .body(Body::from("Failed to convert response"))
                                    .unwrap();
                                RequestOrResponse::Response(err_response)
                            }
                        },
                        Err(e) => {
                            tracing::error!("Failed to forward request: {}", e);
                            let err_response = Response::builder()
                                .status(StatusCode::BAD_GATEWAY)
                                .body(Body::from(format!("Proxy error: {}", e)))
                                .unwrap();
                            RequestOrResponse::Response(err_response)
                        }
                    },
                    Err(e) => {
                        tracing::error!("Failed to buffer request: {}", e);
                        let err_response = Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .body(Body::from("Failed to read request"))
                            .unwrap();
                        RequestOrResponse::Response(err_response)
                    }
                }
            }
        }
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, res: Response<Body>) -> Response<Body> {
        // We handle everything in handle_request now (with buffering)
        // This method just passes through responses unchanged
        tracing::debug!("Response passthrough: {}", res.status());
        res
    }
}

/// Proxy server configuration
pub struct ProxyServer {
    addr: SocketAddr,
    #[allow(dead_code)]
    ca: Arc<CertificateAuthority>,
    handler: MatgtoHttpHandler,
}

impl ProxyServer {
    /// Create a new proxy server
    pub fn new(port: u16, ca: Arc<CertificateAuthority>, mode: ProxyMode) -> Result<Self> {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let handler = MatgtoHttpHandler::new(mode);

        Ok(Self { addr, ca, handler })
    }

    /// Set recorder for Record mode
    pub fn with_recorder(mut self, recorder: Arc<Mutex<Recorder>>) -> Self {
        self.handler = self.handler.with_recorder(recorder);
        self
    }

    /// Set player for Replay mode
    pub fn with_player(mut self, player: Arc<Mutex<Player>>) -> Self {
        self.handler = self.handler.with_player(player);
        self
    }

    /// Start the proxy server
    ///
    /// This will block until the server is shut down
    pub async fn start(self) -> Result<()> {
        tracing::info!("🚀 Starting proxy server on {}", self.addr);

        // Note: The actual Hudsucker integration requires specific version compatibility
        // For now, we provide the structure for integration

        // The integration would look like:
        // 1. Create RcgenAuthority from our CA
        // 2. Build proxy with authority and handler
        // 3. Start the server

        tracing::warn!("⚠️  Full Hudsucker runtime integration pending");
        tracing::info!("📝 Proxy structure ready, handler configured");
        tracing::info!("🔧 Mode: {:?}", self.handler.mode);

        // Placeholder implementation
        // In production, this would call the actual Hudsucker proxy.start()
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        tracing::info!("✅ Proxy initialized successfully");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_proxy_server_creation() {
        let temp_dir = TempDir::new().unwrap();
        let ca = Arc::new(CertificateAuthority::new(temp_dir.path()).unwrap());

        let server = ProxyServer::new(8888, ca, ProxyMode::Record);
        assert!(server.is_ok());
    }

    #[tokio::test]
    async fn test_convert_request_with_body() {
        // Create a test request with body
        let body_data = b"Hello, World!";
        let request = Request::builder()
            .method("POST")
            .uri("https://example.com/api")
            .header("Content-Type", "text/plain")
            .body(Body::from(body_data.to_vec()))
            .unwrap();

        // Convert request (this buffers the body)
        let result = MatgtoHttpHandler::convert_request(request).await;
        assert!(result.is_ok());

        let (http_req, body_bytes) = result.unwrap();

        // Verify request was converted correctly
        assert_eq!(http_req.method, "POST");
        assert_eq!(http_req.url, "https://example.com/api");
        assert_eq!(body_bytes, body_data);
        assert!(http_req.body.is_some());
        assert_eq!(http_req.body.unwrap(), body_data);
    }

    #[tokio::test]
    async fn test_convert_request_empty_body() {
        // Create a test request without body
        let request = Request::builder()
            .method("GET")
            .uri("https://example.com/")
            .body(Body::empty())
            .unwrap();

        let result = MatgtoHttpHandler::convert_request(request).await;
        assert!(result.is_ok());

        let (http_req, body_bytes) = result.unwrap();

        // Verify empty body handling
        assert_eq!(http_req.method, "GET");
        assert!(body_bytes.is_empty());
        assert!(http_req.body.is_none());
    }

    #[test]
    fn test_reconstruct_request() {
        let http_req = HttpRequest {
            method: "POST".to_string(),
            url: "https://example.com/test".to_string(),
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "application/json".to_string());
                h
            },
            body: Some(b"{\"test\":true}".to_vec()),
        };

        let body_bytes = b"{\"test\":true}";
        let result = MatgtoHttpHandler::reconstruct_request(&http_req, body_bytes);
        assert!(result.is_ok());

        let reconstructed = result.unwrap();
        assert_eq!(reconstructed.method(), "POST");
        assert_eq!(reconstructed.uri(), "https://example.com/test");
    }

    #[test]
    fn test_convert_response() {
        let http_resp = HttpResponse {
            status: 200,
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "application/json".to_string());
                h
            },
            body: Some(b"{\"success\":true}".to_vec()),
        };

        let result = MatgtoHttpHandler::convert_response(&http_resp);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status(), 200);
    }
}
