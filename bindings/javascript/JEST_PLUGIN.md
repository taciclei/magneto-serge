# jest-magneto Plugin

Plugin Jest officiel pour **magneto-serge** - enregistrez et rejouez les interactions HTTP/WebSocket dans vos tests Jest.

## 🚀 Installation

```bash
npm install --save-dev magneto-serge
```

## 📖 Usage Basique

### 1. Configuration Jest

**Option A: jest.config.js** (recommandé)
```javascript
// jest.config.js
module.exports = {
  setupFilesAfterEnv: ['./jest.setup.js'],
  magneto: {
    cassetteDir: './test_cassettes',
    mode: 'auto', // auto | record | replay | strict
    port: 8888,
  },
};
```

**Option B: jest.setup.js**
```javascript
// jest.setup.js
const { setup } = require('magneto-serge/jest-magneto');

setup({
  magneto: {
    cassetteDir: './test_cassettes',
    mode: 'auto',
    port: 8888,
  },
});
```

### 2. Utiliser dans les Tests

```javascript
const { useMagneto, getProxyConfig } = require('magneto-serge/jest-magneto');
const axios = require('axios');

describe('API Tests', () => {
  let magneto;

  beforeEach(() => {
    magneto = useMagneto('api-test');
  });

  afterEach(() => {
    if (magneto) {
      magneto.shutdown();
    }
  });

  test('should fetch users', async () => {
    const response = await axios.get('https://api.example.com/users', {
      proxy: getProxyConfig(magneto),
    });

    expect(response.status).toBe(200);
    expect(response.data).toBeDefined();
  });
});
```

**Première exécution** → Enregistre dans `./test_cassettes/api-test.json`
**Exécutions suivantes** → Rejeu depuis la cassette (aucun appel réseau)

## 🎯 Modes

### Mode Auto (par défaut)

```javascript
const magneto = useMagneto('api-test');
// Si cassette existe → replay
// Si cassette manque → record
```

### Mode Record (force l'enregistrement)

```bash
# Via ligne de commande
MAGNETO_MODE=record npm test

# Via configuration
const magneto = useMagneto('api-test', { mode: 'record' });
```

### Mode Replay (rejeu uniquement)

```bash
MAGNETO_MODE=replay npm test
```

### Mode Strict (échoue si pas de match)

```javascript
const magneto = useMagneto('api-test', { mode: 'strict' });
// Échoue si:
// - Cassette manquante
// - Requête non matchée dans la cassette
```

## 🔧 Configuration

### Variables d'Environnement

```bash
# Mode par défaut
MAGNETO_MODE=auto|record|replay|strict

# Répertoire cassettes
MAGNETO_CASSETTE_DIR=./my_cassettes

# Port proxy
MAGNETO_PORT=9999

# Désactiver magneto
MAGNETO_DISABLE=true
```

### Options useMagneto()

```javascript
const magneto = useMagneto('cassette-name', {
  cassetteDir: './test_cassettes',  // Répertoire
  mode: 'auto',                     // Mode
  port: 8888,                       // Port
});
```

## 🧪 Exemples

### Tests API REST avec Axios

```javascript
const { useMagneto, getProxyConfig } = require('magneto-serge/jest-magneto');
const axios = require('axios');

describe('GitHub API', () => {
  let magneto;

  beforeEach(() => {
    magneto = useMagneto('github-api');
  });

  afterEach(() => {
    magneto?.shutdown();
  });

  test('should fetch repository', async () => {
    const response = await axios.get(
      'https://api.github.com/repos/taciclei/magneto-serge',
      { proxy: getProxyConfig(magneto) }
    );

    expect(response.status).toBe(200);
    expect(response.data.name).toBe('magneto-serge');
  });
});
```

### Tests POST avec JSON

```javascript
test('should create user', async () => {
  const magneto = useMagneto('create-user');

  const userData = { name: 'Alice', email: 'alice@example.com' };
  const response = await axios.post(
    'https://api.example.com/users',
    userData,
    { proxy: getProxyConfig(magneto) }
  );

  expect(response.status).toBe(201);
  magneto.shutdown();
});
```

### Tests avec node-fetch

```javascript
const { useMagneto, getProxyUrl } = require('magneto-serge/jest-magneto');
const fetch = require('node-fetch');
const { HttpsProxyAgent } = require('https-proxy-agent');

test('should fetch with node-fetch', async () => {
  const magneto = useMagneto('fetch-test');
  const agent = new HttpsProxyAgent(getProxyUrl(magneto));

  const response = await fetch('https://api.example.com/data', { agent });
  const data = await response.json();

  expect(response.ok).toBe(true);
  magneto.shutdown();
});
```

### Tests paramétrés

