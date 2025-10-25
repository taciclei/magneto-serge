//! URL pattern filtering

use super::RequestFilter;
use crate::cassette::{HttpRequest, HttpResponse};

/// Filter requests by URL pattern
#[derive(Debug, Clone)]
pub struct UrlPatternFilter {
    /// URL patterns to exclude (glob-style: "/static/*", "/assets/*")
    exclude_patterns: Vec<String>,
}

impl UrlPatternFilter {
    /// Create new URL pattern filter
    pub fn new() -> Self {
        Self {
            exclude_patterns: Vec::new(),
        }
    }

    /// Add URL patterns to exclude
    pub fn add_patterns(&mut self, patterns: &[&str]) {
        for pattern in patterns {
            self.exclude_patterns.push(pattern.to_string());
        }
    }

    /// Check if URL matches any excluded pattern
    fn matches_excluded_pattern(&self, url: &str) -> bool {
        // Extract path from URL (own the String to avoid lifetime issues)
        let path = if let Ok(parsed) = url::Url::parse(url) {
            parsed.path().to_string()
        } else {
            url.to_string()
        };

        for pattern in &self.exclude_patterns {
            if self.matches_glob(&path, pattern) {
                return true;
            }
        }

        false
    }

    /// Simple glob matching (* = wildcard)
    fn matches_glob(&self, path: &str, pattern: &str) -> bool {
        if let Some(prefix) = pattern.strip_suffix("/*") {
            // Prefix match: "/static/*" matches "/static/app.js"
            path.starts_with(prefix)
        } else if pattern.contains('*') {
            // TODO: More complex glob matching
            path.contains(&pattern.replace('*', ""))
        } else {
            // Exact match
            path == pattern
        }
    }
}

impl Default for UrlPatternFilter {
    fn default() -> Self {
        let mut filter = Self::new();

        filter.add_patterns(&[
            "/static/*",
            "/assets/*",
            "/_next/static/*",
            "/public/*",
            "/dist/*",
        ]);

        filter
    }
}

impl RequestFilter for UrlPatternFilter {
    fn should_record(&self, request: &HttpRequest, _response: &HttpResponse) -> bool {
        // Return false if URL matches excluded pattern (filter it out)
        !self.matches_excluded_pattern(&request.url)
    }

    fn name(&self) -> &str {
        "url_pattern"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_url_pattern_filter() {
        let mut filter = UrlPatternFilter::new();
        filter.add_patterns(&["/static/*", "/assets/*"]);

        let req_static = HttpRequest {
            method: "GET".to_string(),
            url: "http://example.com/static/app.js".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let req_api = HttpRequest {
            method: "GET".to_string(),
            url: "http://example.com/api/users".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let res = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: None,
        };

        assert!(!filter.should_record(&req_static, &res)); // Should filter /static/*
        assert!(filter.should_record(&req_api, &res)); // Should record /api/*
    }
}
