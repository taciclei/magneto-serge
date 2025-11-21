# 🚀 TypeScript Quick Start Guide

Complete guide to using Magneto-Serge in your TypeScript projects for HTTP/WebSocket recording and deterministic testing.

## 📋 Prerequisites

Before you begin, make sure you have:

- **Node.js 18+** installed (`node --version`)
- **npm 9+** or **pnpm 8+** (`npm --version`)
- **TypeScript 5+** knowledge
- **Jest** test framework familiarity (or similar)

## 🎯 5-Minute Setup

### Step 1: Install Packages

```bash
# Install Magneto-Serge and TypeScript dependencies
npm install --save-dev magneto-serge @types/node @types/jest jest ts-jest typescript

# Or with pnpm
pnpm add -D magneto-serge @types/node @types/jest jest ts-jest typescript
```

> ⚠️ **Note**: `magneto-serge` is currently available via local build or GitHub Packages.
> Publication to npm is planned. See [installation alternatives](#installation-alternatives) below.

### Step 2: Configure TypeScript

Create `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "lib": ["ES2020"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "types": ["node", "jest"]
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

### Step 3: Configure Jest

Create `jest.config.ts`:

```typescript
import type { Config } from 'jest';

const config: Config = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
  testMatch: ['**/__tests__/**/*.test.ts', '**/*.test.ts'],
};

export default config;
```

### Step 4: Setup Magneto-Serge

Create `jest.setup.ts`:

```typescript
// Configure HTTP proxy environment variables
const proxyPort = 8888;
process.env.HTTP_PROXY = `http://localhost:${proxyPort}`;
process.env.HTTPS_PROXY = `http://localhost:${proxyPort}`;

// Set test timeout
jest.setTimeout(30000);
```

### Step 5: Write Your First Test

Create `src/__tests__/api.test.ts`:

```typescript
import { MagnetoProxy, ProxyMode } from 'magneto-serge';
import * as path from 'path';

describe('API Tests with Magneto-Serge', () => {
  let proxy: MagnetoProxy;
  const cassetteDir = path.join(__dirname, '../../__cassettes__');

  beforeEach(() => {
    proxy = new MagnetoProxy(cassetteDir);
    proxy.setPort(8888);
    proxy.setMode(ProxyMode.Auto); // Record if missing, replay if exists
  });

  afterEach(() => {
    if (proxy) {
      proxy.shutdown();
    }
  });

  test('should record and replay HTTP request', async () => {
    proxy.startRecording('github-api-test');

    // Make your HTTP request here (example with fetch)
    // Configure your HTTP client to use proxy localhost:8888

    proxy.stopRecording();

    expect(true).toBe(true); // Your assertions here
  });
});
```

### Step 6: Add NPM Scripts

In your `package.json`:

```json
{
  "scripts": {
    "test": "jest",
    "test:record": "MAGNETO_MODE=record jest",
    "test:replay": "MAGNETO_MODE=replay jest",
    "test:watch": "jest --watch",
    "type-check": "tsc --noEmit"
  }
}
```

### Step 7: Run Tests

```bash
# Run tests (auto mode: record if missing, replay if exists)
npm test

# Force recording mode (always record)
npm run test:record

# Force replay mode (only replay)
npm run test:replay
```

## 📁 Complete Project Structure

```
my-typescript-project/
├── src/
│   ├── __tests__/
│   │   └── api.test.ts           # Your tests
│   └── api-client.ts              # Your code
├── __cassettes__/                 # Recorded HTTP interactions (gitignored)
│   └── github-api-test.json
├── dist/                          # Compiled JavaScript (gitignored)
├── node_modules/                  # Dependencies (gitignored)
├── .gitignore
├── package.json
├── tsconfig.json                  # TypeScript config
├── jest.config.ts                 # Jest config
├── jest.setup.ts                  # Magneto setup
└── README.md
```

## 🔧 Using with HTTP Clients

### With node-fetch

```typescript
import fetch from 'node-fetch';
import { HttpsProxyAgent } from 'https-proxy-agent';

const proxyAgent = new HttpsProxyAgent('http://localhost:8888');

