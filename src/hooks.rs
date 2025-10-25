//! Hook system for intercepting record and replay operations
//!
//! Hooks allow you to modify interactions before they are recorded or replayed,
//! enabling use cases like:
//! - Filtering sensitive data (API keys, tokens, cookies)
//! - Dynamic response modification
//! - Logging and metrics
//! - Custom validation
//!
//! # Example
//!
//! ```rust
//! use magneto_serge::hooks::{RecordHook, ReplayHook};
//! use magneto_serge::cassette::{Interaction, InteractionKind};
//! use magneto_serge::error::Result;
//!
//! struct SensitiveDataFilter;
//!
//! impl RecordHook for SensitiveDataFilter {
//!     fn before_record(&self, interaction: &mut Interaction) -> Result<()> {
//!         if let InteractionKind::Http { request, response } = &mut interaction.kind {
//!             // Filter Authorization header
//!             request.headers.insert(
//!                 "authorization".to_string(),
//!                 "[FILTERED]".to_string()
//!             );
//!
//!             // Filter Set-Cookie header
//!             response.headers.insert(
//!                 "set-cookie".to_string(),
//!                 "[FILTERED]".to_string()
//!             );
//!         }
//!         Ok(())
//!     }
//! }
//! ```

use crate::cassette::Interaction;
use crate::error::Result;
use std::fmt::Debug;
use std::sync::Arc;

/// Hook called before and after recording an interaction
pub trait RecordHook: Send + Sync + Debug {
    /// Called before an interaction is recorded to cassette
    ///
    /// This allows you to modify the interaction before it's saved.
    /// Common uses:
    /// - Filter sensitive data (headers, body)
    /// - Normalize dynamic data (timestamps, IDs)
    /// - Add metadata
    fn before_record(&self, interaction: &mut Interaction) -> Result<()> {
        let _ = interaction;
        Ok(())
    }

    /// Called after an interaction is successfully recorded
    ///
    /// This is called after the interaction has been added to the cassette
    /// but before it's saved to disk. Useful for:
    /// - Logging
    /// - Metrics/telemetry
    /// - Validation
    fn after_record(&self, interaction: &Interaction) -> Result<()> {
        let _ = interaction;
        Ok(())
    }

    /// Hook name for debugging
    fn name(&self) -> &str {
        "RecordHook"
    }
}

/// Hook called before and after replaying an interaction
pub trait ReplayHook: Send + Sync + Debug {
    /// Called before an interaction is replayed
    ///
    /// This allows you to modify the interaction before it's served.
    /// Common uses:
    /// - Dynamic response modification (timestamps, user IDs)
    /// - Add/modify headers
    /// - Transform body content
    fn before_replay(&self, interaction: &mut Interaction) -> Result<()> {
        let _ = interaction;
        Ok(())
    }

    /// Called after an interaction is successfully replayed
    ///
    /// This is called after the response has been sent. Useful for:
    /// - Logging
    /// - Metrics/telemetry
    /// - Validation
    fn after_replay(&self, interaction: &Interaction) -> Result<()> {
        let _ = interaction;
        Ok(())
    }

    /// Hook name for debugging
    fn name(&self) -> &str {
        "ReplayHook"
    }
}

/// Collection of record hooks
#[derive(Default)]
pub struct RecordHooks {
    hooks: Vec<Arc<dyn RecordHook>>,
}

impl RecordHooks {
    /// Create a new empty hook collection
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    /// Add a hook to the collection
    pub fn add<H: RecordHook + 'static>(&mut self, hook: H) {
        self.hooks.push(Arc::new(hook));
    }

    /// Execute before_record on all hooks
    pub fn before_record(&self, interaction: &mut Interaction) -> Result<()> {
        for hook in &self.hooks {
            hook.before_record(interaction)?;
        }
        Ok(())
    }

    /// Execute after_record on all hooks
    pub fn after_record(&self, interaction: &Interaction) -> Result<()> {
        for hook in &self.hooks {
            hook.after_record(interaction)?;
        }
        Ok(())
    }

    /// Get number of hooks
    pub fn len(&self) -> usize {
        self.hooks.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.hooks.is_empty()
    }
}

impl Debug for RecordHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecordHooks")
            .field("count", &self.hooks.len())
            .finish()
    }
}

/// Collection of replay hooks
#[derive(Default)]
pub struct ReplayHooks {
    hooks: Vec<Arc<dyn ReplayHook>>,
}

impl ReplayHooks {
    /// Create a new empty hook collection
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    /// Add a hook to the collection
    pub fn add<H: ReplayHook + 'static>(&mut self, hook: H) {
        self.hooks.push(Arc::new(hook));
    }

    /// Execute before_replay on all hooks
    pub fn before_replay(&self, interaction: &mut Interaction) -> Result<()> {
        for hook in &self.hooks {
            hook.before_replay(interaction)?;
        }
        Ok(())
    }

    /// Execute after_replay on all hooks
    pub fn after_replay(&self, interaction: &Interaction) -> Result<()> {
        for hook in &self.hooks {
            hook.after_replay(interaction)?;
        }
        Ok(())
    }

    /// Get number of hooks
    pub fn len(&self) -> usize {
        self.hooks.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.hooks.is_empty()
    }
}

