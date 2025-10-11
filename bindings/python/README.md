# Magneto-Serge Python Bindings

Bindings Python pour **magneto-serge**, une bibliothèque de test HTTP/WebSocket avec capacités d'enregistrement/rejeu.

## 🚀 Installation

### Depuis les Sources

```bash
# 1. Générer les bindings
cd /path/to/magneto-serge
./scripts/generate-python-bindings.sh

# 2. Copier les fichiers dans votre projet
cp bindings/python/magneto_serge.py votre_projet/
cp bindings/python/libuniffi_magneto_serge.{dylib,so} votre_projet/
```

### Dépendances

- Python 3.7+
- Aucune dépendance Python externe requise

## 📖 Usage

### Exemple Basique

```python
import magneto_serge

# Créer un proxy
proxy = magneto_serge.MagnetoProxy("./cassettes")
proxy.set_port(8888)

# Mode enregistrement
proxy.set_mode(magneto_serge.ProxyMode.RECORD)
proxy.start_recording("api-test")

# Configurez votre app pour utiliser le proxy
# HTTP_PROXY=http://localhost:8888
# HTTPS_PROXY=http://localhost:8888

# ... Faites vos appels API ...

# Mode rejeu
proxy.set_mode(magneto_serge.ProxyMode.REPLAY)
proxy.replay("api-test")
```

### Modes Disponibles

#### 1. **Record** - Enregistrement

```python
proxy.set_mode(magneto_serge.ProxyMode.RECORD)
proxy.start_recording("cassette-name")
```

Enregistre toutes les interactions HTTP/WebSocket dans une cassette.

#### 2. **Replay** - Rejeu

```python
proxy.set_mode(magneto_serge.ProxyMode.REPLAY)
proxy.replay("cassette-name")
```

Rejoue les interactions depuis une cassette existante.

#### 3. **Hybrid** - Mode Auto

```python
proxy.hybrid("cassette-name")
```

- Si la cassette existe → rejeu automatique
- Si la cassette n'existe pas → enregistrement automatique

Parfait pour CI/CD !

#### 4. **Replay Strict** - Rejeu Rigoureux

```python
proxy.replay_strict("cassette-name")
```

- Toutes les requêtes doivent correspondre exactement
- Échoue si une requête ne correspond à aucune interaction
- Idéal pour les tests d'intégration

#### 5. **Once** - Rejeu Unique

```python
proxy.once("cassette-name")
```

- Chaque interaction ne peut être rejouée qu'une fois
- Détecte les requêtes dupliquées
- Utile pour tester l'idempotence

## 🧪 Tests

### Exécuter les Tests

```bash
# Tests des bindings
python3 bindings/python/test_magneto_bindings.py

# Exemples d'utilisation
python3 bindings/python/example_magneto.py
```

### Exemple de Test avec pytest

```python
import pytest
import magneto_serge

@pytest.fixture
def proxy():
    proxy = magneto_serge.MagnetoProxy("./test_cassettes")
    proxy.set_port(8888)
    yield proxy
    proxy.shutdown()

def test_api_replay(proxy):
    # Mode rejeu strict pour les tests
    proxy.replay_strict("api-test")

    # Configurez votre client HTTP pour utiliser le proxy
    # Vos appels API seront servis depuis la cassette

    # Assertions...
    assert True

def test_api_record_once(proxy):
    # Mode hybride: enregistre si absent, rejoue sinon
    proxy.hybrid("new-api-test")

    # Premier run: enregistrement
    # Runs suivants: rejeu
```

## 📋 API Reference

### Classe `MagnetoProxy`

#### Constructeur

```python
proxy = MagnetoProxy(cassette_dir: str)
```

- `cassette_dir`: Répertoire de stockage des cassettes

#### Méthodes de Configuration

```python
proxy.set_port(port: int) -> None
proxy.port() -> int

proxy.set_mode(mode: ProxyMode) -> None
proxy.mode() -> ProxyMode
```

#### Méthodes d'Enregistrement/Rejeu

```python
# Enregistrement
proxy.start_recording(cassette_name: str) -> bool

# Rejeu
proxy.replay(cassette_name: str) -> bool
proxy.replay_strict(cassette_name: str) -> bool

# Modes spéciaux
proxy.hybrid(cassette_name: str) -> bool
proxy.once(cassette_name: str) -> bool

# Arrêt
proxy.stop_hybrid() -> bool
proxy.shutdown() -> None
```

### Enum `ProxyMode`

```python
class ProxyMode(enum.Enum):
    RECORD = 0
    REPLAY = 1
    PASSTHROUGH = 2
```

## 🔧 Configuration Proxy

### Variables d'Environnement

```bash
export HTTP_PROXY=http://localhost:8888
export HTTPS_PROXY=http://localhost:8888
export NO_PROXY=localhost,127.0.0.1
```

### Avec `requests` (Python)

```python
import requests
import magneto_serge

# Démarrer le proxy
proxy = magneto_serge.MagnetoProxy("./cassettes")
proxy.set_port(8888)
proxy.replay("api-test")

# Configurer requests
proxies = {
    'http': 'http://localhost:8888',
    'https': 'http://localhost:8888',
}

response = requests.get('https://api.example.com/data', proxies=proxies)
```

### Avec `httpx` (Python)

```python
import httpx
import magneto_serge

proxy = magneto_serge.MagnetoProxy("./cassettes")
proxy.set_port(8888)
proxy.hybrid("api-test")

async with httpx.AsyncClient(proxies="http://localhost:8888") as client:
    response = await client.get("https://api.example.com/data")
```

## 🐛 Troubleshooting

### Erreur: "dlopen ... no such file"

**Cause**: La bibliothèque partagée n'est pas trouvée.

**Solution**:
```bash
# Vérifier que le fichier existe
ls -la bindings/python/libuniffi_magneto_serge.*

# Régénérer si nécessaire
./scripts/generate-python-bindings.sh
```

### Erreur: "ModuleNotFoundError: No module named 'magneto_serge'"

**Cause**: Le module n'est pas dans le PYTHONPATH.

**Solution**:
```python
import sys
import os
sys.path.insert(0, '/path/to/bindings/python')
import magneto_serge
```

### Proxy ne démarre pas

**Cause**: Port déjà utilisé.

**Solution**:
```python
# Utiliser un port différent
proxy.set_port(9999)
```

## 🛠️ Développement

### Régénérer les Bindings

```bash
# Script automatique
./scripts/generate-python-bindings.sh

# Ou manuellement:
cargo build --bin uniffi-bindgen --features uniffi/cli
cargo build --lib --release

./target/debug/uniffi-bindgen generate \
    src/magneto_serge.udl \
    --language python \
    --out-dir bindings/python

cp target/release/libmagneto_serge.{dylib,so} \
   bindings/python/libuniffi_magneto_serge.{dylib,so}
```

### Structure des Fichiers

```
bindings/python/
├── magneto_serge.py              # Bindings générés (52KB)
├── libuniffi_magneto_serge.dylib # Bibliothèque compilée (2.1MB)
├── test_magneto_bindings.py      # Tests unitaires
├── example_magneto.py            # Exemples d'utilisation
└── README.md                     # Cette documentation
```

## 📚 Ressources

- [Documentation principale](../../README.md)
- [Architecture](../../docs/ARCHITECTURE.md)
- [Latency Simulation](../../docs/LATENCY_SIMULATION.md)
- [Docker Support](../../docs/DOCKER.md)
- [Exemples](../../docs/EXAMPLES.md)

## 📄 Licence

MIT OR Apache-2.0
