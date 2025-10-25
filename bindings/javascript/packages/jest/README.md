# @magneto-serge/jest

Jest integration for Magnéto-Serge, providing automatic cassette management for HTTP/WebSocket recording and replay in Jest tests.

## Installation

```bash
npm install --save-dev @magneto-serge/jest magneto-serge
# or
yarn add --dev @magneto-serge/jest magneto-serge
# or
pnpm add -D @magneto-serge/jest magneto-serge
```

## Quick Start

### Basic Usage

```typescript
import { magnetoTest } from '@magneto-serge/jest';

// Auto-generated cassette name from test name
magnetoTest('should fetch users', async () => {
  const response = await fetch('https://jsonplaceholder.typicode.com/users');
  const users = await response.json();

  expect(response.status).toBe(200);
  expect(users).toHaveLength(10);
});

// Custom cassette name
magnetoTest('custom cassette', { name: 'shared_cassette' }, async () => {
  const response = await fetch('https://api.example.com/data');
  expect(response.ok).toBe(true);
});

// Force re-recording
magnetoTest('force record', { record: 'all' }, async () => {
  const response = await fetch('https://api.example.com/live');
  expect(response.ok).toBe(true);
});
```

### Configuration

Create a setup file (e.g., `jest.setup.ts`):

```typescript
import { setupMagneto } from '@magneto-serge/jest';

setupMagneto({
  cassetteDir: '__cassettes__',
  mode: 'auto',
  port: 8888,
  record: 'new_episodes',
  verbose: false,
});
```

Add to your `jest.config.js`:

```javascript
module.exports = {
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts'],
};
```

## API Reference

### `magnetoTest(name, [options], testFn)`

Wrapper for Jest's `test()` with automatic cassette management.

**Parameters:**
- `name` (string): Test name
- `options` (optional): Cassette options
  - `name`: Custom cassette name (default: auto-generated from test name)
  - `mode`: Recording mode (`'auto'` | `'record'` | `'replay'` | `'passthrough'`)
  - `record`: VCR-compatible mode (`'new_episodes'` | `'once'` | `'all'` | `'none'`)
  - `cassetteDir`: Directory for this cassette (overrides global)
  - `port`: Proxy port (overrides global)
- `testFn`: Async test function

**Examples:**

```typescript
// Auto cassette name
magnetoTest('fetches data', async () => {
  // Cassette: __cassettes__/fetches_data.json
});

// Custom cassette name
magnetoTest('test', { name: 'my_cassette' }, async () => {
  // Cassette: __cassettes__/my_cassette.json
});

// Replay mode only
magnetoTest('test', { mode: 'replay' }, async () => {
  // Strict replay, no recording
});

// VCR-compatible record mode
magnetoTest('test', { record: 'all' }, async () => {
  // Always re-record (mode: 'record')
});
```

### `magnetoDescribe(name, [options], suiteFn)`

Wrapper for Jest's `describe()` with automatic cassette management for all tests.

**Example:**

```typescript
import { magnetoDescribe } from '@magneto-serge/jest';

magnetoDescribe('API Tests', { cassetteDir: 'api_cassettes' }, () => {
  test('fetches users', async () => {
    // Cassette auto-managed in api_cassettes/
    const response = await fetch('https://api.example.com/users');
    expect(response.status).toBe(200);
  });

  test('fetches posts', async () => {
    // Separate cassette auto-managed
    const response = await fetch('https://api.example.com/posts');
    expect(response.status).toBe(200);
  });
});
```

### `useCassette(nameOrOptions, fn)`

Manual cassette control within a test.

**Example:**

```typescript
import { useCassette } from '@magneto-serge/jest';

test('nested cassettes', async () => {
  await useCassette('outer_cassette', async () => {
    const response1 = await fetch('https://api.example.com/users');

    await useCassette('inner_cassette', async () => {
      const response2 = await fetch('https://api.example.com/posts');
      expect(response2.status).toBe(200);
    });

    expect(response1.status).toBe(200);
  });
});

// With options
test('manual control', async () => {
  await useCassette({ name: 'test', mode: 'record' }, async () => {
    // Cassette active for this block only
  });
});
```

