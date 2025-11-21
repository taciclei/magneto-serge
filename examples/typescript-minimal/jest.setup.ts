/**
 * Jest setup file for Magneto-Serge integration
 *
 * This file is executed before all tests to configure the recording/replay behavior.
 */

// Configuration for magneto-serge
// Note: Since @magneto-serge/jest is not yet published, we use the base API directly

const cassetteDir = '__cassettes__';
const proxyPort = 8888;

// Configure HTTP client to use proxy (if using fetch or axios)
process.env.HTTP_PROXY = `http://localhost:${proxyPort}`;
process.env.HTTPS_PROXY = `http://localhost:${proxyPort}`;

// Set mode from environment variable
const mode = process.env.MAGNETO_MODE || 'auto';

console.log(`🎬 Magneto-Serge configured:
  Mode: ${mode}
  Cassette Directory: ${cassetteDir}
  Proxy Port: ${proxyPort}
`);

// Global test timeout (for recording from real APIs)
jest.setTimeout(30000);
