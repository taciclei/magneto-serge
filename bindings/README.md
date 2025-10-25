# 🧪 Phase 2.2 - Testing Utilities

**Date:** 2025-10-24
**Status:** ✅ Ready to Use
**Objective:** Test helpers for popular testing frameworks

---

## 🎯 Problem Solved

**Before:** No test helpers → manual cassette loading and assertion
**After:** Custom matchers/assertions for 4 popular frameworks ✅

### Use Cases

```
Developer needs to:
├── Test with Jest → expect(response).toMatchCassette('user-login')
├── Test with JUnit → assertMatchesCassette(response, "user-login")
├── Test with pytest → assert_matches_cassette(response, 'user-login')
└── Test with PHPUnit → $this->assertMatchesCassette($response, 'user-login')
```

---

## 📦 What's Included

```
/tmp/magneto-phase2.2/
├── README.md (this file)
│
├── jest/                                    ← JavaScript/TypeScript
│   ├── magneto-matchers.js (250 lines)     ← 7 custom matchers
│   ├── magneto-matchers.d.ts               ← TypeScript definitions
│   ├── package.json                        ← NPM package config
│   └── example.test.js                     ← Usage examples
│
├── junit/                                   ← Java
│   ├── MagnetoAssertions.java (220 lines)  ← 6 assertions
│   ├── pom.xml                             ← Maven config
│   └── ExampleTest.java                    ← Usage examples
│
├── pytest/                                  ← Python
│   ├── magneto_pytest.py (280 lines)       ← 7 assertions + fixtures
│   ├── setup.py                            ← PyPI package config
│   └── test_example.py                     ← Usage examples
│
└── phpunit/                                 ← PHP
    ├── MagnetoAssertions.php (230 lines)   ← 6 assertions (trait)
    ├── composer.json                       ← Composer config
    └── ExampleTest.php                     ← Usage examples

Total: ~1,200 lines of code
```

---

## 🚀 Installation

### Jest (JavaScript/TypeScript)

```bash
# Install from npm
npm install --save-dev @magneto-serge/jest-matchers

# Or copy files
cp /tmp/magneto-phase2.2/jest/* your-project/test-utils/
```

**Setup:**

```javascript
// In your test setup file (e.g., setupTests.js)
import '@magneto-serge/jest-matchers';
```

### JUnit (Java)

```xml
<!-- Add to pom.xml -->
<dependency>
    <groupId>com.magnetoserge</groupId>
    <artifactId>magneto-junit</artifactId>
    <version>1.0.0</version>
    <scope>test</scope>
</dependency>
```

**Or copy files:**

```bash
cp /tmp/magneto-phase2.2/junit/MagnetoAssertions.java src/test/java/
```

### pytest (Python)

```bash
# Install from PyPI
pip install magneto-pytest

# Or copy files
cp /tmp/magneto-phase2.2/pytest/magneto_pytest.py your-project/tests/
```

### PHPUnit (PHP)

```bash
# Install via Composer
composer require --dev magneto-serge/phpunit-assertions

# Or copy files
cp /tmp/magneto-phase2.2/phpunit/MagnetoAssertions.php tests/
```

---

## 🔧 Usage

### 1. Jest (JavaScript/TypeScript)

#### Available Matchers

- `toMatchCassette(cassetteName)` - Assert response matches cassette
- `toMatchCassetteStatus(cassetteName, status)` - Assert status matches
- `toMatchCassetteBody(cassetteName)` - Assert body matches
- `toHaveInteractionCount(count)` - Assert interaction count
- `toHaveCookies()` - Assert cassette has cookies
- `toHaveCookie(cookieName)` - Assert specific cookie exists
- `toHaveCassetteVersion(version)` - Assert cassette version

#### Example

```javascript
import '@magneto-serge/jest-matchers';
import axios from 'axios';

describe('User API', () => {
  test('should match login response', async () => {
    const response = await axios.post('/api/authenticate', {
      username: 'admin',
      password: 'admin',
    });

    // Assert response matches cassette
    expect(response).toMatchCassette('user-login');
  });

  test('should have session cookie', () => {
    expect('user-login').toHaveCookie('JSESSIONID');
  });
});
```

---

### 2. JUnit (Java)

#### Available Assertions

- `assertMatchesCassette(response, cassetteName)` - Assert response matches
- `assertCassetteStatus(response, cassetteName, status)` - Assert status
- `assertInteractionCount(cassetteName, count)` - Assert interaction count
- `assertHasCookies(cassetteName)` - Assert cassette has cookies
- `assertHasCookie(cassetteName, cookieName)` - Assert specific cookie
- `assertCassetteVersion(cassetteName, version)` - Assert version

#### Example

