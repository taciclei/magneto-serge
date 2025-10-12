//! Advanced request matching strategies
//!
//! This module provides flexible matching strategies for replaying cassettes,
//! including regex URL matching, partial body matching, header-specific matching,
//! and custom matchers.

use crate::cassette::HttpRequest;
use crate::error::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Matching strategy configuration
#[derive(Clone, Serialize, Deserialize)]
pub struct MatchingStrategy {
    /// Match request method
    pub match_method: bool,

    /// URL matching mode
    pub url_mode: UrlMatchMode,

    /// Body matching mode
    pub body_mode: BodyMatchMode,

    /// Headers to include in matching
    pub match_headers: HashSet<String>,

    /// Headers to ignore in matching
    pub ignore_headers: HashSet<String>,

    /// Query parameters to ignore in matching
    pub ignore_query_params: HashSet<String>,

    /// Custom matchers (not serializable, must be added programmatically)
    #[serde(skip)]
    pub custom_matchers: Vec<Arc<dyn CustomMatcher>>,
}

/// URL matching mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UrlMatchMode {
    /// Exact URL match (default)
    Exact,

    /// Match URL using regex pattern
    Regex { pattern: String },

    /// Match URL ignoring query parameters
    IgnoreQuery,

    /// Match URL with specific query parameters ignored
    IgnoreQueryParams { params: Vec<String> },

    /// Match only the path component (ignore host/port/scheme)
    PathOnly,
}

/// Body matching mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BodyMatchMode {
    /// Hash-based matching (default)
    Hash,

    /// Ignore body in matching
    Ignore,

    /// Match body using JSON path
    JsonPath { path: String },

    /// Match body using regex
    Regex { pattern: String },

    /// Match body size only (useful for binary data)
    SizeOnly,
}

/// Custom matcher trait for user-defined matching logic
pub trait CustomMatcher: Send + Sync + std::fmt::Debug {
    /// Match a request signature against a recorded request
    ///
    /// Returns true if the requests match according to this matcher's logic
    fn matches(&self, signature: &RequestSignature, recorded: &HttpRequest) -> Result<bool>;

    /// Matcher name for debugging
    fn name(&self) -> &str;
}

/// Enhanced request signature with flexible matching
#[derive(Debug, Clone)]
pub struct RequestSignature {
    pub method: String,
    pub url: String,
    pub body: Option<Vec<u8>>,
    pub headers: HashMap<String, String>,
}

impl RequestSignature {
    /// Create a new request signature from an HTTP request
    pub fn from_request(request: &HttpRequest) -> Self {
        Self {
            method: request.method.clone(),
            url: request.url.clone(),
            body: request.body.clone(),
            headers: request.headers.clone(),
        }
    }

