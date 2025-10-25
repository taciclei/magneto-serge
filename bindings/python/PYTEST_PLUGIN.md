# pytest-magneto-serge Plugin

Plugin pytest officiel pour **magneto-serge** - enregistrez et rejouez les interactions HTTP/WebSocket dans vos tests.

## 🚀 Installation

```bash
# Depuis les sources (développement)
cd bindings/python
pip install pytest

# À venir : PyPI
pip install pytest-magneto-serge
```

## 📖 Usage Basique

### 1. Activer le Plugin

**Option A: conftest.py** (recommandé)
```python
# tests/conftest.py
pytest_plugins = ["pytest_magneto"]
```

**Option B: pytest.ini**
```ini
[pytest]
plugins = pytest_magneto
```

### 2. Utiliser dans les Tests

```python
import pytest
import requests

@pytest.mark.magneto(cassette="api-test")
def test_api_call(magneto):
    """Test avec enregistrement/rejeu automatique."""
    proxies = magneto.proxies()
    response = requests.get("https://api.example.com/users", proxies=proxies)
    assert response.status_code == 200
```

**Première exécution** → Enregistre dans `./test_cassettes/api-test.json`
**Exécutions suivantes** → Rejeu depuis la cassette (aucun appel réseau)

## 🎯 Modes

### Mode Auto (par défaut)

```python
@pytest.mark.magneto(cassette="api-test")
def test_api(magneto):
    # Si cassette existe → replay
    # Si cassette manque → record
    pass
```

### Mode Record (force l'enregistrement)

```bash
# Via ligne de commande
pytest tests/ --magneto-mode=record

# Via marker
@pytest.mark.magneto(cassette="api-test", mode="record")
def test_api(magneto):
    # Force l'enregistrement même si cassette existe
    pass
```

### Mode Replay (rejeu uniquement)

```bash
pytest tests/ --magneto-mode=replay
```

### Mode Strict (échoue si pas de match)

```python
@pytest.mark.magneto(cassette="api-test", strict=True)
def test_api(magneto):
    # Échoue si :
    # - Cassette manquante
    # - Requête non matchée dans la cassette
    pass
```

## 🔧 Configuration

### Options Ligne de Commande

```bash
# Répertoire cassettes
pytest --magneto-cassette-dir=./my_cassettes

# Mode par défaut
pytest --magneto-mode=auto|record|replay|strict

# Port proxy
pytest --magneto-port=9999

# Désactiver magneto
pytest --magneto-disable
```

### Fichier pytest.ini

```ini
[pytest]
addopts =
    --magneto-cassette-dir=./test_cassettes
    --magneto-mode=auto
    --magneto-port=8888

markers =
    magneto: Use magneto-serge proxy for HTTP/WebSocket testing
```

### Paramètres Marker

```python
@pytest.mark.magneto(
    cassette="nom-cassette",   # Nom de la cassette
    mode="auto",               # auto|record|replay
    port=8888,                 # Port du proxy
    strict=False               # Mode strict
)
def test_example(magneto):
    pass
```

## 🧪 Fixtures

### `magneto` (function scope)

Proxy dédié pour un test unique.

```python
def test_api(magneto):
    """Chaque test a son propre proxy."""
    proxies = magneto.proxies()
    port = magneto.port()
```

### `magneto_session` (session scope)

Proxy partagé pour toute la session de tests.

```python
def test_api_1(magneto_session):
    """Proxy démarré une fois."""
    pass

def test_api_2(magneto_session):
    """Réutilise le même proxy."""
    pass
```

## 📚 Exemples

### Tests API REST

```python
import pytest
import requests

@pytest.mark.magneto(cassette="github-api")
def test_github_api(magneto):
    """Test API GitHub."""
    response = requests.get(
        "https://api.github.com/repos/taciclei/magneto-serge",
        proxies=magneto.proxies()
    )
    assert response.status_code == 200
    data = response.json()
    assert "name" in data
```

### Tests POST avec JSON

```python
@pytest.mark.magneto(cassette="api-create-user")
def test_create_user(magneto):
    """Test création utilisateur."""
    user_data = {"name": "Alice", "email": "alice@example.com"}
    response = requests.post(
        "https://api.example.com/users",
        json=user_data,
        proxies=magneto.proxies()
    )
    assert response.status_code == 201
```

### Tests avec httpx (async)

```python
import pytest
import httpx

@pytest.mark.magneto(cassette="async-api")
@pytest.mark.asyncio
async def test_async_api(magneto):
    """Test avec httpx asynchrone."""
    proxies = magneto.proxies()
    async with httpx.AsyncClient(proxies=proxies) as client:
        response = await client.get("https://api.example.com/data")
        assert response.status_code == 200
```