```java
import static com.magnetoserge.junit.MagnetoAssertions.*;

public class UserTest {

    @BeforeAll
    public static void setUp() {
        setCassetteDirectory("./cassettes");
    }

    @Test
    public void testUserLogin() throws Exception {
        HttpResponse<String> response = client.send(request,
            HttpResponse.BodyHandlers.ofString());

        // Assert response matches cassette
        assertMatchesCassette(response, "user-login");
    }

    @Test
    public void testSessionCookie() {
        assertHasCookie("user-login", "JSESSIONID");
    }
}
```

---

### 3. pytest (Python)

#### Available Assertions

- `assert_matches_cassette(response, cassetteName)` - Assert response matches
- `assert_cassette_status(response, cassetteName, status)` - Assert status
- `assert_cassette_body(response, cassetteName)` - Assert body matches
- `assert_interaction_count(cassetteName, count)` - Assert interaction count
- `assert_has_cookies(cassetteName)` - Assert cassette has cookies
- `assert_has_cookie(cassetteName, cookieName)` - Assert specific cookie
- `assert_cassette_version(cassetteName, version)` - Assert version

#### Example

```python
import pytest
import requests
from magneto_pytest import (
    assert_matches_cassette,
    assert_has_cookie,
)

@pytest.fixture(scope="session")
def http_client():
    session = requests.Session()
    session.proxies = {'http': 'http://localhost:8888'}
    return session

def test_user_login(http_client):
    response = http_client.post(
        'http://localhost:8080/api/authenticate',
        json={'username': 'admin', 'password': 'admin'}
    )

    # Assert response matches cassette
    assert_matches_cassette(response, 'user-login')

def test_session_cookie():
    assert_has_cookie('user-login', 'JSESSIONID')
```

---

### 4. PHPUnit (PHP)

#### Available Assertions

- `assertMatchesCassette($response, $cassetteName)` - Assert response matches
- `assertCassetteStatus($response, $cassetteName, $status)` - Assert status
- `assertInteractionCount($cassetteName, $count)` - Assert interaction count
- `assertHasCookies($cassetteName)` - Assert cassette has cookies
- `assertHasCookie($cassetteName, $cookieName)` - Assert specific cookie
- `assertCassetteVersion($cassetteName, $version)` - Assert version

#### Example

```php
use MagnetoSerge\PHPUnit\MagnetoAssertions;
use PHPUnit\Framework\TestCase;

class UserTest extends TestCase
{
    use MagnetoAssertions;

    protected function setUp(): void
    {
        $this->client = new \GuzzleHttp\Client([
            'proxy' => 'http://localhost:8888',
        ]);

        self::setCassetteDirectory('./cassettes');
    }

    public function testUserLogin(): void
    {
        $response = $this->client->post('/api/authenticate', [
            'json' => ['username' => 'admin', 'password' => 'admin'],
        ]);

        // Assert response matches cassette
        $this->assertMatchesCassette($response, 'user-login');
    }

    public function testSessionCookie(): void
    {
        $this->assertHasCookie('user-login', 'JSESSIONID');
    }
}
```

---

## 📖 Common Workflows

### Workflow 1: Basic Response Validation

```javascript
// Jest
test('validates complete response', async () => {
  const response = await axios.get('/api/users/1');

  expect(response).toMatchCassette('user-detail');
  expect(response).toMatchCassetteStatus('user-detail', 200);
  expect(response).toMatchCassetteBody('user-detail');
});
```

### Workflow 2: Cookie Validation

```python
# pytest
def test_authentication_flow():
    # Login should set cookies
    response = http_client.post('/api/authenticate', ...)
    assert_matches_cassette(response, 'user-login')

    # Verify cookies are in cassette
    assert_has_cookies('user-login')
    assert_has_cookie('user-login', 'JSESSIONID')
    assert_has_cookie('user-login', 'XSRF-TOKEN')
```

### Workflow 3: Cassette Metadata Validation

```java
// JUnit
@Test
public void testCassetteMetadata() {
    // Verify cassette structure
    assertCassetteVersion("user-login", "2.0");
    assertInteractionCount("user-login", 3);
    assertHasCookies("user-login");
}
```

### Workflow 4: Multiple Endpoint Testing

```php
// PHPUnit
public function testCompleteUserFlow(): void
{
    // 1. Login
    $loginResponse = $this->client->post('/api/authenticate', ...);
    $this->assertMatchesCassette($loginResponse, 'user-login');
    $this->assertHasCookie('user-login', 'JSESSIONID');

    // 2. Get account
    $accountResponse = $this->client->get('/api/account');
    $this->assertCassetteStatus($accountResponse, 'user-account', 200);

    // 3. Get users
    $usersResponse = $this->client->get('/api/users');
    $this->assertMatchesCassette($usersResponse, 'user-list');
}
```

