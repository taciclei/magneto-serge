# Magneto-Serge TypeScript Minimal Example

This is a minimal TypeScript example demonstrating how to use Magneto-Serge for HTTP/WebSocket recording and replay in your tests.

## 📦 What's Included

- ✅ TypeScript configuration (`tsconfig.json`)
- ✅ Jest test framework with TypeScript support (`ts-jest`)
- ✅ Magneto-Serge integration for recording/replay
- ✅ Example tests demonstrating the API
- ✅ NPM scripts for testing in different modes

## 🚀 Quick Start

### 1. Install Dependencies

```bash
npm install
```

This will install:
- TypeScript 5.3+
- Jest 29 with ts-jest
- Magneto-Serge (from local build)
- Type definitions for Node.js and Jest

### 2. Run Tests

```bash
# Run tests in auto mode (record if cassette missing, replay if exists)
npm test

# Force recording mode (always record, overwrite existing cassettes)
npm run test:record

# Force replay mode (only replay, error if cassette missing)
npm run test:replay

# Watch mode for development
npm run test:watch
```

### 3. Check TypeScript Types

```bash
npm run type-check
```

## 📁 Project Structure

```
typescript-minimal/
├── src/
│   └── __tests__/
│       └── basic.test.ts          # Example tests
├── __cassettes__/                 # Recorded HTTP interactions (gitignored)
├── package.json                   # Dependencies and scripts
├── tsconfig.json                  # TypeScript configuration
├── jest.config.ts                 # Jest configuration
├── jest.setup.ts                  # Magneto-Serge setup
└── README.md                      # This file
```

## 🎯 Example Test

```typescript
import { MagnetoProxy, ProxyMode } from 'magneto-serge';

describe('API Tests', () => {
  let proxy: MagnetoProxy;

  beforeEach(() => {
    proxy = new MagnetoProxy('./__cassettes__');
    proxy.setPort(8888);
    proxy.setMode(ProxyMode.Auto);
  });

  afterEach(() => {
    proxy.shutdown();
  });

  test('should record HTTP request', () => {
    proxy.startRecording('my-api-test');

    // Make your HTTP requests here
    // They will be proxied through localhost:8888
    // and recorded to __cassettes__/my-api-test.json

    proxy.stopRecording();
  });
});
```

## 🔧 Configuration

### Proxy Modes

Magneto-Serge supports 4 proxy modes:

| Mode | Behavior |
|------|----------|
| `ProxyMode.Auto` | Record if cassette doesn't exist, replay if it does |
| `ProxyMode.Record` | Always record, overwrite existing cassette |
| `ProxyMode.Replay` | Only replay, error if cassette not found |
| `ProxyMode.Passthrough` | Direct connection, no recording/replay |

### Environment Variables

You can control the mode via environment variable:

```bash
# In your test or CI/CD
export MAGNETO_MODE=record  # or replay, auto, passthrough
npm test
```

### Jest Configuration

The `jest.setup.ts` file configures:
- Cassette directory: `__cassettes__/`
- Proxy port: `8888`
- HTTP/HTTPS proxy environment variables
- Test timeout: 30 seconds

## 📝 Usage with Real HTTP Clients

### With node-fetch

```typescript
import fetch from 'node-fetch';
import { HttpsProxyAgent } from 'https-proxy-agent';

const proxyAgent = new HttpsProxyAgent('http://localhost:8888');

const response = await fetch('https://api.example.com/data', {
  agent: proxyAgent
});
```

### With axios

```typescript
import axios from 'axios';

const client = axios.create({
  proxy: {
    host: 'localhost',
    port: 8888
  }
});

const response = await client.get('https://api.example.com/data');
```

## 🧪 Testing in CI/CD

### Recommended Approach

**Commit cassettes to git** for deterministic, fast, offline tests:

```yaml
# .github/workflows/test.yml
- name: Run tests
  run: npm test
  env:
    MAGNETO_MODE: replay  # Only replay, never record
```

Benefits:
- ✅ Fast (no network calls)
- ✅ Deterministic (same results every time)
- ✅ Works offline
- ✅ No API rate limits

### Alternative Approach

**Generate cassettes in CI** if you want always-fresh data:

```yaml
- name: Run tests
  run: npm test
  env:
    MAGNETO_MODE: record  # Re-record on each CI run
```

## 🐛 Troubleshooting

### Tests fail with "Failed to load native binding"

Make sure the Rust library and bindings are built:

```bash
# From repository root
cargo build --lib --release
cd bindings/javascript
npm run build
```

### Port 8888 already in use

Change the proxy port in `jest.setup.ts` and your tests:

```typescript
proxy.setPort(9999);  // Use different port
```

### Cassette not found errors

If running in replay mode, ensure cassettes exist:

```bash
# Record cassettes first
npm run test:record

# Then replay
npm run test:replay
```

## 📚 Next Steps

- Add real HTTP tests with fetch or axios
- Configure your HTTP client to use the proxy
- Commit cassettes to git for CI/CD
- Use different cassette names for different test scenarios
- Explore WebSocket recording (coming soon in examples)

## 🔗 Links

- [Magneto-Serge Documentation](../../README.md)
- [JavaScript Bindings Guide](../../bindings/javascript/README.md)
- [Jest Plugin Documentation](../../bindings/javascript/packages/jest/README.md)

## 📄 License

MIT or Apache-2.0
