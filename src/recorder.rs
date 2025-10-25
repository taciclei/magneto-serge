//! Recording HTTP/WebSocket interactions to cassettes

use crate::cassette::{
    Cassette, HttpRequest, HttpResponse, Interaction, InteractionKind, NetworkError,
};
use crate::error::Result;
use crate::filters::RecordingFilters;
use crate::hooks::RecordHooks;
use std::fs::File;
use std::path::Path;

/// Records HTTP/WebSocket interactions
#[derive(Debug)]
pub struct Recorder {
    /// Name of the cassette being recorded
    cassette_name: String,

    /// The cassette being built
    cassette: Cassette,

    /// Recording filters
    filters: Option<RecordingFilters>,

    /// Record hooks
    hooks: RecordHooks,
}

impl Recorder {
    /// Create a new recorder
    pub fn new(cassette_name: String) -> Self {
        let cassette = Cassette::new(cassette_name.clone());

        Self {
            cassette_name,
            cassette,
            filters: None,
            hooks: RecordHooks::new(),
        }
    }

    /// Create a new recorder with filters
    pub fn new_with_filters(cassette_name: String, filters: RecordingFilters) -> Self {
        let cassette = Cassette::new(cassette_name.clone());

        Self {
            cassette_name,
            cassette,
            filters: Some(filters),
            hooks: RecordHooks::new(),
        }
    }

    /// Set recording filters
    pub fn set_filters(&mut self, filters: RecordingFilters) {
        self.filters = Some(filters);
    }

    /// Get current filters
    pub fn filters(&self) -> Option<&RecordingFilters> {
        self.filters.as_ref()
    }

