//! Playing back recorded cassettes

use crate::cassette::{Cassette, InteractionKind};
use crate::error::{MatgtoError, Result};
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

/// Plays back recorded interactions from cassettes
#[derive(Debug)]
pub struct Player {
    /// Loaded cassette
    cassette: Option<Cassette>,

    /// Index of interactions for fast lookup
    interactions_index: HashMap<RequestSignature, usize>,

    /// Count how many times each interaction has been replayed
    replay_count: HashMap<usize, usize>,
}

impl Player {
    /// Create a new player (empty)
    pub fn new() -> Self {
        Self {
            cassette: None,
            interactions_index: HashMap::new(),
            replay_count: HashMap::new(),
        }
    }

    /// Load a cassette from disk
    pub fn load(cassette_dir: &Path, name: &str) -> Result<Self> {
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

        tracing::info!(
            "Loaded cassette '{}' with {} interactions",
            name,
            cassette.interactions.len()
        );

        Ok(Self {
            cassette: Some(cassette),
            interactions_index,
            replay_count: HashMap::new(),
        })
    }

    /// Check if a cassette is loaded
    pub fn has_cassette(&self) -> bool {
        self.cassette.is_some()
    }

    /// Get the loaded cassette
    pub fn cassette(&self) -> Option<&Cassette> {
        self.cassette.as_ref()
    }

    /// Find a matching interaction by request signature
    pub fn find_interaction(&mut self, signature: &RequestSignature) -> Result<usize> {
        let idx = self.interactions_index.get(signature).ok_or_else(|| {
            MatgtoError::NoMatchingInteraction {
                method: signature.method.clone(),
                url: signature.url.clone(),
            }
        })?;

        // Increment replay counter
        *self.replay_count.entry(*idx).or_insert(0) += 1;

        Ok(*idx)
    }

    /// Get an interaction by index
    pub fn get_interaction(&self, idx: usize) -> Option<&crate::cassette::Interaction> {
        self.cassette.as_ref()?.interactions.get(idx)
    }

    /// Get total number of replays across all interactions
    pub fn replay_count(&self) -> usize {
        self.replay_count.values().sum()
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
}
