# 🟨 @magneto/serge - JavaScript/Node.js Bindings

Bindings JavaScript/Node.js pour **matgto-serge** - Enregistrez et rejouez vos requêtes HTTP/WebSocket pour des tests déterministes.

[![npm version](https://img.shields.io/npm/v/@magneto/serge.svg)](https://www.npmjs.com/package/@magneto/serge)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## 📦 Installation

```bash
# npm
npm install @magneto/serge

# pnpm (recommandé)
pnpm add @magneto/serge

# yarn
yarn add @magneto/serge
```

## 🚀 Utilisation Rapide

```javascript
const { MagnetoProxy, ProxyMode } = require('@magneto/serge');

// Créer un proxy
const proxy = new MagnetoProxy('./cassettes');

// Configurer
proxy.setPort(8888);
proxy.setMode(ProxyMode.RECORD);

// Enregistrer
if (proxy.startRecording('my_test')) {
    // Faire des requêtes HTTP...
    // Elles seront enregistrées automatiquement

    proxy.stopRecording();
}

proxy.shutdown();
```

## 🎯 TypeScript Support

```typescript
import { MagnetoProxy, ProxyMode, createProxy } from '@magneto/serge';

const proxy: MagnetoProxy = createProxy('./cassettes')!;

proxy.setPort(8888);
proxy.setMode(ProxyMode.RECORD);

if (proxy.startRecording('typescript_test')) {
    // Vos requêtes HTTP...
    proxy.stopRecording();
}
```

## 🧪 Intégration Jest

```javascript
const { MagnetoProxy, ProxyMode } = require('@magneto/serge');
const axios = require('axios');

describe('API Tests', () => {
    let proxy;

    beforeEach(() => {
        proxy = new MagnetoProxy('./cassettes');
        proxy.setPort(8888);
    });

    afterEach(() => {
        proxy.shutdown();
    });

    test('should record API call', async () => {
        // Mode enregistrement
        proxy.setMode(ProxyMode.RECORD);
        proxy.startRecording('api_test');

        // Configurer axios avec le proxy
        const response = await axios.get('https://api.example.com/users', {
            proxy: {
                host: 'localhost',
                port: 8888
            }
        });

        expect(response.status).toBe(200);

        proxy.stopRecording();
    });

    test('should replay API call', async () => {
        // Mode replay - pas besoin de réseau!
        proxy.setMode(ProxyMode.REPLAY);
        proxy.replay('api_test');

        // Les requêtes sont rejouées depuis la cassette
        const response = await axios.get('https://api.example.com/users', {
            proxy: {
                host: 'localhost',
                port: 8888
            }
        });

        expect(response.status).toBe(200);
    });
});
```

## 📖 API

### MagnetoProxy

**Constructeur:**
```javascript
new MagnetoProxy(cassetteDir: string)
createProxy(cassetteDir: string) // Factory function
```

**Configuration:**
```javascript
setPort(port: number): void
setMode(mode: ProxyMode): void

getPort(): number
getMode(): ProxyMode
```

**Enregistrement:**
```javascript
startRecording(cassetteName: string): boolean
stopRecording(): boolean
```

**Replay:**
```javascript
replay(cassetteName: string): boolean
```

**Lifecycle:**
```javascript
shutdown(): void
```

**Statique:**
```javascript
version(): string
```

### ProxyMode (Enum)

```javascript
ProxyMode.AUTO        // 0 - Auto-détection
ProxyMode.RECORD      // 1 - Enregistrement
ProxyMode.REPLAY      // 2 - Replay
ProxyMode.PASSTHROUGH // 3 - Passthrough sans enregistrement
```

## 🔧 Configuration avec Axios

```javascript
const axios = require('axios');

// Configuration du proxy
const axiosInstance = axios.create({
    proxy: {
        host: 'localhost',
        port: 8888,
        protocol: 'http'
    }
});

// Pour HTTPS (certificat auto-signé)
const https = require('https');

const axiosInstance = axios.create({
    proxy: {
        host: 'localhost',
        port: 8888
    },
    httpsAgent: new https.Agent({
        rejectUnauthorized: false // DEV uniquement!
    })
});
```

## 🔧 Configuration avec node-fetch

```javascript
const fetch = require('node-fetch');
const { HttpsProxyAgent } = require('https-proxy-agent');

const proxyAgent = new HttpsProxyAgent('http://localhost:8888');

const response = await fetch('https://api.example.com/users', {
    agent: proxyAgent
});
```

## 🌐 Configuration avec http/https natif

```javascript
const http = require('http');

const options = {
    hostname: 'api.example.com',
    port: 80,
    path: '/users',
    method: 'GET',
    // Passer par le proxy
    agent: new http.Agent({
        host: 'localhost',
        port: 8888
    })
};

http.request(options, (res) => {
    // Traiter la réponse
}).end();
```

## 📁 Structure des Cassettes

Les cassettes sont sauvegardées en JSON:

```
./cassettes/
├── my_test.json
├── api_test.json
└── another_test.json
```

Format d'une cassette:
```json
{
  "version": "1.0",
  "name": "my_test",
  "recorded_at": "2025-10-10T12:00:00Z",
  "interactions": [
    {
      "request": {
        "method": "GET",
        "url": "https://api.example.com/users",
        "headers": {},
        "body": null
      },
      "response": {
        "status": 200,
        "headers": {},
        "body": "[...]"
      }
    }
  ]
}
```

## 🎯 Exemples Complets

### Express.js + Supertest

```javascript
const request = require('supertest');
const express = require('express');
const { MagnetoProxy, ProxyMode } = require('@magneto/serge');

const app = express();

// Votre application Express
app.get('/users', async (req, res) => {
    // Appel API externe
    const response = await fetch('https://api.example.com/users');
    const data = await response.json();
    res.json(data);
});

describe('Express API', () => {
    let proxy;

    beforeAll(() => {
        proxy = new MagnetoProxy('./cassettes');
        proxy.setPort(8888);
    });

    afterAll(() => {
        proxy.shutdown();
    });

    test('GET /users', async () => {
        proxy.setMode(ProxyMode.RECORD);
        proxy.startRecording('express_users');

        const response = await request(app)
            .get('/users')
            .expect(200);

        expect(response.body).toHaveLength(10);

        proxy.stopRecording();
    });
});
```

### Playwright E2E

```javascript
const { test, expect } = require('@playwright/test');
const { MagnetoProxy, ProxyMode } = require('@magneto/serge');

test.describe('E2E Tests', () => {
    let proxy;

    test.beforeAll(async () => {
        proxy = new MagnetoProxy('./cassettes');
        proxy.setPort(8888);
        proxy.setMode(ProxyMode.RECORD);
    });

    test.afterAll(async () => {
        proxy.shutdown();
    });

    test('should record browser requests', async ({ page }) => {
        // Configurer Playwright pour utiliser le proxy
        await page.route('**/*', route => {
            route.continue({
                proxy: 'http://localhost:8888'
            });
        });

        proxy.startRecording('playwright_test');

        await page.goto('https://example.com');
        await expect(page).toHaveTitle(/Example/);

        proxy.stopRecording();
    });
});
```

## 📚 Scripts Package Manager

```json
{
  "scripts": {
    "test": "jest",
    "test:record": "MAGNETO_MODE=record jest",
    "test:replay": "MAGNETO_MODE=replay jest"
  }
}
```

Utilisation:
```bash
# Avec pnpm (recommandé)
pnpm test:record     # Enregistrer les cassettes
pnpm test:replay     # Rejouer depuis les cassettes
pnpm test            # Mode auto (défaut)

# Avec npm
npm run test:record
npm run test:replay
npm test

# Avec yarn
yarn test:record
yarn test:replay
yarn test
```

## ⚙️ Variables d'Environnement

```bash
# Mode du proxy
export MAGNETO_MODE=record    # ou replay, auto, passthrough

# Port du proxy
export MAGNETO_PORT=8888

# Répertoire des cassettes
export MAGNETO_CASSETTE_DIR=./cassettes
```

Usage dans le code:
```javascript
const mode = process.env.MAGNETO_MODE || 'auto';
const port = parseInt(process.env.MAGNETO_PORT || '8888');
const dir = process.env.MAGNETO_CASSETTE_DIR || './cassettes';

const proxy = new MagnetoProxy(dir);
proxy.setPort(port);
proxy.setMode(ProxyMode[mode.toUpperCase()]);
```

## 🐛 Troubleshooting

### Module not found: libuniffi_magneto_serge

Si vous voyez cette erreur, assurez-vous que la bibliothèque native est présente:

```bash
# macOS
cp ../../target/release/libmagneto_serge.dylib lib/libuniffi_magneto_serge.dylib

# Linux
cp ../../target/release/libmagneto_serge.so lib/libuniffi_magneto_serge.so
```

### Proxy not intercepting requests

Vérifiez que:
1. Le port est correctement configuré
2. Votre client HTTP utilise bien le proxy
3. Pour HTTPS, le certificat auto-signé est accepté (dev uniquement)

## 📄 License

MIT OR Apache-2.0

## 🤝 Contributeurs

matgto-serge contributors

## 🔗 Liens

- [Documentation](https://github.com/taciclei/magneto-serge)
- [NPM Package](https://www.npmjs.com/package/@magneto/serge)
- [Issues](https://github.com/taciclei/magneto-serge/issues)
