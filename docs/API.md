# Magneto-Serge REST API Documentation

The Magneto-Serge REST API provides remote control over the HTTP/WebSocket proxy. The API follows [Hydra](https://www.hydra-cg.com/) hypermedia conventions with full [JSON-LD](https://json-ld.org/) support and includes an [OpenAPI 3.0](https://swagger.io/specification/) specification.

## Base URL

By default, the API runs on:
```
http://127.0.0.1:8889
```

## Authentication

The API supports Bearer token authentication via the `Authorization` header:

```
Authorization: Bearer YOUR_API_KEY
```

Authentication can be configured in the `ApiConfig`:

```rust
let config = ApiConfig {
    auth_enabled: true,
    api_key: Some("your-secret-key".to_string()),
    ..Default::default()
};
```

## Content Type

All responses use the JSON-LD content type:
```
Content-Type: application/ld+json
```

## Response Format

All responses follow this Hydra-compliant structure:

```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": { ... },
  "timestamp": "2025-10-12T14:30:00Z",
  "hydra:link": [
    {
      "@type": "hydra:Link",
      "hydra:target": "http://127.0.0.1:8889/proxy/status",
      "title": "Check Proxy Status",
      "hydra:operation": [
        {
          "@type": "http://schema.org/ViewAction",
          "method": "GET",
          "returns": "application/ld+json"
        }
      ]
    }
  ]
}
```

### Error Responses

```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Error",
  "success": false,
  "error": "Error message description",
  "timestamp": "2025-10-12T14:30:00Z"
}
```

## Endpoints

### GET `/` - API Root

Get API information and discover available endpoints via Hydra links.

**Response:**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": {
    "@id": "http://127.0.0.1:8889/",
    "title": "Magneto-Serge API",
    "description": "REST API for controlling Magneto-Serge HTTP/WebSocket proxy",
    "version": "0.1.0",
    "documentation": "https://github.com/taciclei/magneto-serge",
    "openapi": "http://127.0.0.1:8889/openapi.json"
  },
  "timestamp": "2025-10-12T14:30:00Z",
  "hydra:link": [ ... ]
}
```

### GET `/openapi.json` - OpenAPI Specification

Get the complete OpenAPI 3.0 specification for the API.

**Response:** OpenAPI 3.0 JSON document

### GET `/health` - Health Check

Check API server health and uptime.

**Response:**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": {
    "status": "healthy",
    "uptime_seconds": 3600
  },
  "timestamp": "2025-10-12T14:30:00Z",
  "hydra:link": [ ... ]
}
```

### POST `/proxy/start` - Start Proxy

Start the proxy in specified mode.

**Request Body:**
```json
{
  "mode": "auto",
  "cassette_name": "my-test",
  "port": 8888,
  "strict": false
}
```

**Fields:**
- `mode` (required): One of `"auto"`, `"record"`, `"replay"`, or `"passthrough"`
  - `auto`: Replay if cassette exists, record if not
  - `record`: Record all interactions to cassette
  - `replay`: Replay from cassette
  - `passthrough`: Proxy without recording/replaying
- `cassette_name` (required): Name of the cassette to use
- `port` (optional): Proxy port (defaults to configured port)
- `strict` (optional): Enable strict replay mode (default: false)

**Response (200 OK):**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": {
    "message": "Proxy started successfully",
    "mode": "auto",
    "cassette": "my-test",
    "port": 8888
  },
  "timestamp": "2025-10-12T14:30:00Z",
  "hydra:link": [
    {
      "@type": "hydra:Link",
      "hydra:target": "http://127.0.0.1:8889/proxy/status",
      "title": "Check Proxy Status"
    },
    {
      "@type": "hydra:Link",
      "hydra:target": "http://127.0.0.1:8889/proxy/stop",
      "title": "Stop Proxy"
    }
  ]
}
```

**Response (409 Conflict):**
Proxy is already running.

### POST `/proxy/stop` - Stop Proxy

Stop the running proxy.

**Request Body:**
```json
{
  "force": false
}
```

**Fields:**
- `force` (optional): Force stop (default: false)

**Response (200 OK):**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": {
    "message": "Proxy stopped successfully"
  },
  "timestamp": "2025-10-12T14:30:00Z"
}
```

**Response (404 Not Found):**
Proxy is not running.

### GET `/proxy/status` - Get Proxy Status

Get current proxy status and configuration.

**Response (200 OK):**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": {
    "running": true,
    "mode": "Replay",
    "port": 8888,
    "cassette": "my-test",
    "interactions_count": 42,
    "uptime_seconds": 3600
  },
  "timestamp": "2025-10-12T14:30:00Z",
  "hydra:link": [ ... ]
}
```

**Fields:**
- `running`: Whether proxy is currently running
- `mode`: Current proxy mode
- `port`: Proxy port
- `cassette`: Current cassette name (if any)
- `interactions_count`: Number of interactions recorded/replayed
- `uptime_seconds`: Proxy uptime in seconds

### GET `/proxy/stats` - Get Proxy Statistics

Get proxy performance statistics.

**Response (200 OK):**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": {
    "total_requests": 1000,
    "total_responses": 1000,
    "requests_per_second": 10.5,
    "avg_response_time_ms": 25.3,
    "cache_hit_rate": 0.95,
    "memory_mb": 12.5,
    "metrics": {
      "http_requests": 800,
      "websocket_messages": 200
    }
  },
  "timestamp": "2025-10-12T14:30:00Z"
}
```

