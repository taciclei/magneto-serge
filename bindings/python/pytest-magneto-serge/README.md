# pytest-magneto-serge

pytest integration for Magnéto-Serge, providing automatic cassette management for HTTP/WebSocket recording and replay.

## Installation

```bash
pip install pytest-magneto-serge
```

## Quick Start

### Using Pytest Markers

```python
import pytest
import requests

@pytest.mark.magneto_cassette('github_users')
def test_fetch_users():
    """Cassette: tests/cassettes/github_users.json"""
    response = requests.get('https://api.github.com/users')
    assert response.status_code == 200
    assert len(response.json()) > 0

# Auto-generated cassette name
@pytest.mark.magneto_cassette
def test_fetch_posts():
    """Cassette: tests/cassettes/test_module/fetch_posts.json"""
    response = requests.get('https://jsonplaceholder.typicode.com/posts')
    assert response.status_code == 200

# Force re-recording
@pytest.mark.magneto_cassette('live_data', record='all')
def test_live_api():
    """Always re-records this cassette"""
    response = requests.get('https://api.example.com/live')
    assert response.status_code == 200
```

### Using Decorator

```python
from pytest_magneto_serge import magneto_cassette

@magneto_cassette('api_test')
def test_with_decorator():
    """Cassette: tests/cassettes/api_test.json"""
    response = requests.get('https://jsonplaceholder.typicode.com/posts/1')
    assert response.status_code == 200
    assert response.json()['id'] == 1

@magneto_cassette(record='all')
def test_force_record():
    """Auto-generated name, always re-records"""
    response = requests.get('https://api.example.com/data')
    assert response.ok
```

### Using Fixture

```python
def test_with_fixture(magneto_proxy):
    """Using magneto_proxy fixture for manual control"""
    magneto_proxy.auto('manual_cassette')

    response = requests.get('https://jsonplaceholder.typicode.com/users/1')
    assert response.status_code == 200

    magneto_proxy.stop()
```

### Using Context Manager

```python
from pytest_magneto_serge import use_cassette

def test_context_manager():
    """Manual cassette control with context manager"""
    with use_cassette('context_test'):
        response = requests.get('https://jsonplaceholder.typicode.com/posts')
        assert response.status_code == 200

def test_nested_cassettes():
    """Nested cassettes"""
    with use_cassette('outer'):
        users = requests.get('https://jsonplaceholder.typicode.com/users').json()

        with use_cassette('inner'):
            posts = requests.get('https://jsonplaceholder.typicode.com/posts').json()
            assert len(posts) > 0

        assert len(users) > 0
```

## Configuration

### Global Configuration

Create a `conftest.py` file:

```python
import pytest

@pytest.fixture(scope='session', autouse=True)
def configure_magneto(magneto_config):
    """Configure Magneto-Serge globally"""
    magneto_config.cassette_dir = 'tests/fixtures/cassettes'
    magneto_config.mode = 'auto'
    magneto_config.record_mode = 'new_episodes'
    magneto_config.verbose = True
```

### pytest.ini Configuration

```ini
[pytest]
markers =
    magneto_cassette: Mark test to use Magnéto-Serge cassette
```

## API Reference

### Markers

#### @pytest.mark.magneto_cassette

Mark tests to use cassettes with automatic management.

**Parameters:**
- `name` (str, optional): Cassette name (auto-generated if not provided)
- `mode` (str, optional): Recording mode (`'auto'`, `'record'`, `'replay'`, `'passthrough'`)
- `record` (str, optional): VCR-compatible mode (`'new_episodes'`, `'once'`, `'all'`, `'none'`)
- `port` (int, optional): Proxy port

**Examples:**

```python
# Auto-generated cassette name
@pytest.mark.magneto_cassette
def test_api(): ...

# Custom cassette name
@pytest.mark.magneto_cassette('my_cassette')
def test_api(): ...

# Replay mode only
@pytest.mark.magneto_cassette('test', mode='replay')
def test_api(): ...

# Force re-record
@pytest.mark.magneto_cassette('test', record='all')
def test_api(): ...
```

### Decorators

#### @magneto_cassette

Decorator for automatic cassette management.

**Parameters:**
- `name` (str, optional): Cassette name
- `mode` (str, optional): Recording mode
- `record` (str, optional): VCR-compatible mode
- `cassette_dir` (str, optional): Override cassette directory
- `port` (int, optional): Proxy port

