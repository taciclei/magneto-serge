//! Template rendering for dynamic cassette responses
//!
//! This module provides support for Handlebars templates in cassette responses,
//! allowing dynamic content generation during replay.
//!
//! # Features
//!
//! - Environment variable substitution: `{{ env.API_KEY }}`
//! - Dynamic timestamps: `{{ now }}`, `{{ now_iso }}`
//! - Request data access: `{{ request.headers.user_id }}`
//! - Custom helper functions
//!
//! # Example
//!
//! ```rust,ignore
//! use magneto_serge::templates::TemplateEngine;
//! use magneto_serge::cassette::HttpRequest;
//!
//! let mut engine = TemplateEngine::new();
//!
//! // Register custom helper
//! engine.register_helper("random_id", |_| {
//!     format!("id_{}", rand::random::<u32>())
//! });
//!
//! // Render template with request context
//! let request = HttpRequest { /* ... */ };
//! let body = r#"{"api_key":"{{ env.API_KEY }}","timestamp":"{{ now }}","user":"{{ request.headers.x-user-id }}"}"#;
//! let rendered = engine.render(body, &request)?;
//! ```

#[cfg(feature = "templates")]
use handlebars::Handlebars;

#[cfg(feature = "templates")]
use crate::cassette::HttpRequest;
#[cfg(feature = "templates")]
use crate::error::{MatgtoError, Result};
#[cfg(feature = "templates")]
use serde_json::json;

/// Template engine for rendering dynamic cassette responses
#[cfg(feature = "templates")]
pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

#[cfg(feature = "templates")]
impl std::fmt::Debug for TemplateEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateEngine")
            .field("handlebars", &"<Handlebars instance>")
            .finish()
    }
}

#[cfg(feature = "templates")]
impl TemplateEngine {
    /// Create a new template engine with built-in helpers
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();

        // Register built-in helpers
        Self::register_builtin_helpers(&mut handlebars);

        Self { handlebars }
    }

    /// Register built-in helper functions
    fn register_builtin_helpers(hb: &mut Handlebars<'static>) {
        // Environment variable helper
        hb.register_helper(
            "env",
            Box::new(
                |h: &handlebars::Helper,
                 _: &Handlebars,
                 _: &handlebars::Context,
                 _: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output|
                 -> handlebars::HelperResult {
                    let param = h
                        .param(0)
                        .and_then(|v| v.value().as_str())
                        .ok_or_else(|| {
                            handlebars::RenderErrorReason::Other("env helper requires a parameter".to_string())
                        })?;

                    let value = std::env::var(param).unwrap_or_else(|_| String::from(""));
                    out.write(&value)?;
                    Ok(())
                },
            ),
        );

        // Current timestamp (ISO 8601)
        hb.register_helper(
            "now",
            Box::new(
                |_: &handlebars::Helper,
                 _: &Handlebars,
                 _: &handlebars::Context,
                 _: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output|
                 -> handlebars::HelperResult {
                    let now = chrono::Utc::now().to_rfc3339();
                    out.write(&now)?;
                    Ok(())
                },
            ),
        );

        // Current timestamp (Unix epoch)
        hb.register_helper(
            "now_timestamp",
            Box::new(
                |_: &handlebars::Helper,
                 _: &Handlebars,
                 _: &handlebars::Context,
                 _: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output|
                 -> handlebars::HelperResult {
                    let now = chrono::Utc::now().timestamp();
                    out.write(&now.to_string())?;
                    Ok(())
                },
            ),
        );

        // UUID v4 generator
        hb.register_helper(
            "uuid",
            Box::new(
                |_: &handlebars::Helper,
                 _: &Handlebars,
                 _: &handlebars::Context,
                 _: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output|
                 -> handlebars::HelperResult {
                    let uuid = uuid::Uuid::new_v4().to_string();
                    out.write(&uuid)?;
                    Ok(())
                },
            ),
        );
    }

    /// Register a custom helper function
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// engine.register_helper("random_id", |_| {
    ///     format!("id_{}", rand::random::<u32>())
    /// });
    /// ```
    pub fn register_helper<F>(&mut self, name: &str, helper_fn: F)
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        self.handlebars.register_helper(
            name,
            Box::new(
                move |_: &handlebars::Helper,
                      _: &Handlebars,
                      _: &handlebars::Context,
                      _: &mut handlebars::RenderContext,
                      out: &mut dyn handlebars::Output|
                      -> handlebars::HelperResult {
                    out.write(&helper_fn())?;
                    Ok(())
                },
            ),
        );
    }

    /// Render a template with request context
    ///
    /// # Arguments
    ///
    /// * `template` - The template string (may contain Handlebars syntax)
    /// * `request` - The HTTP request providing context data
    ///
    /// # Returns
    ///
    /// The rendered template string
    pub fn render(&self, template: &str, request: &HttpRequest) -> Result<String> {
        // Build context from request
        let context = self.build_context(request);

        // Render template
        self.handlebars
            .render_template(template, &context)
            .map_err(|e| MatgtoError::TemplateError {
                message: format!("Failed to render template: {}", e),
            })
    }

    /// Build template context from HTTP request
    fn build_context(&self, request: &HttpRequest) -> serde_json::Value {
        json!({
            "request": {
                "method": request.method,
                "url": request.url,
                "headers": request.headers,
                "body": request.body.as_ref().and_then(|b| std::str::from_utf8(b).ok()),
            }
        })
    }

    /// Check if a string contains template syntax
    pub fn has_templates(text: &str) -> bool {
        text.contains("{{") && text.contains("}}")
    }
}

