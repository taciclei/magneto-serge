//! OpenAPI 3.0 specification generation for Magneto-Serge API

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// OpenAPI 3.0 specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: OpenApiInfo,
    pub servers: Vec<OpenApiServer>,
    pub paths: HashMap<String, HashMap<String, OpenApiOperation>>,
    pub components: OpenApiComponents,
}

/// API information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiInfo {
    pub title: String,
    pub description: String,
    pub version: String,
    pub contact: OpenApiContact,
    pub license: OpenApiLicense,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiContact {
    pub name: String,
    pub url: String,
    pub email: String,
}

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiLicense {
    pub name: String,
    pub url: String,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiServer {
    pub url: String,
    pub description: String,
}

/// API operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiOperation {
    pub summary: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "requestBody")]
    pub request_body: Option<OpenApiRequestBody>,
    pub responses: HashMap<String, OpenApiResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
}

/// Request body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiRequestBody {
    pub description: String,
    pub required: bool,
    pub content: HashMap<String, OpenApiMediaType>,
}

/// Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiResponse {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<String, OpenApiMediaType>>,
}

/// Media type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiMediaType {
    pub schema: serde_json::Value,
}

/// Components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiComponents {
    pub schemas: HashMap<String, serde_json::Value>,
    #[serde(rename = "securitySchemes")]
    pub security_schemes: HashMap<String, OpenApiSecurityScheme>,
}

/// Security scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSecurityScheme {
    #[serde(rename = "type")]
    pub scheme_type: String,
    pub scheme: String,
    #[serde(rename = "bearerFormat")]
    pub bearer_format: String,
    pub description: String,
}

/// Generate OpenAPI specification for the API
pub fn generate_openapi_spec(host: &str, port: u16) -> OpenApiSpec {
    let base_url = format!("http://{}:{}", host, port);

    OpenApiSpec {
        openapi: "3.0.3".to_string(),
        info: OpenApiInfo {
            title: "Magneto-Serge API".to_string(),
            description: "REST API for controlling Magneto-Serge HTTP/WebSocket proxy with record/replay capabilities".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            contact: OpenApiContact {
                name: "Magneto-Serge Contributors".to_string(),
                url: "https://github.com/taciclei/magneto-serge".to_string(),
                email: "contact@taciclei.com".to_string(),
            },
            license: OpenApiLicense {
                name: "MIT OR Apache-2.0".to_string(),
                url: "https://github.com/taciclei/magneto-serge/blob/main/LICENSE".to_string(),
            },
        },
        servers: vec![OpenApiServer {
            url: base_url,
            description: "Local API server".to_string(),
        }],
        paths: generate_paths(),
        components: generate_components(),
    }
}