const response = await fetch('https://api.github.com/users/octocat', {
  agent: proxyAgent
});

const data = await response.json();
```

### With axios

```typescript
import axios from 'axios';

// Create axios instance with proxy
const api = axios.create({
  proxy: {
    host: 'localhost',
    port: 8888,
    protocol: 'http'
  }
});

const response = await api.get('https://api.github.com/users/octocat');
console.log(response.data);
```

### With native http/https modules

```typescript
import * as https from 'https';
import { HttpsProxyAgent } from 'https-proxy-agent';

const agent = new HttpsProxyAgent('http://localhost:8888');

const options = {
  hostname: 'api.github.com',
  path: '/users/octocat',
  method: 'GET',
  agent: agent,
  headers: {
    'User-Agent': 'magneto-serge-example'
  }
};

https.request(options, (res) => {
  // Handle response
}).end();
```

## 🎬 Recording Modes

Magneto-Serge supports 4 proxy modes:

| Mode | Enum | Behavior |
|------|------|----------|
| **Auto** | `ProxyMode.Auto` | Record if cassette doesn't exist, replay if it does ⭐ Recommended |
| **Record** | `ProxyMode.Record` | Always record, overwrite existing cassette |
| **Replay** | `ProxyMode.Replay` | Only replay, error if cassette not found |
| **Passthrough** | `ProxyMode.Passthrough` | Direct connection, no recording/replay |

### Example Usage

```typescript
// Auto mode (recommended)
proxy.setMode(ProxyMode.Auto);

// Record mode (for updating cassettes)
proxy.setMode(ProxyMode.Record);

// Replay mode (for CI/CD)
proxy.setMode(ProxyMode.Replay);

// Passthrough mode (for debugging)
proxy.setMode(ProxyMode.Passthrough);
```

## 📊 CI/CD Integration

### Recommended: Commit Cassettes

**Benefits:**
- ✅ Fast tests (no network calls)
- ✅ Deterministic (same results every time)
- ✅ Works offline
- ✅ No API rate limits

**.gitignore adjustment:**

```gitignore
# Comment out or remove this line to commit cassettes
# __cassettes__/*.json

# Keep this to ignore node_modules and build output
node_modules/
dist/
```

**GitHub Actions example:**

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: npm ci

      - name: Run tests (replay mode)
        run: npm test
        env:
          MAGNETO_MODE: replay  # Only replay, never record
```

### Alternative: Generate Cassettes in CI

**Benefits:**
- ✅ Always fresh data from APIs
- ✅ Detects API changes automatically

**Drawbacks:**
- ⚠️ Slower (network calls)
- ⚠️ Requires API credentials in CI
- ⚠️ Subject to API rate limits

```yaml
- name: Run tests (record mode)
  run: npm test
  env:
    MAGNETO_MODE: record
    API_TOKEN: ${{ secrets.API_TOKEN }}  # If needed
```

## 🛠️ Advanced Configuration

### Environment Variables

Control Magneto-Serge behavior via environment variables:

```bash
# Proxy mode
export MAGNETO_MODE=record     # or replay, auto, passthrough

# Proxy port
export MAGNETO_PORT=8888

# Cassette directory
export MAGNETO_CASSETTE_DIR=./cassettes

# Verbose logging
export MAGNETO_VERBOSE=true
```

### TypeScript API Reference

```typescript
import { MagnetoProxy, ProxyMode } from 'magneto-serge';

// Create proxy instance
const proxy = new MagnetoProxy(cassetteDir: string);

// Configuration methods
proxy.setPort(port: number): void
proxy.setMode(mode: ProxyMode): void
proxy.getPort(): number

// Recording methods
proxy.startRecording(cassetteName: string): void
proxy.stopRecording(): void

// Replay method
proxy.replay(cassetteName: string): void

// Cleanup
proxy.shutdown(): void

// Get version
import { version } from 'magneto-serge';
console.log(version());  // "0.7.0"
```

### Custom Cassette Naming

```typescript
// Use descriptive cassette names
proxy.startRecording('github-user-octocat');
proxy.startRecording('github-repos-list');
proxy.startRecording('api-authentication-success');
proxy.startRecording('api-error-404');

// Organize by feature/module
proxy.startRecording('auth/login-success');
proxy.startRecording('users/fetch-profile');
proxy.startRecording('orders/create-new-order');
```