**Example:**

```python
from pytest_magneto_serge import magneto_cassette

@magneto_cassette('github_api')
def test_github():
    response = requests.get('https://api.github.com/users/octocat')
    assert response.status_code == 200

@magneto_cassette(record='all', cassette_dir='custom/path')
def test_custom():
    # Custom configuration
    pass
```

### Fixtures

#### magneto_proxy

Fixture providing MagnetoProxy instance.

**Usage:**

```python
def test_manual_control(magneto_proxy):
    magneto_proxy.auto('my_cassette')
    # ... make HTTP requests ...
    magneto_proxy.stop()

@pytest.mark.magneto_cassette('auto_cassette')
def test_with_marker(magneto_proxy):
    # magneto_proxy auto-configured from marker
    response = requests.get('https://api.example.com/data')
    assert response.ok
```

#### magneto_config

Global configuration fixture.

**Usage:**

```python
def test_config(magneto_config):
    assert magneto_config.cassette_dir == 'tests/cassettes'
    magneto_config.verbose = True
```

### Context Manager

#### use_cassette

Context manager for cassette lifecycle.

**Parameters:**
- `name` (str): Cassette name
- `mode` (str, optional): Recording mode
- `record` (str, optional): VCR-compatible mode
- `cassette_dir` (str, optional): Override cassette directory

**Usage:**

```python
from pytest_magneto_serge import use_cassette

# As context manager
def test_api():
    with use_cassette('my_cassette'):
        response = requests.get('https://api.example.com/data')

# As decorator
@use_cassette('my_cassette')
def test_api():
    response = requests.get('https://api.example.com/data')

# With options
with use_cassette('test', mode='record'):
    # Always records
    pass
```

## Recording Modes

### Mode Translation (VCR-compatible)

| `record` value | Magneto `mode` | Behavior |
|----------------|----------------|----------|
| `new_episodes` | `auto` | Record new, replay existing (default) |
| `once` | `replay` | Replay only, error if missing |
| `all` | `record` | Always re-record, overwrite existing |
| `none` | `replay` | Replay only, never record |

### Magneto Modes

- **`auto`**: Record if cassette doesn't exist, replay if it does (default)
- **`record`**: Always record, overwrite existing cassette
- **`replay`**: Only replay, error if cassette not found
- **`passthrough`**: Direct connection, no recording or replay

## Examples

### Basic HTTP Recording

```python
import pytest
import requests

@pytest.mark.magneto_cassette('jsonplaceholder_posts')
def test_fetch_posts():
    response = requests.get('https://jsonplaceholder.typicode.com/posts')
    posts = response.json()

    assert response.status_code == 200
    assert len(posts) == 100

@pytest.mark.magneto_cassette('jsonplaceholder_post_1')
def test_fetch_single_post():
    response = requests.get('https://jsonplaceholder.typicode.com/posts/1')
    post = response.json()

    assert response.status_code == 200
    assert post['id'] == 1
    assert post['userId'] == 1
```

### Different HTTP Methods

```python
import pytest
import requests

class TestHTTPMethods:
    @pytest.mark.magneto_cassette
    def test_get_request(self):
        response = requests.get('https://jsonplaceholder.typicode.com/posts/1')
        assert response.status_code == 200

    @pytest.mark.magneto_cassette
    def test_post_request(self):
        data = {'title': 'Test', 'body': 'Content', 'userId': 1}
        response = requests.post('https://jsonplaceholder.typicode.com/posts', json=data)
        assert response.status_code == 201

    @pytest.mark.magneto_cassette
    def test_put_request(self):
        data = {'id': 1, 'title': 'Updated', 'body': 'Updated', 'userId': 1}
        response = requests.put('https://jsonplaceholder.typicode.com/posts/1', json=data)
        assert response.status_code == 200

    @pytest.mark.magneto_cassette
    def test_delete_request(self):
        response = requests.delete('https://jsonplaceholder.typicode.com/posts/1')
        assert response.status_code == 200
```

### Using httpx

```python
import pytest
import httpx

@pytest.mark.magneto_cassette('httpx_test')
async def test_async_httpx():
    async with httpx.AsyncClient() as client:
        response = await client.get('https://jsonplaceholder.typicode.com/users')
        assert response.status_code == 200
        assert len(response.json()) > 0
```