impl Debug for ReplayHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplayHooks")
            .field("count", &self.hooks.len())
            .finish()
    }
}

/// Built-in hooks for common use cases
pub mod builtins {
    use super::*;
    use crate::cassette::InteractionKind;
    use regex::Regex;
    use std::collections::HashSet;

    /// Filter sensitive headers from requests and responses
    #[derive(Debug, Clone)]
    pub struct SensitiveHeaderFilter {
        sensitive_headers: HashSet<String>,
    }

    impl SensitiveHeaderFilter {
        /// Create with default sensitive headers
        pub fn new() -> Self {
            let mut headers = HashSet::new();
            headers.insert("authorization".to_string());
            headers.insert("x-api-key".to_string());
            headers.insert("cookie".to_string());
            headers.insert("set-cookie".to_string());
            headers.insert("proxy-authorization".to_string());
            headers.insert("x-auth-token".to_string());

            Self {
                sensitive_headers: headers,
            }
        }

        /// Add custom sensitive header
        pub fn add_header(&mut self, header: impl Into<String>) {
            self.sensitive_headers.insert(header.into().to_lowercase());
        }
    }

    impl Default for SensitiveHeaderFilter {
        fn default() -> Self {
            Self::new()
        }
    }

    impl RecordHook for SensitiveHeaderFilter {
        fn before_record(&self, interaction: &mut Interaction) -> Result<()> {
            if let InteractionKind::Http { request, response } = &mut interaction.kind {
                // Filter request headers
                for header in &self.sensitive_headers {
                    if request.headers.contains_key(header) {
                        request
                            .headers
                            .insert(header.clone(), "[FILTERED]".to_string());
                    }
                }

                // Filter response headers
                for header in &self.sensitive_headers {
                    if response.headers.contains_key(header) {
                        response
                            .headers
                            .insert(header.clone(), "[FILTERED]".to_string());
                    }
                }
            }
            Ok(())
        }

        fn name(&self) -> &str {
            "SensitiveHeaderFilter"
        }
    }

    /// Replace patterns in request/response bodies
    #[derive(Debug)]
    pub struct BodyPatternReplacer {
        patterns: Vec<(Regex, String)>,
    }

    impl BodyPatternReplacer {
        /// Create new replacer
        pub fn new() -> Self {
            Self {
                patterns: Vec::new(),
            }
        }

        /// Add a pattern to replace
        pub fn add_pattern(&mut self, pattern: &str, replacement: impl Into<String>) -> Result<()> {
            let regex = Regex::new(pattern).map_err(|e| {
                crate::error::MatgtoError::Config(format!("Invalid regex pattern: {}", e))
            })?;
            self.patterns.push((regex, replacement.into()));
            Ok(())
        }
    }

    impl Default for BodyPatternReplacer {
        fn default() -> Self {
            Self::new()
        }
    }

    impl RecordHook for BodyPatternReplacer {
        fn before_record(&self, interaction: &mut Interaction) -> Result<()> {
            if let InteractionKind::Http { request, response } = &mut interaction.kind {
                // Replace in request body
                if let Some(body) = &request.body {
                    if let Ok(text) = String::from_utf8(body.clone()) {
                        let mut modified = text;
                        for (pattern, replacement) in &self.patterns {
                            modified = pattern.replace_all(&modified, replacement).to_string();
                        }
                        request.body = Some(modified.into_bytes());
                    }
                }

                // Replace in response body
                if let Some(body) = &response.body {
                    if let Ok(text) = String::from_utf8(body.clone()) {
                        let mut modified = text;
                        for (pattern, replacement) in &self.patterns {
                            modified = pattern.replace_all(&modified, replacement).to_string();
                        }
                        response.body = Some(modified.into_bytes());
                    }
                }
            }
            Ok(())
        }

        fn name(&self) -> &str {
            "BodyPatternReplacer"
        }
    }

    /// Log interactions to stderr
    #[derive(Debug, Default)]
    pub struct LoggingHook {
        verbose: bool,
    }

    impl LoggingHook {
        /// Create new logging hook
        pub fn new() -> Self {
            Self { verbose: false }
        }

        /// Enable verbose logging
        pub fn verbose(mut self) -> Self {
            self.verbose = true;
            self
        }
    }

    impl RecordHook for LoggingHook {
        fn before_record(&self, interaction: &mut Interaction) -> Result<()> {
            if let InteractionKind::Http { request, .. } = &interaction.kind {
                eprintln!("📝 Recording: {} {}", request.method, request.url);
                if self.verbose {
                    eprintln!("   Headers: {:?}", request.headers.keys());
                }
            }
            Ok(())
        }

