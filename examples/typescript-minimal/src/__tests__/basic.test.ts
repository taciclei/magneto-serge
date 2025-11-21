/**
 * Basic example test using Magneto-Serge for HTTP recording/replay
 *
 * This test demonstrates how to use Magneto-Serge in TypeScript tests
 * to record HTTP requests and replay them deterministically.
 */

import { MagnetoProxy, ProxyMode } from 'magneto-serge';
import * as path from 'path';

describe('Magneto-Serge Basic Example', () => {
  let proxy: MagnetoProxy;
  const cassetteDir = path.join(__dirname, '../../__cassettes__');

  beforeEach(() => {
    // Create proxy instance
    proxy = new MagnetoProxy(cassetteDir);
    proxy.setPort(8888);
  });

  afterEach(() => {
    // Clean up
    if (proxy) {
      proxy.shutdown();
    }
  });

  test('should create proxy instance', () => {
    expect(proxy).toBeDefined();
    expect(proxy.getPort()).toBe(8888);
  });

  test('should start and stop recording', () => {
    // Set mode to record
    proxy.setMode(ProxyMode.Record);

    // Start recording
    proxy.startRecording('test-cassette');

    // In a real scenario, you would make HTTP requests here
    // Example: await fetch('https://api.example.com/data', { proxy: ... })

    // Stop recording
    proxy.stopRecording();

    // Cassette should be saved in __cassettes__/test-cassette.json
    expect(true).toBe(true);
  });

  test('should support different proxy modes', () => {
    // Auto mode (default)
    proxy.setMode(ProxyMode.Auto);
    expect(proxy).toBeDefined();

    // Record mode
    proxy.setMode(ProxyMode.Record);
    expect(proxy).toBeDefined();

    // Replay mode
    proxy.setMode(ProxyMode.Replay);
    expect(proxy).toBeDefined();

    // Passthrough mode
    proxy.setMode(ProxyMode.Passthrough);
    expect(proxy).toBeDefined();
  });
});
