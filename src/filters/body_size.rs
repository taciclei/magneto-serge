//! Body size filtering

use super::RequestFilter;
use crate::cassette::{HttpRequest, HttpResponse};

/// Filter requests by response body size
#[derive(Debug, Clone)]
pub struct BodySizeFilter {
    /// Maximum body size in bytes (responses larger are filtered)
    max_size_bytes: usize,
}

impl BodySizeFilter {
    /// Create new body size filter
    pub fn new(max_size_bytes: usize) -> Self {
        Self { max_size_bytes }
    }

    /// Create filter with max size in KB
    pub fn new_kb(max_size_kb: usize) -> Self {
        Self::new(max_size_kb * 1024)
    }

    /// Create filter with max size in MB
    pub fn new_mb(max_size_mb: usize) -> Self {
        Self::new(max_size_mb * 1024 * 1024)
    }
}

impl Default for BodySizeFilter {
    fn default() -> Self {
        Self::new_mb(1) // Default: 1MB
    }
}

impl RequestFilter for BodySizeFilter {
    fn should_record(&self, _request: &HttpRequest, response: &HttpResponse) -> bool {
        if let Some(body) = &response.body {
            // Return false if body too large (filter it out)
            body.len() <= self.max_size_bytes
        } else {
            true // No body = record
        }
    }

    fn name(&self) -> &str {
        "body_size"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_body_size_filter() {
        let filter = BodySizeFilter::new(1024); // 1KB max

        let req = HttpRequest {
            method: "GET".to_string(),
            url: "http://example.com/test".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let res_small = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(vec![0u8; 512]), // 512 bytes
        };

        let res_large = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(vec![0u8; 2048]), // 2KB
        };

        assert!(filter.should_record(&req, &res_small)); // Should record small
        assert!(!filter.should_record(&req, &res_large)); // Should filter large
    }
}
