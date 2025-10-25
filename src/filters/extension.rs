//! Extension-based filtering

use super::RequestFilter;
use crate::cassette::{HttpRequest, HttpResponse};
use std::collections::HashSet;

/// Filter requests by file extension
#[derive(Debug, Clone)]
pub struct ExtensionFilter {
    /// Extensions to exclude (e.g., ".js", ".css")
    exclude_extensions: HashSet<String>,
}

impl ExtensionFilter {
    /// Create new extension filter
    pub fn new() -> Self {
        Self {
            exclude_extensions: HashSet::new(),
        }
    }

    /// Add extensions to exclude
    pub fn add_extensions(&mut self, extensions: &[&str]) {
        for ext in extensions {
            self.exclude_extensions.insert(ext.to_lowercase());
        }
    }

    /// Check if URL has excluded extension
    fn has_excluded_extension(&self, url: &str) -> bool {
        // Work directly on the URL to avoid lifetime issues
        let url_lower = url.to_lowercase();

        for ext in &self.exclude_extensions {
            if url_lower.ends_with(ext) {
                return true;
            }
        }

        false
    }
}

impl Default for ExtensionFilter {
    fn default() -> Self {
        let mut filter = Self::new();

        // Default web assets
        filter.add_extensions(&[
            // JavaScript
            ".js", ".mjs", ".cjs", ".jsx",
            // CSS
            ".css", ".scss", ".sass", ".less",
            // Images
            ".png", ".jpg", ".jpeg", ".gif", ".svg", ".webp", ".ico", ".bmp",
            // Fonts
            ".woff", ".woff2", ".ttf", ".otf", ".eot",
            // Media
            ".mp4", ".webm", ".mp3", ".wav", ".ogg",
            // Archives
            ".zip", ".tar", ".gz", ".7z",
        ]);

        filter
    }
}

impl RequestFilter for ExtensionFilter {
    fn should_record(&self, request: &HttpRequest, _response: &HttpResponse) -> bool {
        // Return false if URL has excluded extension (filter it out)
        !self.has_excluded_extension(&request.url)
    }

    fn name(&self) -> &str {
        "extension"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_extension_filter() {
        let mut filter = ExtensionFilter::new();
        filter.add_extensions(&[".js", ".css"]);

        let req_js = HttpRequest {
            method: "GET".to_string(),
            url: "http://example.com/app.js".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let req_html = HttpRequest {
            method: "GET".to_string(),
            url: "http://example.com/index.html".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let res = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: None,
        };

        assert!(!filter.should_record(&req_js, &res)); // Should filter .js
        assert!(filter.should_record(&req_html, &res)); // Should record .html
    }
}
