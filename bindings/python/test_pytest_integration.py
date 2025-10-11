"""
Tests d'intégration pytest avec magneto-serge

Pour exécuter :
    pytest test_pytest_integration.py -v
    pytest test_pytest_integration.py --magneto-mode=record -v
    pytest test_pytest_integration.py --magneto-mode=replay -v
"""

import pytest
import sys
import os

# Add current directory to path
sys.path.insert(0, os.path.dirname(__file__))

# Import the plugin
pytest_plugins = ["pytest_magneto"]


class TestMagnetoBasic:
    """Tests basiques sans utiliser le plugin."""

    def test_proxy_creation(self):
        """Test création proxy simple."""
        from magneto_serge import MagnetoProxy

        proxy = MagnetoProxy("./test_cassettes")
        assert proxy is not None

        # Test methods
        proxy.set_port(9999)
        assert proxy.port() == 9999


class TestMagnetoPlugin:
    """Tests utilisant le plugin pytest."""

    @pytest.mark.magneto(cassette="test-api-get")
    def test_with_marker(self, magneto):
        """Test avec marker magneto - cassette explicite."""
        assert magneto is not None
        assert magneto.port() == 8888

        # Le proxy devrait être démarré en mode auto
        proxies = magneto.proxies()
        assert "http" in proxies
        assert "https" in proxies

    @pytest.mark.magneto(cassette="test-api-post", port=9000)
    def test_with_custom_port(self, magneto):
        """Test avec port personnalisé."""
        assert magneto is not None
        assert magneto.port() == 9000

    @pytest.mark.magneto(cassette="test-api-strict", strict=True)
    def test_strict_mode(self, magneto):
        """Test en mode strict."""
        assert magneto is not None
        # En mode strict, le proxy échouera si la cassette n'existe pas
        # ou si une requête ne correspond pas

    def test_auto_cassette_name(self, magneto):
        """
        Test sans cassette explicite - nom généré depuis le test.

        Cassette générée : test_auto_cassette_name
        """
        assert magneto is not None

    @pytest.mark.magneto(cassette="test-parametrized", mode="record")
    @pytest.mark.parametrize("value", [1, 2, 3])
    def test_parametrized(self, magneto, value):
        """Test paramétrisé avec magneto."""
        assert magneto is not None
        assert value in [1, 2, 3]
        # Chaque exécution utilisera la même cassette


class TestMagnetoSession:
    """Tests utilisant magneto_session (session scope)."""

    def test_session_1(self, magneto_session):
        """Premier test avec proxy session."""
        assert magneto_session is not None
        proxies = magneto_session.proxies()
        assert proxies["http"] == "http://localhost:8888"

    def test_session_2(self, magneto_session):
        """Deuxième test - même proxy."""
        assert magneto_session is not None
        # Réutilise la même instance de proxy


class TestRealHTTP:
    """
    Tests HTTP réels (désactivés par défaut, nécessitent réseau).

    Pour activer : pytest test_pytest_integration.py -k TestRealHTTP -v
    """

    @pytest.mark.magneto(cassette="httpbin-get", mode="auto")
    @pytest.mark.skip(reason="Nécessite réseau et requests library")
    def test_httpbin_get(self, magneto):
        """Test GET réel avec httpbin.org."""
        import requests

        proxies = magneto.proxies()
        response = requests.get("https://httpbin.org/get", proxies=proxies)
        assert response.status_code == 200
        assert "headers" in response.json()

    @pytest.mark.magneto(cassette="httpbin-post", mode="auto")
    @pytest.mark.skip(reason="Nécessite réseau et requests library")
    def test_httpbin_post(self, magneto):
        """Test POST réel avec httpbin.org."""
        import requests

        proxies = magneto.proxies()
        data = {"key": "value", "test": True}
        response = requests.post(
            "https://httpbin.org/post", json=data, proxies=proxies
        )
        assert response.status_code == 200
        json_response = response.json()
        assert json_response["json"] == data


# Tests de configuration du plugin

def test_plugin_loaded():
    """Vérifie que le plugin est chargé."""
    import pytest_magneto
    assert hasattr(pytest_magneto, "pytest_addoption")
    assert hasattr(pytest_magneto, "pytest_configure")


def test_marker_exists(pytestconfig):
    """Vérifie que le marker magneto existe."""
    markers = pytestconfig.getini("markers")
    magneto_marker = [m for m in markers if "magneto" in m.lower()]
    assert len(magneto_marker) > 0


# Documentation examples

"""
## Usage Examples

### 1. Mode Auto (par défaut)

```python
@pytest.mark.magneto(cassette="api-test")
def test_api(magneto):
    # Si cassette existe → replay
    # Si cassette manque → record
    proxies = magneto.proxies()
    response = requests.get("https://api.example.com", proxies=proxies)
    assert response.status_code == 200
```

### 2. Mode Record (force enregistrement)

```bash
pytest test_pytest_integration.py --magneto-mode=record
```

```python
@pytest.mark.magneto(cassette="api-test", mode="record")
def test_api(magneto):
    # Force l'enregistrement même si cassette existe
    pass
```

### 3. Mode Replay (strict)

```python
@pytest.mark.magneto(cassette="api-test", strict=True)
def test_api(magneto):
    # Mode strict : échoue si cassette manquante ou requête non matchée
    pass
```

### 4. Session Scope (partagé)

```python
def test_api_1(magneto_session):
    # Proxy démarré une seule fois pour toute la session
    pass

def test_api_2(magneto_session):
    # Réutilise le même proxy
    pass
```

### 5. Désactiver magneto

```bash
pytest test_pytest_integration.py --magneto-disable
```

### 6. Configuration pytest.ini

```ini
[pytest]
addopts =
    --magneto-cassette-dir=./my_cassettes
    --magneto-mode=auto
    --magneto-port=9999
```

### 7. Fixture customisée

```python
@pytest.fixture
def api_client(magneto):
    import requests
    session = requests.Session()
    session.proxies = magneto.proxies()
    yield session
    session.close()

def test_with_client(api_client):
    response = api_client.get("https://api.example.com")
    assert response.ok
```
"""