### GET `/cassettes` - List Cassettes

List all available cassettes in the cassette directory.

**Response (200 OK):**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": [
    {
      "name": "my-test.json",
      "size_bytes": 102400,
      "interactions": 42,
      "created_at": "2025-10-12T14:00:00Z",
      "format": "json"
    }
  ],
  "timestamp": "2025-10-12T14:30:00Z"
}
```

### GET `/cassettes/{name}` - Get Cassette

Get cassette content by name.

**Response (200 OK):**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": {
    "name": "my-test.json",
    "content": "{ ... cassette content ... }"
  },
  "timestamp": "2025-10-12T14:30:00Z"
}
```

**Response (404 Not Found):**
Cassette not found.

### DELETE `/cassettes/{name}` - Delete Cassette

Delete a cassette by name.

**Response (200 OK):**
```json
{
  "@context": "https://www.w3.org/ns/hydra/core",
  "@type": "hydra:Resource",
  "success": true,
  "data": {
    "message": "Cassette deleted: my-test.json"
  },
  "timestamp": "2025-10-12T14:30:00Z"
}
```

**Response (404 Not Found):**
Cassette not found.

## Example Usage

### Using curl

```bash
# Get API root
curl http://localhost:8889/

# Start proxy in auto mode
curl -X POST http://localhost:8889/proxy/start \
  -H "Content-Type: application/json" \
  -d '{
    "mode": "auto",
    "cassette_name": "my-test",
    "port": 8888
  }'

# Check proxy status
curl http://localhost:8889/proxy/status

# Stop proxy
curl -X POST http://localhost:8889/proxy/stop \
  -H "Content-Type: application/json" \
  -d '{"force": false}'

# List cassettes
curl http://localhost:8889/cassettes
```

### Using curl with Authentication

```bash
curl http://localhost:8889/proxy/status \
  -H "Authorization: Bearer your-api-key"
```

### Using Python (requests)

```python
import requests

# Start proxy
response = requests.post(
    "http://localhost:8889/proxy/start",
    json={
        "mode": "auto",
        "cassette_name": "my-test",
        "port": 8888
    }
)
print(response.json())

# Get status with authentication
response = requests.get(
    "http://localhost:8889/proxy/status",
    headers={"Authorization": "Bearer your-api-key"}
)
print(response.json())
```

### Using JavaScript (fetch)

```javascript
// Start proxy
const response = await fetch("http://localhost:8889/proxy/start", {
  method: "POST",
  headers: {
    "Content-Type": "application/json"
  },
  body: JSON.stringify({
    mode: "auto",
    cassette_name: "my-test",
    port: 8888
  })
});
const data = await response.json();
console.log(data);

// Navigate using Hydra links
const statusLink = data["hydra:link"].find(
  link => link.title === "Check Proxy Status"
);
const statusResponse = await fetch(statusLink["hydra:target"]);
const statusData = await statusResponse.json();
console.log(statusData);
```

## Hypermedia Navigation

The API follows HATEOAS principles - every response includes `hydra:link` entries that describe available actions:

```json
{
  "hydra:link": [
    {
      "@type": "hydra:Link",
      "hydra:target": "http://127.0.0.1:8889/proxy/start",
      "title": "Start Proxy",
      "hydra:operation": [
        {
          "@type": "http://schema.org/ActivateAction",
          "method": "POST",
          "expects": "StartProxyRequest",
          "returns": "application/ld+json"
        }
      ]
    }
  ]
}
```

Clients can discover and navigate the API dynamically using these links without hardcoding URLs.

## OpenAPI Integration

The API provides a complete OpenAPI 3.0 specification at `/openapi.json`. You can:

1. Import it into tools like [Swagger UI](https://swagger.io/tools/swagger-ui/)
2. Generate client SDKs using [OpenAPI Generator](https://openapi-generator.tech/)
3. Use it for API testing and validation

## Configuration

Configure the API server using `ApiConfig`:

```rust
use magneto_serge::{ApiConfig, ApiServer};

let config = ApiConfig {
    host: "0.0.0.0".to_string(),        // Listen on all interfaces
    port: 8889,                          // API server port
    proxy_port: 8888,                    // Default proxy port
    cassette_dir: "./cassettes".to_string(),
    auth_enabled: true,                  // Enable authentication
    api_key: Some("secret".to_string()), // API key
};

let server = ApiServer::new(config);
server.start().await?;
```

## Status Codes

- `200 OK`: Request successful
- `401 Unauthorized`: Missing or invalid API key
- `404 Not Found`: Resource not found (cassette, endpoint, etc.)
- `409 Conflict`: Operation conflict (e.g., proxy already running)
- `500 Internal Server Error`: Server error

## Rate Limiting

Currently, no rate limiting is implemented. This may be added in future versions.

## CORS

The API includes CORS headers allowing cross-origin requests:
```
Access-Control-Allow-Origin: *
```

## WebSocket Events (Future)

WebSocket support for real-time proxy events is planned for a future release. This will allow clients to subscribe to:

- Request/response events
- Proxy state changes
- Error notifications
- Performance metrics

## Further Reading

- [Hydra Specification](https://www.hydra-cg.com/spec/latest/core/)
- [JSON-LD](https://json-ld.org/)
- [OpenAPI 3.0](https://swagger.io/specification/)
- [Magneto-Serge Documentation](https://github.com/taciclei/magneto-serge)
