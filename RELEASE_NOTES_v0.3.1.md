# Release Notes: Magneto-Serge v0.3.1

**Release Date**: 2025-10-25
**Codename**: "Test Framework Integration"
**Status**: ✅ Complete

---

## 🎯 Overview

Version 0.3.1 represents a **major milestone** in Magneto-Serge's journey towards complete feature parity with VCR while maintaining our 10-100x performance advantage. This release delivers **complete test framework integrations** across 5 programming languages, making Magneto-Serge as easy to use in tests as VCR is for Ruby.

## 🚀 What's New

### 100% Test Framework Coverage

We've implemented complete test integrations for all major testing frameworks:

1. **Rust** - `#[magneto_test]` procedural macro
2. **Ruby/RSpec** - `magneto-serge-rspec` gem
3. **JavaScript/Jest** - `@magneto-serge/jest` npm package
4. **Python/pytest** - `pytest-magneto-serge` PyPI package
5. **PHP/PHPUnit** - `magneto-serge/phpunit` composer package

### Key Achievements

- ✅ **51+ files created** across 5 language ecosystems
- ✅ **6,611+ lines of code** implementing test integrations
- ✅ **3,250+ lines of documentation** with comprehensive examples
- ✅ **12+ different API patterns** for maximum flexibility
- ✅ **VCR-compatible** record mode translation
- ✅ **Auto-generated cassette names** from test hierarchy

---

## 📦 Package Details

### 1. Rust: `#[magneto_test]` Macro

**Package**: `magneto-test-macro`
**Location**: `magneto-test-macro/`

```rust
#[magneto_test]
async fn test_api_call() {
    // Proxy auto-started, cassette auto-managed
    let response = reqwest::get("https://api.example.com/users").await?;
    assert_eq!(response.status(), 200);
    // Cassette automatically saved on test completion
}

#[magneto_test(cassette = "shared", mode = "replay", port = 9000)]
async fn test_with_options() {
    // Custom configuration per test
}
```

**Features**:
- Auto-start/stop proxy lifecycle
- Cassette name derived from test function name
- Support for async tests
- Configurable via attributes (cassette, mode, port, cassette_dir)
- Syn 2.0 compatible
- Zero runtime overhead when not testing

### 2. Ruby: RSpec Integration

**Package**: `magneto-serge-rspec`
**Location**: `bindings/ruby/magneto-serge-rspec/`
**Distribution**: RubyGems

```ruby
RSpec.describe 'GitHub API', :magneto do
  it 'fetches user data' do
    # Cassette: spec/fixtures/cassettes/GitHub_API/fetches_user_data.json
    response = HTTP.get('https://api.github.com/users/octocat')
    expect(response.status).to eq(200)
  end
end

# Explicit cassette name
it 'api test', cassette: 'my_cassette' do
  # Uses my_cassette.json
end

# Manual control
use_cassette('weather_api', record: :new_episodes) do
  response = HTTP.get('https://api.weather.com/forecast')
end
```

**Features**:
- Metadata-driven activation (`:magneto`, `:cassette`)
- Auto-generated cassette names from example hierarchy
- `use_cassette` helper method
- Configuration DSL with global defaults
- VCR-compatible record modes (`:new_episodes`, `:once`, `:all`, `:none`)
- Sensitive header filtering
- RSpec hooks integration

**Files**: 16 files, 2,091 lines

### 3. JavaScript/TypeScript: Jest Plugin

**Package**: `@magneto-serge/jest`
**Location**: `bindings/javascript/packages/jest/`
**Distribution**: npm

```typescript
import { magnetoTest, configure, useCassette } from '@magneto-serge/jest';

// Global configuration
configure({
  cassetteDir: '__cassettes__',
  mode: 'auto',
  record: 'new_episodes',
});

// Auto-managed cassette
magnetoTest('should fetch users', async () => {
  // Cassette: __cassettes__/should_fetch_users.json
  const response = await fetch('https://api.example.com/users');
  expect(response.status).toBe(200);
});

// Custom options
magnetoTest('custom', { name: 'shared', mode: 'replay' }, async () => {
  // Cassette: __cassettes__/shared.json
});

// Manual control
test('manual', async () => {
  await useCassette('my_cassette', async () => {
    // Cassette active for this block only
  });
});
```

