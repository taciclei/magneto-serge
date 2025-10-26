//! Interaction Hydra Resource
//!
//! Hypermedia representation of HTTP/WebSocket interactions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cassette::{HttpRequest, HttpResponse, Interaction, InteractionKind, WebSocketMessage};
use crate::hydra::{HydraLink, HydraOperation};

/// Interaction Resource
///
/// Hypermedia representation of an interaction with links and operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum InteractionResource {
    Http {
        #[serde(rename = "@id")]
        id: String,

        request: HttpRequestResource,
        response: HttpResponseResource,

        #[serde(rename = "_links")]
        links: InteractionLinks,
    },
    WebSocket {
        #[serde(rename = "@id")]
        id: String,

        url: String,
        messages: Vec<WebSocketMessageResource>,

        #[serde(rename = "_links")]
        links: InteractionLinks,
    },
}

impl InteractionResource {
    /// Create from an interaction
    pub fn from_interaction(
        interaction: &Interaction,
        cassette_name: &str,
        index: usize,
        base_url: &str,
    ) -> Self {
        let interaction_url = format!(
            "{}/api/cassettes/{}/interactions/{}",
            base_url, cassette_name, index
        );

        match &interaction.kind {
            InteractionKind::Http { request, response } => Self::Http {
                id: interaction_url.clone(),
                request: HttpRequestResource::from_request(request),
                response: HttpResponseResource::from_response(response),
                links: InteractionLinks::new(&interaction_url, cassette_name, index),
            },
            InteractionKind::HttpError { request, error } => {
                // For HTTP errors, create a synthetic 500 response with error details
                let error_response = HttpResponse {
                    status: 500,
                    headers: std::collections::HashMap::new(),
                    body: Some(
                        serde_json::json!({
                            "error": format!("{:?}", error)
                        })
                        .to_string()
                        .into_bytes(),
                    ),
                };

                Self::Http {
                    id: interaction_url.clone(),
                    request: HttpRequestResource::from_request(request),
                    response: HttpResponseResource::from_response(&error_response),
                    links: InteractionLinks::new(&interaction_url, cassette_name, index),
                }
            }
            InteractionKind::WebSocket {
                url,
                messages,
                close_frame: _,
            } => Self::WebSocket {
                id: interaction_url.clone(),
                url: url.clone(),
                messages: messages
                    .iter()
                    .map(WebSocketMessageResource::from_message)
                    .collect(),
                links: InteractionLinks::new(&interaction_url, cassette_name, index),
            },
        }
    }

    /// Get available operations
    pub fn operations() -> Vec<HydraOperation> {
        vec![
            HydraOperation::get("Retrieve interaction", "Interaction"),
            HydraOperation::put("Update interaction", "InteractionInput", "Interaction"),
            HydraOperation::delete("Delete interaction"),
        ]
    }
}

/// HTTP Request Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequestResource {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl HttpRequestResource {
    fn from_request(request: &HttpRequest) -> Self {
        Self {
            method: request.method.clone(),
            url: request.url.clone(),
            headers: request.headers.clone(),
            body: request
                .body
                .as_ref()
                .and_then(|b| String::from_utf8(b.clone()).ok()),
        }
    }
}

/// HTTP Response Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponseResource {
    pub status: u16,
    pub headers: HashMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Indicates if response contains templates (v0.4.0)
    #[serde(rename = "hasTemplates")]
    pub has_templates: bool,
}

impl HttpResponseResource {
    fn from_response(response: &HttpResponse) -> Self {
        let body_str = response
            .body
            .as_ref()
            .and_then(|b| String::from_utf8(b.clone()).ok());

        let has_templates = body_str
            .as_ref()
            .map(|s| s.contains("{{") && s.contains("}}"))
            .unwrap_or(false);

        Self {
            status: response.status,
            headers: response.headers.clone(),
            body: body_str,
            has_templates,
        }
    }
}

/// WebSocket Message Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessageResource {
    pub direction: String,

    #[serde(rename = "timestampMs")]
    pub timestamp_ms: u64,

    #[serde(rename = "msgType")]
    pub msg_type: String,

    pub data: String,
}

impl WebSocketMessageResource {
    fn from_message(message: &WebSocketMessage) -> Self {
        use crate::cassette::MessagePayload;

        let (msg_type, data) = match &message.payload {
            MessagePayload::Text { data } => ("Text".to_string(), data.clone()),
            MessagePayload::Binary { data } => {
                ("Binary".to_string(), format!("[{} bytes]", data.len()))
            }
            MessagePayload::Ping { data } => {
                ("Ping".to_string(), format!("[{} bytes]", data.len()))
            }
            MessagePayload::Pong { data } => {
                ("Pong".to_string(), format!("[{} bytes]", data.len()))
            }
        };

        Self {
            direction: format!("{:?}", message.direction),
            timestamp_ms: message.timestamp_ms,
            msg_type,
            data,
        }
    }
}

/// Interaction Links
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionLinks {
    #[serde(rename = "self")]
    pub self_link: HydraLink,

    pub cassette: HydraLink,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<HydraLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<HydraLink>,
}

impl InteractionLinks {
    fn new(interaction_url: &str, cassette_name: &str, index: usize) -> Self {
        let cassette_url = format!("/api/cassettes/{}", cassette_name);

        Self {
            self_link: HydraLink::new(interaction_url),
            cassette: HydraLink::new(&cassette_url).with_title("Parent cassette"),
            previous: if index > 0 {
                Some(
                    HydraLink::new(&format!("{}/interactions/{}", cassette_url, index - 1))
                        .with_title("Previous interaction"),
                )
            } else {
                None
            },
            next: Some(
                HydraLink::new(&format!("{}/interactions/{}", cassette_url, index + 1))
                    .with_title("Next interaction"),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_interaction_resource() {
        let request = HttpRequest {
            method: "GET".to_string(),
            url: "https://api.example.com/test".to_string(),
            headers: HashMap::new(),
            body: None,
        };

        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(b"{\"test\":true}".to_vec()),
        };

        let interaction = Interaction {
            kind: InteractionKind::Http { request, response },
            recorded_at: chrono::Utc::now(),
            response_time_ms: Some(100),
        };

        let resource =
            InteractionResource::from_interaction(&interaction, "test", 0, "http://localhost:8889");

        match resource {
            InteractionResource::Http {
                request, response, ..
            } => {
                assert_eq!(request.method, "GET");
                assert_eq!(response.status, 200);
                assert!(!response.has_templates);
            }
            _ => panic!("Expected Http interaction"),
        }
    }

    #[test]
    fn test_template_detection() {
        let response = HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: Some(b"{\"token\":\"{{ env \\\"API_KEY\\\" }}\"}".to_vec()),
        };

        let resource = HttpResponseResource::from_response(&response);
        assert!(resource.has_templates);
    }
}
