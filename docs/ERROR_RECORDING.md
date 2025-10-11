# Error Recording and Replay

This document explains how magneto-serge records and replays network errors (timeouts, DNS failures, connection refused, etc.).

## Overview

When testing applications, it's crucial to verify how they handle network failures. Magneto-serge allows you to record and replay various types of network errors, including:

- DNS resolution failures
- Connection refused errors
- Connection timeouts
- TLS/SSL errors
- Connection reset by peer
- Too many redirects
- Other generic network errors

## Error Types

### NetworkError Enum

The `NetworkError` enum represents different types of network failures:

```rust
pub enum NetworkError {
    /// DNS resolution failed
    DnsResolutionFailed { message: String },

    /// Connection refused by server
    ConnectionRefused { message: String },

    /// Connection timed out
    Timeout { message: String, timeout_ms: u64 },

    /// TLS/SSL error
    TlsError { message: String },

    /// Connection reset by peer
    ConnectionReset { message: String },

    /// Too many redirects
    TooManyRedirects { message: String, redirect_count: usize },

    /// Other network error
    Other { message: String },
}
```

## Recording Errors

### Using the Recorder API

```rust
use magneto_serge::recorder::Recorder;
use magneto_serge::cassette::{HttpRequest, NetworkError};
use std::collections::HashMap;

// Create a recorder
let mut recorder = Recorder::new("test-errors".to_string());

// Record a timeout error
let request = HttpRequest {
    method: "GET".to_string(),
    url: "https://slow-api.example.com/endpoint".to_string(),
    headers: HashMap::new(),
    body: None,
};

let error = NetworkError::timeout("Connection timed out after 5000ms", 5000);
recorder.record_http_error(request, error);

// Save the cassette
recorder.save(Path::new("./cassettes"))?;
```

### Helper Methods

The `NetworkError` enum provides convenient builder methods:

```rust
// DNS failure
let error = NetworkError::dns_failed("Failed to resolve domain");

// Connection refused
let error = NetworkError::connection_refused("Connection refused on port 8080");

// Timeout with duration
let error = NetworkError::timeout("Request timed out", 5000);

// TLS error
let error = NetworkError::tls_error("Certificate validation failed");

// Connection reset
let error = NetworkError::connection_reset("Connection reset by peer");

// Too many redirects
let error = NetworkError::too_many_redirects("Redirect limit exceeded", 10);

// Generic error
let error = NetworkError::other("Unknown network error");
```

### Using Cassette API

You can also record errors directly on a cassette:

```rust
use magneto_serge::cassette::Cassette;

let mut cassette = Cassette::new("test-errors".to_string());

let request = HttpRequest {
    method: "POST".to_string(),
    url: "https://api.example.com/data".to_string(),
    headers: HashMap::new(),
    body: Some(b"{\"data\":1}".to_vec()),
};

let error = NetworkError::dns_failed("DNS lookup failed");
cassette.add_error(request, error);
```

## Cassette Format

Errors are stored as `HttpError` interactions in the cassette:

```json
{
  "version": "1.0",
  "name": "test-errors",
  "recorded_at": "2025-10-11T14:30:00Z",
  "interactions": [
    {
      "type": "HttpError",
      "request": {
        "method": "GET",
        "url": "https://slow-api.example.com/endpoint",
        "headers": {},
        "body": null
      },
      "error": {
        "error_type": "Timeout",
        "message": "Connection timed out after 5000ms",
        "timeout_ms": 5000
      },
      "recorded_at": "2025-10-11T14:30:05Z"
    },
    {
      "type": "HttpError",
      "request": {
        "method": "POST",
        "url": "https://nonexistent.invalid/api",
        "headers": {
          "Content-Type": "application/json"
        },
        "body": [123, 34, 100, 97, 116, 97, 34, 58, 49, 125]
      },
      "error": {
        "error_type": "DnsResolutionFailed",
        "message": "Failed to resolve domain: nonexistent.invalid"
      },
      "recorded_at": "2025-10-11T14:30:10Z"
    }
  ]
}
```

## Replaying Errors

When replaying a cassette containing errors, the proxy will:

1. Match the incoming request to the recorded error interaction
2. Return the appropriate error to the client
3. Preserve error details (timeout duration, error message, etc.)

```rust
use magneto_serge::player::Player;
use std::path::Path;

// Load a cassette with recorded errors
let player = Player::load(
    Path::new("./cassettes"),
    "test-errors"
)?;

// During replay, requests that match error interactions will:
// - Return the appropriate error to the client
// - Maintain consistent behavior across test runs
```

## Use Cases

### 1. Testing Timeout Handling

Record how your application behaves when requests timeout:

```rust
let error = NetworkError::timeout("Request timed out after 5000ms", 5000);
recorder.record_http_error(request, error);
```

Your tests can verify:
- Proper timeout handling
- Retry logic
- User-facing error messages
- Graceful degradation

### 2. Testing DNS Failures

Simulate DNS resolution failures:

```rust
let error = NetworkError::dns_failed("Failed to resolve domain: api.example.com");
recorder.record_http_error(request, error);
```

