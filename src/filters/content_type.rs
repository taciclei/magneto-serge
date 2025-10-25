//! Content-Type header filtering

use super::RequestFilter;
use crate::cassette::{HttpRequest, HttpResponse};
use std::collections::HashSet;

/// Filter requests by Content-Type header
#[derive(Debug, Clone)]
pub struct ContentTypeFilter {
    /// Content-Type patterns to exclude (e.g., "image/*", "font/*")
    exclude_patterns: HashSet<String>,
}

impl ContentTypeFilter {
    /// Create new Content-Type filter
    pub fn new() -> Self {
        Self {
            exclude_patterns: HashSet::new(),
        }
    }

    /// Add Content-Type patterns to exclude
    pub fn add_patterns(&mut self, patterns: &[&str]) {
        for pattern in patterns {
            self.exclude_patterns.insert(pattern.to_lowercase());
        }
    }

    /// Check if Content-Type matches any excluded pattern
    fn matches_excluded_pattern(&self, content_type: &str) -> bool {
        let content_type = content_type.to_lowercase();

        for pattern in &self.exclude_patterns {
            if pattern.ends_with("/*") {
                // Wildcard pattern (e.g., "image/*")
                let prefix = &pattern[..pattern.len() - 2];
                if content_type.starts_with(prefix) {
                    return true;
                }
            } else if content_type.contains(pattern) {
                // Exact or partial match
                return true;
            }
        }

        false
    }
}

impl Default for ContentTypeFilter {
    fn default() -> Self {
        let mut filter = Self::new();

        filter.add_patterns(&[
            "image/*",
            "font/*",
            "video/*",
            "audio/*",
            "text/css",
            "application/javascript",
            "application/x-javascript",
            "application/octet-stream",
        ]);

        filter
    }
}

impl RequestFilter for ContentTypeFilter {
    fn should_record(&self, _request: &HttpRequest, response: &HttpResponse) -> bool {
        // Check Content-Type header in response
        if let Some(content_type) = response.headers.get("content-type") {
            // Return false if matches excluded pattern (filter it out)
            !self.matches_excluded_pattern(content_type)
        } else {
            true // No Content-Type header = record
        }
    }

    fn name(&self) -> &str {
        "content_type"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_content_type_filter() {
        let mut filter = ContentTypeFilter::new();
        filter.add_patterns(&["image/*", "text/css"]);

        let req = HttpRequest {
            method: "GET".to_string(),
            url: "http://example.com/test".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let mut res_image = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: None,
        };
        res_image.headers.insert("content-type".to_string(), "image/png".to_string());

        let mut res_json = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: None,
        };
        res_json.headers.insert("content-type".to_string(), "application/json".to_string());

        assert!(!filter.should_record(&req, &res_image)); // Should filter image
        assert!(filter.should_record(&req, &res_json)); // Should record JSON
    }
}
