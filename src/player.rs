//! Playing back recorded cassettes

use crate::cassette::{Cassette, Interaction, InteractionKind};
use crate::cookies::CookieJar;
use crate::error::{MatgtoError, Result};
use crate::hooks::ReplayHooks;
use crate::matching::{MatchingStrategy, RequestSignature as MatchingSignature};
use crate::templates::TemplateEngine;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Signature used to match requests
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RequestSignature {
    pub method: String,
    pub url: String,
    pub body_hash: Option<u64>,
}

impl From<crate::cassette::HttpRequest> for RequestSignature {
    fn from(request: crate::cassette::HttpRequest) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let body_hash = request.body.as_ref().map(|b| {
            let mut hasher = DefaultHasher::new();
            b.hash(&mut hasher);
            hasher.finish()
        });

        Self {
            method: request.method,
            url: request.url,
            body_hash,
        }
    }
}

/// Latency simulation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatencyMode {
    /// No latency simulation
    None,

    /// Use recorded response times from cassette
    Recorded,

    /// Fixed delay for all responses (ms)
    Fixed(u64),

    /// Multiply recorded times by factor (e.g., 0.5 for half speed, 2.0 for double)
    Scaled(u64), // Store as integer percentage (100 = 1.0x, 200 = 2.0x)
}

/// Plays back recorded interactions from cassettes
#[derive(Debug)]
pub struct Player {
    /// Loaded cassette
    cassette: Option<Cassette>,

    /// Index of interactions for fast lookup
    interactions_index: HashMap<RequestSignature, usize>,

    /// Count how many times each interaction has been replayed
    replay_count: HashMap<usize, usize>,

    /// Strict mode: fail fast on missing interactions
    strict_mode: bool,

    /// Latency simulation mode
    latency_mode: LatencyMode,

    /// Advanced matching strategy
    matching_strategy: MatchingStrategy,

    /// Cookie jar for preserving cookies between requests (Phase 1.1)
    cookie_jar: CookieJar,

    /// Replay hooks
    hooks: ReplayHooks,

    /// Template engine for dynamic response rendering
    template_engine: TemplateEngine,
}

impl Player {
    /// Create a new player (empty)
    pub fn new() -> Self {
        Self {
            cassette: None,
            interactions_index: HashMap::new(),
            replay_count: HashMap::new(),
            strict_mode: false,
            latency_mode: LatencyMode::None,
            matching_strategy: MatchingStrategy::default(),
            cookie_jar: CookieJar::new(),
            hooks: ReplayHooks::new(),
            template_engine: TemplateEngine::new(),
        }
    }

    /// Create a new player in strict mode
    pub fn new_strict() -> Self {
        Self {
            cassette: None,
            interactions_index: HashMap::new(),
            replay_count: HashMap::new(),
            strict_mode: true,
            latency_mode: LatencyMode::None,
            matching_strategy: MatchingStrategy::strict(),
            cookie_jar: CookieJar::new(),
            hooks: ReplayHooks::new(),
            template_engine: TemplateEngine::new(),
        }
    }

    /// Set latency simulation mode
    pub fn with_latency(mut self, mode: LatencyMode) -> Self {
        self.latency_mode = mode;
        self
    }

    /// Get current latency mode
    pub fn latency_mode(&self) -> LatencyMode {
        self.latency_mode
    }

    /// Set matching strategy
    pub fn with_matching_strategy(mut self, strategy: MatchingStrategy) -> Self {
        self.matching_strategy = strategy;
        self
    }

    /// Get matching strategy
    pub fn matching_strategy(&self) -> &MatchingStrategy {
        &self.matching_strategy
    }

    /// Calculate delay for an interaction based on latency mode
    pub fn calculate_delay(&self, interaction: &crate::cassette::Interaction) -> Option<u64> {
        match self.latency_mode {
            LatencyMode::None => None,
            LatencyMode::Recorded => interaction.response_time_ms,
            LatencyMode::Fixed(ms) => Some(ms),
            LatencyMode::Scaled(percentage) => interaction
                .response_time_ms
                .map(|ms| (ms * percentage) / 100),
        }
    }

    /// Load a cassette from disk
    pub fn load(cassette_dir: &Path, name: &str) -> Result<Self> {
        Self::load_with_mode(cassette_dir, name, false)
    }

    /// Load a cassette from disk in strict mode
    pub fn load_strict(cassette_dir: &Path, name: &str) -> Result<Self> {
        Self::load_with_mode(cassette_dir, name, true)
    }