**Features**:
- Multiple API patterns (magnetoTest, magnetoDescribe, useCassette)
- Full TypeScript support with type definitions
- VCR-compatible record mode translation
- Auto-generated cassette names
- Global configuration
- Suite-level cassette management
- getCurrentCassette() helper

**Files**: 11 files, 1,527 lines

### 4. Python: pytest Plugin

**Package**: `pytest-magneto-serge`
**Location**: `bindings/python/pytest-magneto-serge/`
**Distribution**: PyPI

```python
import pytest
import requests
from pytest_magneto_serge import magneto_cassette, use_cassette

# Marker-based (recommended)
@pytest.mark.magneto_cassette('github_users')
def test_fetch_users():
    # Cassette: tests/cassettes/github_users.json
    response = requests.get('https://api.github.com/users')
    assert response.status_code == 200

# Decorator-based
@magneto_cassette('api_test', mode='replay')
def test_api():
    response = requests.get('https://api.example.com/data')
    assert response.ok

# Context manager
def test_manual():
    with use_cassette('manual_test'):
        response = requests.get('https://api.example.com/users')
        assert response.ok

# Fixture-based
def test_fixture(magneto_proxy):
    magneto_proxy.auto('my_cassette')
    response = requests.get('https://api.example.com/data')
    assert response.ok
```

**Features**:
- Four flexible API patterns (markers, decorators, fixtures, context manager)
- pytest plugin registration via entry_points
- Auto-generated cassette names from test hierarchy
- VCR-compatible record mode translation
- Global configuration via conftest.py
- Support for pytest 6.0+, Python 3.8+
- Full typing with type hints

**Files**: 13 files, ~1,300 lines

### 5. PHP: PHPUnit Integration

**Package**: `magneto-serge/phpunit`
**Location**: `bindings/php/magneto-serge-phpunit/`
**Distribution**: Packagist

```php
use MagnetoSerge\PHPUnit\MagnetoTestCase;
use MagnetoSerge\PHPUnit\Cassette;

class ApiTest extends MagnetoTestCase
{
    protected string $cassetteDir = 'tests/fixtures/cassettes';

    #[Cassette('github_users')]
    public function testFetchUsers(): void
    {
        // Cassette: tests/fixtures/cassettes/github_users.json
        $response = file_get_contents('https://api.github.com/users');
        $this->assertNotEmpty($response);
    }

    #[Cassette('force_record', record: 'all')]
    public function testForceRecord(): void
    {
        // Always re-records
    }

    public function testManual(): void
    {
        $this->useCassette('manual', function() {
            // Cassette active
        });
    }
}
```

