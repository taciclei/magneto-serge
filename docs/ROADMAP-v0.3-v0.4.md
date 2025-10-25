# 🗺️ Roadmap: Magneto-Serge v0.3.x - v0.4.x

**Reaching Feature Parity with VCR and Beyond**

This roadmap outlines the path from v0.2.0 to v0.4.x, focusing on closing the feature gap identified in our [VCR comparison analysis](COMPARISON-VCR.md) while maintaining Magneto-Serge's unique advantages (WebSocket support, multi-language bindings, Rust performance).

---

## 🎯 Strategic Goals

1. **Feature Parity**: Match or exceed VCR's core capabilities (hook system, test integration)
2. **Developer Experience**: Make Magneto-Serge the easiest HTTP recording library to use
3. **Multi-Language First**: Leverage UniFFI for Python, Kotlin, Swift, Java bindings
4. **Performance Leader**: Maintain 10-100x performance advantage over VCR

---

## ✅ v0.2.0 (Current Release)

**Released**: 2025-10-25

### Features
- ✅ HTTP/HTTPS proxy with MITM
- ✅ WebSocket record/replay with timing
- ✅ Advanced matching strategies (regex, JSON path, size-only)
- ✅ Recording filters with presets
- ✅ Cookie preservation (Phase 1.1)
- ✅ CLI tool (8 commands)
- ✅ JavaScript/TypeScript bindings (NAPI-RS)
- ✅ Latency simulation modes
- ✅ Strict mode for CI/CD
- ✅ Docker support (Alpine images)
- ✅ Homebrew distribution
- ✅ GitHub Actions release automation

### Known Gaps (vs VCR)
- ❌ Hook system (CRITICAL)
- ❌ Test framework integration
- ❌ Template/ERB support
- ❌ Cassette re-recording
- ❌ Better error messages

---

## 🚀 v0.3.0 - Hook System (CRITICAL PRIORITY)

**Target**: 2025-11-05 (2 weeks)
**Status**: ✅ **COMPLETED** (2025-10-25)

### Phase 1: Core Hook Traits ✅

**Effort**: 1 day
**Completed**: 2025-10-25

- [x] Define `RecordHook` trait with `before_record` and `after_record`
- [x] Define `ReplayHook` trait with `before_replay` and `after_replay`
- [x] Create `RecordHooks` and `ReplayHooks` collection types
- [x] Thread-safe Arc-based storage for hooks
- [x] Default implementations for optional hook methods

**API**:
```rust
pub trait RecordHook: Send + Sync + Debug {
    fn before_record(&self, interaction: &mut Interaction) -> Result<()>;
    fn after_record(&self, interaction: &Interaction) -> Result<()>;
    fn name(&self) -> &str;
}

pub trait ReplayHook: Send + Sync + Debug {
    fn before_replay(&self, interaction: &mut Interaction) -> Result<()>;
    fn after_replay(&self, interaction: &Interaction) -> Result<()>;
    fn name(&self) -> &str;
}
```

### Phase 2: Built-in Hooks ✅

**Effort**: 1 day
**Completed**: 2025-10-25

- [x] `SensitiveHeaderFilter` - Filters Authorization, Cookie, API keys
- [x] `BodyPatternReplacer` - Regex-based body replacement
- [x] `LoggingHook` - Logs interactions to stderr (verbose mode)
- [x] Tests for all built-in hooks

**Usage**:
```rust
let mut recorder = Recorder::new("my-cassette".to_string());

// Add sensitive header filter
let mut filter = SensitiveHeaderFilter::new();
filter.add_header("x-custom-api-key");
recorder.add_hook(filter);

// Add body pattern replacer
let mut replacer = BodyPatternReplacer::new();
replacer.add_pattern(r#""password":"[^"]*""#, r#""password":"[FILTERED]""#)?;
recorder.add_hook(replacer);

// Add logging hook
recorder.add_hook(LoggingHook::new().verbose());
```

### Phase 3: Recorder/Player Integration ✅

**Effort**: 1 day
**Completed**: 2025-10-25

