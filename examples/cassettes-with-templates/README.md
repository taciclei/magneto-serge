# Cassette Examples with Templates

This directory contains example cassettes demonstrating template functionality in Magneto-Serge v0.4.0+.

## Overview

Templates allow dynamic content generation during replay, enabling:
- Environment variable substitution
- Dynamic timestamps
- UUID generation
- Request context access
- Custom helper functions

## Requirements

Compile with `templates` feature:

```bash
cargo build --features templates
```

## Examples

### 1. API Authentication with Environment Variables

**File:** `api-auth-with-env.json`

Demonstrates using environment variables for sensitive data like API tokens:

```json
{
  "body": "{\"access_token\":\"{{ env \"API_ACCESS_TOKEN\" }}\",\"issued_at\":\"{{ now }}\",\"request_id\":\"{{ uuid }}\"}"
}
```

**Usage:**

```rust
use magneto_serge::{Player, ProxyMode};
use std::path::Path;

// Set environment variable
std::env::set_var("API_ACCESS_TOKEN", "sk-test-1234567890");

// Load cassette with templates
let player = Player::load(Path::new("examples/cassettes-with-templates"), "api-auth-with-env")?;

// During replay, templates are automatically rendered
let interaction = player.get_interaction(0)?;
// Response body will contain actual token from env var
```

**Result:**

```json
{
  "access_token": "sk-test-1234567890",
  "issued_at": "2025-10-26T08:30:45Z",
  "request_id": "a1b2c3d4-e5f6-4789-a0b1-c2d3e4f5g6h7"
}
```

### 2. Webhooks with Request Data

**File:** `webhook-with-request-data.json`

Shows accessing request headers, method, and URL in response templates:

```json
{
  "body": "{\"webhook_id\":\"{{ uuid }}\",\"user_id\":\"{{ request.headers.x-user-id }}\",\"method\":\"{{ request.method }}\",\"endpoint\":\"{{ request.url }}\"}"
}
```

**Usage:**

```rust
use magneto_serge::{Player, cassette::HttpRequest};
use std::collections::HashMap;

let player = Player::load(Path::new("examples/cassettes-with-templates"), "webhook-with-request-data")?;

// Request headers are used in template rendering
let mut headers = HashMap::new();
headers.insert("x-user-id".to_string(), "user-12345".to_string());

let request = HttpRequest {
    method: "POST".to_string(),
    url: "https://webhooks.example.com/api/events".to_string(),
    headers,
    body: Some(b"{\"event\":\"user.created\"}".to_vec()),
};

let idx = player.find_interaction_advanced(&request)?;
let interaction = player.get_interaction_with_hooks(idx)?;

// Template values come from the request
```

**Result:**

```json
{
  "webhook_id": "f9e8d7c6-b5a4-4321-9876-543210fedcba",
  "user_id": "user-12345",
  "method": "POST",
  "endpoint": "https://webhooks.example.com/api/events"
}
```

### 3. Dynamic Timestamps

**File:** `dynamic-timestamps.json`

Demonstrates multiple timestamp formats:

```json
{
  "body": "{\"iso8601\":\"{{ now }}\",\"unix_epoch\":{{ now_timestamp }}}"
}
```

**Built-in Helpers:**

| Helper | Output | Example |
|--------|--------|---------|
| `{{ now }}` | ISO 8601 timestamp | `2025-10-26T08:30:45Z` |
| `{{ now_timestamp }}` | Unix epoch (seconds) | `1729930245` |
| `{{ uuid }}` | UUID v4 | `a1b2c3d4-e5f6-4789-a0b1-c2d3e4f5g6h7` |
| `{{ env "VAR" }}` | Environment variable | Value of `$VAR` |

## Template Syntax

### Environment Variables

```handlebars
{{ env "VARIABLE_NAME" }}
```

Returns empty string if variable not set.

### Timestamps

```handlebars
Current time (ISO 8601): {{ now }}
Unix epoch: {{ now_timestamp }}
```

### UUID Generation

```handlebars
Unique ID: {{ uuid }}
```

### Request Context

Access request data during replay:

```handlebars
Method: {{ request.method }}
URL: {{ request.url }}
Header: {{ request.headers.header-name }}
```

## Custom Helpers

You can register custom helpers:

```rust
use magneto_serge::Player;

let mut player = Player::load(path, cassette)?;

// Register custom helper
player.template_engine_mut().register_helper("random_id", || {
    format!("id_{}", rand::random::<u32>())
});

// Use in cassette:
// "body": "{\"id\":\"{{ random_id }}\"}"
```

## Integration with Player

Templates are rendered automatically when using `Player::render_templates_in_response()`:

```rust
use magneto_serge::{Player, cassette::InteractionKind};

let player = Player::load(path, cassette)?;
let interaction = player.get_interaction(0)?;

if let InteractionKind::Http { request, mut response } = interaction.kind.clone() {
    // Render templates in response body
    player.render_templates_in_response(&request, &mut response)?;

    // Response body now contains rendered values
    println!("{}", String::from_utf8(response.body.unwrap())?);
}
```

## Use Cases

### 1. API Testing with Dynamic Tokens

Test APIs that require fresh tokens on each request:

```json
{
  "body": "{\"Authorization\":\"Bearer {{ env \"FRESH_TOKEN\" }}\"}"
}
```

### 2. Time-Sensitive APIs

Test APIs with timestamp validation:

```json
{
  "body": "{\"created_at\":\"{{ now }}\",\"expires_at\":\"{{ now }}\"}"
}
```

### 3. Idempotency Keys

Generate unique request IDs:

```json
{
  "body": "{\"idempotency_key\":\"{{ uuid }}\"}"
}
```

### 4. User Context Injection

Include user-specific data from request headers:

```json
{
  "body": "{\"user\":\"{{ request.headers.x-user-id }}\",\"role\":\"{{ request.headers.x-user-role }}\"}"
}
```

## Best Practices

1. **Security**: Use environment variables for sensitive data, never hardcode secrets
2. **Timestamps**: Use `{{ now }}` for ISO 8601 or `{{ now_timestamp }}` for Unix epoch
3. **UUIDs**: Use `{{ uuid }}` for unique identifiers that must be different each replay
4. **Request Data**: Access headers with `{{ request.headers.header-name }}` (lowercase, hyphens preserved)
5. **Testing**: Templates are only rendered when feature is enabled; tests without `--features templates` will use original cassette data

## Debugging

Enable logging to see template rendering:

```bash
RUST_LOG=magneto_serge=debug cargo test --features templates
```

## Limitations

- Templates only work in HTTP response bodies (not WebSocket messages in v0.4.0)
- Response body must be valid UTF-8
- Template syntax uses Handlebars (double braces `{{ }}`)
- Nested templates are not supported

## Further Reading

- [Handlebars Documentation](https://handlebarsjs.com/)
- [Magneto-Serge Templates Module](../../src/templates.rs)
- [Integration Tests](../../tests/test_templates.rs)

---

**Version:** 0.4.0
**Feature:** `templates` (optional)
