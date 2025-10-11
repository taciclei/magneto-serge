//! Recording filters and transformations
//!
//! This module provides filtering and transformation capabilities for recording:
//! - URL filtering: Skip recording certain URLs (analytics, tracking, etc.)
//! - Header filtering: Mask sensitive headers (Authorization, API keys, etc.)
//! - Body transformation: Redact sensitive data from request/response bodies
//! - Conditional recording: Record only based on status code, content-type, etc.

use crate::cassette::{HttpRequest, HttpResponse};
use regex::Regex;
use std::collections::HashMap;

/// List of headers that should be filtered by default (security)
pub const DEFAULT_SENSITIVE_HEADERS: &[&str] = &[
    "authorization",
    "x-api-key",
    "x-api-token",
    "api-key",
    "api-token",
    "cookie",
    "set-cookie",
    "proxy-authorization",
    "x-auth-token",
    "x-csrf-token",
    "x-session-token",
];

/// Placeholder for filtered sensitive data
pub const FILTERED_PLACEHOLDER: &str = "[FILTERED]";

/// Configuration for recording filters
#[derive(Debug, Clone)]
pub struct RecordingFilters {
    /// URL patterns to ignore (regex)
    pub ignore_urls: Vec<Regex>,

    /// Headers to filter (case-insensitive)
    pub filter_headers: Vec<String>,

    /// Whether to filter request bodies
    pub filter_request_bodies: bool,

    /// Whether to filter response bodies
    pub filter_response_bodies: bool,

    /// Status codes to skip recording (e.g., 404, 500)
    pub skip_status_codes: Vec<u16>,

    /// Content types to skip (e.g., "image/", "video/")
    pub skip_content_types: Vec<String>,

    /// Maximum body size to record (in bytes, None = unlimited)
    pub max_body_size: Option<usize>,
}

impl Default for RecordingFilters {
    fn default() -> Self {
        Self {
            ignore_urls: Vec::new(),
            filter_headers: DEFAULT_SENSITIVE_HEADERS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            filter_request_bodies: false,
            filter_response_bodies: false,
            skip_status_codes: Vec::new(),
            skip_content_types: Vec::new(),
            max_body_size: None,
        }
    }
}

impl RecordingFilters {
    /// Create a new filter configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a URL pattern to ignore (regex)
    pub fn ignore_url(mut self, pattern: &str) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern)?;
        self.ignore_urls.push(regex);
        Ok(self)
    }

    /// Add multiple URL patterns to ignore
    pub fn ignore_urls_from_slice(mut self, patterns: &[&str]) -> Result<Self, regex::Error> {
        for pattern in patterns {
            let regex = Regex::new(pattern)?;
            self.ignore_urls.push(regex);
        }
        Ok(self)
    }

    /// Add a header to filter (case-insensitive)
    pub fn filter_header(mut self, header: String) -> Self {
        self.filter_headers.push(header.to_lowercase());
        self
    }

    /// Add multiple headers to filter
    pub fn filter_headers_from_slice(mut self, headers: &[String]) -> Self {
        for header in headers {
            self.filter_headers.push(header.to_lowercase());
        }
        self
    }

    /// Enable request body filtering
    pub fn filter_request_bodies(mut self, enabled: bool) -> Self {
        self.filter_request_bodies = enabled;
        self
    }

    /// Enable response body filtering
    pub fn filter_response_bodies(mut self, enabled: bool) -> Self {
        self.filter_response_bodies = enabled;
        self
    }

    /// Skip recording specific status codes
    pub fn skip_status_code(mut self, status: u16) -> Self {
        self.skip_status_codes.push(status);
        self
    }

    /// Skip recording specific status codes from slice
    pub fn skip_status_codes_from_slice(mut self, codes: &[u16]) -> Self {
        self.skip_status_codes.extend_from_slice(codes);
        self
    }

    /// Skip recording specific content types (partial match)
    pub fn skip_content_type(mut self, content_type: String) -> Self {
        self.skip_content_types.push(content_type.to_lowercase());
        self
    }

    /// Set maximum body size to record
    pub fn max_body_size(mut self, size: usize) -> Self {
        self.max_body_size = Some(size);
        self
    }

    /// Check if a URL should be ignored
    pub fn should_ignore_url(&self, url: &str) -> bool {
        for pattern in &self.ignore_urls {
            if pattern.is_match(url) {
                return true;
            }
        }
        false
    }

    /// Check if a status code should be skipped
    pub fn should_skip_status(&self, status: u16) -> bool {
        self.skip_status_codes.contains(&status)
    }

    /// Check if a content type should be skipped
    pub fn should_skip_content_type(&self, content_type: &str) -> bool {
        let content_type_lower = content_type.to_lowercase();
        for skip_type in &self.skip_content_types {
            if content_type_lower.contains(skip_type) {
                return true;
            }
        }
        false
    }

    /// Check if an interaction should be recorded
    pub fn should_record(&self, request: &HttpRequest, response: &HttpResponse) -> bool {
        // Check URL
        if self.should_ignore_url(&request.url) {
            return false;
        }

        // Check status code
        if self.should_skip_status(response.status) {
            return false;
        }

        // Check content type (case-insensitive header lookup)
        for (key, value) in &response.headers {
            if key.to_lowercase() == "content-type" && self.should_skip_content_type(value) {
                return false;
            }
        }

        true
    }

    /// Filter headers (mask sensitive ones)
    pub fn filter_headers_map(&self, headers: &HashMap<String, String>) -> HashMap<String, String> {
        headers
            .iter()
            .map(|(key, value)| {
                let key_lower = key.to_lowercase();
                if self.filter_headers.contains(&key_lower) {
                    (key.clone(), FILTERED_PLACEHOLDER.to_string())
                } else {
                    (key.clone(), value.clone())
                }
            })
            .collect()
    }

    /// Filter body (truncate if too large, optionally redact)
    pub fn filter_body(&self, body: Option<Vec<u8>>, should_filter: bool) -> Option<Vec<u8>> {
        body.map(|mut data| {
            // Truncate if too large
            if let Some(max_size) = self.max_body_size {
                if data.len() > max_size {
                    data.truncate(max_size);
                }
            }

            // If filtering is enabled, replace with placeholder
            if should_filter {
                return FILTERED_PLACEHOLDER.as_bytes().to_vec();
            }

            data
        })
    }

    /// Apply all filters to a request
    pub fn apply_to_request(&self, mut request: HttpRequest) -> HttpRequest {
        // Filter headers
        request.headers = self.filter_headers_map(&request.headers);

        // Filter body
        request.body = self.filter_body(request.body, self.filter_request_bodies);

        request
    }

    /// Apply all filters to a response
    pub fn apply_to_response(&self, mut response: HttpResponse) -> HttpResponse {
        // Filter headers
        response.headers = self.filter_headers_map(&response.headers);

        // Filter body
        response.body = self.filter_body(response.body, self.filter_response_bodies);

        response
    }
}

