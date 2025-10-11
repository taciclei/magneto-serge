# Exemples d'Utilisation - matgto-serge

**Version:** 1.0
**Date:** 2025-10-10

Ce document contient des exemples complets d'utilisation de matgto-serge dans différents langages et frameworks.

---

## 📋 Table des Matières

1. [Java / Spring Boot](#java--spring-boot)
2. [JavaScript / Node.js](#javascript--nodejs)
3. [Python / pytest](#python--pytest)
4. [Ruby / RSpec](#ruby--rspec)
5. [Kotlin / Android](#kotlin--android)
6. [TypeScript / React](#typescript--react)
7. [Go (Third-Party)](#go-third-party)
8. [Configuration Avancée](#configuration-avancée)

---

## ☕ Java / Spring Boot

### Installation

```gradle
// build.gradle
dependencies {
    testImplementation 'com.magneto:serge:1.0.0'
    testImplementation 'org.junit.jupiter:junit-jupiter:5.10.0'
    testImplementation 'org.springframework.boot:spring-boot-starter-test'
}
```

### Exemple 1 : Test REST API Simple

```java
package com.example.tests;

import com.magneto.serge.MagnetoProxy;
import com.magneto.serge.ProxyMode;
import org.junit.jupiter.api.*;
import org.springframework.web.client.RestTemplate;

import java.net.URI;

class UserApiTest {
    private static MagnetoProxy proxy;
    private RestTemplate restTemplate;

    @BeforeAll
    static void setupProxy() {
        proxy = new MagnetoProxy("./cassettes")
            .withPort(8888)
            .withMode(ProxyMode.AUTO);
    }

    @BeforeEach
    void setupClient() {
        // Configurer RestTemplate pour utiliser le proxy
        restTemplate = new RestTemplate();
        System.setProperty("http.proxyHost", "localhost");
        System.setProperty("http.proxyPort", "8888");
        System.setProperty("https.proxyHost", "localhost");
        System.setProperty("https.proxyPort", "8888");
    }

    @Test
    @DisplayName("Should fetch user list from API")
    void testGetUsers() throws Exception {
        // Démarrer enregistrement
        proxy.startRecording("get-users");

        // Appel API réel (ou depuis cassette si elle existe)
        String response = restTemplate.getForObject(
            "https://api.example.com/users",
            String.class
        );

        // Assertions
        assertNotNull(response);
        assertTrue(response.contains("\"users\""));

        // Arrêter enregistrement
        proxy.stopRecording();
    }

    @AfterAll
    static void teardownProxy() {
        proxy.shutdown();
    }
}
```

### Exemple 2 : Extension JUnit 5 (Plus Élégant)

```java
// MatgtoExtension.java
package com.magneto.serge.junit;

import com.magneto.serge.MagnetoProxy;
import org.junit.jupiter.api.extension.*;

public class MatgtoExtension implements BeforeAllCallback, AfterAllCallback, BeforeEachCallback, AfterEachCallback {
    private static final ExtensionContext.Namespace NAMESPACE =
        ExtensionContext.Namespace.create(MatgtoExtension.class);

    private static MagnetoProxy proxy;

    @Override
    public void beforeAll(ExtensionContext context) {
        proxy = new MagnetoProxy("./cassettes")
            .withPort(8888)
            .withMode(ProxyMode.AUTO);

        context.getStore(NAMESPACE).put("proxy", proxy);
    }

    @Override
    public void beforeEach(ExtensionContext context) throws Exception {
        Matgto annotation = context.getRequiredTestMethod()
            .getAnnotation(Matgto.class);

        if (annotation != null) {
            String cassetteName = annotation.cassette().isEmpty()
                ? context.getRequiredTestMethod().getName()
                : annotation.cassette();

            proxy.startRecording(cassetteName);
        }
    }

    @Override
    public void afterEach(ExtensionContext context) throws Exception {
        proxy.stopRecording();
    }

    @Override
    public void afterAll(ExtensionContext context) {
        proxy.shutdown();
    }
}

// Annotation personnalisée
@Target(ElementType.METHOD)
@Retention(RetentionPolicy.RUNTIME)
public @interface Matgto {
    String cassette() default "";
}

// Utilisation
@ExtendWith(MatgtoExtension.class)
class UserApiTest {
    private RestTemplate restTemplate = new RestTemplate();

    @Test
    @Matgto(cassette = "get-users")
    void testGetUsers() {
        String response = restTemplate.getForObject(
            "https://api.example.com/users",
            String.class
        );

        assertNotNull(response);
    }

    @Test
    @Matgto  // Utilise le nom de la méthode comme cassette
    void testCreateUser() {
        // Test automatiquement enregistré dans "testCreateUser.json"
    }
}
```

### Exemple 3 : WebSocket avec Spring

```java
package com.example.tests;

import com.magneto.serge.MagnetoProxy;
import org.junit.jupiter.api.*;
import org.springframework.web.socket.WebSocketSession;
import org.springframework.web.socket.client.standard.StandardWebSocketClient;
import org.springframework.web.socket.handler.TextWebSocketHandler;
import org.springframework.web.socket.TextMessage;

import java.util.concurrent.*;

@ExtendWith(MatgtoExtension.class)
class WebSocketTest {
    private static MagnetoProxy proxy;

    @BeforeAll
    static void setup() {
        proxy = new MagnetoProxy("./cassettes").withPort(8888);
    }

    @Test
    @Matgto(cassette = "websocket-notifications")
    void testWebSocketMessages() throws Exception {
        CountDownLatch latch = new CountDownLatch(3);
        List<String> receivedMessages = new CopyOnWriteArrayList<>();

        StandardWebSocketClient client = new StandardWebSocketClient();
        WebSocketSession session = client.doHandshake(
            new TextWebSocketHandler() {
                @Override
                protected void handleTextMessage(WebSocketSession session, TextMessage message) {
                    receivedMessages.add(message.getPayload());
                    latch.countDown();
                }
            },
            "ws://localhost:8888/notifications"  // Via proxy
        ).get(5, TimeUnit.SECONDS);

        // Envoyer message
        session.sendMessage(new TextMessage("{\"action\":\"subscribe\"}"));

        // Attendre 3 messages
        assertTrue(latch.await(10, TimeUnit.SECONDS));

        assertEquals(3, receivedMessages.size());
        assertTrue(receivedMessages.get(0).contains("connected"));
    }
}
```

---

## 🟨 JavaScript / Node.js

### Installation

```bash
npm install --save-dev @magneto/serge
# ou
yarn add --dev @magneto/serge
```

### Exemple 1 : Tests avec Jest

```javascript
// users.test.js
const { MagnetoProxy } = require('@magneto/serge');
const axios = require('axios');

describe('User API Tests', () => {
  let proxy;

  beforeAll(() => {
    proxy = new MagnetoProxy('./cassettes');
    proxy.withPort(8888);
  });

  afterAll(() => {
    proxy.shutdown();
  });

  test('should fetch users list', async () => {
    proxy.startRecording('get-users');

    // Configurer axios pour utiliser le proxy
    const response = await axios.get('https://api.example.com/users', {
      proxy: {
        host: 'localhost',
        port: 8888,
      },
    });

    expect(response.status).toBe(200);
    expect(response.data).toHaveProperty('users');

    proxy.stopRecording();
  });

  test('should create new user', async () => {
    proxy.startRecording('create-user');

    const newUser = {
      name: 'John Doe',
      email: 'john@example.com',
    };

    const response = await axios.post(
      'https://api.example.com/users',
      newUser,
      {
        proxy: { host: 'localhost', port: 8888 },
      }
    );

    expect(response.status).toBe(201);
    expect(response.data.name).toBe('John Doe');

    proxy.stopRecording();
  });
});
```

### Exemple 2 : Plugin Jest Custom

```javascript
// jest-matgto-plugin.js
const { MagnetoProxy } = require('@magneto/serge');

let proxy;

module.exports = {
  async setup() {
    proxy = new MagnetoProxy('./cassettes');
    proxy.withPort(8888);
    global.__MATGTO_PROXY__ = proxy;
  },

  async teardown() {
    if (proxy) {
      proxy.shutdown();
    }
  },

  // Helper pour tests
  withCassette(cassetteName, testFn) {
    return async () => {
      proxy.startRecording(cassetteName);
      try {
        await testFn();
      } finally {
        proxy.stopRecording();
      }
    };
  },
};

// jest.config.js
module.exports = {
  setupFilesAfterEnv: ['./jest-matgto-plugin.js'],
};

// Utilisation dans tests
const { withCassette } = require('./jest-matgto-plugin');

test('fetch users', withCassette('get-users', async () => {
  const response = await fetch('https://api.example.com/users');
  expect(response.ok).toBe(true);
}));
```

### Exemple 3 : WebSocket avec ws library

```javascript
// websocket.test.js
const { MagnetoProxy } = require('@magneto/serge');
const WebSocket = require('ws');

describe('WebSocket Tests', () => {
  let proxy;

  beforeAll(() => {
    proxy = new MagnetoProxy('./cassettes').withPort(8888);
  });

  afterAll(() => proxy.shutdown());

  test('should receive notifications via WebSocket', (done) => {
    proxy.startRecording('websocket-notifications');

    // Connexion via proxy
    const ws = new WebSocket('ws://api.example.com/notifications', {
      agent: new HttpsProxyAgent('http://localhost:8888'),
    });

    const messages = [];

    ws.on('open', () => {
      ws.send(JSON.stringify({ action: 'subscribe' }));
    });

    ws.on('message', (data) => {
      messages.push(JSON.parse(data));

      if (messages.length === 3) {
        expect(messages[0]).toHaveProperty('event', 'connected');
        proxy.stopRecording();
        ws.close();
        done();
      }
    });

    ws.on('error', done);
  });
});
```

### Exemple 4 : TypeScript + Vitest

```typescript
// users.test.ts
import { describe, test, expect, beforeAll, afterAll } from 'vitest';
import { MagnetoProxy, ProxyMode } from '@magneto/serge';
import axios from 'axios';

describe('User API with TypeScript', () => {
  let proxy: MagnetoProxy;

  beforeAll(() => {
    proxy = new MagnetoProxy('./cassettes')
      .withPort(8888)
      .withMode(ProxyMode.Auto);
  });

  afterAll(() => {
    proxy.shutdown();
  });

  test('should fetch users with type safety', async () => {
    proxy.startRecording('get-users-typed');

    interface User {
      id: number;
      name: string;
      email: string;
    }

    interface UsersResponse {
      users: User[];
      total: number;
    }

    const { data } = await axios.get<UsersResponse>(
      'https://api.example.com/users',
      {
        proxy: { host: 'localhost', port: 8888 },
      }
    );

    expect(data.users).toBeInstanceOf(Array);
    expect(data.total).toBeGreaterThan(0);

    proxy.stopRecording();
  });
});
```

---

## 🐍 Python / pytest

### Installation

```bash
pip install matgto-serge
# ou
poetry add --group dev matgto-serge
```

### Exemple 1 : Tests pytest basiques

```python
# test_users.py
import pytest
import requests
from matgto_serge import MagnetoProxy, ProxyMode

@pytest.fixture(scope="module")
def matgto_proxy():
    """Fixture pytest pour MagnetoProxy"""
    proxy = MagnetoProxy(cassette_dir="./cassettes")
    proxy.with_port(8888)
    proxy.with_mode(ProxyMode.AUTO)

    yield proxy

    proxy.shutdown()

@pytest.fixture
def proxied_session():
    """Session requests configurée pour le proxy"""
    session = requests.Session()
    session.proxies = {
        'http': 'http://localhost:8888',
        'https': 'http://localhost:8888',
    }
    return session

def test_get_users(matgto_proxy, proxied_session):
    """Test récupération liste utilisateurs"""
    matgto_proxy.start_recording("get-users")

    response = proxied_session.get("https://api.example.com/users")

    assert response.status_code == 200
    data = response.json()
    assert "users" in data
    assert len(data["users"]) > 0

    matgto_proxy.stop_recording()

def test_create_user(matgto_proxy, proxied_session):
    """Test création utilisateur"""
    matgto_proxy.start_recording("create-user")

    new_user = {
        "name": "Jane Doe",
        "email": "jane@example.com"
    }

    response = proxied_session.post(
        "https://api.example.com/users",
        json=new_user
    )

    assert response.status_code == 201
    assert response.json()["name"] == "Jane Doe"

    matgto_proxy.stop_recording()
```

### Exemple 2 : Plugin pytest personnalisé

```python
# conftest.py
import pytest
from matgto_serge import MagnetoProxy, ProxyMode

def pytest_configure(config):
    """Enregistrer marker custom"""
    config.addinivalue_line(
        "markers", "matgto(cassette): use matgto cassette for test"
    )

@pytest.fixture(scope="session")
def matgto_proxy():
    """Proxy global pour toute la session"""
    proxy = MagnetoProxy("./cassettes")
    proxy.with_port(8888).with_mode(ProxyMode.AUTO)
    yield proxy
    proxy.shutdown()

@pytest.fixture(autouse=True)
def auto_cassette(request, matgto_proxy):
    """Auto-enregistrer tests avec marker @pytest.mark.matgto"""
    marker = request.node.get_closest_marker("matgto")

    if marker:
        cassette_name = marker.kwargs.get("cassette") or request.node.name
        matgto_proxy.start_recording(cassette_name)

        yield

        matgto_proxy.stop_recording()
    else:
        yield

# test_users_with_plugin.py
import requests
import pytest

@pytest.mark.matgto(cassette="get-users")
def test_get_users():
    """Test automatiquement enregistré!"""
    response = requests.get(
        "https://api.example.com/users",
        proxies={"https": "http://localhost:8888"}
    )
    assert response.status_code == 200

@pytest.mark.matgto  # Utilise le nom de la fonction
def test_create_user():
    """Enregistré dans cassette 'test_create_user'"""
    response = requests.post(
        "https://api.example.com/users",
        json={"name": "Test"},
        proxies={"https": "http://localhost:8888"}
    )
    assert response.status_code == 201
```

### Exemple 3 : WebSocket avec websockets library

```python
# test_websocket.py
import pytest
import asyncio
import websockets
from matgto_serge import MagnetoProxy

@pytest.fixture(scope="module")
def event_loop():
    """Event loop pour tests async"""
    loop = asyncio.get_event_loop_policy().new_event_loop()
    yield loop
    loop.close()

@pytest.mark.asyncio
@pytest.mark.matgto(cassette="websocket-notifications")
async def test_websocket_messages():
    """Test messages WebSocket"""
    messages = []

    # Connexion via proxy (TODO: support proxy dans websockets)
    async with websockets.connect(
        "ws://localhost:8888/notifications"
    ) as websocket:
        # Envoyer subscription
        await websocket.send('{"action":"subscribe"}')

        # Recevoir 3 messages
        for _ in range(3):
            message = await websocket.recv()
            messages.append(message)

    assert len(messages) == 3
    assert "connected" in messages[0]
```

### Exemple 4 : FastAPI Tests avec httpx

```python
# test_fastapi.py
import pytest
import httpx
from matgto_serge import MagnetoProxy

@pytest.fixture
def http_client():
    """Client HTTP async avec proxy"""
    return httpx.AsyncClient(
        proxies={
            "http://": "http://localhost:8888",
            "https://": "http://localhost:8888",
        },
        verify=False,  # Accepter certificat MITM
    )

@pytest.mark.asyncio
@pytest.mark.matgto(cassette="fastapi-users")
async def test_fastapi_endpoint(http_client):
    """Test endpoint FastAPI via proxy"""
    response = await http_client.get("https://api.example.com/users")

    assert response.status_code == 200
    data = response.json()
    assert isinstance(data["users"], list)
```

---

## 💎 Ruby / RSpec

### Installation

```ruby
# Gemfile
group :test do
  gem 'matgto-serge', '~> 1.0'
  gem 'rspec', '~> 3.12'
end
```

```bash
bundle install
```

### Exemple 1 : Tests RSpec basiques

```ruby
# spec/users_spec.rb
require 'matgto_serge'
require 'net/http'
require 'uri'

RSpec.describe 'User API' do
  let(:proxy) { MatgtoSerge::Proxy.new(cassette_dir: './cassettes') }

  before(:all) do
    @proxy = MatgtoSerge::Proxy.new(cassette_dir: './cassettes')
    @proxy.with_port(8888)
  end

  after(:all) do
    @proxy.shutdown
  end

  it 'fetches users list' do
    @proxy.start_recording('get-users')

    uri = URI('https://api.example.com/users')
    response = Net::HTTP.start(
      uri.host, uri.port,
      use_ssl: true,
      proxy_address: 'localhost',
      proxy_port: 8888
    ) do |http|
      http.get(uri.path)
    end

    expect(response.code).to eq('200')
    expect(JSON.parse(response.body)).to have_key('users')

    @proxy.stop_recording
  end
end
```

### Exemple 2 : RSpec Extension (Style VCR)

```ruby
# spec/support/matgto_helper.rb
module MatgtoHelper
  def self.included(base)
    base.extend ClassMethods
  end

  module ClassMethods
    def use_matgto_cassette(cassette_name)
      around(:each) do |example|
        $matgto_proxy.start_recording(cassette_name)
        example.run
        $matgto_proxy.stop_recording
      end
    end
  end
end

RSpec.configure do |config|
  config.include MatgtoHelper

  config.before(:suite) do
    $matgto_proxy = MatgtoSerge::Proxy.new(cassette_dir: './cassettes')
    $matgto_proxy.with_port(8888)
  end

  config.after(:suite) do
    $matgto_proxy.shutdown
  end
end

# spec/users_spec.rb
RSpec.describe 'User API with helper' do
  use_matgto_cassette 'get-users'

  it 'fetches users' do
    # Cassette automatiquement gérée!
    uri = URI('https://api.example.com/users')
    response = Net::HTTP.get(uri)

    expect(response).to include('users')
  end
end
```

---

## 📱 Kotlin / Android

### Installation

```kotlin
// build.gradle.kts (app module)
dependencies {
    testImplementation("com.magneto:serge:1.0.0")
    testImplementation("junit:junit:4.13.2")
    testImplementation("com.squareup.okhttp3:okhttp:4.12.0")
}
```

### Exemple : Tests Android avec OkHttp

```kotlin
// UserApiTest.kt
package com.example.app

import com.magneto.serge.MagnetoProxy
import com.magneto.serge.ProxyMode
import okhttp3.OkHttpClient
import okhttp3.Request
import org.junit.After
import org.junit.Before
import org.junit.Test
import java.net.InetSocketAddress
import java.net.Proxy
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class UserApiTest {
    private lateinit var proxy: MagnetoProxy
    private lateinit var httpClient: OkHttpClient

    @Before
    fun setup() {
        proxy = MagnetoProxy("./cassettes")
            .withPort(8888)
            .withMode(ProxyMode.AUTO)

        // Configurer OkHttp pour utiliser le proxy
        httpClient = OkHttpClient.Builder()
            .proxy(Proxy(Proxy.Type.HTTP, InetSocketAddress("localhost", 8888)))
            .build()
    }

    @Test
    fun `should fetch users from API`() {
        proxy.startRecording("get-users")

        val request = Request.Builder()
            .url("https://api.example.com/users")
            .build()

        val response = httpClient.newCall(request).execute()

        assertEquals(200, response.code)
        assertTrue(response.body!!.string().contains("users"))

        proxy.stopRecording()
    }

    @After
    fun teardown() {
        proxy.shutdown()
    }
}
```

---

## ⚛️ TypeScript / React

### Installation

```bash
npm install --save-dev @magneto/serge @testing-library/react vitest
```

### Exemple : Tests composants React avec API

```typescript
// UserList.test.tsx
import { describe, test, expect, beforeAll, afterAll } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import { MagnetoProxy } from '@magneto/serge';
import UserList from './UserList';

describe('UserList Component', () => {
  let proxy: MagnetoProxy;

  beforeAll(() => {
    proxy = new MagnetoProxy('./cassettes').withPort(8888);

    // Configurer fetch global pour utiliser le proxy
    global.fetch = new Proxy(global.fetch, {
      apply(target, thisArg, args) {
        const [url, options = {}] = args;
        return Reflect.apply(target, thisArg, [
          url,
          {
            ...options,
            // Note: fetch ne supporte pas proxy directement
            // Utiliser node-fetch ou axios à la place
          },
        ]);
      },
    });
  });

  afterAll(() => {
    proxy.shutdown();
  });

  test('should render users from API', async () => {
    proxy.startRecording('react-user-list');

    render(<UserList />);

    // Attendre que les users s'affichent
    await waitFor(() => {
      expect(screen.getByText(/John Doe/i)).toBeInTheDocument();
    });

    const users = screen.getAllByRole('listitem');
    expect(users.length).toBeGreaterThan(0);

    proxy.stopRecording();
  });
});
```

---

## 🔧 Configuration Avancée

### Fichier matgto.toml

```toml
# matgto.toml (racine du projet)
[matgto]
cassette_dir = "./cassettes"
proxy_port = 8888
mode = "auto"  # auto | record | replay | passthrough
strict = true  # Erreur si pas de match en mode replay

[matching]
# Ignorer headers dynamiques
ignore_headers = [
    "User-Agent",
    "Date",
    "X-Request-Id",
    "X-Correlation-Id",
    "Cookie",
]

# Ignorer query params
ignore_query_params = [
    "timestamp",
    "_t",
    "cache_buster",
]

# Matching personnalisé avec regex
[matching.url_patterns]
"/users/\\d+" = "/users/:id"  # Normaliser IDs dynamiques
"/api/v\\d+/.*" = "/api/v:version/*"  # Versions API

[recording]
# Filtrer données sensibles
filter_headers = [
    "Authorization",
    "X-API-Key",
    "Cookie",
    "Set-Cookie",
]

# Compression cassettes > 1MB
compress = true
format = "json"  # json | msgpack

[replay]
# Ajouter délai artificiel (simuler latence réseau)
latency_ms = 0

# Mode strict : erreur si requête non trouvée
strict = true

# Fallback si cassette incomplète
fallback_to_real = false

[logging]
level = "info"  # trace | debug | info | warn | error
format = "text"  # text | json
output = "stdout"  # stdout | file

[tls]
# Certificat CA custom (optionnel)
# ca_cert = "./certs/matgto-ca.pem"
# ca_key = "./certs/matgto-ca-key.pem"

# Installer automatiquement dans trust store
auto_install_cert = true
```

### Variables d'Environnement

```bash
# Mode de fonctionnement
export MATGTO_MODE=replay

# Répertoire cassettes
export MATGTO_CASSETTE_DIR=/path/to/cassettes

# Port proxy
export MATGTO_PROXY_PORT=9999

# Logging
export MATGTO_LOG_LEVEL=debug
export RUST_LOG=matgto_serge=trace

# Désactiver vérification SSL (développement uniquement!)
export MATGTO_INSECURE=true
```

### Configuration Programmatique

```rust
// Rust (configuration avancée)
use matgto_serge::{MagnetoProxy, Config, MatchingStrategy};

let config = Config::builder()
    .cassette_dir("./cassettes")
    .proxy_port(8888)
    .mode(ProxyMode::Auto)
    .matching_strategy(MatchingStrategy::Custom(Box::new(|req, cassette_req| {
        // Matching personnalisé
        req.method == cassette_req.method
            && normalize_url(&req.url) == normalize_url(&cassette_req.url)
    })))
    .header_filter(vec!["Authorization", "Cookie"])
    .compression(true)
    .build()?;

let proxy = MagnetoProxy::with_config(config)?;
```

---

## 🚀 Patterns d'Utilisation Avancés

### Pattern 1 : Tests CI/CD

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Cache cassettes
        uses: actions/cache@v3
        with:
          path: ./cassettes
          key: cassettes-${{ hashFiles('cassettes/**') }}

      - name: Run tests (replay mode)
        env:
          MATGTO_MODE: replay
          MATGTO_STRICT: true
        run: |
          npm test
          # Tous les tests utilisent cassettes → ultra-rapide!
```

### Pattern 2 : Record/Replay Sélectif

```javascript
// Enregistrer seulement si CI=true
const mode = process.env.CI ? 'replay' : 'auto';
const proxy = new MagnetoProxy('./cassettes').withMode(mode);

// En local : enregistre nouvelles cassettes
// En CI : replay strict (erreur si cassette manquante)
```

### Pattern 3 : Cassettes Partagées Multi-Langages

```
project/
├── cassettes/                 # Cassettes partagées
│   ├── get-users.json
│   ├── create-user.json
│   └── websocket-updates.json
├── backend/                   # Java Spring Boot
│   └── src/test/java/
│       └── UserApiTest.java   # Utilise cassettes/
├── frontend/                  # React TypeScript
│   └── src/__tests__/
│       └── UserList.test.tsx  # Utilise cassettes/
└── mobile/                    # Kotlin Android
    └── app/src/test/kotlin/
        └── UserApiTest.kt     # Utilise cassettes/

# Même cassette, 3 langages différents!
```

---

## 📝 Résumé

### Langages Supportés

| Langage | Package | Framework | Status |
|---------|---------|-----------|--------|
| **Java** | `com.magneto:serge` | JUnit 5, Spring Boot | ✅ Stable |
| **JavaScript** | `@magneto/serge` | Jest, Vitest, Mocha | ✅ Stable |
| **Python** | `matgto-serge` | pytest, unittest | ✅ Stable |
| **Ruby** | `matgto-serge` | RSpec, Minitest | ✅ Stable |
| **Kotlin** | `com.magneto:serge` | JUnit, Android Test | ✅ Stable |
| **TypeScript** | `@magneto/serge` | Vitest, Jest | ✅ Stable |
| **Go** | `github.com/matgto/serge-go` | testing, testify | 🟡 Community |
| **C#** | `MatgtoSerge.NET` | xUnit, NUnit | 🟡 Community |

### Prochains Exemples à Ajouter

- [ ] Playwright E2E tests
- [ ] Cypress integration
- [ ] gRPC support
- [ ] GraphQL subscriptions
- [ ] Server-Sent Events (SSE)
- [ ] Docker Compose integration
- [ ] Kubernetes testing

---

**Dernière mise à jour :** 2025-10-10
**Contributeurs :** Exemples communautaires bienvenus!
