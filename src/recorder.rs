//! Recording HTTP/WebSocket interactions to cassettes

use crate::cassette::{Cassette, InteractionKind, HttpRequest, HttpResponse};
use crate::error::Result;
use std::path::Path;
use std::fs::File;

/// Records HTTP/WebSocket interactions
#[derive(Debug)]
pub struct Recorder {
    /// Name of the cassette being recorded
    cassette_name: String,
    
    /// The cassette being built
    cassette: Cassette,
}

impl Recorder {
    /// Create a new recorder
    pub fn new(cassette_name: String) -> Self {
        let cassette = Cassette::new(cassette_name.clone());
        
        Self {
            cassette_name,
            cassette,
        }
    }
    
    /// Record an HTTP interaction
    pub fn record_http(&mut self, request: HttpRequest, response: HttpResponse) {
        let interaction = InteractionKind::Http { request, response };
        self.cassette.add_interaction(interaction);
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
}