```javascript
describe.each([
  { userId: 1, name: 'Alice' },
  { userId: 2, name: 'Bob' },
  { userId: 3, name: 'Charlie' },
])('User $userId', ({ userId, name }) => {
  test(`should fetch user ${name}`, async () => {
    const magneto = useMagneto(`user-${userId}`);

    const response = await axios.get(
      `https://api.example.com/users/${userId}`,
      { proxy: getProxyConfig(magneto) }
    );

    expect(response.data.name).toBe(name);
    magneto.shutdown();
  });
});
```

### Fixture Personnalisée

```javascript
// testHelpers.js
const { useMagneto, getProxyConfig } = require('magneto-serge/jest-magneto');
const axios = require('axios');

function createApiClient(cassetteName) {
  const magneto = useMagneto(cassetteName);
  const client = axios.create({
    baseURL: 'https://api.example.com',
    proxy: getProxyConfig(magneto),
    headers: { Authorization: 'Bearer token' },
  });

  return {
    client,
    cleanup: () => magneto.shutdown(),
  };
}

module.exports = { createApiClient };

// test.js
const { createApiClient } = require('./testHelpers');

test('should use custom client', async () => {
  const { client, cleanup } = createApiClient('api-test');

  const response = await client.get('/users');
  expect(response.status).toBe(200);

  cleanup();
});
```

### Nom de cassette automatique

```javascript
test('automatic cassette naming', async () => {
  const magneto = useMagneto(); // Nom généré depuis le test

  const response = await axios.get(
    'https://api.example.com',
    { proxy: getProxyConfig(magneto) }
  );

  expect(response.ok).toBe(true);
  magneto.shutdown();
});
// Cassette: automatic-cassette-naming.json
```

## 🔄 Workflow Typique

### 1. Développement (premier run)

```bash
# Enregistre toutes les interactions
MAGNETO_MODE=record npm test
```

Cassettes créées dans `./test_cassettes/`

### 2. CI/CD (tests rapides)

```bash
# Rejeu uniquement, échoue si cassette manquante
MAGNETO_MODE=strict npm test
```

Aucun appel réseau → tests ultra-rapides ⚡

### 3. Mise à jour API

```bash
# Ré-enregistre une cassette spécifique
MAGNETO_MODE=record npm test -- api.test.js
```

### 4. Debugging

```bash
# Mode auto pour développement
npm test

# Désactiver magneto temporairement
MAGNETO_DISABLE=true npm test
```

## ⚙️ API Reference

### `useMagneto(cassetteName, options)`

Crée et configure une instance MagnetoProxy.

**Paramètres**:
- `cassetteName` (string, optionnel) - Nom de la cassette
- `options` (object, optionnel) - Options de configuration
  - `cassetteDir` (string) - Répertoire des cassettes
  - `mode` (string) - Mode: "auto", "record", "replay", "strict"
  - `port` (number) - Port du proxy

**Retourne**: Instance `MagnetoProxy`

### `getProxyConfig(magneto)`

Retourne la configuration proxy pour axios.

**Paramètres**:
- `magneto` (MagnetoProxy) - Instance proxy

**Retourne**:
```javascript
{
  host: 'localhost',
  port: 8888,
  protocol: 'http'
}
```

### `getProxyUrl(magneto)`

Retourne l'URL du proxy pour node-fetch.

**Paramètres**:
- `magneto` (MagnetoProxy) - Instance proxy

**Retourne**: `"http://localhost:8888"`

### Matcher Jest `toHaveCassette`

Vérifie qu'une cassette existe.

```javascript
expect('api-test').toHaveCassette();
expect('missing-cassette').not.toHaveCassette();
```

## 🐛 Troubleshooting

### Erreur: "magneto-serge not installed"

```bash
npm install --save-dev magneto-serge
```

### Proxy ne démarre pas

```javascript
// Utiliser un port différent
const magneto = useMagneto('api-test', { port: 9999 });
```

### Cassettes non créées

```bash
# Vérifier le répertoire
ls -la test_cassettes/

# Forcer mode record
MAGNETO_MODE=record npm test
```

### Mode strict échoue

```bash
# Vérifier que la cassette existe
ls test_cassettes/nom-cassette.json

# Ou passer en mode auto
MAGNETO_MODE=auto npm test
```

## 🎓 Best Practices

### 1. Une cassette par test

```javascript
test('specific api test', async () => {
  const magneto = useMagneto('specific-test');
  // Cassette dédiée = meilleure isolation
  magneto.shutdown();
});
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
  run: MAGNETO_MODE=strict npm test
  env:
    CI: true
```

### 5. Documentation des cassettes

```javascript
/**
 * Test API GitHub v3
 *
 * Cassette: github-api-v3.json
 * Enregistré: 2025-10-12
 * Endpoint: https://api.github.com/repos/...
 */
test('github api', async () => {
  const magneto = useMagneto('github-api-v3');
  // ...
  magneto.shutdown();
});
```

## 🔗 Ressources

- [Documentation magneto-serge](../../README.md)
- [Bindings JavaScript](./README.md)
- [Exemples](./example.js)
- [Tests](./test/)

## 📄 Licence

MIT OR Apache-2.0