/// Builder for creating common filter configurations
pub struct FilterPresets;

impl FilterPresets {
    /// Security-focused preset (filters all sensitive data)
    pub fn security() -> RecordingFilters {
        RecordingFilters::new()
            .filter_request_bodies(false)
            .filter_response_bodies(false)
    }

    /// Strict preset (filters everything sensitive)
    pub fn strict() -> RecordingFilters {
        RecordingFilters::new()
            .filter_request_bodies(true)
            .filter_response_bodies(true)
    }

    /// Analytics-free preset (ignores common analytics/tracking URLs)
    pub fn no_analytics() -> Result<RecordingFilters, regex::Error> {
        RecordingFilters::new().ignore_urls_from_slice(&[
            r"google-analytics\.com",
            r"googletagmanager\.com",
            r"doubleclick\.net",
            r"facebook\.com/tr",
            r"analytics\.js",
            r"segment\.com",
            r"mixpanel\.com",
            r"amplitude\.com",
        ])
    }

    /// Media-free preset (skips images, videos, fonts)
    pub fn no_media() -> RecordingFilters {
        RecordingFilters::new()
            .skip_content_type("image/".to_string())
            .skip_content_type("video/".to_string())
            .skip_content_type("audio/".to_string())
            .skip_content_type("font/".to_string())
    }

    /// Small bodies only (limits body size)
    pub fn small_bodies(max_size: usize) -> RecordingFilters {
        RecordingFilters::new().max_body_size(max_size)
    }

    /// Success only (skip 4xx and 5xx errors)
    pub fn success_only() -> RecordingFilters {
        let mut filters = RecordingFilters::new();

        // Skip 4xx errors
        for code in 400..500 {
            filters = filters.skip_status_code(code);
        }

        // Skip 5xx errors
        for code in 500..600 {
            filters = filters.skip_status_code(code);
        }

        filters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_request() -> HttpRequest {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert(
            "Authorization".to_string(),
            "Bearer secret-token".to_string(),
        );
        headers.insert("X-API-Key".to_string(), "abc123".to_string());

        HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/users".to_string(),
            headers,
            body: Some(b"{\"secret\":\"data\"}".to_vec()),
        }
    }