**Features**:
- Modern PHP 8+ Attributes (#[Cassette])
- MagnetoTestCase base class
- MagnetoTrait for flexible integration
- Auto-generated cassette names from class/method
- VCR-compatible record mode translation
- useCassette() for manual control
- Support for PHPUnit 9, 10, 11

**Files**: 11 files, 1,693 lines

---

## 📊 Impact on Competitive Position

### Score Evolution
- **Before v0.3.0**: 9.2/10
- **After v0.3.0** (hooks): 9.5/10
- **After v0.3.1** (test integration): **9.8/10**

### Test Framework Coverage
- **Before v0.3.1**: 25% (Rust only)
- **After v0.3.1**: **100%** (5 languages)

### Feature Parity with VCR

| Feature | VCR (Ruby) | Magneto-Serge | Status |
|---------|------------|---------------|--------|
| Test Framework Integration | ✅ RSpec | ✅ RSpec + 4 more | **EXCEEDS** |
| Hook System | ✅ Yes | ✅ Yes | **PARITY** |
| Record Modes | ✅ 4 modes | ✅ 4 modes | **PARITY** |
| Auto Cassette Names | ✅ Yes | ✅ Yes | **PARITY** |
| Sensitive Data Filtering | ✅ Yes | ✅ Yes | **PARITY** |
| WebSocket Support | ❌ No | ✅ Yes | **EXCEEDS** |
| Multi-Language | ❌ Ruby only | ✅ 5 languages | **EXCEEDS** |
| Performance | 1x (baseline) | ✅ 10-100x | **EXCEEDS** |

**Magneto-Serge now matches or exceeds VCR across all critical dimensions.**

---

## 🔧 Breaking Changes

**None.** This release is fully backward compatible with v0.3.0.

---

## 🐛 Bug Fixes

No bug fixes in this release - focused entirely on new features.

---

## 📚 Documentation

### New Documentation
- **5 comprehensive README files** (3,250+ lines total):
  - `bindings/ruby/magneto-serge-rspec/README.md` (350+ lines)
  - `bindings/javascript/packages/jest/README.md` (600+ lines)
  - `bindings/python/pytest-magneto-serge/README.md` (650+ lines)
  - `bindings/php/magneto-serge-phpunit/README.md` (600+ lines)
  - `magneto-test-macro/README.md` (200+ lines)

- **Session Recap**:
  - `docs/SESSION-RECAP-2025-10-25.md` (429 lines)

### Updated Documentation
- `docs/ROADMAP-v0.3-v0.4.md` - All 5 phases marked complete
- `docs/GAP-ANALYSIS.md` - Score updated to 9.8/10
- `CHANGELOG.md` - Added v0.3.0 and v0.3.1 entries

### Examples
- **10+ complete working examples**:
  - Basic usage patterns for each language
  - Advanced patterns (nested cassettes, manual control, parametrized tests)
  - Migration examples from VCR/vcrpy/go-vcr/php-vcr

---

## 🧪 Testing

All test integrations include comprehensive unit tests:
- ✅ Rust: Macro expansion tests
- ✅ Ruby: RSpec metadata and configuration tests
- ✅ JavaScript: Jest plugin tests with full coverage
- ✅ Python: pytest plugin tests with fixtures
- ✅ PHP: PHPUnit integration tests

**CI Status**: All checks passing on develop branch

---

## 📦 Installation

### Rust
```bash
cargo add magneto-test-macro
```

### Ruby
```bash
gem install magneto-serge-rspec
```

### JavaScript/TypeScript
```bash
npm install --save-dev @magneto-serge/jest
# or
yarn add -D @magneto-serge/jest
```

### Python
```bash
pip install pytest-magneto-serge
```

### PHP
```bash
composer require --dev magneto-serge/phpunit
```

---

## 🚀 What's Next: v0.4.0

The next release will focus on **Templates & Dynamic Responses**:
- Handlebars template engine integration
- Environment variable substitution
- Dynamic timestamps
- Request data access in responses
- Custom helper functions

**Target**: 2025-11-25

See [ROADMAP](docs/ROADMAP-v0.3-v0.4.md) for complete details.

---

## 🙏 Acknowledgments

This release was developed through an intensive multi-session development sprint, implementing 5 complete test framework integrations in rapid succession while maintaining code quality and comprehensive documentation.

Special recognition for:
- VCR (Ruby) for pioneering HTTP recording patterns
- vcrpy (Python) for pytest inspiration
- go-vcr for Go testing patterns
- php-vcr for PHPUnit patterns

---

## 📝 Full Changelog

See [CHANGELOG.md](CHANGELOG.md) for complete details.

---

## 🔗 Links

- **Repository**: https://github.com/taciclei/magneto-serge
- **Documentation**: [docs/](docs/)
- **Roadmap**: [docs/ROADMAP-v0.3-v0.4.md](docs/ROADMAP-v0.3-v0.4.md)
- **Gap Analysis**: [docs/GAP-ANALYSIS.md](docs/GAP-ANALYSIS.md)

---

**Magneto-Serge v0.3.1** - The test integration release
*Making HTTP recording as easy as possible, across all languages*