fn generate_paths() -> HashMap<String, HashMap<String, OpenApiOperation>> {
    let mut paths = HashMap::new();

    // GET /
    let mut root_methods = HashMap::new();
    root_methods.insert(
        "get".to_string(),
        OpenApiOperation {
            summary: "API Entry Point".to_string(),
            description: "Get API information and available links (Hydra/JSON-LD)".to_string(),
            tags: Some(vec!["API".to_string()]),
            request_body: None,
            responses: generate_response_200("API information with Hydra links"),
            security: None,
        },
    );
    paths.insert("/".to_string(), root_methods);

    // GET /health
    let mut health_methods = HashMap::new();
    health_methods.insert(
        "get".to_string(),
        OpenApiOperation {
            summary: "Health Check".to_string(),
            description: "Check API server health and uptime".to_string(),
            tags: Some(vec!["Monitoring".to_string()]),
            request_body: None,
            responses: generate_response_200("Health status"),
            security: None,
        },
    );
    paths.insert("/health".to_string(), health_methods);

    // POST /proxy/start
    let mut start_methods = HashMap::new();
    start_methods.insert(
        "post".to_string(),
        OpenApiOperation {
            summary: "Start Proxy".to_string(),
            description: "Start the proxy in specified mode (auto, record, replay, passthrough)"
                .to_string(),
            tags: Some(vec!["Proxy Control".to_string()]),
            request_body: Some(OpenApiRequestBody {
                description: "Proxy start configuration".to_string(),
                required: true,
                content: generate_json_content(json!({
                    "type": "object",
                    "required": ["mode", "cassette_name"],
                    "properties": {
                        "mode": {
                            "type": "string",
                            "enum": ["auto", "record", "replay", "passthrough"],
                            "description": "Proxy mode"
                        },
                        "cassette_name": {
                            "type": "string",
                            "description": "Name of the cassette to use"
                        },
                        "port": {
                            "type": "integer",
                            "description": "Proxy port (optional, defaults to config)"
                        },
                        "strict": {
                            "type": "boolean",
                            "description": "Strict mode for replay (optional, default false)"
                        }
                    }
                })),
            }),
            responses: {
                let mut responses = generate_response_200("Proxy started successfully");
                responses.insert(
                    "409".to_string(),
                    OpenApiResponse {
                        description: "Proxy already running".to_string(),
                        content: Some(generate_json_content(
                            json!({"$ref": "#/components/schemas/ErrorResponse"}),
                        )),
                    },
                );
                responses
            },
            security: Some(vec![{
                let mut auth = HashMap::new();
                auth.insert("bearerAuth".to_string(), vec![]);
                auth
            }]),
        },
    );
    paths.insert("/proxy/start".to_string(), start_methods);

    // POST /proxy/stop
    let mut stop_methods = HashMap::new();
    stop_methods.insert(
        "post".to_string(),
        OpenApiOperation {
            summary: "Stop Proxy".to_string(),
            description: "Stop the running proxy".to_string(),
            tags: Some(vec!["Proxy Control".to_string()]),
            request_body: Some(OpenApiRequestBody {
                description: "Stop configuration".to_string(),
                required: false,
                content: generate_json_content(json!({
                    "type": "object",
                    "properties": {
                        "force": {
                            "type": "boolean",
                            "description": "Force stop (optional, default false)"
                        }
                    }
                })),
            }),
            responses: {
                let mut responses = generate_response_200("Proxy stopped successfully");
                responses.insert(
                    "404".to_string(),
                    OpenApiResponse {
                        description: "Proxy not running".to_string(),
                        content: Some(generate_json_content(
                            json!({"$ref": "#/components/schemas/ErrorResponse"}),
                        )),
                    },
                );
                responses
            },
            security: Some(vec![{
                let mut auth = HashMap::new();
                auth.insert("bearerAuth".to_string(), vec![]);
                auth
            }]),
        },
    );
    paths.insert("/proxy/stop".to_string(), stop_methods);

    // GET /proxy/status
    let mut status_methods = HashMap::new();
    status_methods.insert(
        "get".to_string(),
        OpenApiOperation {
            summary: "Get Proxy Status".to_string(),
            description: "Get current proxy status and configuration".to_string(),
            tags: Some(vec!["Proxy Control".to_string()]),
            request_body: None,
            responses: generate_response_200("Proxy status"),
            security: Some(vec![{
                let mut auth = HashMap::new();
                auth.insert("bearerAuth".to_string(), vec![]);
                auth
            }]),
        },
    );
    paths.insert("/proxy/status".to_string(), status_methods);

    // GET /proxy/stats
    let mut stats_methods = HashMap::new();
    stats_methods.insert(
        "get".to_string(),
        OpenApiOperation {
            summary: "Get Proxy Statistics".to_string(),
            description: "Get proxy performance statistics".to_string(),
            tags: Some(vec!["Monitoring".to_string()]),
            request_body: None,
            responses: generate_response_200("Proxy statistics"),
            security: Some(vec![{
                let mut auth = HashMap::new();
                auth.insert("bearerAuth".to_string(), vec![]);
                auth
            }]),
        },
    );
    paths.insert("/proxy/stats".to_string(), stats_methods);

    // GET /cassettes
    let mut cassettes_methods = HashMap::new();
    cassettes_methods.insert(
        "get".to_string(),
        OpenApiOperation {
            summary: "List Cassettes".to_string(),
            description: "List all available cassettes".to_string(),
            tags: Some(vec!["Cassettes".to_string()]),
            request_body: None,
            responses: generate_response_200("List of cassettes"),
            security: Some(vec![{
                let mut auth = HashMap::new();
                auth.insert("bearerAuth".to_string(), vec![]);
                auth
            }]),
        },
    );
    paths.insert("/cassettes".to_string(), cassettes_methods);

    // GET /cassettes/{name}
    let mut cassette_get_methods = HashMap::new();
    cassette_get_methods.insert(
        "get".to_string(),
        OpenApiOperation {
            summary: "Get Cassette".to_string(),
            description: "Get cassette content by name".to_string(),
            tags: Some(vec!["Cassettes".to_string()]),
            request_body: None,
            responses: {
                let mut responses = generate_response_200("Cassette content");
                responses.insert(
                    "404".to_string(),
                    OpenApiResponse {
                        description: "Cassette not found".to_string(),
                        content: Some(generate_json_content(
                            json!({"$ref": "#/components/schemas/ErrorResponse"}),
                        )),
                    },
                );
                responses
            },
            security: Some(vec![{
                let mut auth = HashMap::new();
                auth.insert("bearerAuth".to_string(), vec![]);
                auth
            }]),
        },
    );
    paths.insert("/cassettes/{name}".to_string(), cassette_get_methods);

    // DELETE /cassettes/{name}
    let mut cassette_delete_methods = HashMap::new();
    cassette_delete_methods.insert(
        "delete".to_string(),
        OpenApiOperation {
            summary: "Delete Cassette".to_string(),
            description: "Delete a cassette by name".to_string(),
            tags: Some(vec!["Cassettes".to_string()]),
            request_body: None,
            responses: {
                let mut responses = generate_response_200("Cassette deleted");
                responses.insert(
                    "404".to_string(),
                    OpenApiResponse {
                        description: "Cassette not found".to_string(),
                        content: Some(generate_json_content(
                            json!({"$ref": "#/components/schemas/ErrorResponse"}),
                        )),
                    },
                );
                responses
            },
            security: Some(vec![{
                let mut auth = HashMap::new();
                auth.insert("bearerAuth".to_string(), vec![]);
                auth
            }]),
        },
    );
    paths.insert("/cassettes/{name}".to_string(), cassette_delete_methods);

    paths
}

