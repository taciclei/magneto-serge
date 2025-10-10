//! HTTP client for forwarding requests to real servers

use crate::error::{MatgtoError, Result};
use crate::cassette::{HttpRequest, HttpResponse};

use hyper::{Body, Client, Request, Uri};
use hyper_rustls::HttpsConnectorBuilder;
use std::collections::HashMap;
use std::convert::TryFrom;

/// HTTP client for forwarding proxied requests
pub struct HttpForwarder {
    client: Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl std::fmt::Debug for HttpForwarder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpForwarder")
            .field("client", &"<HttpsClient>")
            .finish()
    }
}

impl Clone for HttpForwarder {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
        }
    }
}

impl HttpForwarder {
    /// Create a new HTTP forwarder
    pub fn new() -> Self {
        // Build HTTPS connector with rustls
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();

        let client = Client::builder().build(https);

        Self { client }
    }

    /// Forward an HTTP request to the real server
    ///
    /// # Arguments
    /// * `http_req` - The HTTP request to forward
    ///
    /// # Returns
    /// The HTTP response from the server
    pub async fn forward(&self, http_req: &HttpRequest) -> Result<HttpResponse> {
        tracing::debug!("Forwarding request: {} {}", http_req.method, http_req.url);

        // Parse URI
        let uri = http_req.url.parse::<Uri>().map_err(|e| {
            MatgtoError::ProxyStartFailed {
                reason: format!("Invalid URL: {}", e),
            }
        })?;

        // Build hyper request
        let mut builder = Request::builder()
            .method(http_req.method.as_str())
            .uri(uri);

        // Add headers
        for (name, value) in &http_req.headers {
            builder = builder.header(name, value);
        }

        // Add body if present
        let body = if let Some(body_data) = &http_req.body {
            Body::from(body_data.clone())
        } else {
            Body::empty()
        };

        let request = builder.body(body).map_err(|e| {
            MatgtoError::ProxyStartFailed {
                reason: format!("Failed to build request: {}", e),
            }
        })?;

        // Send request
        let response = self.client.request(request).await.map_err(|e| {
            MatgtoError::ProxyStartFailed {
                reason: format!("Request failed: {}", e),
            }
        })?;

        // Extract status
        let status = response.status().as_u16();

        // Extract headers
        let mut headers = HashMap::new();
        for (name, value) in response.headers().iter() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(name.to_string(), value_str.to_string());
            }
        }

        // Read body
        let body_bytes = hyper::body::to_bytes(response.into_body())
            .await
            .map_err(|e| MatgtoError::ProxyStartFailed {
                reason: format!("Failed to read response body: {}", e),
            })?;

        let body = if !body_bytes.is_empty() {
            Some(body_bytes.to_vec())
        } else {
            None
        };

        tracing::debug!("Response received: status={}", status);

        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}

impl Default for HttpForwarder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_forwarder_creation() {
        let forwarder = HttpForwarder::new();
        // Just verify it can be created
        drop(forwarder);
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it requires network access
    async fn test_forward_real_request() {
        let forwarder = HttpForwarder::new();

        let req = HttpRequest {
            method: "GET".to_string(),
            url: "https://httpbin.org/get".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = forwarder.forward(&req).await;
        assert!(response.is_ok());

        let resp = response.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it requires network access
    async fn test_forward_post_request() {
        let forwarder = HttpForwarder::new();

        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let req = HttpRequest {
            method: "POST".to_string(),
            url: "https://httpbin.org/post".to_string(),
            headers,
            body: Some(b"{\"test\":\"data\"}".to_vec()),
        };

        let response = forwarder.forward(&req).await;
        assert!(response.is_ok());

        let resp = response.unwrap();
        assert_eq!(resp.status, 200);
    }
}