### Recording Modes

```python
import pytest
import requests

# Auto mode (default) - record if missing, replay if exists
@pytest.mark.magneto_cassette('auto_mode')
def test_auto_mode():
    response = requests.get('https://api.example.com/data')
    assert response.ok

# Force re-recording
@pytest.mark.magneto_cassette('force_record', record='all')
def test_force_record():
    response = requests.get('https://api.example.com/live')
    assert response.ok

# Replay only (strict)
@pytest.mark.magneto_cassette('replay_only', record='none')
def test_replay_only():
    response = requests.get('https://api.example.com/cached')
    assert response.ok

# Passthrough (no recording/replay)
@pytest.mark.magneto_cassette('passthrough', mode='passthrough')
def test_passthrough():
    response = requests.get('https://api.example.com/realtime')
    assert response.ok
```

### Manual Control

```python
from pytest_magneto_serge import use_cassette

def test_multiple_cassettes():
    # First cassette
    with use_cassette('users'):
        response = requests.get('https://jsonplaceholder.typicode.com/users/1')
        user = response.json()
        assert user['id'] == 1

    # Second cassette
    with use_cassette('posts'):
        response = requests.get('https://jsonplaceholder.typicode.com/posts/1')
        post = response.json()
        assert post['id'] == 1

    # Third cassette with options
    with use_cassette('comments', record='all'):
        response = requests.get('https://jsonplaceholder.typicode.com/comments/1')
        comment = response.json()
        assert comment['id'] == 1
```

### Error Handling

```python
import pytest
import requests

@pytest.mark.magneto_cassette('404_error')
def test_handles_404():
    response = requests.get('https://jsonplaceholder.typicode.com/posts/999999')
    assert response.status_code == 404

@pytest.mark.magneto_cassette('500_error')
def test_handles_500():
    response = requests.get('https://httpstat.us/500')
    assert response.status_code == 500
```

### Parametrized Tests

```python
import pytest
import requests

@pytest.mark.parametrize('post_id', [1, 2, 3])
@pytest.mark.magneto_cassette
def test_fetch_multiple_posts(post_id):
    # Each parameter gets its own cassette
    response = requests.get(f'https://jsonplaceholder.typicode.com/posts/{post_id}')
    post = response.json()
    assert post['id'] == post_id
```

### Class-Based Tests

```python
import pytest
import requests

class TestGitHubAPI:
    @pytest.mark.magneto_cassette('github_octocat')
    def test_fetch_user(self):
        response = requests.get('https://api.github.com/users/octocat')
        user = response.json()
        assert user['login'] == 'octocat'

    @pytest.mark.magneto_cassette('github_repos')
    def test_fetch_repos(self):
        response = requests.get('https://api.github.com/users/octocat/repos')
        repos = response.json()
        assert isinstance(repos, list)
```

## Configuration Examples

### conftest.py

```python
import pytest

@pytest.fixture(scope='session', autouse=True)
def setup_magneto(magneto_config):
    """Configure Magneto-Serge for all tests"""
    magneto_config.cassette_dir = 'tests/fixtures/cassettes'
    magneto_config.mode = 'auto'
    magneto_config.record_mode = 'new_episodes'
    magneto_config.verbose = False

# Override for specific test module
@pytest.fixture(scope='module')
def api_cassette_dir(magneto_config):
    """Use different directory for API tests"""
    original = magneto_config.cassette_dir
    magneto_config.cassette_dir = 'tests/fixtures/api_cassettes'
    yield
    magneto_config.cassette_dir = original
```

## Comparison with vcrpy

| Feature | vcrpy | pytest-magneto-serge |
|---------|-------|----------------------|
| HTTP Recording | ✅ | ✅ |
| WebSocket Recording | ❌ | ✅ |
| pytest Integration | ✅ | ✅ |
| Fixture Support | ✅ | ✅ |
| Decorator Support | ✅ | ✅ |
| Marker Support | ❌ | ✅ |
| Context Manager | ✅ | ✅ |
| Performance | ~1000 req/s | ~5000+ req/s |
| Multi-language | ❌ | ✅ |

## Requirements

- Python 3.8+
- pytest 6.0+
- magneto-serge 0.2+

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/taciclei/magneto-serge.

## License

MIT License - see LICENSE file for details.
