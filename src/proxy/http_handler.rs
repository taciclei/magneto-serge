//! HTTP/HTTPS proxy handler using Hudsucker

use crate::cassette::{HttpRequest, HttpResponse};
use crate::error::{MatgtoError, Result};
use crate::player::Player;
use crate::proxy::ProxyMode;
use crate::recorder::Recorder;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// HTTP handler for intercepting requests/responses
pub struct HttpHandler {
    /// Current proxy mode
    mode: ProxyMode,

    /// Recorder for capturing interactions (Record mode)
    recorder: Option<Arc<Mutex<Recorder>>>,

    /// Player for replaying interactions (Replay mode)
    player: Option<Arc<Mutex<Player>>>,
}

impl HttpHandler {
    /// Create a new HTTP handler
    pub fn new(mode: ProxyMode) -> Self {
        Self {
            mode,
            recorder: None,
            player: None,
        }
    }

    /// Set the recorder for Record mode
    pub fn with_recorder(mut self, recorder: Arc<Mutex<Recorder>>) -> Self {
        self.recorder = Some(recorder);
        self
    }

    /// Set the player for Replay mode
    pub fn with_player(mut self, player: Arc<Mutex<Player>>) -> Self {
        self.player = Some(player);
        self
    }

    /// Handle an HTTP request (to be integrated with Hudsucker)
    pub fn handle_request<'a>(
        &'a mut self,
        method: String,
        url: String,
        headers: HashMap<String, String>,
        body: Option<Vec<u8>>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<HttpResponse>> + Send + 'a>>
    {
        Box::pin(async move {
            match self.mode {
                ProxyMode::Record => {
                    // Forward request to real server and record
                    tracing::debug!("Recording request: {} {}", method, url);

                    // TODO: Actually forward the request
                    // For now, return a mock response
                    let response = HttpResponse {
                        status: 200,
                        headers: HashMap::new(),
                        body: Some(b"{}".to_vec()),
                    };

                    // Record the interaction
                    if let Some(recorder) = &self.recorder {
                        let request = HttpRequest {
                            method: method.clone(),
                            url: url.clone(),
                            headers: headers.clone(),
                            body: body.clone(),
                        };

                        recorder.lock().await.record_http(request, response.clone());
                    }

                    Ok(response)
                }

                ProxyMode::Replay | ProxyMode::ReplayStrict => {
                    // Match request against cassette
                    let is_strict = matches!(self.mode, ProxyMode::ReplayStrict);

                    if is_strict {
                        tracing::debug!("🔒 STRICT replaying request: {} {}", method, url);
                    } else {
                        tracing::debug!("Replaying request: {} {}", method, url);
                    }

                    if let Some(_player) = &self.player {
                        // TODO: Implement request matching and response replay
                        // For now, return a mock response
                        Ok(HttpResponse {
                            status: 200,
                            headers: HashMap::new(),
                            body: Some(b"{}".to_vec()),
                        })
                    } else {
                        Err(MatgtoError::ProxyStartFailed {
                            reason: format!("No player configured for {:?} mode", self.mode),
                        })
                    }
                }

                ProxyMode::Auto => {
                    // Check if cassette exists, decide mode dynamically
                    if let Some(player) = &self.player {
                        if player.lock().await.has_cassette() {
                            tracing::debug!("Auto mode: Using replay");
                            // Switch to replay
                            let mut handler =
                                Self::new(ProxyMode::Replay).with_player(player.clone());
                            return handler.handle_request(method, url, headers, body).await;
                        }
                    }

                    // Fall back to record mode
                    tracing::debug!("Auto mode: Using record");
                    if let Some(recorder) = &self.recorder {
                        let mut handler =
                            Self::new(ProxyMode::Record).with_recorder(recorder.clone());
                        return handler.handle_request(method, url, headers, body).await;
                    }

                    Err(MatgtoError::ProxyStartFailed {
                        reason: "Neither recorder nor player configured for Auto mode".to_string(),
                    })
                }

                ProxyMode::Passthrough => {
                    // Forward request without recording
                    tracing::debug!("Passthrough request: {} {}", method, url);

                    // TODO: Forward to actual server
                    Ok(HttpResponse {
                        status: 200,
                        headers: HashMap::new(),
                        body: Some(b"{}".to_vec()),
                    })
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;
    use crate::recorder::Recorder;

    #[tokio::test]
    async fn test_http_handler_record_mode() {
        let recorder = Arc::new(Mutex::new(Recorder::new("test".to_string())));
        let mut handler = HttpHandler::new(ProxyMode::Record).with_recorder(recorder.clone());

        let response = handler
            .handle_request(
                "GET".to_string(),
                "https://api.example.com/users".to_string(),
                HashMap::new(),
                None,
            )
            .await
            .unwrap();

        assert_eq!(response.status, 200);

        // Verify interaction was recorded
        let recorder_lock = recorder.lock().await;
        assert_eq!(recorder_lock.cassette().interactions.len(), 1);
    }

    #[tokio::test]
    async fn test_http_handler_replay_mode() {
        let player = Arc::new(Mutex::new(Player::new()));
        let mut handler = HttpHandler::new(ProxyMode::Replay).with_player(player);

        let response = handler
            .handle_request(
                "GET".to_string(),
                "https://api.example.com/users".to_string(),
                HashMap::new(),
                None,
            )
            .await
            .unwrap();

        assert_eq!(response.status, 200);
    }
}