### `configure(config)`

Global configuration for all tests.

**Example:**

```typescript
import { configure } from '@magneto-serge/jest';

configure({
  cassetteDir: 'tests/fixtures/cassettes',
  mode: 'auto',
  port: 8888,
  record: 'new_episodes',
  verbose: true,
});
```

### `setupMagneto([config])`

Setup function for Jest environment (use in `setupFilesAfterEnv`).

**Example:**

```typescript
// jest.setup.ts
import { setupMagneto } from '@magneto-serge/jest';

setupMagneto({
  cassetteDir: '__cassettes__',
  verbose: process.env.CI === 'true',
});
```

### `getCurrentCassette()`

Get the currently active cassette name.

**Example:**

```typescript
import { magnetoTest, getCurrentCassette } from '@magneto-serge/jest';

magnetoTest('check cassette name', async () => {
  expect(getCurrentCassette()).toBe('check_cassette_name');
});
```

## Configuration Options

### Global Config (via `configure()` or `setupMagneto()`)

```typescript
interface MagnetoJestConfig {
  /** Directory where cassettes are stored (default: '__cassettes__') */
  cassetteDir?: string;

  /** Recording mode (default: 'auto') */
  mode?: 'auto' | 'record' | 'replay' | 'passthrough';

  /** Proxy port (default: 8888) */
  port?: number;

  /** VCR-compatible record mode (default: 'new_episodes') */
  record?: 'new_episodes' | 'once' | 'all' | 'none';

  /** Enable verbose logging (default: false) */
  verbose?: boolean;
}
```

### Cassette Options (per-test)

```typescript
interface CassetteOptions {
  /** Custom cassette name (auto-generated if not provided) */
  name?: string;

  /** Recording mode (overrides global) */
  mode?: 'auto' | 'record' | 'replay' | 'passthrough';

  /** Cassette directory (overrides global) */
  cassetteDir?: string;

  /** Proxy port (overrides global) */
  port?: number;

  /** VCR-compatible record mode (overrides global) */
  record?: 'new_episodes' | 'once' | 'all' | 'none';
}
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

```typescript
import { magnetoTest } from '@magneto-serge/jest';

describe('JSONPlaceholder API', () => {
  magnetoTest('fetches posts', async () => {
    // Cassette: __cassettes__/fetches_posts.json
    const response = await fetch('https://jsonplaceholder.typicode.com/posts');
    const posts = await response.json();

    expect(response.status).toBe(200);
    expect(posts).toHaveLength(100);
  });

  magnetoTest('fetches single post', { name: 'post_1' }, async () => {
    // Cassette: __cassettes__/post_1.json
    const response = await fetch('https://jsonplaceholder.typicode.com/posts/1');
    const post = await response.json();

    expect(response.status).toBe(200);
    expect(post.id).toBe(1);
  });
});
```

### Nested Describe Blocks

```typescript
import { magnetoDescribe, magnetoTest } from '@magneto-serge/jest';

describe('GitHub API', () => {
  magnetoDescribe('users', () => {
    test('fetches user profile', async () => {
      // Auto cassette management
      const response = await fetch('https://api.github.com/users/octocat');
      expect(response.status).toBe(200);
    });
  });

  describe('repositories', () => {
    magnetoTest('lists repos', async () => {
      const response = await fetch('https://api.github.com/users/octocat/repos');
      expect(response.status).toBe(200);
    });
  });
});
```

### Different Recording Modes

```typescript
import { magnetoTest } from '@magneto-serge/jest';

