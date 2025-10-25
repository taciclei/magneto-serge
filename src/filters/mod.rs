//! Request filtering for smart cassette recording
//!
//! This module provides filtering capabilities to exclude static assets and large bodies
//! from cassette recordings, reducing cassette size by 70-95%.
//!
//! # Example
//! ```rust,ignore
//! use magneto_serge::filters::{FilterChain, ExtensionFilter, ContentTypeFilter};
//! use magneto_serge::cassette::{HttpRequest, HttpResponse};
//!
//! let mut chain = FilterChain::new();
//! chain.add_filter(ExtensionFilter::default());
//! chain.add_filter(ContentTypeFilter::default());
//!
//! // Example usage (requires actual request/response)
//! # let request = HttpRequest { method: "GET".into(), url: "https://example.com".into(), headers: Default::default(), body: None };
//! # let response = HttpResponse { status: 200, headers: Default::default(), body: Some(vec![]) };
//! if chain.should_record(&request, &response) {
//!     // Record interaction
//! } else {
//!     // Skip (filtered)
//! }
//! ```

pub mod body_size;
pub mod content_type;
pub mod extension;
pub mod status_code;
pub mod url_pattern;

use crate::cassette::{HttpRequest, HttpResponse};
// use crate::error::Result;  // Not used yet
use serde::{Deserialize, Serialize};

pub use body_size::BodySizeFilter;
pub use content_type::ContentTypeFilter;
pub use extension::ExtensionFilter;
pub use status_code::StatusCodeFilter;
pub use url_pattern::UrlPatternFilter;

/// Alias for backward compatibility
pub type RecordingFilters = FilterChain;

/// Trait for request/response filters
pub trait RequestFilter: Send + Sync + std::fmt::Debug {
    /// Check if this request/response should be recorded
    ///
    /// Returns `true` if should record, `false` if should filter out
    fn should_record(&self, request: &HttpRequest, response: &HttpResponse) -> bool;

    /// Get filter name for debugging
    fn name(&self) -> &str;
}

/// Chain of filters with AND/OR logic
#[derive(Debug)]
pub struct FilterChain {
    filters: Vec<Box<dyn RequestFilter>>,
    logic: FilterLogic,
}

/// Filter combination logic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterLogic {
    /// All filters must pass (AND)
    All,

    /// Any filter can pass (OR)
    Any,
}

impl FilterChain {
    /// Create a new filter chain with AND logic (all filters must pass)
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            logic: FilterLogic::All,
        }
    }

    /// Create a filter chain with OR logic (any filter can pass)
    pub fn new_or() -> Self {
        Self {
            filters: Vec::new(),
            logic: FilterLogic::Any,
        }
    }

    /// Add a filter to the chain
    pub fn add_filter<F: RequestFilter + 'static>(&mut self, filter: F) {
        self.filters.push(Box::new(filter));
    }

    /// Check if request/response should be recorded
    pub fn should_record(&self, request: &HttpRequest, response: &HttpResponse) -> bool {
        if self.filters.is_empty() {
            return true; // No filters = record everything
        }

        match self.logic {
            FilterLogic::All => {
                // All filters must return true
                self.filters
                    .iter()
                    .all(|f| f.should_record(request, response))
            }
            FilterLogic::Any => {
                // At least one filter must return true
                self.filters
                    .iter()
                    .any(|f| f.should_record(request, response))
            }
        }
    }

    /// Get number of filters in chain
    pub fn len(&self) -> usize {
        self.filters.len()
    }

    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }
}

impl Default for FilterChain {
    fn default() -> Self {
        Self::new()
    }
}

/// Filter presets for common use cases
pub struct FilterPresets;

impl FilterPresets {
    /// Preset for web applications (filter JS/CSS/images/fonts)
    pub fn web_assets() -> FilterChain {
        let mut chain = FilterChain::new();

        // Filter by extension
        let mut ext_filter = ExtensionFilter::default();
        ext_filter.add_extensions(&[
            ".js", ".mjs", ".cjs", ".css", ".scss", ".sass", ".png", ".jpg", ".jpeg", ".gif",
            ".svg", ".webp", ".ico", ".woff", ".woff2", ".ttf", ".otf", ".eot",
        ]);
        chain.add_filter(ext_filter);

        // Filter by content-type
        let mut ct_filter = ContentTypeFilter::default();
        ct_filter.add_patterns(&[
            "image/*",
            "font/*",
            "text/css",
            "application/javascript",
            "application/x-javascript",
        ]);
        chain.add_filter(ct_filter);

        chain
    }

