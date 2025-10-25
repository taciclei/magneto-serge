/**
 * Magnéto-Serge Jest Matchers
 *
 * Custom Jest matchers for testing with Magnéto-Serge cassettes.
 *
 * Installation:
 *   npm install --save-dev @magneto-serge/jest-matchers
 *
 * Usage:
 *   import '@magneto-serge/jest-matchers';
 *
 *   test('should match cassette', () => {
 *     expect(response).toMatchCassette('user-login');
 *   });
 */

const fs = require('fs');
const path = require('path');

/**
 * Load a cassette from disk
 * @param {string} name - Cassette name (without extension)
 * @param {string} cassetteDir - Directory containing cassettes
 * @returns {Object} Parsed cassette
 */
function loadCassette(name, cassetteDir = './cassettes') {
  const jsonPath = path.join(cassetteDir, `${name}.json`);
  const msgpackPath = path.join(cassetteDir, `${name}.msgpack`);

  let cassettePath;
  if (fs.existsSync(jsonPath)) {
    cassettePath = jsonPath;
  } else if (fs.existsSync(msgpackPath)) {
    cassettePath = msgpackPath;
    // TODO: Support msgpack
    throw new Error('MessagePack cassettes not yet supported in Jest matchers');
  } else {
    throw new Error(`Cassette not found: ${name}`);
  }

  const content = fs.readFileSync(cassettePath, 'utf-8');
  return JSON.parse(content);
}

/**
 * Find a matching interaction in cassette
 * @param {Object} cassette - Loaded cassette
 * @param {Object} request - Request to match
 * @returns {Object|null} Matching interaction or null
 */
function findMatchingInteraction(cassette, request) {
  const { method, url, body } = request;

  for (const interaction of cassette.interactions) {
    if (interaction.kind !== 'Http') continue;

    const req = interaction.kind.Http.request;

    // Match method and URL
    if (req.method === method && req.url === url) {
      // If body is provided, match it too
      if (body !== undefined) {
        const reqBody = req.body ? Buffer.from(req.body).toString() : null;
        if (reqBody !== JSON.stringify(body)) {
          continue;
        }
      }

      return interaction;
    }
  }

  return null;
}