- [x] Integrate hooks into `Recorder::record_http()`
- [x] Integrate hooks into `Recorder::record_http_error()`
- [x] Integrate hooks into `Player::get_interaction_with_hooks()`
- [x] Add `Player::mark_replayed()` for after_replay hooks
- [x] Error handling (log warnings, don't crash on hook errors)

### Phase 4: Documentation & Examples (PENDING)

**Effort**: 1 day
**Target**: 2025-10-26

- [ ] Create `examples/hooks_basic.rs` - SensitiveHeaderFilter
- [ ] Create `examples/hooks_advanced.rs` - Custom hook implementation
- [ ] Update README with hook examples
- [ ] Add hook section to architecture docs
- [ ] UniFFI bindings for hooks (if feasible)

**Example**:
```rust
// examples/hooks_advanced.rs
use magneto_serge::hooks::RecordHook;

struct TimestampNormalizer;

impl RecordHook for TimestampNormalizer {
    fn before_record(&self, interaction: &mut Interaction) -> Result<()> {
        if let InteractionKind::Http { response, .. } = &mut interaction.kind {
            if let Some(body) = &response.body {
                if let Ok(text) = String::from_utf8(body.clone()) {
                    // Replace ISO timestamps with fixed value
                    let normalized = regex::Regex::new(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z")
                        .unwrap()
                        .replace_all(&text, "2025-01-01T00:00:00Z");
                    response.body = Some(normalized.into_bytes());
                }
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "TimestampNormalizer"
    }
}
```

### Phase 1.5: MagnetoProxy API Additions (REQUIRED)

**Effort**: 1 day
**Priority**: BLOCKER for Phase 2-4

The `#[magneto_test]` macro requires these new methods on `MagnetoProxy`:

```rust
impl MagnetoProxy {
    // Mode control
    pub fn set_mode(&self, mode: ProxyMode) -> Result<()>;
    pub fn set_port(&mut self, port: u16) -> Result<()>;

    // Recording control
    pub fn start_recording(&self, cassette_name: impl AsRef<str>) -> Result<()>;
    pub fn stop_recording(&self) -> Result<()>;

    // Replay control
    pub fn start_replay(&self, cassette_name: impl AsRef<str>) -> Result<()>;
    pub fn stop_replay(&self) -> Result<()>;

    // Passthrough mode
    pub fn start_passthrough(&self) -> Result<()>;
    pub fn stop_passthrough(&self) -> Result<()>;
}
```

### Release Checklist

- [x] All hook traits implemented and tested
- [x] Built-in hooks working
- [x] Integration tests pass
- [ ] Documentation complete
- [ ] Examples created
- [ ] Changelog updated
- [ ] Version bumped to 0.3.0
- [ ] Release notes prepared

### v0.3.1 Release Checklist

- [x] `#[magneto_test]` macro implemented
- [x] Attribute parsing complete
- [x] Documentation written
- [ ] MagnetoProxy API additions
- [ ] Macro integration tests
- [ ] Examples with real HTTP calls
- [ ] Jest plugin
- [ ] pytest plugin
- [ ] RSpec integration
- [ ] Version bumped to 0.3.1

---

## 🧪 v0.3.1 - Test Framework Integration

**Target**: 2025-11-15 (1 week)
**Priority**: HIGH
**Status**: 🟡 **IN PROGRESS** (50% complete)

### Goals

Make Magneto-Serge as easy to use in tests as VCR's RSpec integration.

### Phase 1: Rust Macro ✅ **COMPLETED** (2025-10-25)

**Effort**: 2 days (actual: 1 day)

- [x] Create `#[magneto_test]` proc macro ✅
- [x] Auto-start proxy with cassette name from test function name ✅
- [x] Auto-stop and save cassette after test ✅
- [x] Support cassette name override: `#[magneto_test(cassette = "custom")]` ✅
- [x] Support mode override: `#[magneto_test(mode = "replay")]` ✅
- [x] Support port override: `#[magneto_test(port = 9000)]` ✅
- [x] Support cassette_dir override ✅
- [x] Syn 2.0 compatibility ✅
- [x] Documentation and README ✅

**Status**: ⚠️ Macro implemented but requires MagnetoProxy API additions (see below)

**API**:
```rust
#[magneto_test]
async fn test_api_get_users() {
    // Proxy auto-started with cassette "test_api_get_users"
    let response = reqwest::get("http://api.example.com/users").await?;
    assert_eq!(response.status(), 200);
    // Proxy auto-stopped, cassette saved
}

#[magneto_test(cassette = "shared_cassette", mode = "replay")]
async fn test_with_shared_cassette() {
    // Use shared cassette in replay mode
}
```

### Phase 2: JavaScript/Jest Plugin

**Effort**: 2 days

- [ ] Create Jest plugin: `@magneto-serge/jest`
- [ ] Automatic cassette management per test
- [ ] Setup/teardown hooks
- [ ] TypeScript types

**API**:
```javascript
import { magnetoTest } from '@magneto-serge/jest';

magnetoTest('should fetch users', async () => {
  // Proxy auto-configured
  const response = await fetch('http://api.example.com/users');
  expect(response.status).toBe(200);
});

magnetoTest('custom cassette', { cassette: 'shared', mode: 'replay' }, async () => {
  // Use shared cassette
});
```

### Phase 3: Python/pytest Plugin

**Effort**: 2 days

- [ ] Create pytest plugin: `pytest-magneto-serge`
- [ ] Decorator-based API: `@magneto_cassette("name")`
- [ ] Fixture-based API: `magneto_proxy` fixture
- [ ] Auto cassette naming from test name

**API**:
```python
import pytest
from magneto_serge import magneto_cassette

@magneto_cassette("test_get_users")
def test_get_users():
    # Proxy auto-started
    response = requests.get("http://api.example.com/users")
    assert response.status_code == 200
    # Proxy auto-stopped

# Or use fixture
def test_with_fixture(magneto_proxy):
    magneto_proxy.set_cassette("my_cassette")
    magneto_proxy.set_mode("record")
    response = requests.get("http://api.example.com/users")
    assert response.status_code == 200
```

### Phase 4: Ruby/RSpec Integration (Compete with VCR!)

**Effort**: 2 days

- [ ] Create RSpec helper: `magneto-serge-rspec` gem
- [ ] Tag-based cassette naming: `it "works", :magneto`
- [ ] Configuration block like VCR
- [ ] Drop-in replacement for VCR

**API**:
```ruby
require 'magneto/serge/rspec'

RSpec.configure do |config|
  config.include MagnetoSerge::RSpec

  config.magneto_serge do |m|
    m.cassette_dir = 'spec/cassettes'
    m.default_mode = :auto
  end
end

RSpec.describe MyAPI do
  it "fetches users", :magneto do
    # Cassette auto-named "MyAPI/fetches_users"
    response = HTTParty.get('http://api.example.com/users')
    expect(response.code).to eq(200)
  end

  it "custom cassette", magneto: { cassette: 'shared', mode: :replay } do
    # Use shared cassette
  end
end
```

### Release Checklist

- [ ] Rust macro implemented and tested
- [ ] Jest plugin published to npm
- [ ] pytest plugin published to PyPI
- [ ] RSpec gem published to RubyGems
- [ ] Documentation for all integrations
- [ ] Migration guide from VCR to Magneto-Serge
- [ ] Version bumped to 0.3.1

---

## 🎨 v0.4.0 - Templates & Dynamic Responses

**Target**: 2025-11-25 (1.5 weeks)
**Priority**: MEDIUM

### Goals

Support dynamic responses with template substitution, similar to VCR's ERB support.

### Features

**Effort**: 4 days

- [ ] Handlebars template engine integration
- [ ] Template syntax in cassette JSON/MessagePack
- [ ] Environment variable substitution: `{{ env.API_KEY }}`
- [ ] Dynamic timestamps: `{{ now }}`
- [ ] Request data access: `{{ request.headers.user_id }}`
- [ ] Custom helper functions

**Cassette Format**:
```json
{
  "interactions": [
    {
      "type": "Http",
      "request": { "method": "GET", "url": "..." },
      "response": {
        "status": 200,
        "body": "{\"api_key\":\"{{ env.API_KEY }}\",\"timestamp\":\"{{ now }}\",\"user_id\":\"{{ request.headers.x-user-id }}\"}"
      }
    }
  ]
}
```

**Rust API**:
```rust
let mut player = Player::load(dir, "template_cassette")?;

// Register custom helper
player.register_template_helper("random_id", |_| {
    format!("id_{}", rand::random::<u32>())
});

// Templates applied automatically during replay
```

### Release Checklist

- [ ] Handlebars integration implemented
- [ ] Template rendering tested
- [ ] Documentation with examples
- [ ] Migration guide from static cassettes
- [ ] Version bumped to 0.4.0

---

## 💡 v0.4.1 - Better Error Messages

**Target**: 2025-12-05 (1 week)
**Priority**: LOW

### Goals

Provide helpful, actionable error messages when cassettes don't match.

### Features

**Effort**: 3 days

- [ ] Similarity scoring for requests
- [ ] "Did you mean?" suggestions
- [ ] Diff tool for request mismatch
- [ ] Suggest matcher strategies
- [ ] Colorized terminal output

**Error Output**:
```
❌ No matching interaction found for:
   GET https://api.example.com/users/123

📋 Similar interactions in cassette "my_cassette":
   1. GET https://api.example.com/users/456 (score: 0.95)
      ❓ Difference: URL path differs (/users/123 vs /users/456)

   2. GET https://api.example.com/users     (score: 0.80)
      ❓ Difference: URL path differs (/users/123 vs /users)

💡 Suggestions:
   1. Use UrlMatchMode::Regex with pattern: /users/\d+

      strategy.url_match_mode = UrlMatchMode::Regex {
          pattern: r"/users/\d+".to_string()
      };

   2. Use UrlMatchMode::PathOnly to ignore user ID

      strategy.url_match_mode = UrlMatchMode::PathOnly;

   3. Re-record cassette in record mode:

      cargo run --bin magneto record my_cassette

🔍 Run with RUST_LOG=debug for detailed request comparison
```

### Release Checklist

- [ ] Similarity scoring algorithm
- [ ] Suggestion engine
- [ ] Colorized output
- [ ] Documentation
- [ ] Version bumped to 0.4.1

---

## 🌍 v0.5.0 - Multi-Language Bindings (Stretch Goal)

**Target**: 2025-12-20 (2 weeks)
**Priority**: MEDIUM

### Goals

Deliver production-ready bindings for Python, Kotlin, Swift, Java.

### Phase 1: Python Bindings

**Effort**: 3 days

- [ ] UniFFI bindings generated
- [ ] Pythonic API wrapper
- [ ] Type hints (PEP 484)
- [ ] pytest plugin integration
- [ ] Publish to PyPI: `magneto-serge`

**API**:
```python
from magneto_serge import MagnetoProxy, ProxyMode

proxy = MagnetoProxy("/path/to/cassettes")
proxy.start_recording("my_cassette", mode=ProxyMode.AUTO)

# Configure app to use http://localhost:8888
# ... make requests ...

proxy.stop_recording()
```

### Phase 2: Kotlin/Android Bindings

**Effort**: 3 days

- [ ] UniFFI bindings for JVM
- [ ] Kotlin DSL wrapper
- [ ] Gradle plugin for Android
- [ ] Publish to Maven Central

**API**:
```kotlin
val proxy = MagnetoProxy("/path/to/cassettes")
proxy.startRecording("my_cassette", ProxyMode.AUTO)

// Configure OkHttp to use proxy
val client = OkHttpClient.Builder()
    .proxy(Proxy(Proxy.Type.HTTP, InetSocketAddress("localhost", 8888)))
    .build()

// ... make requests ...

proxy.stopRecording()
```

### Phase 3: Swift/iOS Bindings

**Effort**: 3 days

- [ ] UniFFI bindings for Swift
- [ ] Swift Package Manager support
- [ ] CocoaPods support
- [ ] URLSession integration

**API**:
```swift
let proxy = try MagnetoProxy(cassetteDir: "/path/to/cassettes")
try proxy.startRecording(cassetteName: "my_cassette", mode: .auto)

// Configure URLSession to use proxy
let config = URLSessionConfiguration.default
config.connectionProxyDictionary = [
    kCFProxyTypeHTTP: "localhost",
    kCFProxyPortNumberHTTP: 8888
]

// ... make requests ...

try proxy.stopRecording()
```

### Release Checklist

- [ ] All bindings published to respective package managers
- [ ] Documentation for each language
- [ ] Example projects
- [ ] Version bumped to 0.5.0

---

## 🎯 v1.0.0 - Production Release (Long-term Goal)

**Target**: Q1 2026
**Priority**: HIGH

### Requirements for 1.0

- [ ] All critical features from VCR comparison implemented
- [ ] Hook system mature and stable
- [ ] Test framework integration for Rust, JS, Python, Ruby
- [ ] Multi-language bindings (Python, Kotlin, Swift)
- [ ] Template/ERB support
- [ ] Comprehensive documentation
- [ ] Migration guides from VCR, go-vcr
- [ ] Performance benchmarks published
- [ ] Security audit completed
- [ ] Breaking changes finalized
- [ ] Semantic versioning commitment

---

## 📊 Success Metrics

### v0.3.0 (Hook System)
- [ ] 95% test coverage for hooks module
- [ ] 3+ built-in hooks
- [ ] Zero performance regression (<1% overhead)
- [ ] Documentation completeness: 100%

### v0.3.1 (Test Integration)
- [ ] 4 test framework integrations (Rust, Jest, pytest, RSpec)
- [ ] 50% reduction in boilerplate code vs manual setup
- [ ] Published packages: npm, PyPI, RubyGems

### v0.4.0 (Templates)
- [ ] Template rendering performance: <1ms overhead
- [ ] Support for 10+ built-in helpers
- [ ] Backward compatibility with static cassettes

### v1.0.0 (Production)
- [ ] 10,000+ downloads across all package managers
- [ ] 500+ GitHub stars
- [ ] 90%+ user satisfaction (surveys)
- [ ] Zero critical security issues

---

## 🚧 Out of Scope (Future Consideration)

These features are not planned for v0.3-0.5 but may be considered later:

- **Cassette encryption** - Encrypt sensitive cassettes at rest
- **Distributed recording** - Record across multiple proxy instances
- **GraphQL-specific matching** - Operation name + variables matching
- **gRPC support** - Record/replay gRPC calls
- **Database recording** - Mock database queries
- **Cloud cassette storage** - S3/GCS backend for cassettes

---

## 🤝 Contributing

We welcome contributions! Priority areas for v0.3-0.4:

1. **High Impact**: Test framework integrations, hook examples
2. **Medium Impact**: Template helpers, error message improvements
3. **Low Impact**: Documentation, examples, bug fixes

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

*Last updated: 2025-10-25*
*Next review: 2025-11-01*
