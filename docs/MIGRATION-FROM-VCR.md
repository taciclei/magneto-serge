# Migration Guide: From VCR to Magneto-Serge

This guide helps you migrate from various VCR implementations to Magneto-Serge across different programming languages.

---

## Table of Contents

1. [Ruby: From VCR to magneto-serge-rspec](#ruby-from-vcr-to-magneto-serge-rspec)
2. [Python: From vcrpy to pytest-magneto-serge](#python-from-vcrpy-to-pytest-magneto-serge)
3. [PHP: From php-vcr to magneto-serge/phpunit](#php-from-php-vcr-to-magneto-sergephpunit)
4. [JavaScript: From nock to @magneto-serge/jest](#javascript-from-nock-to-magneto-sergejest)
5. [Benefits of Migrating](#benefits-of-migrating)
6. [Common Patterns](#common-patterns)

---

## Ruby: From VCR to magneto-serge-rspec

### Installation

**VCR:**
```ruby
# Gemfile
gem 'vcr'
gem 'webmock'  # or fakeweb
```

**Magneto-Serge:**
```ruby
# Gemfile
gem 'magneto-serge-rspec'
```

### Configuration

**VCR:**
```ruby
# spec/spec_helper.rb
require 'vcr'

VCR.configure do |config|
  config.cassette_library_dir = 'spec/fixtures/vcr_cassettes'
  config.hook_into :webmock
  config.configure_rspec_metadata!
  config.default_cassette_options = {
    record: :new_episodes,
    match_requests_on: [:method, :uri, :body]
  }
  config.filter_sensitive_data('<GITHUB_TOKEN>') { ENV['GITHUB_TOKEN'] }
end
```

**Magneto-Serge:**
```ruby
# spec/spec_helper.rb
require 'magneto/serge/rspec'

Magneto::Serge::RSpec.configure do |config|
  config.cassette_library_dir = 'spec/fixtures/cassettes'
  config.default_cassette_options = {
    record: :new_episodes,
    mode: :auto,
    match_requests_on: [:method, :uri, :body]
  }
  config.filter_sensitive_headers = %w[Authorization X-API-Key]
end
```

### Basic Usage

**VCR:**
```ruby
RSpec.describe 'GitHub API', :vcr do
  it 'fetches user' do
    # Cassette: spec/fixtures/vcr_cassettes/GitHub_API/fetches_user.yml
    response = HTTP.get('https://api.github.com/users/octocat')
    expect(response.status).to eq(200)
  end
end
```

**Magneto-Serge:**
```ruby
RSpec.describe 'GitHub API', :magneto do
  it 'fetches user' do
    # Cassette: spec/fixtures/cassettes/GitHub_API/fetches_user.json
    response = HTTP.get('https://api.github.com/users/octocat')
    expect(response.status).to eq(200)
  end
end
```

### Custom Cassette Names

**VCR:**
```ruby
it 'test', vcr: { cassette_name: 'custom' } do
  # Uses custom.yml
end
```

**Magneto-Serge:**
```ruby
it 'test', cassette: 'custom' do
  # Uses custom.json
end
```

### Recording Modes

**VCR:**
```ruby
it 'test', vcr: { record: :all } do
  # Force re-record
end
```

**Magneto-Serge:**
```ruby
it 'test', magneto: { record: :all } do
  # Force re-record
end
```

### Manual Control

**VCR:**
```ruby
VCR.use_cassette('my_cassette') do
  # HTTP calls recorded
end
```

**Magneto-Serge:**
```ruby
use_cassette('my_cassette', record: :new_episodes) do
  # HTTP calls recorded
end
```

---

## Python: From vcrpy to pytest-magneto-serge

### Installation

**vcrpy:**
```bash
pip install vcrpy pytest-recording
```

**Magneto-Serge:**
```bash
pip install pytest-magneto-serge
```

### Configuration

**vcrpy:**
```python
# conftest.py
import pytest
import vcr

@pytest.fixture(scope="module")
def vcr_config():
    return {
        "cassette_library_dir": "tests/cassettes",
        "record_mode": "new_episodes",
        "match_on": ["method", "scheme", "host", "port", "path"],
        "filter_headers": ["authorization"],
    }
```

**Magneto-Serge:**
```python
# conftest.py
import pytest
from pytest_magneto_serge import configure

configure({
    'cassette_dir': 'tests/cassettes',
    'record': 'new_episodes',
    'mode': 'auto',
    'filter_headers': ['Authorization', 'X-API-Key']
})
```

### Basic Usage

**vcrpy:**
```python
import vcr
import requests

@vcr.use_cassette('tests/cassettes/github_users.yaml')
def test_fetch_users():
    response = requests.get('https://api.github.com/users')
    assert response.status_code == 200
```

**Magneto-Serge (Marker):**
```python
import pytest
import requests

@pytest.mark.magneto_cassette('github_users')
def test_fetch_users():
    # Cassette: tests/cassettes/github_users.json
    response = requests.get('https://api.github.com/users')
    assert response.status_code == 200
```

**Magneto-Serge (Decorator):**
```python
from pytest_magneto_serge import magneto_cassette
import requests

@magneto_cassette('github_users')
def test_fetch_users():
    response = requests.get('https://api.github.com/users')
    assert response.status_code == 200
```

### Context Manager

**vcrpy:**
```python
def test_manual():
    with vcr.use_cassette('tests/cassettes/manual.yaml'):
        response = requests.get('https://api.example.com/data')
        assert response.ok
```

**Magneto-Serge:**
```python
from pytest_magneto_serge import use_cassette

def test_manual():
    with use_cassette('manual'):
        response = requests.get('https://api.example.com/data')
        assert response.ok
```

### Recording Modes

**vcrpy:**
```python
@vcr.use_cassette('cassette.yaml', record_mode='all')
def test_force_record():
    pass
```

**Magneto-Serge:**
```python
@magneto_cassette('cassette', record='all')
def test_force_record():
    pass
```

---

## PHP: From php-vcr to magneto-serge/phpunit

### Installation

**php-vcr:**
```bash
composer require --dev php-vcr/php-vcr php-vcr/phpunit-testlistener-vcr
```

**Magneto-Serge:**
```bash
composer require --dev magneto-serge/phpunit
```

### Configuration

**php-vcr:**
```php
// tests/bootstrap.php
use VCR\VCR;

VCR::configure()
    ->setCassettePath('tests/fixtures/cassettes')
    ->setMode('new_episodes')
    ->enableRequestMatchers(['method', 'url', 'body']);

VCR::turnOn();
```

**Magneto-Serge:**
```php
// No global configuration needed - configured per-test class
```

### Basic Usage

**php-vcr:**
```php
use PHPUnit\Framework\TestCase;
use VCR\VCR;

class ApiTest extends TestCase
{
    /**
     * @vcr github_users
     */
    public function testFetchUsers(): void
    {
        $response = file_get_contents('https://api.github.com/users');
        $this->assertNotEmpty($response);
    }
}
```

**Magneto-Serge:**
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
}
```

### Recording Modes

**php-vcr:**
```php
VCR::insertCassette('my_cassette', ['mode' => 'all']);
```

**Magneto-Serge:**
```php
#[Cassette('my_cassette', record: 'all')]
public function testForceRecord(): void
{
    // Always re-records
}
```

### Manual Control

**php-vcr:**
```php
VCR::turnOn();
VCR::insertCassette('my_cassette');
// HTTP calls
VCR::eject();
```

**Magneto-Serge:**
```php
$this->useCassette('my_cassette', function() {
    // HTTP calls recorded
});
```

---

## JavaScript: From nock to @magneto-serge/jest

**Note:** Magneto-Serge provides real HTTP recording/replay, unlike nock which is purely a mocking library.

### Installation

**nock:**
```bash
npm install --save-dev nock
```

**Magneto-Serge:**
```bash
npm install --save-dev @magneto-serge/jest
```

### Basic Usage

**nock:**
```javascript
import nock from 'nock';

test('should fetch users', async () => {
  nock('https://api.github.com')
    .get('/users')
    .reply(200, [{ id: 1, login: 'octocat' }]);

  const response = await fetch('https://api.github.com/users');
  expect(response.status).toBe(200);
});
```

**Magneto-Serge:**
```typescript
import { magnetoTest } from '@magneto-serge/jest';

magnetoTest('should fetch users', async () => {
  // First run: Records real HTTP response
  // Subsequent runs: Replays from cassette
  const response = await fetch('https://api.github.com/users');
  expect(response.status).toBe(200);
});
```

### Manual Control

**nock:**
```javascript
test('manual', async () => {
  const scope = nock('https://api.example.com')
    .get('/data')
    .reply(200, { message: 'ok' });

  // Test code

  scope.done();
});
```

**Magneto-Serge:**
```typescript
import { useCassette } from '@magneto-serge/jest';

test('manual', async () => {
  await useCassette('my_cassette', async () => {
    const response = await fetch('https://api.example.com/data');
    expect(response.ok).toBe(true);
  });
});
```

### Recording Real Responses

**nock (with recorder):**
```javascript
import nock from 'nock';

// Record mode
nock.recorder.rec();
// Make real HTTP calls
nock.recorder.play();
```

**Magneto-Serge (automatic):**
```typescript
import { magnetoTest, configure } from '@magneto-serge/jest';

configure({
  record: 'new_episodes',  // Records if cassette missing
});

magnetoTest('test', async () => {
  // Automatically records first time, replays after
});
```

---

## Benefits of Migrating

### 1. Performance

| Library | Performance | Language |
|---------|------------|----------|
| VCR | Baseline (1x) | Ruby |
| vcrpy | ~1-2x slower than VCR | Python |
| php-vcr | ~1-2x slower than VCR | PHP |
| **Magneto-Serge** | **10-100x faster** | All (Rust core) |

### 2. Multi-Language Support

**VCR Ecosystem:**
- Separate libraries per language
- Different APIs and configurations
- No cross-language cassette sharing

**Magneto-Serge:**
- Single Rust core with language bindings
- Consistent API across all languages
- Share cassettes between languages
- Same performance everywhere

### 3. WebSocket Support

**VCR:** ❌ No WebSocket support

**Magneto-Serge:** ✅ Full WebSocket record/replay with:
- Bidirectional message capture
- Timing preservation
- Direction tracking (sent/received)

### 4. Modern Features

**VCR:**
- YAML cassettes (slow to parse)
- Limited matching strategies
- Basic filtering

**Magneto-Serge:**
- JSON (fast) or MessagePack (compact)
- Advanced matching (regex, JSON path, size-only)
- Hook system for custom processing
- Latency simulation
- REST API for cassette management
- CLI tools (8 commands)

### 5. Active Development

**VCR:** Last major release 2019

**Magneto-Serge:** Active development (2025), modern tooling

---

## Common Patterns

### 1. Auto-Generated Cassette Names

**All VCR Implementations:**
Cassette names typically derived from test hierarchy

**Magneto-Serge:**
Same behavior - automatic hierarchical naming:
```
RSpec: "ClassName/method_name"
pytest: "module/class/function"
Jest: "describe/test_name"
PHPUnit: "ClassName_methodName"
```

### 2. Sensitive Data Filtering

**VCR (Ruby):**
```ruby
VCR.configure do |config|
  config.filter_sensitive_data('<TOKEN>') { ENV['API_TOKEN'] }
end
```

**Magneto-Serge (All Languages):**
Automatic header filtering + custom hooks:
```ruby
# Ruby
config.filter_sensitive_headers = %w[Authorization X-API-Key]
```

```python
# Python
configure({'filter_headers': ['Authorization', 'X-API-Key']})
```

### 3. Record Modes

All implementations support the same record modes:

| Mode | VCR Name | Magneto-Serge | Behavior |
|------|----------|---------------|----------|
| Auto | `:new_episodes` | `new_episodes` | Record if missing, replay if exists |
| Replay Only | `:once` | `once` or `replay` | Error if cassette missing |
| Force Record | `:all` | `all` or `record` | Always re-record |
| Passthrough | `:none` | `none` or `passthrough` | Never record/replay |

### 4. Cassette Format Migration

**Converting existing VCR cassettes:**

Magneto-Serge uses JSON format instead of YAML. You'll need to re-record:

```bash
# Delete old cassettes
rm -rf spec/fixtures/vcr_cassettes/*.yml

# Re-run tests in record mode
MAGNETO_MODE=record rspec
```

Or use the migration tool (coming in v0.4.x):
```bash
magneto migrate vcr-cassettes/ magneto-cassettes/ --from yaml --to json
```

---

## Migration Checklist

### Ruby (VCR → magneto-serge-rspec)

- [ ] Install `gem 'magneto-serge-rspec'`
- [ ] Remove `gem 'vcr'` and `gem 'webmock'`
- [ ] Update `spec_helper.rb` configuration
- [ ] Change `:vcr` metadata to `:magneto` or `:cassette`
- [ ] Update cassette directory path
- [ ] Re-record cassettes (delete old `.yml` files)
- [ ] Update CI configuration if needed

### Python (vcrpy → pytest-magneto-serge)

- [ ] Install `pip install pytest-magneto-serge`
- [ ] Remove `vcrpy` and `pytest-recording`
- [ ] Update `conftest.py` configuration
- [ ] Change `@vcr.use_cassette()` to `@magneto_cassette()` or markers
- [ ] Update cassette directory path
- [ ] Re-record cassettes (delete old `.yaml` files)
- [ ] Update CI configuration if needed

### PHP (php-vcr → magneto-serge/phpunit)

- [ ] Install `composer require --dev magneto-serge/phpunit`
- [ ] Remove `php-vcr` packages
- [ ] Extend `MagnetoTestCase` instead of `TestCase`
- [ ] Change `@vcr` annotations to `#[Cassette]` attributes
- [ ] Update cassette directory path in test class
- [ ] Re-record cassettes
- [ ] Update CI configuration if needed

### JavaScript (nock → @magneto-serge/jest)

- [ ] Install `npm install --save-dev @magneto-serge/jest`
- [ ] Remove `nock` (if migrating from recorded cassettes)
- [ ] Replace `test()` with `magnetoTest()`
- [ ] Remove manual mock setup code
- [ ] Add global configuration if needed
- [ ] Record real HTTP responses
- [ ] Update CI configuration if needed

---

## Getting Help

- **Documentation**: [docs/](/)
- **Examples**: Each integration package includes comprehensive examples
- **Issues**: [GitHub Issues](https://github.com/taciclei/magneto-serge/issues)
- **Migration Support**: See package-specific READMEs for detailed migration guides

---

## Next Steps

After migrating:

1. **Re-record cassettes** in your CI/CD to ensure consistency
2. **Update documentation** to reflect new cassette paths
3. **Configure hooks** for custom data filtering (v0.3.0+)
4. **Explore advanced features** like WebSocket recording, REST API, CLI tools

---

*Last updated: 2025-10-25 (v0.3.1)*
