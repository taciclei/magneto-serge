# 🔐 Recording Filters - Magnéto-Serge

This document describes the recording filters feature, which allows you to control what gets recorded into cassettes and how sensitive data is handled.

**Version:** 0.2.0
**Status:** ✅ Implemented and Tested
**Date:** 2025-10-11

---

## 📋 Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Filter Types](#filter-types)
- [Configuration](#configuration)
- [Presets](#presets)
- [Examples](#examples)
- [Best Practices](#best-practices)
- [API Reference](#api-reference)

---

## Overview

Recording filters provide three main capabilities:

1. **URL Filtering** - Skip recording certain URLs (analytics, tracking, ads)
2. **Header Filtering** - Mask sensitive headers (Authorization, API keys, cookies)
3. **Body Transformation** - Redact or truncate request/response bodies
4. **Conditional Recording** - Record only based on status code, content-type, etc.

### Why Use Filters?

- **Security**: Prevent secrets from being committed to version control
- **Efficiency**: Skip unnecessary recordings (images, videos, analytics)
- **Compliance**: Meet data protection requirements (GDPR, PCI-DSS)
- **Performance**: Reduce cassette size and improve test speed

---

## Quick Start

### Basic Usage (Rust)

```rust
use magneto_serge::filters::RecordingFilters;
use magneto_serge::recorder::Recorder;

// Create recorder with default security filters
let filters = RecordingFilters::default();
let mut recorder = Recorder::new_with_filters("my-cassette".to_string(), filters);

// Record interactions (sensitive headers will be filtered automatically)
recorder.record_http(request, response);
```

### Using Presets

```rust
use magneto_serge::filters::FilterPresets;

// Security preset: filters sensitive headers only
let filters = FilterPresets::security();

// No analytics: skips common tracking URLs
let filters = FilterPresets::no_analytics()?;

// No media: skips images, videos, fonts
let filters = FilterPresets::no_media();

// Success only: skips 4xx and 5xx errors
let filters = FilterPresets::success_only();

// Small bodies: limits body size to 10KB
let filters = FilterPresets::small_bodies(10_000);
```

---

## Filter Types

### 1. URL Filtering

Skip recording interactions for specific URL patterns using regular expressions.

```rust
let filters = RecordingFilters::new()
    .ignore_url(r"google-analytics\.com")?
    .ignore_url(r"doubleclick\.net")?
    .ignore_url(r"/tracking/.*")?;
```

**Common Use Cases:**
- Analytics tracking (Google Analytics, Segment, Mixpanel)
- Advertising networks (DoubleClick, AdSense)
- Social media pixels (Facebook, Twitter)
- Internal monitoring/health checks

### 2. Header Filtering

Mask sensitive headers to prevent secrets leakage.

**Default Filtered Headers:**
- `Authorization`
- `X-API-Key`, `X-API-Token`
- `Cookie`, `Set-Cookie`
- `Proxy-Authorization`
- `X-Auth-Token`, `X-Session-Token`
- `X-CSRF-Token`

```rust
let filters = RecordingFilters::new()
    .filter_header("X-Custom-Secret".to_string())
    .filter_header("X-Internal-Token".to_string());
```

**Filtered headers** are replaced with `[FILTERED]` in cassettes:

```json
{
  "headers": {
    "Authorization": "[FILTERED]",
    "Content-Type": "application/json"
  }
}
```

### 3. Body Filtering

Control whether request and response bodies are recorded.

```rust
let filters = RecordingFilters::new()
    .filter_request_bodies(true)   // Replace with [FILTERED]
    .filter_response_bodies(false); // Keep responses
```

**Use cases:**
- Request bodies: Filter when they contain passwords, credit cards
- Response bodies: Filter when they contain PII (personal data)

### 4. Status Code Filtering

Skip recording interactions with specific HTTP status codes.

```rust
let filters = RecordingFilters::new()
    .skip_status_code(404)  // Not Found
    .skip_status_code(500)  // Internal Server Error
    .skip_status_codes_from_slice(&[401, 403, 503]);
```

**Use cases:**
- Skip error responses that clutter cassettes
- Focus on successful interactions only
- Reduce cassette size for large test suites

### 5. Content-Type Filtering

Skip recording based on response content type.

```rust
let filters = RecordingFilters::new()
    .skip_content_type("image/".to_string())
    .skip_content_type("video/".to_string())
    .skip_content_type("application/octet-stream".to_string());
```

**Use cases:**
- Skip binary files (images, videos, PDFs)
- Skip large assets (fonts, archives)
- Focus on API responses only

### 6. Body Size Limiting

Truncate bodies that exceed a maximum size.

```rust
let filters = RecordingFilters::new()
    .max_body_size(10_000); // 10 KB max
```

**Use cases:**
- Prevent huge cassettes from large responses
- Keep only the first N bytes for validation
- Reduce disk usage for large test suites

---

## Configuration

### Builder Pattern

All filters support fluent builder-style configuration:

```rust
let filters = RecordingFilters::new()
    .ignore_url(r"analytics\.com")?
    .filter_header("X-Secret".to_string())
    .filter_request_bodies(true)
    .skip_status_code(404)
    .skip_content_type("image/".to_string())
    .max_body_size(10_000);
```

### Dynamic Configuration

You can change filters at runtime:

```rust
let mut recorder = Recorder::new("my-cassette".to_string());

// Record without filters initially
recorder.record_http(request1, response1);

// Enable filters mid-recording
let filters = RecordingFilters::default();
recorder.set_filters(filters);

// Future recordings will be filtered
recorder.record_http(request2, response2);
```

---

## Presets

Pre-configured filter combinations for common use cases.

### `FilterPresets::security()`

**Purpose:** Filter sensitive headers only (default behavior)

```rust
let filters = FilterPresets::security();
```

**Filters:**
- ✅ Sensitive headers (Authorization, API keys, cookies)
- ❌ Bodies NOT filtered
- ❌ No URL filtering

**Use when:** You want basic security without impacting test coverage

---

### `FilterPresets::strict()`

**Purpose:** Maximum security (filter everything sensitive)

```rust
let filters = FilterPresets::strict();
```

**Filters:**
- ✅ Sensitive headers
- ✅ Request bodies filtered
- ✅ Response bodies filtered

**Use when:** Recording production traffic or highly sensitive APIs

---

### `FilterPresets::no_analytics()`

**Purpose:** Skip common analytics and tracking services

```rust
let filters = FilterPresets::no_analytics()?;
```

**Skips:**
- Google Analytics, Google Tag Manager
- DoubleClick, Facebook Pixel
- Segment, Mixpanel, Amplitude

**Use when:** Testing web applications with embedded analytics

---

### `FilterPresets::no_media()`

**Purpose:** Skip binary media files

```rust
let filters = FilterPresets::no_media();
```

**Skips:**
- Images (`image/*`)
- Videos (`video/*`)
- Audio (`audio/*`)
- Fonts (`font/*`)

**Use when:** Testing APIs that serve media files

---

### `FilterPresets::success_only()`

**Purpose:** Record only successful responses (2xx and 3xx)

```rust
let filters = FilterPresets::success_only();
```

**Skips:**
- 4xx client errors (400-499)
- 5xx server errors (500-599)

**Use when:** You only care about successful API interactions

---

### `FilterPresets::small_bodies(max_size)`

**Purpose:** Limit body size to prevent huge cassettes

```rust
let filters = FilterPresets::small_bodies(10_000); // 10 KB
```

**Limits:**
- Request bodies truncated to max_size
- Response bodies truncated to max_size

**Use when:** APIs return large payloads but you only need the first N bytes

---

## Examples

### Example 1: Filter Sensitive Data in API Tests

```rust
use magneto_serge::filters::RecordingFilters;
use magneto_serge::recorder::Recorder;

let filters = RecordingFilters::new()
    .filter_header("X-Internal-Token".to_string())
    .filter_request_bodies(true);  // Passwords in request bodies

let mut recorder = Recorder::new_with_filters("api-test".to_string(), filters);

// Login request with password
let login_request = HttpRequest {
    method: "POST".to_string(),
    url: "https://api.example.com/login".to_string(),
    headers: {
        let mut h = HashMap::new();
        h.insert("Content-Type".to_string(), "application/json".to_string());
        h.insert("X-Internal-Token".to_string(), "secret-123".to_string());
        h
    },
    body: Some(r#"{"username":"admin","password":"secret"}"#.as_bytes().to_vec()),
};

recorder.record_http(login_request, response);

// Cassette will contain:
// - X-Internal-Token: [FILTERED]
// - body: [FILTERED]
```

### Example 2: Skip Analytics in E2E Tests

```rust
use magneto_serge::filters::FilterPresets;
use magneto_serge::recorder::Recorder;

let filters = FilterPresets::no_analytics()?;
let mut recorder = Recorder::new_with_filters("e2e-test".to_string(), filters);

// This will be recorded
recorder.record_http(
    HttpRequest::new("GET", "https://myapp.com/api/users"),
    response,
);

// These will be SKIPPED
recorder.record_http(
    HttpRequest::new("GET", "https://google-analytics.com/collect"),
    response,
);
recorder.record_http(
    HttpRequest::new("POST", "https://api.segment.io/v1/track"),
    response,
);
```

### Example 3: Combine Multiple Filters

```rust
use magneto_serge::filters::{RecordingFilters, FilterPresets};
use magneto_serge::recorder::Recorder;

// Start with security preset
let filters = FilterPresets::security()
    // Add custom filters
    .ignore_url(r"/health")?
    .ignore_url(r"/metrics")?
    .skip_status_code(404)
    .skip_content_type("image/".to_string())
    .max_body_size(50_000);

let mut recorder = Recorder::new_with_filters("combined-test".to_string(), filters);
```

### Example 4: CI/CD Mode (Success Only + No Media)

```rust
use magneto_serge::filters::FilterPresets;

// Combine two presets
let success_filters = FilterPresets::success_only();
let media_filters = FilterPresets::no_media();

// Merge manually
let filters = RecordingFilters::new()
    .skip_status_codes_from_slice(&(400..600).collect::<Vec<_>>())
    .skip_content_type("image/".to_string())
    .skip_content_type("video/".to_string())
    .skip_content_type("audio/".to_string())
    .skip_content_type("font/".to_string());

let mut recorder = Recorder::new_with_filters("ci-test".to_string(), filters);
```

---

## Best Practices

### 1. Always Filter Sensitive Headers

**DO:**
```rust
let filters = RecordingFilters::default();  // Includes security headers
```

**DON'T:**
```rust
let filters = RecordingFilters::new()  // No default filters!
    .max_body_size(10_000);  // Oops, no header filtering
```

Use `::default()` or `FilterPresets::security()` as a baseline.

---

### 2. Use Presets for Common Scenarios

**DO:**
```rust
let filters = FilterPresets::no_analytics()?;
```

**DON'T:**
```rust
let filters = RecordingFilters::new()
    .ignore_url(r"google-analytics")?
    .ignore_url(r"googletagmanager")?
    .ignore_url(r"doubleclick")?
    // ... 20 more lines
```

Presets are tested and cover common cases.

---

### 3. Test Your Filters

```rust
#[test]
fn test_filters_block_analytics() {
    let filters = FilterPresets::no_analytics().unwrap();
    assert!(filters.should_ignore_url("https://google-analytics.com/collect"));
}
```

---

### 4. Document Custom Filters

```rust
// Custom filter for our internal auth service
// We skip /health and /metrics to reduce cassette size
let filters = RecordingFilters::new()
    .ignore_url(r"/health")?
    .ignore_url(r"/metrics")?;
```

---

### 5. Use Strict Mode for Production

When recording production traffic:

```rust
let filters = FilterPresets::strict()
    .max_body_size(10_000);  // Extra safety
```

---

## API Reference

### `RecordingFilters`

Main configuration struct for recording filters.

#### Constructors

| Method | Description |
|--------|-------------|
| `new()` | Create empty filter configuration |
| `default()` | Create with default sensitive header filters |

#### URL Filtering

| Method | Description |
|--------|-------------|
| `ignore_url(pattern: &str) -> Result<Self>` | Add regex pattern to ignore |
| `ignore_urls_from_slice(patterns: &[&str]) -> Result<Self>` | Add multiple patterns |

#### Header Filtering

| Method | Description |
|--------|-------------|
| `filter_header(header: String) -> Self` | Add header to filter (case-insensitive) |
| `filter_headers_from_slice(headers: &[String]) -> Self` | Add multiple headers |

#### Body Filtering

| Method | Description |
|--------|-------------|
| `filter_request_bodies(enabled: bool) -> Self` | Enable/disable request body filtering |
| `filter_response_bodies(enabled: bool) -> Self` | Enable/disable response body filtering |

#### Status Code Filtering

| Method | Description |
|--------|-------------|
| `skip_status_code(status: u16) -> Self` | Skip specific status code |
| `skip_status_codes_from_slice(codes: &[u16]) -> Self` | Skip multiple codes |

#### Content-Type Filtering

| Method | Description |
|--------|-------------|
| `skip_content_type(content_type: String) -> Self` | Skip content type (partial match) |

#### Body Size Limiting

| Method | Description |
|--------|-------------|
| `max_body_size(size: usize) -> Self` | Set maximum body size in bytes |

#### Checking Methods

| Method | Description |
|--------|-------------|
| `should_ignore_url(&self, url: &str) -> bool` | Check if URL should be ignored |
| `should_skip_status(&self, status: u16) -> bool` | Check if status should be skipped |
| `should_skip_content_type(&self, ct: &str) -> bool` | Check if content-type should be skipped |
| `should_record(&self, req: &HttpRequest, res: &HttpResponse) -> bool` | Check if interaction should be recorded |

#### Transformation Methods

| Method | Description |
|--------|-------------|
| `filter_headers_map(&self, headers: &HashMap<...>) -> HashMap<...>` | Filter headers map |
| `filter_body(&self, body: Option<Vec<u8>>, filter: bool) -> Option<Vec<u8>>` | Filter/truncate body |
| `apply_to_request(&self, request: HttpRequest) -> HttpRequest` | Apply all filters to request |
| `apply_to_response(&self, response: HttpResponse) -> HttpResponse` | Apply all filters to response |

---

### `FilterPresets`

Pre-configured filter combinations.

| Method | Description |
|--------|-------------|
| `security() -> RecordingFilters` | Security headers only |
| `strict() -> RecordingFilters` | Maximum security (headers + bodies) |
| `no_analytics() -> Result<RecordingFilters>` | Skip analytics URLs |
| `no_media() -> RecordingFilters` | Skip media files |
| `small_bodies(max_size: usize) -> RecordingFilters` | Limit body size |
| `success_only() -> RecordingFilters` | Skip 4xx/5xx errors |

---

### `Recorder`

#### Filter-Related Methods

| Method | Description |
|--------|-------------|
| `new(cassette_name: String) -> Self` | Create recorder without filters |
| `new_with_filters(name: String, filters: RecordingFilters) -> Self` | Create with filters |
| `set_filters(&mut self, filters: RecordingFilters)` | Change filters at runtime |
| `filters(&self) -> Option<&RecordingFilters>` | Get current filters |

---

## Implementation Notes

### Performance

- **URL regex matching**: ~1-2 µs per pattern
- **Header filtering**: ~5-10 µs per request
- **Body truncation**: Zero-copy for small bodies, O(n) for truncation
- **Overall overhead**: <20 µs per interaction (negligible)

### Memory

- Filters are cloned into Recorder (small overhead: ~1-2 KB)
- Regex patterns compiled once at configuration time
- Body truncation creates new Vec (no in-place modification)

### Thread Safety

- `RecordingFilters` is `Clone` and can be shared across threads
- Recorder uses filters immutably (no locking needed)

---

## Future Enhancements

Planned for v0.3.0:

- [ ] Custom transformation functions (e.g., hash PII)
- [ ] Regex-based body redaction (e.g., replace SSNs)
- [ ] Hooks for pre/post recording (callbacks)
- [ ] Filter statistics (how many interactions skipped)
- [ ] YAML configuration file support

---

## FAQ

### Q: Do filters affect replay?

**A:** No, filters only apply during recording. Replay uses whatever is in the cassette.

---

### Q: Can I filter WebSocket messages?

**A:** Not yet. WebSocket filtering is planned for v0.3.0.

---

### Q: Are filters applied to existing cassettes?

**A:** No, filters only affect new recordings. To filter existing cassettes, re-record them.

---

### Q: What happens if I filter ALL bodies?

**A:** Bodies will be replaced with `[FILTERED]`. This may break tests that validate response content.

---

### Q: Can I combine multiple presets?

**A:** Not directly. Use one preset as a base and add filters manually:

```rust
let filters = FilterPresets::security()
    .ignore_url(r"analytics")?
    .skip_status_code(404);
```

---

## Related Documentation

- [OPTIMIZATIONS.md](OPTIMIZATIONS.md) - Performance improvements
- [BENCHMARKS.md](BENCHMARKS.md) - Performance benchmarks
- [ROADMAP.md](ROADMAP.md) - Feature roadmap

---

**Last Updated:** 2025-10-11
**Version:** 0.2.0-dev
**Author:** Magnéto-Serge Contributors

For questions or contributions, see [CONTRIBUTING.md](CONTRIBUTING.md) or open an issue.