Verify that your application:
- Handles DNS errors gracefully
- Shows appropriate error messages
- Falls back to cached data (if applicable)

### 3. Testing Connection Failures

Record connection refused errors:

```rust
let error = NetworkError::connection_refused("Connection refused on port 8080");
recorder.record_http_error(request, error);
```

Test scenarios:
- Service unavailable
- Port closed
- Firewall blocking connections

### 4. Testing TLS/SSL Errors

Capture certificate validation failures:

```rust
let error = NetworkError::tls_error("Certificate validation failed: expired certificate");
recorder.record_http_error(request, error);
```

Verify handling of:
- Expired certificates
- Self-signed certificates
- Certificate mismatch
- Invalid certificate chains

### 5. Testing Mixed Success and Failure

Record both successful responses and errors:

```rust
let mut recorder = Recorder::new("mixed-scenarios".to_string());

// Successful request
recorder.record_http(request1, response1);

// Timeout error
recorder.record_http_error(request2, NetworkError::timeout("Timeout", 5000));

// Another success
recorder.record_http(request3, response3);

// DNS error
recorder.record_http_error(request4, NetworkError::dns_failed("DNS failed"));
```

## CLI Support

The magneto CLI displays error interactions with red highlighting:

```bash
magneto inspect test-errors
```

Output:
```
  Name: test-errors
  Version: 1.0
  Recorded: 2025-10-11 14:30:00 UTC
  Interactions: 4

  Interactions:
    1. GET https://api.example.com/success
    2. GET https://slow-api.example.com/timeout (Error: Timeout)
    3. POST https://api.example.com/data
    4. GET https://nonexistent.invalid/api (Error: DnsResolutionFailed)
```

## Best Practices

### 1. Record Real Errors

Record actual network errors that occur during development or testing:

```rust
// In your HTTP handler
match client.get(url).send().await {
    Ok(response) => {
        recorder.record_http(request, response);
    }
    Err(e) => {
        let error = if e.is_timeout() {
            NetworkError::timeout(e.to_string(), 5000)
        } else if e.is_connect() {
            NetworkError::connection_refused(e.to_string())
        } else {
            NetworkError::other(e.to_string())
        };
        recorder.record_http_error(request, error);
    }
}
```

### 2. Use Descriptive Error Messages

Include context in error messages:

```rust
// Good
NetworkError::timeout("GET /api/users timed out after 5000ms", 5000)

// Bad
NetworkError::timeout("timeout", 5000)
```

### 3. Test Error Recovery

Use error cassettes to verify recovery logic:

```rust
#[test]
fn test_retry_on_timeout() {
    let proxy = MagnetoProxy::new()
        .with_mode(ProxyMode::Replay);

    proxy.replay("timeout-then-success");

    // First request times out (from cassette)
    // Second request succeeds (from cassette)
    // Verify retry logic works correctly
}
```

### 4. Combine with Filters

Use recording filters to avoid capturing sensitive data in error scenarios:

```rust
use magneto_serge::filters::{RecordingFilter, FilterPreset};

let filter = RecordingFilter::from_preset(FilterPreset::Security);
let recorder = Recorder::new_with_filters("errors".to_string(), filter);

// Authorization headers will be filtered even in error recordings
recorder.record_http_error(request_with_auth, error);
```

## Advanced Usage

### Error Equality

The `NetworkError` enum implements `PartialEq` and `Eq` for testing:

```rust
let error1 = NetworkError::timeout("timeout", 5000);
let error2 = NetworkError::timeout("timeout", 5000);
assert_eq!(error1, error2);

let error3 = NetworkError::timeout("timeout", 3000);
assert_ne!(error1, error3); // Different timeout duration
```

### Serialization

Errors are serialized with tagged enum format:

```json
{
  "error_type": "Timeout",
  "message": "Connection timed out",
  "timeout_ms": 5000
}
```

This format is:
- Human-readable
- Language-agnostic
- Easy to parse
- Extensible

## Limitations

1. **Error Details**: Some low-level network error details may be lost during recording
2. **Platform Differences**: Error messages may vary across platforms (Windows/Linux/macOS)
3. **Timing**: Error timing is not preserved (errors are returned immediately during replay)
4. **State**: Connection state before error is not captured

## Future Enhancements

Planned improvements:

- [ ] Error timing simulation (delay before returning error)
- [ ] Partial response errors (connection dropped mid-response)
- [ ] Error rate configuration (randomly inject errors)
- [ ] Error transformation (convert specific errors during replay)

## Related Documentation

- [LATENCY_SIMULATION.md](LATENCY_SIMULATION.md) - Simulating response delays
- [STRICT_MODE.md](STRICT_MODE.md) - Strict matching for CI/CD
- [FILTERS.md](FILTERS.md) - Recording filters for sensitive data
- [ARCHITECTURE.md](ARCHITECTURE.md) - Overall system architecture

## Issues

This feature was implemented to address:
- Issue #4: Record and replay timeout/errors

## Examples

See the following test files for complete examples:
- `tests/test_error_recording.rs` - Integration tests for error recording
- `src/recorder.rs` - Unit tests for the recorder API

---

**Last updated**: 2025-10-11