    /// Load a cassette from disk with specified mode
    fn load_with_mode(cassette_dir: &Path, name: &str, strict: bool) -> Result<Self> {
        let path = cassette_dir.join(format!("{}.json", name));

        if !path.exists() {
            return Err(MatgtoError::CassetteNotFound {
                name: name.to_string(),
            });
        }

        let file = File::open(path)?;
        let cassette: Cassette = serde_json::from_reader(file)?;

        // Build index for fast lookup
        let mut interactions_index = HashMap::new();

        for (idx, interaction) in cassette.interactions.iter().enumerate() {
            if let InteractionKind::Http { request, .. } = &interaction.kind {
                let signature = RequestSignature {
                    method: request.method.clone(),
                    url: request.url.clone(),
                    body_hash: request.body.as_ref().map(|b| {
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        b.hash(&mut hasher);
                        hasher.finish()
                    }),
                };
                interactions_index.insert(signature, idx);
            }
        }

        // Initialize cookie jar from cassette (Phase 1.1)
        let mut cookie_jar = CookieJar::new();
        if let Some(cookies) = &cassette.cookies {
            for cookie in cookies {
                cookie_jar.store(cookie.clone());
            }
        }

        if strict {
            tracing::info!(
                "🔒 Loaded cassette '{}' in STRICT mode with {} interactions",
                name,
                cassette.interactions.len()
            );
        } else {
            tracing::info!(
                "Loaded cassette '{}' with {} interactions",
                name,
                cassette.interactions.len()
            );
        }

        let matching_strategy = if strict {
            MatchingStrategy::strict()
        } else {
            MatchingStrategy::default()
        };

        Ok(Self {
            cassette: Some(cassette),
            interactions_index,
            replay_count: HashMap::new(),
            strict_mode: strict,
            latency_mode: LatencyMode::None,
            matching_strategy,
            cookie_jar,
            hooks: ReplayHooks::new(),
            template_engine: TemplateEngine::new(),
        })
    }