fn generate_components() -> OpenApiComponents {
    let mut schemas = HashMap::new();

    schemas.insert(
        "ErrorResponse".to_string(),
        json!({
            "type": "object",
            "properties": {
                "@context": {
                    "type": "string",
                    "example": "https://www.w3.org/ns/hydra/core"
                },
                "@type": {
                    "type": "string",
                    "example": "hydra:Error"
                },
                "success": {
                    "type": "boolean",
                    "example": false
                },
                "error": {
                    "type": "string",
                    "example": "Error message"
                },
                "timestamp": {
                    "type": "string",
                    "format": "date-time"
                }
            }
        }),
    );

    let mut security_schemes = HashMap::new();
    security_schemes.insert(
        "bearerAuth".to_string(),
        OpenApiSecurityScheme {
            scheme_type: "http".to_string(),
            scheme: "bearer".to_string(),
            bearer_format: "JWT".to_string(),
            description: "Bearer token authentication".to_string(),
        },
    );

    OpenApiComponents {
        schemas,
        security_schemes,
    }
}

fn generate_response_200(description: &str) -> HashMap<String, OpenApiResponse> {
    let mut responses = HashMap::new();
    responses.insert(
        "200".to_string(),
        OpenApiResponse {
            description: description.to_string(),
            content: Some(generate_json_content(json!({
                "type": "object",
                "properties": {
                    "@context": {
                        "type": "string",
                        "example": "https://www.w3.org/ns/hydra/core"
                    },
                    "@type": {
                        "type": "string",
                        "example": "hydra:Resource"
                    },
                    "success": {
                        "type": "boolean",
                        "example": true
                    },
                    "data": {
                        "type": "object"
                    },
                    "timestamp": {
                        "type": "string",
                        "format": "date-time"
                    },
                    "hydra:link": {
                        "type": "array",
                        "items": {
                            "type": "object"
                        }
                    }
                }
            }))),
        },
    );
    responses
}

fn generate_json_content(schema: serde_json::Value) -> HashMap<String, OpenApiMediaType> {
    let mut content = HashMap::new();
    content.insert(
        "application/ld+json".to_string(),
        OpenApiMediaType { schema },
    );
    content
}
