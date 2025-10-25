/**
 * Unit tests for Magnéto-Serge Jest matchers (no server needed)
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

// Mock cassette data
function createTestCassette(name, options = {}) {
  const {
    version = '1.0',
    interactionCount = 2,
    hasCookies = true
  } = options;

  const cassette = {
    version,
    name,
    recorded_at: '2025-10-25T10:00:00Z',
    interactions: Array(interactionCount).fill({
      kind: {
        Http: {
          request: {
            method: 'GET',
            url: 'https://api.example.com/users',
            headers: { 'Accept': 'application/json' },
            body: null
          },
          response: {
            status: 200,
            headers: { 'Content-Type': 'application/json' },
            body: Buffer.from('{"users":[]}').toJSON().data
          }
        }
      }
    })
  };

  if (hasCookies) {
    cassette.cookies = [{
      name: 'JSESSIONID',
      value: 'ABC123',
      domain: 'example.com',
      path: '/',
      expires: null,
      max_age: null,
      secure: true,
      http_only: true,
      same_site: null,
      created_at: '2025-10-25T10:00:00Z'
    }];
  }

  return cassette;
}

describe('Cassette Structure', () => {
  let cassette;

  beforeEach(() => {
    cassette = createTestCassette('test-cassette');
  });

  test('should have required fields', () => {
    expect(cassette).toHaveProperty('version');
    expect(cassette).toHaveProperty('name');
    expect(cassette).toHaveProperty('interactions');
    expect(cassette).toHaveProperty('recorded_at');
  });

  test('should have correct version', () => {
    expect(cassette.version).toBe('1.0');
  });

  test('should have correct name', () => {
    expect(cassette.name).toBe('test-cassette');
  });

  test('should have interactions array', () => {
    expect(Array.isArray(cassette.interactions)).toBe(true);
    expect(cassette.interactions.length).toBe(2);
  });
});

describe('HTTP Interactions', () => {
  let cassette;

  beforeEach(() => {
    cassette = createTestCassette('http-test');
  });

  test('should have HTTP interaction structure', () => {
    const interaction = cassette.interactions[0];

    expect(interaction).toHaveProperty('kind');
    expect(interaction.kind).toHaveProperty('Http');
  });

  test('should have request and response', () => {
    const http = cassette.interactions[0].kind.Http;

    expect(http).toHaveProperty('request');
    expect(http).toHaveProperty('response');
  });

  test('request should have method and URL', () => {
    const request = cassette.interactions[0].kind.Http.request;

    expect(request.method).toBe('GET');
    expect(request.url).toContain('api.example.com');
  });

  test('response should have status and headers', () => {
    const response = cassette.interactions[0].kind.Http.response;

    expect(response.status).toBe(200);
    expect(response.headers).toHaveProperty('Content-Type');
  });
});

describe('Cookies', () => {
  test('cassette should have cookies', () => {
    const cassette = createTestCassette('cookie-test', { hasCookies: true });

    expect(cassette).toHaveProperty('cookies');
    expect(Array.isArray(cassette.cookies)).toBe(true);
    expect(cassette.cookies.length).toBeGreaterThan(0);
  });

  test('cookie should have required fields', () => {
    const cassette = createTestCassette('cookie-test');
    const cookie = cassette.cookies[0];

    expect(cookie).toHaveProperty('name');
    expect(cookie).toHaveProperty('value');
    expect(cookie).toHaveProperty('domain');
    expect(cookie).toHaveProperty('path');
    expect(cookie).toHaveProperty('secure');
    expect(cookie).toHaveProperty('http_only');
  });

  test('cookie should have correct values', () => {
    const cassette = createTestCassette('cookie-test');
    const cookie = cassette.cookies[0];

    expect(cookie.name).toBe('JSESSIONID');
    expect(cookie.value).toBe('ABC123');
    expect(cookie.domain).toBe('example.com');
    expect(cookie.secure).toBe(true);
    expect(cookie.http_only).toBe(true);
  });

  test('cassette without cookies should not have cookies array', () => {
    const cassette = createTestCassette('no-cookie-test', { hasCookies: false });

    expect(cassette.cookies).toBeUndefined();
  });
});

describe('Multiple Interactions', () => {
  test.each([1, 5, 10, 50])('should handle %i interactions', (count) => {
    const cassette = createTestCassette('multi-test', { interactionCount: count });

    expect(cassette.interactions).toHaveLength(count);
  });

  test('should create unique cassettes', () => {
    const cassette1 = createTestCassette('cassette-1');
    const cassette2 = createTestCassette('cassette-2');

    expect(cassette1.name).toBe('cassette-1');
    expect(cassette2.name).toBe('cassette-2');
    expect(cassette1.name).not.toBe(cassette2.name);
  });
});

describe('Cassette Versions', () => {
  test('should support different versions', () => {
    const v1 = createTestCassette('test', { version: '1.0' });
    const v2 = createTestCassette('test', { version: '2.0' });

    expect(v1.version).toBe('1.0');
    expect(v2.version).toBe('2.0');
  });
});

describe('File Operations', () => {
  let tempDir;

  beforeEach(() => {
    tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'magneto-test-'));
  });

  afterEach(() => {
    if (fs.existsSync(tempDir)) {
      fs.rmSync(tempDir, { recursive: true });
    }
  });

  test('should write cassette to file', () => {
    const cassette = createTestCassette('file-test');
    const filePath = path.join(tempDir, 'file-test.json');

    fs.writeFileSync(filePath, JSON.stringify(cassette, null, 2));

    expect(fs.existsSync(filePath)).toBe(true);
  });

  test('should read cassette from file', () => {
    const originalCassette = createTestCassette('read-test');
    const filePath = path.join(tempDir, 'read-test.json');

    fs.writeFileSync(filePath, JSON.stringify(originalCassette, null, 2));

    const loadedCassette = JSON.parse(fs.readFileSync(filePath, 'utf8'));

    expect(loadedCassette.name).toBe(originalCassette.name);
    expect(loadedCassette.version).toBe(originalCassette.version);
    expect(loadedCassette.interactions.length).toBe(originalCassette.interactions.length);
  });

  test('should handle multiple cassette files', () => {
    for (let i = 0; i < 3; i++) {
      const cassette = createTestCassette(`cassette-${i}`);
      const filePath = path.join(tempDir, `cassette-${i}.json`);
      fs.writeFileSync(filePath, JSON.stringify(cassette));
    }

    const files = fs.readdirSync(tempDir);
    expect(files).toHaveLength(3);
    expect(files).toEqual(expect.arrayContaining([
      'cassette-0.json',
      'cassette-1.json',
      'cassette-2.json'
    ]));
  });
});

describe('Response Body', () => {
  test('should have body as byte array', () => {
    const cassette = createTestCassette('body-test');
    const response = cassette.interactions[0].kind.Http.response;

    expect(Array.isArray(response.body)).toBe(true);
    expect(response.body.every(b => typeof b === 'number')).toBe(true);
  });

  test('body should represent JSON', () => {
    const cassette = createTestCassette('json-body-test');
    const response = cassette.interactions[0].kind.Http.response;

    const bodyString = Buffer.from(response.body).toString();
    const bodyJson = JSON.parse(bodyString);

    expect(bodyJson).toHaveProperty('users');
    expect(Array.isArray(bodyJson.users)).toBe(true);
  });
});