### Tests paramétrés

```python
@pytest.mark.magneto(cassette="users-api")
@pytest.mark.parametrize("user_id", [1, 2, 3])
def test_get_users(magneto, user_id):
    """Test avec plusieurs utilisateurs."""
    response = requests.get(
        f"https://api.example.com/users/{user_id}",
        proxies=magneto.proxies()
    )
    assert response.status_code == 200
```

### Fixture customisée

```python
@pytest.fixture
def api_client(magneto):
    """Client API configuré avec proxy."""
    session = requests.Session()
    session.proxies = magneto.proxies()
    session.headers.update({"Authorization": "Bearer token"})
    yield session
    session.close()

def test_with_client(api_client):
    """Test avec client personnalisé."""
    response = api_client.get("https://api.example.com/protected")
    assert response.status_code == 200
```

### Nom cassette automatique

```python
def test_automatic_naming(magneto):
    """
    Pas de cassette spécifiée → nom généré depuis le test.
    Cassette: test_automatic_naming.json
    """
    response = requests.get(
        "https://api.example.com",
        proxies=magneto.proxies()
    )
    assert response.ok
```

## 🔄 Workflow Typique

### 1. Développement (premier run)

```bash
# Enregistre toutes les interactions
pytest tests/ --magneto-mode=record -v
```

Cassettes créées dans `./test_cassettes/`

### 2. CI/CD (tests rapides)

```bash
# Rejeu uniquement, échoue si cassette manquante
pytest tests/ --magneto-mode=strict -v
```

Aucun appel réseau → tests ultra-rapides ⚡

### 3. Mise à jour API

```bash
# Ré-enregistre une cassette spécifique
pytest tests/test_api.py::test_users --magneto-mode=record -v
```

### 4. Debugging

```bash
# Mode auto pour développement
pytest tests/test_api.py -v

# Désactiver magneto temporairement
pytest tests/ --magneto-disable
```

## ⚙️ API Reference

### Fixture `magneto`

**Méthodes**:
- `magneto.port()` → Port du proxy (int)
- `magneto.mode()` → Mode actuel (ProxyMode)
- `magneto.proxies()` → Dict pour requests/httpx
  ```python
  {
      "http": "http://localhost:8888",
      "https": "http://localhost:8888"
  }
  ```

### Marker `@pytest.mark.magneto`

**Paramètres**:
- `cassette` (str, optionnel) - Nom de la cassette
- `mode` (str, optionnel) - Mode: "auto", "record", "replay"
- `port` (int, optionnel) - Port du proxy
- `strict` (bool, optionnel) - Mode strict (échec si pas de match)

## 🐛 Troubleshooting

### Erreur: "magneto-serge not installed"

```bash
# Installer les bindings Python
cd bindings/python
python -m pip install -e .
```

### Proxy ne démarre pas

```python
# Vérifier que le port n'est pas utilisé
@pytest.mark.magneto(port=9999)  # Utiliser un autre port
def test_api(magneto):
    pass
```

### Cassettes non créées

```bash
# Vérifier le répertoire
ls -la test_cassettes/

# Forcer mode record
pytest --magneto-mode=record -v
```

### Mode strict échoue

```bash
# Vérifier que la cassette existe
ls test_cassettes/nom-cassette.json

# Ou passer en mode auto
pytest --magneto-mode=auto
```

## 🎓 Best Practices

### 1. Une cassette par test

```python
@pytest.mark.magneto(cassette="specific-test")
def test_specific_api(magneto):
    # Cassette dédiée = meilleure isolation
    pass
```

### 2. Gitignorer les cassettes sensibles

```gitignore
# .gitignore
test_cassettes/*-secret.json
test_cassettes/*-auth.json
```

### 3. Versionner les cassettes stables

```bash
git add test_cassettes/stable-api-*.json
git commit -m "test: add stable API cassettes"
```

### 4. CI/CD en mode strict

```yaml
# .github/workflows/test.yml
- name: Run tests
  run: pytest --magneto-mode=strict --cov
```

### 5. Documentation des cassettes

```python
@pytest.mark.magneto(cassette="github-api-v3")
def test_github_api(magneto):
    """
    Test API GitHub v3.

    Cassette: github-api-v3.json
    Enregistré: 2025-10-11
    Endpoint: https://api.github.com/repos/...
    """
    pass
```

## 🔗 Ressources

- [Documentation magneto-serge](../../README.md)
- [Bindings Python](./README.md)
- [Examples](./example_magneto.py)
- [Tests](./test_magneto_bindings.py)

## 📄 Licence

MIT OR Apache-2.0