#[cfg(feature = "templates")]
impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Stub implementation when templates feature is disabled
#[cfg(not(feature = "templates"))]
#[derive(Debug)]
pub struct TemplateEngine;

#[cfg(not(feature = "templates"))]
impl TemplateEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, template: &str, _request: &crate::cassette::HttpRequest) -> crate::error::Result<String> {
        // When templates feature is disabled, just return the template as-is
        Ok(template.to_string())
    }

    pub fn has_templates(_text: &str) -> bool {
        false
    }
}

#[cfg(not(feature = "templates"))]
impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, feature = "templates"))]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_request() -> HttpRequest {
        let mut headers = HashMap::new();
        headers.insert("x-user-id".to_string(), "user123".to_string());
        headers.insert("authorization".to_string(), "Bearer token".to_string());

        HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/users".to_string(),
            headers,
            body: None,
        }
    }

    #[test]
    fn test_has_templates() {
        assert!(TemplateEngine::has_templates("Hello {{ name }}"));
        assert!(TemplateEngine::has_templates("{{ env.API_KEY }}"));
        assert!(!TemplateEngine::has_templates("Plain text"));
        assert!(!TemplateEngine::has_templates("Just { one brace"));
    }

    #[test]
    fn test_render_plain_text() {
        let engine = TemplateEngine::new();
        let request = create_test_request();

        let result = engine.render("Plain text", &request).unwrap();
        assert_eq!(result, "Plain text");
    }

    #[test]
    fn test_render_request_headers() {
        let engine = TemplateEngine::new();
        let request = create_test_request();

        let template = r#"User: {{ request.headers.x-user-id }}"#;
        let result = engine.render(template, &request).unwrap();
        assert_eq!(result, "User: user123");
    }

    #[test]
    fn test_render_uuid() {
        let engine = TemplateEngine::new();
        let request = create_test_request();

        let template = r#"ID: {{ uuid }}"#;
        let result = engine.render(template, &request).unwrap();
        assert!(result.starts_with("ID: "));
        assert_eq!(result.len(), 40); // "ID: " + UUID
    }

    #[test]
    fn test_render_now() {
        let engine = TemplateEngine::new();
        let request = create_test_request();

        let template = r#"Timestamp: {{ now }}"#;
        let result = engine.render(template, &request).unwrap();
        assert!(result.starts_with("Timestamp: "));
        assert!(result.contains("T")); // ISO 8601 format
    }

    #[test]
    fn test_render_env_variable() {
        std::env::set_var("TEST_API_KEY", "secret123");

        let engine = TemplateEngine::new();
        let request = create_test_request();

        let template = r#"Key: {{ env "TEST_API_KEY" }}"#;
        let result = engine.render(template, &request).unwrap();
        assert_eq!(result, "Key: secret123");

        std::env::remove_var("TEST_API_KEY");
    }

    #[test]
    fn test_custom_helper() {
        let mut engine = TemplateEngine::new();
        engine.register_helper("custom", || "custom_value".to_string());

        let request = create_test_request();
        let template = r#"Value: {{ custom }}"#;
        let result = engine.render(template, &request).unwrap();
        assert_eq!(result, "Value: custom_value");
    }

    #[test]
    fn test_complex_template() {
        std::env::set_var("API_URL", "https://api.example.com");

        let engine = TemplateEngine::new();
        let request = create_test_request();

        let template = r#"{
  "user_id": "{{ request.headers.x-user-id }}",
  "api_url": "{{ env "API_URL" }}",
  "timestamp": "{{ now }}",
  "request_id": "{{ uuid }}"
}"#;

        let result = engine.render(template, &request).unwrap();
        assert!(result.contains(r#""user_id": "user123""#));
        assert!(result.contains(r#""api_url": "https://api.example.com""#));
        assert!(result.contains(r#""timestamp": ""#));
        assert!(result.contains(r#""request_id": ""#));

        std::env::remove_var("API_URL");
    }
}
