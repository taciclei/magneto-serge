# Migration Guide

Guide de migration pour Magneto-Serge.

## Table des Matières

- [Migration vers v0.1.0](#migration-vers-v010)
- [Migration depuis VCR (Ruby)](#migration-depuis-vcr-ruby)
- [Migration depuis Polly.JS (Node.js)](#migration-depuis-pollyjs-nodejs)
- [Migration depuis Betamax (Python)](#migration-depuis-betamax-python)
- [Migration depuis WireMock (Java)](#migration-depuis-wiremock-java)

---

## Migration vers v0.1.0

### Depuis matgto-serge (nom précédent)

Si vous utilisiez une version pré-release nommée `matgto-serge`, voici les changements:

#### Renommage du projet

Le projet a été renommé de **matgto-serge** à **magneto-serge**.

**Avant**:
```rust
use matgto_serge::{MatgtoProxy, ProxyMode};

let proxy = MatgtoProxy::new("./cassettes");
```

**Après**:
```rust
use magneto_serge::{MagnetoProxy, ProxyMode};

let proxy = MagnetoProxy::new("./cassettes");
```

#### CLI

**Avant**:
```bash
matgto record my-cassette
matgto replay my-cassette
```

**Après**:
```bash
magneto record my-cassette
magneto replay my-cassette
```

#### Configuration

**Avant** (`matgto.toml`):
```toml
[matgto]
cassette_dir = "./cassettes"
```

**Après** (`magneto.toml`):
```toml
[magneto]
cassette_dir = "./cassettes"
```

#### Variables d'environnement

**Avant**:
```bash
MATGTO_MODE=record
MATGTO_CASSETTE_DIR=./cassettes
MATGTO_PROXY_PORT=8888
```

**Après**:
```bash
MAGNETO_MODE=record
MAGNETO_CASSETTE_DIR=./cassettes
MAGNETO_PROXY_PORT=8888
```

#### Certificats CA

**Avant**: `matgto-ca.pem`
**Après**: `magneto-ca.pem`

Régénérez le certificat CA:
```bash
rm -f matgto-ca.pem
magneto init
```

---

## Migration depuis VCR (Ruby)

[VCR](https://github.com/vcr/vcr) est une bibliothèque Ruby populaire pour record/replay HTTP.

### Concepts Équivalents

| VCR (Ruby) | Magneto-Serge | Notes |
|------------|---------------|-------|
| `VCR.use_cassette` | `MagnetoProxy::hybrid()` | Mode automatique |
| `:record => :new_episodes` | `ProxyMode::Hybrid` | Record si absent |
| `:record => :all` | `ProxyMode::Record` | Force record |
| `:record => :none` | `ProxyMode::Replay` | Replay uniquement |
| `:record => :once` | `ProxyMode::Once` | Record une fois |
| `cassette_library_dir` | `cassette_dir` | Répertoire cassettes |
| `match_requests_on` | Matching automatique | Méthode + URL + body |

### Exemple de Migration

**Avant (VCR + Ruby)**:
```ruby
VCR.configure do |c|
  c.cassette_library_dir = 'spec/cassettes'
  c.hook_into :webmock
end

VCR.use_cassette('github_api') do
  response = HTTParty.get('https://api.github.com/users')
  expect(response.code).to eq(200)
end
```

**Après (Magneto + Python)**:
```python
import pytest
import requests

@pytest.mark.magneto(cassette='github_api')
def test_github_api(magneto):
    response = requests.get(
        'https://api.github.com/users',
        proxies=magneto.proxies()
    )
    assert response.status_code == 200
```

**Après (Magneto + Rust)**:
```rust
use magneto_serge::{MagnetoProxy, ProxyMode};

#[tokio::test]
async fn test_github_api() {
    let mut proxy = MagnetoProxy::new("./test_cassettes");
    proxy.set_port(8888);
    proxy.hybrid("github_api");

    // Configure HTTP client to use proxy
    let response = reqwest::get("https://api.github.com/users").await?;
    assert_eq!(response.status(), 200);

    proxy.shutdown();
}
```

### Différences Clés

1. **Architecture**: VCR intercepte directement les bibliothèques HTTP (monkeypatching), Magneto utilise un proxy HTTP
2. **Performance**: Magneto est ~10-100x plus rapide (écrit en Rust)
3. **Multi-langage**: Magneto supporte 5+ langages via UniFFI
4. **WebSocket**: Magneto supporte WebSocket, VCR non

---

## Migration depuis Polly.JS (Node.js)

[Polly.JS](https://netflix.github.io/pollyjs) est une bibliothèque JavaScript pour record/replay HTTP.

### Concepts Équivalents

| Polly.JS | Magneto-Serge | Notes |
|----------|---------------|-------|
| `polly.record()` | `MagnetoProxy::start_recording()` | Enregistrement |
| `polly.replay()` | `MagnetoProxy::replay()` | Rejeu |
| `polly.passthrough()` | `ProxyMode::Passthrough` | Passthrough |
| `recordingsDir` | `cassette_dir` | Répertoire cassettes |
| `matchRequestsBy` | Matching automatique | Méthode + URL + body |
| `persister` | Format cassette | JSON ou MessagePack |

### Exemple de Migration

**Avant (Polly.JS)**:
```javascript
import { Polly } from '@pollyjs/core';
import NodeHttpAdapter from '@pollyjs/adapter-node-http';
import FSPersister from '@pollyjs/persister-fs';

Polly.register(NodeHttpAdapter);
Polly.register(FSPersister);

const polly = new Polly('github-api', {
  adapters: ['node-http'],
  persister: 'fs',
  recordingsDir: './recordings',
});

const response = await fetch('https://api.github.com/users');
expect(response.status).toBe(200);

await polly.stop();
```

**Après (Magneto + Jest)**:
```javascript
const { useMagneto, getProxyConfig } = require('magneto-serge/jest-magneto');
const axios = require('axios');

test('github api', async () => {
  const magneto = useMagneto('github-api');

  const response = await axios.get(
    'https://api.github.com/users',
    { proxy: getProxyConfig(magneto) }
  );

  expect(response.status).toBe(200);
  magneto.shutdown();
});
```

### Différences Clés

1. **Architecture**: Polly intercepte via adapters, Magneto utilise un proxy HTTP
2. **Performance**: Magneto est ~5-10x plus rapide (core Rust)
3. **Modes**: Magneto a des modes supplémentaires (Hybrid, Once, Strict)
4. **WebSocket**: Magneto supporte WebSocket nativement

---

## Migration depuis Betamax (Python)

[Betamax](https://github.com/betamaxpy/betamax) est une bibliothèque Python pour record/replay HTTP.

### Concepts Équivalents

| Betamax | Magneto-Serge | Notes |
|---------|---------------|-------|
| `@betamax.use_cassette` | `@pytest.mark.magneto` | Décorateur pytest |
| `record='new_episodes'` | `mode='auto'` | Mode automatique |
| `record='all'` | `mode='record'` | Force record |
| `record='none'` | `mode='replay'` | Replay uniquement |
| `record='once'` | `mode='once'` | Record une fois |
| `cassette_library_dir` | `cassetteDir` | Répertoire cassettes |
| `match_requests_on` | Matching automatique | Méthode + URL + body |

### Exemple de Migration

**Avant (Betamax)**:
```python
import betamax
import requests

with betamax.Betamax.configure() as config:
    config.cassette_library_dir = 'tests/cassettes'

@betamax.use_cassette('github_api')
def test_github_api(session):
    response = session.get('https://api.github.com/users')
    assert response.status_code == 200
```

**Après (Magneto + pytest)**:
```python
import pytest
import requests

@pytest.mark.magneto(cassette='github_api')
def test_github_api(magneto):
    response = requests.get(
        'https://api.github.com/users',
        proxies=magneto.proxies()
    )
    assert response.status_code == 200
```

### Différences Clés

1. **Architecture**: Betamax patche requests, Magneto utilise un proxy HTTP
2. **Performance**: Magneto est ~10-50x plus rapide (core Rust)
3. **Multi-bibliothèques**: Magneto fonctionne avec requests, httpx, urllib, etc.
4. **WebSocket**: Magneto supporte WebSocket, Betamax non

---

## Migration depuis WireMock (Java)

[WireMock](https://wiremock.org/) est une bibliothèque Java pour stubbing et mocking HTTP.

### Concepts Équivalents

| WireMock | Magneto-Serge | Notes |
|----------|---------------|-------|
| `stubFor()` | `MagnetoProxy::start_recording()` | Enregistrement |
| `WireMockServer` | `MagnetoProxy` | Serveur proxy |
| `recordSpec()` | `ProxyMode::Record` | Enregistrement |
| Mappings JSON | Cassette JSON | Format cassette |
| `matching()` | Matching automatique | Méthode + URL + body |

### Exemple de Migration

**Avant (WireMock)**:
```java
import com.github.tomakehurst.wiremock.WireMockServer;
import static com.github.tomakehurst.wiremock.client.WireMock.*;

WireMockServer wireMockServer = new WireMockServer(8080);
wireMockServer.start();

stubFor(get(urlEqualTo("/users"))
    .willReturn(aResponse()
        .withStatus(200)
        .withBody("{\"users\": []}")));

// Test code
String response = httpClient.get("http://localhost:8080/users");
assertEquals(200, response.statusCode());

wireMockServer.stop();
```

**Après (Magneto + JUnit 5)**:
```java
import com.magneto.MagnetoProxy;
import com.magneto.junit.MagnetoExtension;
import com.magneto.junit.MagnetoExtension.Magneto;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "users-api")
class UsersApiTest {

    @Test
    void testGetUsers(MagnetoProxy magneto) {
        // Configure HTTP client to use proxy
        OkHttpClient client = new OkHttpClient.Builder()
            .proxy(new Proxy(Proxy.Type.HTTP,
                new InetSocketAddress("localhost", magneto.port())))
            .build();

        Response response = client.newCall(
            new Request.Builder()
                .url("https://api.example.com/users")
                .build()
        ).execute();

        assertEquals(200, response.code());
    }
}
```

### Différences Clés

1. **Approche**: WireMock est un stub/mock, Magneto est un proxy record/replay
2. **Stubs manuels**: WireMock nécessite des stubs manuels, Magneto enregistre automatiquement
3. **WebSocket**: Magneto supporte WebSocket, WireMock non (sauf extension)
4. **Multi-langage**: Magneto supporte 5+ langages, WireMock est JVM-only

---

## Checklist de Migration

### Préparation

- [ ] Lire la documentation Magneto-Serge
- [ ] Identifier les cassettes/recordings existants
- [ ] Choisir le binding approprié (Python, Kotlin, Swift, Java, JavaScript)
- [ ] Vérifier la compatibilité des versions

### Migration du Code

- [ ] Installer Magneto-Serge dans votre projet
- [ ] Configurer le proxy (port, cassette_dir)
- [ ] Remplacer les appels VCR/Polly/Betamax/WireMock par Magneto
- [ ] Configurer les clients HTTP pour utiliser le proxy
- [ ] Adapter les tests

### Migration des Cassettes

- [ ] Convertir les cassettes existantes au format Magneto (si possible)
- [ ] Ou ré-enregistrer les cassettes avec Magneto
- [ ] Vérifier que les cassettes fonctionnent en mode replay
- [ ] Committer les nouvelles cassettes

### Tests

- [ ] Exécuter les tests en mode record
- [ ] Exécuter les tests en mode replay
- [ ] Vérifier que tous les tests passent
- [ ] Vérifier les performances (temps d'exécution)

### CI/CD

- [ ] Configurer CI pour mode strict (rejeu uniquement)
- [ ] Ajouter les cassettes au version control (si approprié)
- [ ] Documenter le workflow de mise à jour des cassettes

### Documentation

- [ ] Documenter l'utilisation de Magneto dans le projet
- [ ] Mettre à jour le README avec les instructions
- [ ] Former l'équipe sur Magneto

---

## Support

Si vous avez des questions ou problèmes pendant la migration:

- **Issues**: https://github.com/taciclei/magneto-serge/issues
- **Discussions**: https://github.com/taciclei/magneto-serge/discussions
- **Documentation**: https://github.com/taciclei/magneto-serge/tree/main/docs

---

## Ressources

- [Documentation complète](README.md)
- [Examples](docs/EXAMPLES.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)

---

🦀 Built with Rust for maximum performance and safety