        fn after_record(&self, _interaction: &Interaction) -> Result<()> {
            if self.verbose {
                eprintln!("✅ Recorded successfully");
            }
            Ok(())
        }

        fn name(&self) -> &str {
            "LoggingHook"
        }
    }

    impl ReplayHook for LoggingHook {
        fn before_replay(&self, interaction: &mut Interaction) -> Result<()> {
            if let InteractionKind::Http { request, response } = &interaction.kind {
                eprintln!(
                    "🔁 Replaying: {} {} -> {}",
                    request.method, request.url, response.status
                );
                if self.verbose {
                    eprintln!("   Headers: {:?}", request.headers.keys());
                }
            }
            Ok(())
        }

        fn after_replay(&self, _interaction: &Interaction) -> Result<()> {
            if self.verbose {
                eprintln!("✅ Replayed successfully");
            }
            Ok(())
        }

        fn name(&self) -> &str {
            "LoggingHook"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cassette::{HttpRequest, HttpResponse, Interaction, InteractionKind};
    use std::collections::HashMap;

    #[test]
    fn test_sensitive_header_filter() {
        let mut filter = builtins::SensitiveHeaderFilter::new();
        filter.add_header("x-custom-secret");

        let mut interaction = Interaction {
            kind: InteractionKind::Http {
                request: HttpRequest {
                    method: "GET".to_string(),
                    url: "https://api.example.com".to_string(),
                    headers: HashMap::from([
                        ("authorization".to_string(), "Bearer secret123".to_string()),
                        ("x-custom-secret".to_string(), "my-secret".to_string()),
                        ("content-type".to_string(), "application/json".to_string()),
                    ]),
                    body: None,
                },
                response: HttpResponse {
                    status: 200,
                    headers: HashMap::from([
                        ("set-cookie".to_string(), "session=abc123".to_string()),
                        ("content-type".to_string(), "application/json".to_string()),
                    ]),
                    body: None,
                },
            },
            recorded_at: chrono::Utc::now(),
            response_time_ms: None,
        };

        filter.before_record(&mut interaction).unwrap();

        if let InteractionKind::Http { request, response } = &interaction.kind {
            assert_eq!(
                request.headers.get("authorization"),
                Some(&"[FILTERED]".to_string())
            );
            assert_eq!(
                request.headers.get("x-custom-secret"),
                Some(&"[FILTERED]".to_string())
            );
            assert_eq!(
                request.headers.get("content-type"),
                Some(&"application/json".to_string())
            );

            assert_eq!(
                response.headers.get("set-cookie"),
                Some(&"[FILTERED]".to_string())
            );
            assert_eq!(
                response.headers.get("content-type"),
                Some(&"application/json".to_string())
            );
        } else {
            panic!("Expected HTTP interaction");
        }
    }

    #[test]
    fn test_body_pattern_replacer() {
        let mut replacer = builtins::BodyPatternReplacer::new();
        replacer
            .add_pattern(r#""password":"[^"]*""#, r#""password":"[FILTERED]""#)
            .unwrap();
        replacer
            .add_pattern(r#""token":"[^"]*""#, r#""token":"[FILTERED]""#)
            .unwrap();

        let mut interaction = Interaction {
            kind: InteractionKind::Http {
                request: HttpRequest {
                    method: "POST".to_string(),
                    url: "https://api.example.com/login".to_string(),
                    headers: HashMap::new(),
                    body: Some(br#"{"username":"alice","password":"secret123"}"#.to_vec()),
                },
                response: HttpResponse {
                    status: 200,
                    headers: HashMap::new(),
                    body: Some(br#"{"token":"jwt-token-xyz","user":"alice"}"#.to_vec()),
                },
            },
            recorded_at: chrono::Utc::now(),
            response_time_ms: None,
        };

        replacer.before_record(&mut interaction).unwrap();

        if let InteractionKind::Http { request, response } = &interaction.kind {
            let req_body = String::from_utf8(request.body.clone().unwrap()).unwrap();
            assert!(req_body.contains(r#""password":"[FILTERED]""#));
            assert!(!req_body.contains("secret123"));

            let res_body = String::from_utf8(response.body.clone().unwrap()).unwrap();
            assert!(res_body.contains(r#""token":"[FILTERED]""#));
            assert!(!res_body.contains("jwt-token-xyz"));
        }
    }

    #[test]
    fn test_hook_collections() {
        let mut record_hooks = RecordHooks::new();
        record_hooks.add(builtins::SensitiveHeaderFilter::new());
        record_hooks.add(builtins::LoggingHook::new());

        assert_eq!(record_hooks.len(), 2);

        let mut replay_hooks = ReplayHooks::new();
        replay_hooks.add(builtins::LoggingHook::new());

        assert_eq!(replay_hooks.len(), 1);
    }
}