## 🧪 Testing Patterns

### Pattern 1: One Cassette Per Test

```typescript
describe('GitHub API', () => {
  test('fetches user profile', () => {
    proxy.startRecording('github-user-profile');
    // Test code...
    proxy.stopRecording();
  });

  test('fetches user repos', () => {
    proxy.startRecording('github-user-repos');
    // Test code...
    proxy.stopRecording();
  });
});
```

### Pattern 2: Shared Cassette for Suite

```typescript
describe('GitHub API', () => {
  beforeAll(() => {
    proxy.startRecording('github-api-suite');
  });

  afterAll(() => {
    proxy.stopRecording();
  });

  test('fetches user profile', () => {
    // All requests recorded to same cassette
  });

  test('fetches user repos', () => {
    // All requests recorded to same cassette
  });
});
```

### Pattern 3: Conditional Recording

```typescript
const isCI = process.env.CI === 'true';

beforeEach(() => {
  proxy.setMode(isCI ? ProxyMode.Replay : ProxyMode.Auto);
});
```

## 🐛 Troubleshooting

### Issue: "Failed to load native binding"

**Solution:** Make sure Rust library is built:

```bash
# From repository root
cargo build --lib --release
cd bindings/javascript
npm run build
```

### Issue: Port 8888 already in use

**Solution:** Use a different port:

```typescript
proxy.setPort(9999);
process.env.HTTP_PROXY = 'http://localhost:9999';
process.env.HTTPS_PROXY = 'http://localhost:9999';
```

### Issue: Cassette not found in replay mode

**Solution:** Record cassettes first:

```bash
npm run test:record  # Record cassettes
npm run test:replay  # Then replay
```

### Issue: TypeScript compilation errors

**Solution:** Check your `tsconfig.json`:

```bash
npm run type-check  # Check types without running tests
npx tsc --noEmit    # Alternative way
```

### Issue: Tests timeout

**Solution:** Increase timeout in `jest.setup.ts`:

```typescript
jest.setTimeout(60000);  // 60 seconds
```

## 📦 Installation Alternatives

### Option 1: Local Build (Current)

```bash
# Clone repository
git clone https://github.com/taciclei/magneto-serge.git
cd magneto-serge

# Build library
cargo build --lib --release

# Build bindings
cd bindings/javascript
npm install
npm run build

# Link locally
npm link

# In your project
npm link magneto-serge
```

### Option 2: Direct Path (package.json)

```json
{
  "dependencies": {
    "magneto-serge": "file:../path/to/magneto-serge/bindings/javascript"
  }
}
```

### Option 3: GitHub Packages (Requires authentication)

```bash
# Create .npmrc
echo "@taciclei:registry=https://npm.pkg.github.com" >> .npmrc

# Install
npm install @taciclei/magneto-serge
```

### Option 4: NPM (Coming Soon)

```bash
# When published
npm install magneto-serge
```

## 🎓 Examples

Check out complete examples in the repository:

- [`examples/typescript-minimal/`](../examples/typescript-minimal/) - Minimal TypeScript setup
- [`examples/nodejs-backend/`](../examples/nodejs-backend/) - Node.js backend example
- [`examples/angular-client/`](../examples/angular-client/) - Angular frontend example

## 📚 Further Reading

- [Main README](../README.md) - Project overview
- [JavaScript Bindings](../bindings/javascript/README.md) - JavaScript API reference
- [Jest Plugin](../bindings/javascript/packages/jest/README.md) - Jest integration
- [Architecture](./ARCHITECTURE.md) - Technical architecture
- [API Documentation](./API.md) - REST API reference

## 🤝 Contributing

Found an issue or want to contribute?

1. Check [existing issues](https://github.com/taciclei/magneto-serge/issues)
2. Open a new issue or PR
3. Follow the [Contributing Guide](../CONTRIBUTING.md)

## 📄 License

MIT or Apache-2.0

---

**Need help?** Open an issue on [GitHub](https://github.com/taciclei/magneto-serge/issues)