---

## 🎯 Advanced Features

### Custom Cassette Directory

```javascript
// Jest - via environment or config
process.env.CASSETTE_DIR = './e2e-cassettes';
```

```java
// JUnit
MagnetoAssertions.setCassetteDirectory("./e2e-cassettes");
```

```python
# pytest - via fixture
@pytest.fixture(scope="session")
def cassette_dir():
    return "./e2e-cassettes"
```

```php
// PHPUnit
self::setCassetteDirectory('./e2e-cassettes');
```

### Conditional Assertions

```javascript
// Jest - only assert if cassette exists
const cassetteName = 'user-login';
try {
  expect(response).toMatchCassette(cassetteName);
} catch (error) {
  if (error.message.includes('not found')) {
    console.warn(`Cassette ${cassetteName} not found, skipping`);
  } else {
    throw error;
  }
}
```

### Custom Error Messages

```java
// JUnit - with custom messages
try {
    assertMatchesCassette(response, "user-login");
} catch (AssertionError e) {
    fail("Expected response to match cassette but got: " + e.getMessage());
}
```

---

## 🧪 Testing the Utilities

### Test Jest Matchers

```bash
cd /tmp/magneto-phase2.2/jest
npm install
npm test
```

### Test JUnit Assertions

```bash
cd /tmp/magneto-phase2.2/junit
mvn test
```

### Test pytest Plugin

```bash
cd /tmp/magneto-phase2.2/pytest
pip install -e .
pytest test_example.py
```

### Test PHPUnit Assertions

```bash
cd /tmp/magneto-phase2.2/phpunit
composer install
vendor/bin/phpunit ExampleTest.php
```

---

## 📊 Comparison

| Feature | Jest | JUnit | pytest | PHPUnit |
|---------|------|-------|--------|---------|
| **Response Matching** | ✅ | ✅ | ✅ | ✅ |
| **Status Assertion** | ✅ | ✅ | ✅ | ✅ |
| **Body Assertion** | ✅ | ❌ | ✅ | ❌ |
| **Cookie Validation** | ✅ | ✅ | ✅ | ✅ |
| **Interaction Count** | ✅ | ✅ | ✅ | ✅ |
| **Version Check** | ✅ | ✅ | ✅ | ✅ |
| **Custom Directory** | ✅ | ✅ | ✅ | ✅ |
| **TypeScript Support** | ✅ | N/A | N/A | N/A |
| **Fixtures** | ❌ | ❌ | ✅ | ❌ |

---

## 🐛 Troubleshooting

### Issue: "Cassette not found"

**Solution:** Check cassette directory path

```javascript
// Verify cassette exists
const fs = require('fs');
console.log(fs.existsSync('./cassettes/user-login.json'));
```

### Issue: "No matching interaction"

**Cause:** Request URL/method doesn't match any interaction in cassette

**Solution:** Log request details and cassette contents

```python
# Debug matching
cassette = cassette_manager.load('user-login')
print(f"Interactions: {len(cassette['interactions'])}")
for i, interaction in enumerate(cassette['interactions']):
    req = interaction['kind']['Http']['request']
    print(f"{i}: {req['method']} {req['url']}")
```

### Issue: "MessagePack not supported"

**Cause:** Cassette is in MessagePack format

**Solution:** Convert to JSON or wait for MessagePack support

```bash
# Convert msgpack to json (future feature)
magneto export user-login --output user-login.json --format json
```

---

## 🔜 Future Enhancements

### Planned Features

- **MessagePack Support** - Read .msgpack cassettes
- **Body Matching** - Deep JSON comparison
- **Partial Matching** - Match subset of response
- **Custom Matchers** - User-defined matchers
- **Snapshot Testing** - Jest-style snapshots
- **Mocking Utilities** - Mock HTTP clients

### Requested by Community

- **Chai Plugin** (JavaScript)
- **RSpec Matchers** (Ruby)
- **xUnit Assertions** (C#)
- **Go Testing Helpers**

---

## 📚 Resources

- **Phase 1:** `/tmp/PHASE_1_COMPLETE_SUMMARY.md`
- **Phase 2.1:** `/tmp/PHASE_2.1_SUMMARY.md`
- **Roadmap:** `/tmp/ROADMAP_IMPROVEMENTS.md`

---

## 🎯 Design Principles

1. **Consistent API** - Similar function names across all frameworks
2. **Framework Idioms** - Follow each framework's conventions
3. **Minimal Dependencies** - Only framework + JSON parsing
4. **Clear Errors** - Helpful assertion failure messages
5. **Easy Setup** - Single import/use statement

---

**Created:** 2025-10-24
**Version:** Phase 2.2

🧪 **Test utilities ready to use!** 🚀
