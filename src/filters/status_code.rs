//! HTTP status code filtering

use super::RequestFilter;
use crate::cassette::{HttpRequest, HttpResponse};
use std::collections::HashSet;

/// Filter requests by HTTP status code
#[derive(Debug, Clone)]
pub struct StatusCodeFilter {
    /// Status codes to exclude (e.g., 404, 500)
    exclude_codes: HashSet<u16>,
    /// Status code ranges to exclude (e.g., 400-499 for client errors)
    exclude_ranges: Vec<(u16, u16)>,
}

impl StatusCodeFilter {
    /// Create new status code filter
    pub fn new() -> Self {
        Self {
            exclude_codes: HashSet::new(),
            exclude_ranges: Vec::new(),
        }
    }

    /// Add status codes to exclude
    pub fn add_codes(&mut self, codes: &[u16]) {
        for code in codes {
            self.exclude_codes.insert(*code);
        }
    }

    /// Add status code range to exclude (inclusive)
    pub fn add_range(&mut self, start: u16, end: u16) {
        self.exclude_ranges.push((start, end));
    }

    /// Exclude all 4xx client errors
    pub fn exclude_client_errors(mut self) -> Self {
        self.add_range(400, 499);
        self
    }

    /// Exclude all 5xx server errors
    pub fn exclude_server_errors(mut self) -> Self {
        self.add_range(500, 599);
        self
    }

    /// Check if status code should be excluded
    fn is_excluded(&self, status: u16) -> bool {
        // Check exact codes
        if self.exclude_codes.contains(&status) {
            return true;
        }

        // Check ranges
        for (start, end) in &self.exclude_ranges {
            if status >= *start && status <= *end {
                return true;
            }
        }

        false
    }
}

impl Default for StatusCodeFilter {
    fn default() -> Self {
        Self::new()
        // By default, don't exclude any status codes
        // Users can configure as needed
    }
}

impl RequestFilter for StatusCodeFilter {
    fn should_record(&self, _request: &HttpRequest, response: &HttpResponse) -> bool {
        // Return false if status code is excluded (filter it out)
        !self.is_excluded(response.status)
    }

    fn name(&self) -> &str {
        "status_code"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_status_code_filter() {
        let mut filter = StatusCodeFilter::new();
        filter.add_codes(&[404, 500]);

        let req = HttpRequest {
            method: "GET".to_string(),
            url: "http://example.com/test".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let res_200 = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: None,
        };

        let res_404 = HttpResponse {
            status: 404,
            headers: HashMap::new(),
            body: None,
        };

        assert!(filter.should_record(&req, &res_200)); // Should record 200
        assert!(!filter.should_record(&req, &res_404)); // Should filter 404
    }

    #[test]
    fn test_status_code_range() {
        let filter = StatusCodeFilter::new().exclude_client_errors();

        let req = HttpRequest {
            method: "GET".to_string(),
            url: "http://example.com/test".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let res_200 = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: None,
        };

        let res_403 = HttpResponse {
            status: 403,
            headers: HashMap::new(),
            body: None,
        };

        assert!(filter.should_record(&req, &res_200)); // Should record 200
        assert!(!filter.should_record(&req, &res_403)); // Should filter 403 (client error)
    }
}