// Custom matchers
const matchers = {
  /**
   * Assert that response matches a cassette
   * @param {Object} response - HTTP response object
   * @param {string} cassetteName - Name of cassette to match against
   */
  toMatchCassette(response, cassetteName) {
    const cassette = loadCassette(cassetteName);

    // Extract request info from response (varies by HTTP client)
    const request = response.config || response.request || {};
    const method = (request.method || 'GET').toUpperCase();
    const url = request.url || '';

    const interaction = findMatchingInteraction(cassette, { method, url });

    if (!interaction) {
      return {
        pass: false,
        message: () =>
          `Expected to find matching interaction in cassette '${cassetteName}' for ${method} ${url}`,
      };
    }

    const expectedResponse = interaction.kind.Http.response;
    const actualStatus = response.status || response.statusCode;

    if (actualStatus !== expectedResponse.status) {
      return {
        pass: false,
        message: () =>
          `Expected status ${expectedResponse.status} but got ${actualStatus}`,
      };
    }

    return {
      pass: true,
      message: () => `Response matches cassette '${cassetteName}'`,
    };
  },

  /**
   * Assert that response status matches cassette
   * @param {Object} response - HTTP response object
   * @param {string} cassetteName - Name of cassette
   * @param {number} expectedStatus - Expected status code
   */
  toMatchCassetteStatus(response, cassetteName, expectedStatus) {
    const cassette = loadCassette(cassetteName);
    const request = response.config || response.request || {};
    const method = (request.method || 'GET').toUpperCase();
    const url = request.url || '';

    const interaction = findMatchingInteraction(cassette, { method, url });

    if (!interaction) {
      return {
        pass: false,
        message: () =>
          `Expected to find matching interaction in cassette '${cassetteName}'`,
      };
    }

    const actualStatus = interaction.kind.Http.response.status;

    return {
      pass: actualStatus === expectedStatus,
      message: () =>
        actualStatus === expectedStatus
          ? `Status matches expected ${expectedStatus}`
          : `Expected status ${expectedStatus} but cassette has ${actualStatus}`,
    };
  },

  /**
   * Assert that response body matches cassette
   * @param {Object} response - HTTP response object
   * @param {string} cassetteName - Name of cassette
   */
  toMatchCassetteBody(response, cassetteName) {
    const cassette = loadCassette(cassetteName);
    const request = response.config || response.request || {};
    const method = (request.method || 'GET').toUpperCase();
    const url = request.url || '';

    const interaction = findMatchingInteraction(cassette, { method, url });

    if (!interaction) {
      return {
        pass: false,
        message: () =>
          `Expected to find matching interaction in cassette '${cassetteName}'`,
      };
    }

    const expectedBody = interaction.kind.Http.response.body;
    const actualBody = response.data || response.body;

    let expectedBodyStr;
    if (expectedBody) {
      expectedBodyStr = Buffer.from(expectedBody).toString();
    }

    let actualBodyStr;
    if (typeof actualBody === 'string') {
      actualBodyStr = actualBody;
    } else if (actualBody) {
      actualBodyStr = JSON.stringify(actualBody);
    }

    return {
      pass: expectedBodyStr === actualBodyStr,
      message: () =>
        expectedBodyStr === actualBodyStr
          ? 'Body matches cassette'
          : `Expected body to match cassette but got differences`,
    };
  },

  /**
   * Assert that cassette has specific number of interactions
   * @param {string} cassetteName - Name of cassette
   * @param {number} expectedCount - Expected interaction count
   */
  toHaveInteractionCount(cassetteName, expectedCount) {
    const cassette = loadCassette(cassetteName);
    const actualCount = cassette.interactions.length;

    return {
      pass: actualCount === expectedCount,
      message: () =>
        actualCount === expectedCount
          ? `Cassette has ${expectedCount} interactions`
          : `Expected ${expectedCount} interactions but found ${actualCount}`,
    };
  },

  /**
   * Assert that cassette contains cookies
   * @param {string} cassetteName - Name of cassette
   */
  toHaveCookies(cassetteName) {
    const cassette = loadCassette(cassetteName);
    const hasCookies = cassette.cookies && cassette.cookies.length > 0;

    return {
      pass: hasCookies,
      message: () =>
        hasCookies
          ? `Cassette has ${cassette.cookies.length} cookies`
          : 'Cassette has no cookies',
    };
  },

  /**
   * Assert that cassette has specific cookie
   * @param {string} cassetteName - Name of cassette
   * @param {string} cookieName - Name of cookie to find
   */
  toHaveCookie(cassetteName, cookieName) {
    const cassette = loadCassette(cassetteName);

    if (!cassette.cookies) {
      return {
        pass: false,
        message: () => 'Cassette has no cookies',
      };
    }

    const cookie = cassette.cookies.find(c => c.name === cookieName);

    return {
      pass: !!cookie,
      message: () =>
        cookie
          ? `Cassette has cookie '${cookieName}'`
          : `Cassette does not have cookie '${cookieName}'`,
    };
  },

  /**
   * Assert that cassette version matches
   * @param {string} cassetteName - Name of cassette
   * @param {string} expectedVersion - Expected version (e.g., "2.0")
   */
  toHaveCassetteVersion(cassetteName, expectedVersion) {
    const cassette = loadCassette(cassetteName);
    const actualVersion = cassette.version;

    return {
      pass: actualVersion === expectedVersion,
      message: () =>
        actualVersion === expectedVersion
          ? `Cassette version is ${expectedVersion}`
          : `Expected version ${expectedVersion} but found ${actualVersion}`,
    };
  },
};

// Export for use with expect.extend()
module.exports = matchers;

// Auto-extend expect if in Jest environment
if (typeof expect !== 'undefined' && expect.extend) {
  expect.extend(matchers);
}