    /// Add a replay hook
    pub fn add_hook<H: crate::hooks::ReplayHook + 'static>(&mut self, hook: H) {
        self.hooks.add(hook);
    }

    /// Get hooks
    pub fn hooks(&self) -> &ReplayHooks {
        &self.hooks
    }

    /// Check if a cassette is loaded
    pub fn has_cassette(&self) -> bool {
        self.cassette.is_some()
    }

    /// Get the loaded cassette
    pub fn cassette(&self) -> Option<&Cassette> {
        self.cassette.as_ref()
    }

    /// Find a matching interaction by request signature (legacy, exact matching)
    pub fn find_interaction(&mut self, signature: &RequestSignature) -> Result<usize> {
        let idx = self.interactions_index.get(signature).ok_or_else(|| {
            if self.strict_mode {
                tracing::error!(
                    "🔒 STRICT MODE: No matching interaction found for {} {}",
                    signature.method,
                    signature.url
                );
                tracing::error!(
                    "💡 Available interactions in cassette: {}",
                    self.interactions_index.len()
                );
            }

            MatgtoError::NoMatchingInteraction {
                method: signature.method.clone(),
                url: signature.url.clone(),
            }
        })?;

        // Increment replay counter
        *self.replay_count.entry(*idx).or_insert(0) += 1;

        if self.strict_mode {
            tracing::debug!(
                "🔒 STRICT MODE: Found interaction #{} for {} {}",
                idx,
                signature.method,
                signature.url
            );
        }

        Ok(*idx)
    }

    /// Find a matching interaction using advanced matching strategy
    pub fn find_interaction_advanced(
        &mut self,
        request: &crate::cassette::HttpRequest,
    ) -> Result<usize> {
        let signature = MatchingSignature::from_request(request);

        // Find first matching interaction
        let cassette =
            self.cassette
                .as_ref()
                .ok_or_else(|| MatgtoError::NoMatchingInteraction {
                    method: request.method.clone(),
                    url: request.url.clone(),
                })?;

        for (idx, interaction) in cassette.interactions.iter().enumerate() {
            if let InteractionKind::Http {
                request: recorded_request,
                ..
            } = &interaction.kind
            {
                if signature.matches(recorded_request, &self.matching_strategy)? {
                    // Increment replay counter
                    *self.replay_count.entry(idx).or_insert(0) += 1;

                    if self.strict_mode {
                        tracing::debug!(
                            "🔒 STRICT MODE (advanced): Found interaction #{} for {} {}",
                            idx,
                            request.method,
                            request.url
                        );
                    }

                    return Ok(idx);
                }
            }
        }

        // No match found
        if self.strict_mode {
            tracing::error!(
                "🔒 STRICT MODE (advanced): No matching interaction found for {} {}",
                request.method,
                request.url
            );
            tracing::error!(
                "💡 Available interactions in cassette: {}",
                cassette.interactions.len()
            );
        }

        Err(MatgtoError::NoMatchingInteraction {
            method: request.method.clone(),
            url: request.url.clone(),
        })
    }

    /// Check if player is in strict mode
    pub fn is_strict(&self) -> bool {
        self.strict_mode
    }

    /// Get an interaction by index
    pub fn get_interaction(&self, idx: usize) -> Option<&crate::cassette::Interaction> {
        self.cassette.as_ref()?.interactions.get(idx)
    }

    /// Get an interaction by index with hooks applied (creates a clone)
    pub fn get_interaction_with_hooks(&self, idx: usize) -> Result<Interaction> {
        let interaction = self
            .cassette
            .as_ref()
            .and_then(|c| c.interactions.get(idx))
            .ok_or_else(|| {
                MatgtoError::Config(format!("Interaction index {} out of bounds", idx))
            })?;

        // Clone the interaction for mutation
        let mut interaction_clone = interaction.clone();

        // Apply before_replay hooks
        self.hooks.before_replay(&mut interaction_clone)?;

        Ok(interaction_clone)
    }

    /// Mark an interaction as replayed and call after_replay hooks
    pub fn mark_replayed(&self, interaction: &Interaction) -> Result<()> {
        // Call after_replay hooks
        self.hooks.after_replay(interaction)?;
        Ok(())
    }

    /// Get total number of replays across all interactions
    pub fn replay_count(&self) -> usize {
        self.replay_count.values().sum()
    }

    /// Get cookie jar (Phase 1.1)
    pub fn cookie_jar(&self) -> &CookieJar {
        &self.cookie_jar
    }

    /// Get mutable cookie jar (Phase 1.1)
    pub fn cookie_jar_mut(&mut self) -> &mut CookieJar {
        &mut self.cookie_jar
    }

    /// Get template engine
    pub fn template_engine(&self) -> &TemplateEngine {
        &self.template_engine
    }

    /// Get mutable template engine (for registering custom helpers)
    pub fn template_engine_mut(&mut self) -> &mut TemplateEngine {
        &mut self.template_engine
    }

    /// Render templates in an HTTP response body (if templates feature is enabled)
    #[cfg(feature = "templates")]
    pub fn render_templates_in_response(
        &self,
        request: &crate::cassette::HttpRequest,
        response: &mut crate::cassette::HttpResponse,
    ) -> Result<()> {
        // Only render if body contains template syntax
        if let Some(body) = &response.body {
            if let Ok(body_str) = std::str::from_utf8(body) {
                if TemplateEngine::has_templates(body_str) {
                    let rendered = self.template_engine.render(body_str, request)?;
                    response.body = Some(rendered.into_bytes());
                }
            }
        }
        Ok(())
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cassette::{HttpRequest, HttpResponse};
    use crate::error::MatgtoError;
    use crate::recorder::Recorder;
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_player_creation() {
        let player = Player::new();
        assert!(!player.has_cassette());
    }

    #[test]
    fn test_load_cassette() {
        // Create and save a test cassette
        let mut recorder = Recorder::new("test-load".to_string());

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

        recorder.record_http(request.clone(), response);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        // Load the cassette
        let player = Player::load(dir.path(), "test-load").unwrap();

        assert!(player.has_cassette());
        assert_eq!(player.cassette().unwrap().interactions.len(), 1);
    }

    #[test]
    fn test_load_nonexistent_cassette() {
        let dir = tempdir().unwrap();
        let result = Player::load(dir.path(), "nonexistent");

        assert!(result.is_err());
    }

    #[test]
    fn test_find_interaction() {
        let mut recorder = Recorder::new("test-find".to_string());

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

        recorder.record_http(request.clone(), response);

        let dir = tempdir().unwrap();
        recorder.save(dir.path()).unwrap();

        let mut player = Player::load(dir.path(), "test-find").unwrap();

        let signature = RequestSignature {
            method: request.method.clone(),
            url: request.url.clone(),
            body_hash: None,
        };

        let idx = player.find_interaction(&signature).unwrap();
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_player_strict_mode() {
        // Create and save a test cassette
        let mut recorder = Recorder::new("test-strict".to_string());

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

        // Load in strict mode
        let player = Player::load_strict(dir.path(), "test-strict").unwrap();

        assert!(player.has_cassette());
        assert!(player.is_strict());
        assert_eq!(player.cassette().unwrap().interactions.len(), 1);
    }

    #[test]
    fn test_strict_mode_missing_interaction() {
        // Create and save a test cassette with one interaction
        let mut recorder = Recorder::new("test-strict-missing".to_string());

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

        // Load in strict mode
        let mut player = Player::load_strict(dir.path(), "test-strict-missing").unwrap();

        // Try to find a different interaction that doesn't exist
        let missing_signature = RequestSignature {
            method: "POST".to_string(),
            url: "https://api.example.com/posts".to_string(),
            body_hash: None,
        };

        let result = player.find_interaction(&missing_signature);
        assert!(result.is_err());

        // Verify error is NoMatchingInteraction
        match result {
            Err(MatgtoError::NoMatchingInteraction { method, url }) => {
                assert_eq!(method, "POST");
                assert_eq!(url, "https://api.example.com/posts");
            }
            _ => panic!("Expected NoMatchingInteraction error"),
        }
    }

    #[test]
    fn test_non_strict_mode() {
        // Create and save a test cassette
        let mut recorder = Recorder::new("test-non-strict".to_string());

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

        // Load in normal mode (not strict)
        let player = Player::load(dir.path(), "test-non-strict").unwrap();

        assert!(player.has_cassette());
        assert!(!player.is_strict());
    }
}