    fn create_test_response() -> HttpResponse {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Set-Cookie".to_string(), "session=xyz".to_string());

        HttpResponse {
            status: 200,
            headers,
            body: Some(b"{\"result\":\"ok\"}".to_vec()),
        }
    }

    #[test]
    fn test_default_filters() {
        let filters = RecordingFilters::default();
        assert!(!filters.filter_headers.is_empty());
        assert!(filters
            .filter_headers
            .contains(&"authorization".to_string()));
    }

    #[test]
    fn test_ignore_url() {
        let filters = RecordingFilters::new()
            .ignore_url(r"analytics\.com")
            .unwrap();

        assert!(filters.should_ignore_url("https://analytics.com/track"));
        assert!(!filters.should_ignore_url("https://api.example.com"));
    }

    #[test]
    fn test_filter_headers() {
        let filters = RecordingFilters::new();
        let request = create_test_request();

        let filtered_headers = filters.filter_headers_map(&request.headers);

        assert_eq!(
            filtered_headers.get("Authorization"),
            Some(&FILTERED_PLACEHOLDER.to_string())
        );
        assert_eq!(
            filtered_headers.get("X-API-Key"),
            Some(&FILTERED_PLACEHOLDER.to_string())
        );
        assert_eq!(
            filtered_headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_apply_to_request() {
        let filters = RecordingFilters::new();
        let request = create_test_request();

        let filtered = filters.apply_to_request(request);

        assert_eq!(
            filtered.headers.get("Authorization"),
            Some(&FILTERED_PLACEHOLDER.to_string())
        );
        assert!(filtered.body.is_some());
    }

    #[test]
    fn test_apply_to_response() {
        let filters = RecordingFilters::new();
        let response = create_test_response();

        let filtered = filters.apply_to_response(response);

        assert_eq!(
            filtered.headers.get("Set-Cookie"),
            Some(&FILTERED_PLACEHOLDER.to_string())
        );
    }

    #[test]
    fn test_filter_request_bodies() {
        let filters = RecordingFilters::new().filter_request_bodies(true);
        let request = create_test_request();

        let filtered = filters.apply_to_request(request);

        assert_eq!(
            filtered.body,
            Some(FILTERED_PLACEHOLDER.as_bytes().to_vec())
        );
    }

    #[test]
    fn test_skip_status_code() {
        let filters = RecordingFilters::new()
            .skip_status_code(404)
            .skip_status_code(500);

        assert!(filters.should_skip_status(404));
        assert!(filters.should_skip_status(500));
        assert!(!filters.should_skip_status(200));
    }

    #[test]
    fn test_skip_content_type() {
        let filters = RecordingFilters::new().skip_content_type("image/".to_string());

        assert!(filters.should_skip_content_type("image/png"));
        assert!(filters.should_skip_content_type("image/jpeg"));
        assert!(!filters.should_skip_content_type("application/json"));
    }

    #[test]
    fn test_should_record() {
        let filters = RecordingFilters::new()
            .ignore_url(r"analytics\.com")
            .unwrap()
            .skip_status_code(404);

        let mut request = create_test_request();
        let mut response = create_test_response();

        // Should record normal interaction
        assert!(filters.should_record(&request, &response));

        // Should not record analytics URL
        request.url = "https://analytics.com/track".to_string();
        assert!(!filters.should_record(&request, &response));

        // Should not record 404
        request.url = "https://api.example.com/users".to_string();
        response.status = 404;
        assert!(!filters.should_record(&request, &response));
    }

    #[test]
    fn test_max_body_size() {
        let filters = RecordingFilters::new().max_body_size(10);

        let large_body = vec![0u8; 100];
        let filtered = filters.filter_body(Some(large_body), false);

        assert_eq!(filtered.unwrap().len(), 10);
    }

    #[test]
    fn test_preset_security() {
        let filters = FilterPresets::security();
        assert!(!filters.filter_headers.is_empty());
    }

    #[test]
    fn test_preset_no_analytics() {
        let filters = FilterPresets::no_analytics().unwrap();
        assert!(filters.should_ignore_url("https://google-analytics.com/collect"));
        assert!(!filters.should_ignore_url("https://api.example.com"));
    }

    #[test]
    fn test_preset_no_media() {
        let filters = FilterPresets::no_media();
        assert!(filters.should_skip_content_type("image/png"));
        assert!(filters.should_skip_content_type("video/mp4"));
        assert!(!filters.should_skip_content_type("application/json"));
    }

    #[test]
    fn test_preset_success_only() {
        let filters = FilterPresets::success_only();
        assert!(filters.should_skip_status(404));
        assert!(filters.should_skip_status(500));
        assert!(!filters.should_skip_status(200));
    }
}