    /// Check if this signature matches a recorded request using the given strategy
    pub fn matches(&self, recorded: &HttpRequest, strategy: &MatchingStrategy) -> Result<bool> {
        // 1. Match method
        if strategy.match_method && self.method != recorded.method {
            return Ok(false);
        }

        // 2. Match URL
        if !self.matches_url(&recorded.url, &strategy.url_mode)? {
            return Ok(false);
        }

        // 3. Match body
        if !self.matches_body(recorded.body.as_deref(), &strategy.body_mode)? {
            return Ok(false);
        }

        // 4. Match headers
        if !self.matches_headers(&recorded.headers, strategy)? {
            return Ok(false);
        }

        // 5. Apply custom matchers
        for matcher in &strategy.custom_matchers {
            if !matcher.matches(self, recorded)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Match URL according to the URL matching mode
    fn matches_url(&self, recorded_url: &str, mode: &UrlMatchMode) -> Result<bool> {
        match mode {
            UrlMatchMode::Exact => Ok(self.url == recorded_url),

            UrlMatchMode::Regex { pattern } => {
                let re = Regex::new(pattern)?;
                // Both signature and recorded URL must match the pattern
                Ok(re.is_match(&self.url) && re.is_match(recorded_url))
            }

            UrlMatchMode::IgnoreQuery => {
                let self_base = self.url.split('?').next().unwrap_or(&self.url);
                let recorded_base = recorded_url.split('?').next().unwrap_or(recorded_url);
                Ok(self_base == recorded_base)
            }

            UrlMatchMode::IgnoreQueryParams { params } => {
                Ok(urls_match_ignoring_params(&self.url, recorded_url, params)?)
            }

            UrlMatchMode::PathOnly => {
                let self_path = extract_path(&self.url)?;
                let recorded_path = extract_path(recorded_url)?;
                Ok(self_path == recorded_path)
            }
        }
    }

    /// Match body according to the body matching mode
    fn matches_body(&self, recorded_body: Option<&[u8]>, mode: &BodyMatchMode) -> Result<bool> {
        match mode {
            BodyMatchMode::Hash => {
                // Default hash-based matching
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};

                let self_hash = self.body.as_ref().map(|b| {
                    let mut hasher = DefaultHasher::new();
                    b.hash(&mut hasher);
                    hasher.finish()
                });

                let recorded_hash = recorded_body.map(|b| {
                    let mut hasher = DefaultHasher::new();
                    b.hash(&mut hasher);
                    hasher.finish()
                });

                Ok(self_hash == recorded_hash)
            }

            BodyMatchMode::Ignore => Ok(true),

            BodyMatchMode::JsonPath { path } => {
                match_json_path(self.body.as_deref(), recorded_body, path)
            }

            BodyMatchMode::Regex { pattern } => {
                let re = Regex::new(pattern)?;
                let self_str = self
                    .body
                    .as_ref()
                    .and_then(|b| std::str::from_utf8(b).ok())
                    .unwrap_or("");
                let recorded_str = recorded_body
                    .and_then(|b| std::str::from_utf8(b).ok())
                    .unwrap_or("");
                Ok(re.is_match(self_str) && re.is_match(recorded_str))
            }

            BodyMatchMode::SizeOnly => {
                let self_size = self.body.as_ref().map(|b| b.len());
                let recorded_size = recorded_body.map(|b| b.len());
                Ok(self_size == recorded_size)
            }
        }
    }

    /// Match headers according to the strategy
    fn matches_headers(
        &self,
        recorded_headers: &HashMap<String, String>,
        strategy: &MatchingStrategy,
    ) -> Result<bool> {
        // Check required headers
        for header in &strategy.match_headers {
            let self_value = self.headers.get(header);
            let recorded_value = recorded_headers.get(header);
            if self_value != recorded_value {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

impl std::fmt::Debug for MatchingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MatchingStrategy")
            .field("match_method", &self.match_method)
            .field("url_mode", &self.url_mode)
            .field("body_mode", &self.body_mode)
            .field("match_headers", &self.match_headers)
            .field("ignore_headers", &self.ignore_headers)
            .field("ignore_query_params", &self.ignore_query_params)
            .field("custom_matchers_count", &self.custom_matchers.len())
            .finish()
    }
}

impl Default for MatchingStrategy {
    fn default() -> Self {
        Self {
            match_method: true,
            url_mode: UrlMatchMode::Exact,
            body_mode: BodyMatchMode::Hash,
            match_headers: HashSet::new(),
            ignore_headers: HashSet::new(),
            ignore_query_params: HashSet::new(),
            custom_matchers: Vec::new(),
        }
    }
}

impl MatchingStrategy {
    /// Create a new default matching strategy
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a lenient matching strategy (ignores query params, ignores body)
    pub fn lenient() -> Self {
        Self {
            match_method: true,
            url_mode: UrlMatchMode::IgnoreQuery,
            body_mode: BodyMatchMode::Ignore,
            match_headers: HashSet::new(),
            ignore_headers: HashSet::new(),
            ignore_query_params: HashSet::new(),
            custom_matchers: Vec::new(),
        }
    }

    /// Create a strict matching strategy (exact matches on everything)
    pub fn strict() -> Self {
        Self {
            match_method: true,
            url_mode: UrlMatchMode::Exact,
            body_mode: BodyMatchMode::Hash,
            match_headers: HashSet::new(),
            ignore_headers: HashSet::new(),
            ignore_query_params: HashSet::new(),
            custom_matchers: Vec::new(),
        }
    }

    /// Set URL matching mode
    pub fn with_url_mode(mut self, mode: UrlMatchMode) -> Self {
        self.url_mode = mode;
        self
    }

    /// Set body matching mode
    pub fn with_body_mode(mut self, mode: BodyMatchMode) -> Self {
        self.body_mode = mode;
        self
    }

    /// Add a header to match
    pub fn match_header(mut self, header: String) -> Self {
        self.match_headers.insert(header);
        self
    }

    /// Add a header to ignore
    pub fn ignore_header(mut self, header: String) -> Self {
        self.ignore_headers.insert(header);
        self
    }

    /// Add a query parameter to ignore
    pub fn ignore_query_param(mut self, param: String) -> Self {
        self.ignore_query_params.insert(param);
        self
    }

    /// Add a custom matcher
    pub fn with_custom_matcher(mut self, matcher: Arc<dyn CustomMatcher>) -> Self {
        self.custom_matchers.push(matcher);
        self
    }

    /// Check whether method matching is enabled
    pub fn is_method_matching(&self) -> bool {
        self.match_method
    }
}

/// Helper: Match URLs while ignoring specific query parameters
fn urls_match_ignoring_params(url1: &str, url2: &str, ignore_params: &[String]) -> Result<bool> {
    use url::Url;

    let parsed1 = Url::parse(url1)?;
    let parsed2 = Url::parse(url2)?;

    // Check base URL (scheme, host, port, path)
    if parsed1.scheme() != parsed2.scheme()
        || parsed1.host_str() != parsed2.host_str()
        || parsed1.port() != parsed2.port()
        || parsed1.path() != parsed2.path()
    {
        return Ok(false);
    }

    // Check query parameters (ignoring specified params)
    let params1: HashMap<_, _> = parsed1
        .query_pairs()
        .filter(|(k, _)| !ignore_params.contains(&k.to_string()))
        .collect();

    let params2: HashMap<_, _> = parsed2
        .query_pairs()
        .filter(|(k, _)| !ignore_params.contains(&k.to_string()))
        .collect();

    Ok(params1 == params2)
}

/// Helper: Extract path from URL
fn extract_path(url: &str) -> Result<String> {
    use url::Url;
    let parsed = Url::parse(url)?;
    Ok(parsed.path().to_string())
}

/// Helper: Match JSON bodies using JSON path
fn match_json_path(body1: Option<&[u8]>, body2: Option<&[u8]>, path: &str) -> Result<bool> {
    // Parse both bodies as JSON
    let json1 = match body1 {
        Some(b) => serde_json::from_slice::<serde_json::Value>(b)?,
        None => return Ok(body2.is_none()),
    };

    let json2 = match body2 {
        Some(b) => serde_json::from_slice::<serde_json::Value>(b)?,
        None => return Ok(false),
    };

    // Extract values at the given path
    let value1 = extract_json_value(&json1, path);
    let value2 = extract_json_value(&json2, path);

    Ok(value1 == value2)
}

/// Helper: Extract value from JSON using a simple path syntax (e.g., "user.name")
fn extract_json_value<'a>(
    json: &'a serde_json::Value,
    path: &str,
) -> Option<&'a serde_json::Value> {
    let mut current = json;

    for segment in path.split('.') {
        current = match current {
            serde_json::Value::Object(map) => map.get(segment)?,
            serde_json::Value::Array(arr) => {
                let index: usize = segment.parse().ok()?;
                arr.get(index)?
            }
            _ => return None,
        };
    }

    Some(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_url_matching() {
        let sig = RequestSignature {
            method: "GET".to_string(),
            url: "https://api.example.com/users".to_string(),
            body: None,
            headers: HashMap::new(),
        };

        let strategy = MatchingStrategy::default();

        assert!(sig
            .matches_url("https://api.example.com/users", &strategy.url_mode)
            .unwrap());
        assert!(!sig
            .matches_url("https://api.example.com/posts", &strategy.url_mode)
            .unwrap());
    }

    #[test]
    fn test_regex_url_matching() {
        let sig = RequestSignature {
            method: "GET".to_string(),
            url: "https://api.example.com/users/123".to_string(),
            body: None,
            headers: HashMap::new(),
        };

        let mode = UrlMatchMode::Regex {
            pattern: r"^https://api\.example\.com/users/\d+$".to_string(),
        };

        assert!(sig
            .matches_url("https://api.example.com/users/123", &mode)
            .unwrap());
        assert!(sig
            .matches_url("https://api.example.com/users/456", &mode)
            .unwrap());
        assert!(!sig
            .matches_url("https://api.example.com/posts/123", &mode)
            .unwrap());
    }

    #[test]
    fn test_ignore_query_matching() {
        let sig = RequestSignature {
            method: "GET".to_string(),
            url: "https://api.example.com/users?page=1".to_string(),
            body: None,
            headers: HashMap::new(),
        };

        let mode = UrlMatchMode::IgnoreQuery;

        assert!(sig
            .matches_url("https://api.example.com/users?page=2", &mode)
            .unwrap());
        assert!(sig
            .matches_url("https://api.example.com/users", &mode)
            .unwrap());
    }

    #[test]
    fn test_ignore_specific_query_params() {
        let sig = RequestSignature {
            method: "GET".to_string(),
            url: "https://api.example.com/users?page=1&sort=name".to_string(),
            body: None,
            headers: HashMap::new(),
        };

        let mode = UrlMatchMode::IgnoreQueryParams {
            params: vec!["page".to_string()],
        };

        // Same sort, different page - should match
        assert!(sig
            .matches_url("https://api.example.com/users?page=2&sort=name", &mode)
            .unwrap());

        // Different sort - should not match
        assert!(!sig
            .matches_url("https://api.example.com/users?page=1&sort=date", &mode)
            .unwrap());
    }

    #[test]
    fn test_path_only_matching() {
        let sig = RequestSignature {
            method: "GET".to_string(),
            url: "https://api.example.com/users".to_string(),
            body: None,
            headers: HashMap::new(),
        };

        let mode = UrlMatchMode::PathOnly;

        assert!(sig
            .matches_url("http://other.example.com/users", &mode)
            .unwrap());
        assert!(sig
            .matches_url("https://localhost:8080/users", &mode)
            .unwrap());
        assert!(!sig
            .matches_url("https://api.example.com/posts", &mode)
            .unwrap());
    }

    #[test]
    fn test_body_ignore_matching() {
        let sig = RequestSignature {
            method: "POST".to_string(),
            url: "https://api.example.com/users".to_string(),
            body: Some(b"body1".to_vec()),
            headers: HashMap::new(),
        };

        let mode = BodyMatchMode::Ignore;

        assert!(sig.matches_body(Some(b"body2"), &mode).unwrap());
        assert!(sig.matches_body(None, &mode).unwrap());
    }

    #[test]
    fn test_body_size_only_matching() {
        let sig = RequestSignature {
            method: "POST".to_string(),
            url: "https://api.example.com/users".to_string(),
            body: Some(b"12345".to_vec()),
            headers: HashMap::new(),
        };

        let mode = BodyMatchMode::SizeOnly;

        assert!(sig.matches_body(Some(b"abcde"), &mode).unwrap());
        assert!(!sig.matches_body(Some(b"abc"), &mode).unwrap());
        assert!(!sig.matches_body(None, &mode).unwrap());
    }

    #[test]
    fn test_json_path_matching() {
        let body1 = serde_json::json!({
            "user": {
                "id": 123,
                "name": "Alice"
            }
        });

        let body2 = serde_json::json!({
            "user": {
                "id": 123,
                "name": "Bob"
            }
        });

        let sig = RequestSignature {
            method: "POST".to_string(),
            url: "https://api.example.com/users".to_string(),
            body: Some(serde_json::to_vec(&body1).unwrap()),
            headers: HashMap::new(),
        };

        let mode = BodyMatchMode::JsonPath {
            path: "user.id".to_string(),
        };

        // Same ID, different name - should match
        assert!(sig
            .matches_body(Some(&serde_json::to_vec(&body2).unwrap()), &mode)
            .unwrap());

        // Different ID - should not match
        let body3 = serde_json::json!({
            "user": {
                "id": 456,
                "name": "Alice"
            }
        });
        assert!(!sig
            .matches_body(Some(&serde_json::to_vec(&body3).unwrap()), &mode)
            .unwrap());
    }

    #[test]
    fn test_extract_json_value() {
        let json = serde_json::json!({
            "user": {
                "id": 123,
                "name": "Alice",
                "addresses": [
                    { "city": "Paris" },
                    { "city": "London" }
                ]
            }
        });

        assert_eq!(
            extract_json_value(&json, "user.id"),
            Some(&serde_json::json!(123))
        );

        assert_eq!(
            extract_json_value(&json, "user.name"),
            Some(&serde_json::json!("Alice"))
        );

        assert_eq!(
            extract_json_value(&json, "user.addresses.0.city"),
            Some(&serde_json::json!("Paris"))
        );

        assert_eq!(extract_json_value(&json, "user.invalid"), None);
    }

    #[test]
    fn test_matching_strategy_builder() {
        let strategy = MatchingStrategy::new()
            .with_url_mode(UrlMatchMode::IgnoreQuery)
            .with_body_mode(BodyMatchMode::Ignore)
            .match_header("Authorization".to_string())
            .ignore_query_param("timestamp".to_string());

        assert_eq!(strategy.url_mode, UrlMatchMode::IgnoreQuery);
        assert_eq!(strategy.body_mode, BodyMatchMode::Ignore);
        assert!(strategy.match_headers.contains("Authorization"));
        assert!(strategy.ignore_query_params.contains("timestamp"));
    }
}