    /// Preset for filtering only images
    pub fn images() -> FilterChain {
        let mut chain = FilterChain::new();

        let mut ext_filter = ExtensionFilter::default();
        ext_filter.add_extensions(&[
            ".png", ".jpg", ".jpeg", ".gif", ".svg", ".webp", ".ico", ".bmp",
        ]);
        chain.add_filter(ext_filter);

        let mut ct_filter = ContentTypeFilter::default();
        ct_filter.add_patterns(&["image/*"]);
        chain.add_filter(ct_filter);

        chain
    }

    /// Preset for filtering fonts
    pub fn fonts() -> FilterChain {
        let mut chain = FilterChain::new();

        let mut ext_filter = ExtensionFilter::default();
        ext_filter.add_extensions(&[".woff", ".woff2", ".ttf", ".otf", ".eot"]);
        chain.add_filter(ext_filter);

        let mut ct_filter = ContentTypeFilter::default();
        ct_filter.add_patterns(&["font/*"]);
        chain.add_filter(ct_filter);

        chain
    }

    /// Comprehensive preset (filter everything static + large bodies)
    pub fn comprehensive() -> FilterChain {
        let mut chain = FilterChain::new();

        // Web assets
        chain.add_filter(ExtensionFilter::default());
        chain.add_filter(ContentTypeFilter::default());

        // Large bodies (>1MB)
        chain.add_filter(BodySizeFilter::new(1024 * 1024));

        // Common static paths
        let mut url_filter = UrlPatternFilter::new();
        url_filter.add_patterns(&[
            "/static/*",
            "/assets/*",
            "/_next/static/*",
            "/public/*",
            "/dist/*",
        ]);
        chain.add_filter(url_filter);

        chain
    }
}

/// Filter statistics for reporting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterStats {
    /// Total interactions seen
    pub total: usize,

    /// Interactions recorded
    pub recorded: usize,

    /// Interactions filtered
    pub filtered: usize,

    /// Breakdown by filter reason
    pub reasons: std::collections::HashMap<String, usize>,
}

impl FilterStats {
    /// Create new stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an interaction
    pub fn record(&mut self) {
        self.total += 1;
        self.recorded += 1;
    }

    /// Filter an interaction with reason
    pub fn filter(&mut self, reason: &str) {
        self.total += 1;
        self.filtered += 1;
        *self.reasons.entry(reason.to_string()).or_insert(0) += 1;
    }

    /// Get reduction percentage
    pub fn reduction_percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.filtered as f64 / self.total as f64) * 100.0
        }
    }

    /// Format stats for display
    pub fn summary(&self) -> String {
        format!(
            "Recorded: {}, Filtered: {} ({:.1}% reduction)",
            self.recorded,
            self.filtered,
            self.reduction_percent()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_chain_all_logic() {
        // Mock filter that always returns true
        #[derive(Debug)]
        struct AlwaysPass;
        impl RequestFilter for AlwaysPass {
            fn should_record(&self, _: &HttpRequest, _: &HttpResponse) -> bool {
                true
            }
            fn name(&self) -> &str {
                "always_pass"
            }
        }

        // Mock filter that always returns false
        #[derive(Debug)]
        struct AlwaysFail;
        impl RequestFilter for AlwaysFail {
            fn should_record(&self, _: &HttpRequest, _: &HttpResponse) -> bool {
                false
            }
            fn name(&self) -> &str {
                "always_fail"
            }
        }

        let mut chain = FilterChain::new();
        chain.add_filter(AlwaysPass);
        chain.add_filter(AlwaysPass);

        let req = HttpRequest {
            method: "GET".to_string(),
            url: "http://example.com".to_string(),
            headers: Default::default(),
            body: None,
        };
        let res = HttpResponse {
            status: 200,
            headers: Default::default(),
            body: None,
        };

        assert!(chain.should_record(&req, &res)); // All pass

        let mut chain2 = FilterChain::new();
        chain2.add_filter(AlwaysPass);
        chain2.add_filter(AlwaysFail); // One fails

        assert!(!chain2.should_record(&req, &res)); // Not all pass
    }

    #[test]
    fn test_filter_stats() {
        let mut stats = FilterStats::new();

        stats.record();
        stats.record();
        stats.filter("extension");
        stats.filter("extension");
        stats.filter("size");

        assert_eq!(stats.total, 5);
        assert_eq!(stats.recorded, 2);
        assert_eq!(stats.filtered, 3);
        assert_eq!(stats.reduction_percent(), 60.0);
        assert_eq!(stats.reasons.get("extension"), Some(&2));
    }
}