    /// Add a record hook
    pub fn add_hook<H: crate::hooks::RecordHook + 'static>(&mut self, hook: H) {
        self.hooks.add(hook);
    }

    /// Get hooks
    pub fn hooks(&self) -> &RecordHooks {
        &self.hooks
    }

    /// Record an HTTP interaction
    pub fn record_http(&mut self, request: HttpRequest, response: HttpResponse) {
        // Apply filters if configured
        if let Some(filters) = &self.filters {
            // Check if interaction should be recorded
            if !filters.should_record(&request, &response) {
                tracing::debug!(
                    "Skipping recording for {} {} (filtered)",
                    request.method,
                    request.url
                );
                return;
            }
        }

        // Create interaction
        let mut interaction = Interaction {
            kind: InteractionKind::Http { request, response },
            recorded_at: chrono::Utc::now(),
            response_time_ms: None,
        };

        // Call before_record hooks
        if let Err(e) = self.hooks.before_record(&mut interaction) {
            tracing::error!("Hook before_record failed: {}", e);
            return;
        }

        // Add to cassette
        self.cassette.interactions.push(interaction.clone());

        // Call after_record hooks
        if let Err(e) = self.hooks.after_record(&interaction) {
            tracing::warn!("Hook after_record failed: {}", e);
        }
    }

    /// Record an HTTP error (timeout, DNS failure, connection refused, etc.)
    pub fn record_http_error(&mut self, request: HttpRequest, error: NetworkError) {
        // Log before consuming values
        tracing::info!(
            "Recording network error: {} {} - {:?}",
            request.method,
            request.url,
            error
        );

        // Create interaction
        let mut interaction = Interaction {
            kind: InteractionKind::HttpError { request, error },
            recorded_at: chrono::Utc::now(),
            response_time_ms: None,
        };

        // Call before_record hooks
        if let Err(e) = self.hooks.before_record(&mut interaction) {
            tracing::error!("Hook before_record failed for error: {}", e);
            return;
        }

        // Add to cassette
        self.cassette.interactions.push(interaction.clone());

        // Call after_record hooks
        if let Err(e) = self.hooks.after_record(&interaction) {
            tracing::warn!("Hook after_record failed for error: {}", e);
        }
    }

    /// Save the cassette to disk
    pub fn save(&self, cassette_dir: &Path) -> Result<()> {
        // Ensure directory exists
        std::fs::create_dir_all(cassette_dir)?;

        let path = cassette_dir.join(format!("{}.json", self.cassette_name));
        let file = File::create(path)?;

        serde_json::to_writer_pretty(file, &self.cassette)?;

        tracing::info!(
            "Saved cassette '{}' with {} interactions",
            self.cassette_name,
            self.cassette.interactions.len()
        );

        Ok(())
    }

    /// Get the current cassette
    pub fn cassette(&self) -> &Cassette {
        &self.cassette
    }

    /// Get mutable reference to the cassette (for hybrid mode)
    pub fn cassette_mut(&mut self) -> &mut Cassette {
        &mut self.cassette
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_recorder_creation() {
        let recorder = Recorder::new("test".to_string());
        assert_eq!(recorder.cassette().name, "test");
        assert_eq!(recorder.cassette().interactions.len(), 0);
    }

    #[test]
    fn test_record_http() {
        let mut recorder = Recorder::new("test".to_string());

        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/users".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(b"{\"users\":[]}".to_vec()),
        };

        recorder.record_http(request, response);

        assert_eq!(recorder.cassette().interactions.len(), 1);
    }

    #[test]
    fn test_save_cassette() {
        let mut recorder = Recorder::new("test-save".to_string());

        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/users".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(b"{\"users\":[]}".to_vec()),
        };

        recorder.record_http(request, response);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        // Verify file was created
        let cassette_path = dir.path().join("test-save.json");
        assert!(cassette_path.exists());
    }

    #[test]
    fn test_record_http_error_timeout() {
        let mut recorder = Recorder::new("test-error".to_string());

        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://slow-api.example.com/timeout".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let error = NetworkError::timeout("Connection timed out after 5000ms", 5000);

        recorder.record_http_error(request, error);

        assert_eq!(recorder.cassette().interactions.len(), 1);

        let interaction = &recorder.cassette().interactions[0];
        match &interaction.kind {
            InteractionKind::HttpError { request, error } => {
                assert_eq!(request.method, "GET");
                assert_eq!(request.url, "https://slow-api.example.com/timeout");
                match error {
                    NetworkError::Timeout { timeout_ms, .. } => {
                        assert_eq!(*timeout_ms, 5000);
                    }
                    _ => panic!("Expected Timeout error"),
                }
            }
            _ => panic!("Expected HttpError interaction"),
        }
    }

    #[test]
    fn test_record_http_error_dns() {
        let mut recorder = Recorder::new("test-dns-error".to_string());

        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://nonexistent.invalid/api".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let error = NetworkError::dns_failed("Failed to resolve domain");

        recorder.record_http_error(request, error);

        assert_eq!(recorder.cassette().interactions.len(), 1);

        let interaction = &recorder.cassette().interactions[0];
        match &interaction.kind {
            InteractionKind::HttpError { error, .. } => {
                assert!(matches!(error, NetworkError::DnsResolutionFailed { .. }));
            }
            _ => panic!("Expected HttpError interaction"),
        }
    }

    #[test]
    fn test_record_http_error_connection_refused() {
        let mut recorder = Recorder::new("test-connection-refused".to_string());

        let request = HttpRequest {
            method: "POST".to_string(),
            url: "http://localhost:9999/api".to_string(),
            headers: HashMap::new(),
            body: Some(b"{\"test\":true}".to_vec()),
        };

        let error = NetworkError::connection_refused("Connection refused on port 9999");

        recorder.record_http_error(request, error);

        assert_eq!(recorder.cassette().interactions.len(), 1);

        let interaction = &recorder.cassette().interactions[0];
        match &interaction.kind {
            InteractionKind::HttpError { request, error } => {
                assert_eq!(request.method, "POST");
                assert_eq!(request.url, "http://localhost:9999/api");
                assert!(matches!(error, NetworkError::ConnectionRefused { .. }));
            }
            _ => panic!("Expected HttpError interaction"),
        }
    }

    #[test]
    fn test_record_mixed_interactions() {
        let mut recorder = Recorder::new("test-mixed".to_string());

        // Record successful HTTP
        let request1 = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/success".to_string(),
            headers: HashMap::new(),
            body: None,
        };
        let response1 = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(b"OK".to_vec()),
        };
        recorder.record_http(request1, response1);

        // Record error
        let request2 = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/timeout".to_string(),
            headers: HashMap::new(),
            body: None,
        };
        let error = NetworkError::timeout("Request timed out", 3000);
        recorder.record_http_error(request2, error);

        // Record another success
        let request3 = HttpRequest {
            method: "POST".to_string(),
            url: "https://api.example.com/data".to_string(),
            headers: HashMap::new(),
            body: Some(b"{\"data\":1}".to_vec()),
        };
        let response3 = HttpResponse {
            status: 201,
            headers: HashMap::new(),
            body: Some(b"{\"id\":123}".to_vec()),
        };
        recorder.record_http(request3, response3);

        // Verify we have 3 interactions
        assert_eq!(recorder.cassette().interactions.len(), 3);

        // Verify order and types
        assert!(matches!(
            &recorder.cassette().interactions[0].kind,
            InteractionKind::Http { .. }
        ));
        assert!(matches!(
            &recorder.cassette().interactions[1].kind,
            InteractionKind::HttpError { .. }
        ));
        assert!(matches!(
            &recorder.cassette().interactions[2].kind,
            InteractionKind::Http { .. }
        ));
    }
}