describe('Recording Modes', () => {
  // Auto mode (default) - record if missing, replay if exists
  magnetoTest('auto mode', async () => {
    const response = await fetch('https://api.example.com/data');
    expect(response.status).toBe(200);
  });

  // Force re-recording
  magnetoTest('force record', { record: 'all' }, async () => {
    const response = await fetch('https://api.example.com/live');
    expect(response.status).toBe(200);
  });

  // Strict replay (error if cassette missing)
  magnetoTest('strict replay', { record: 'none' }, async () => {
    const response = await fetch('https://api.example.com/cached');
    expect(response.status).toBe(200);
  });

  // Passthrough (no recording/replay)
  magnetoTest('passthrough', { mode: 'passthrough' }, async () => {
    const response = await fetch('https://api.example.com/realtime');
    expect(response.ok).toBe(true);
  });
});
```

### Manual Cassette Control

```typescript
import { useCassette } from '@magneto-serge/jest';

test('multiple cassettes in one test', async () => {
  // First cassette
  await useCassette('users_cassette', async () => {
    const users = await fetch('https://api.example.com/users');
    expect(users.status).toBe(200);
  });

  // Second cassette
  await useCassette('posts_cassette', async () => {
    const posts = await fetch('https://api.example.com/posts');
    expect(posts.status).toBe(200);
  });

  // Third cassette with options
  await useCassette({ name: 'admin', mode: 'record' }, async () => {
    const admin = await fetch('https://api.example.com/admin');
    expect(admin.status).toBe(200);
  });
});
```

### Error Handling

```typescript
import { magnetoTest } from '@magneto-serge/jest';

describe('Error Handling', () => {
  magnetoTest('handles 404 errors', async () => {
    const response = await fetch('https://jsonplaceholder.typicode.com/posts/999999');
    expect(response.status).toBe(404);
  });

  magnetoTest('handles network timeouts', async () => {
    await expect(async () => {
      const controller = new AbortController();
      setTimeout(() => controller.abort(), 100);

      await fetch('https://httpstat.us/524?sleep=5000', {
        signal: controller.signal,
      });
    }).rejects.toThrow();
  });
});
```

### Testing with axios

```typescript
import axios from 'axios';
import { magnetoTest } from '@magneto-serge/jest';

magnetoTest('axios GET request', async () => {
  const response = await axios.get('https://jsonplaceholder.typicode.com/users');

  expect(response.status).toBe(200);
  expect(response.data).toHaveLength(10);
});

magnetoTest('axios POST request', { record: 'all' }, async () => {
  const response = await axios.post(
    'https://jsonplaceholder.typicode.com/posts',
    {
      title: 'Test Post',
      body: 'This is a test',
      userId: 1,
    }
  );

  expect(response.status).toBe(201);
  expect(response.data.id).toBeDefined();
});
```

## TypeScript Support

Full TypeScript support with type definitions included.

```typescript
import {
  magnetoTest,
  configure,
  MagnetoJestConfig,
  CassetteOptions
} from '@magneto-serge/jest';

const config: MagnetoJestConfig = {
  cassetteDir: 'cassettes',
  mode: 'auto',
};

configure(config);

const options: CassetteOptions = {
  name: 'test',
  mode: 'replay',
};

magnetoTest('typed test', options, async () => {
  // Type-safe!
});
```

## Migration from VCR (Ruby)

If you're familiar with VCR for Ruby, the API is very similar:

| VCR (Ruby) | @magneto-serge/jest |
|------------|---------------------|
| `VCR.use_cassette('name') { }` | `await useCassette('name', async () => {})` |
| `record: :new_episodes` | `{ record: 'new_episodes' }` |
| `record: :once` | `{ record: 'once' }` |
| `record: :all` | `{ record: 'all' }` |
| `record: :none` | `{ record: 'none' }` |

## Comparison with Polly.js

| Feature | @magneto-serge/jest | Polly.js |
|---------|---------------------|----------|
| HTTP Recording | ✅ | ✅ |
| WebSocket Recording | ✅ | ❌ |
| Multi-language Support | ✅ (Rust, Ruby, Python, etc.) | ❌ (JS only) |
| Jest Integration | ✅ Native | ✅ Via adapter |
| TypeScript | ✅ | ✅ |
| Performance | ~5000+ req/s | ~2000 req/s |
| Cassette Format | JSON/MessagePack | HAR |

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/taciclei/magneto-serge.

## License

MIT License - see [LICENSE](LICENSE) file for details.
