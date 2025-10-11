/**
 * Magnéto-Serge - Multi-language HTTP/WebSocket proxy library with record/replay
 *
 * @packageDocumentation
 */

/**
 * Proxy operation mode
 */
export enum ProxyMode {
  /**
   * Auto mode: Record if cassette doesn't exist, otherwise replay
   */
  Auto = 0,

  /**
   * Always record (overwrites existing cassette)
   */
  Record = 1,

  /**
   * Always replay (errors if cassette doesn't exist)
   */
  Replay = 2,

  /**
   * Transparent proxy without record/replay
   */
  Passthrough = 3
}

/**
 * Main proxy class for HTTP/HTTPS and WebSocket record/replay
 *
 * @example
 * ```typescript
 * const proxy = new MagnetoProxy('./cassettes');
 * proxy.setPort(8888);
 * proxy.setMode(ProxyMode.Auto);
 * proxy.startRecording('my-test');
 *
 * // Make your HTTP requests through localhost:8888
 *
 * proxy.stopRecording();
 * proxy.shutdown();
 * ```
 */
export class MagnetoProxy {
  /**
   * Create a new proxy instance
   *
   * @param cassetteDir - Directory where cassettes will be stored
   * @throws Error if the directory cannot be created or accessed
   *
   * @example
   * ```typescript
   * const proxy = new MagnetoProxy('./test-cassettes');
   * ```
   */
  constructor(cassetteDir: string);

  /**
   * Set the proxy listening port
   *
   * @param port - Port number (1-65535)
   *
   * @example
   * ```typescript
   * proxy.setPort(9999);
   * ```
   */
  setPort(port: number): void;

  /**
   * Set the proxy operation mode
   *
   * @param mode - One of: Auto, Record, Replay, Passthrough
   *
   * @example
   * ```typescript
   * proxy.setMode(ProxyMode.Record);
   * ```
   */
  setMode(mode: ProxyMode): void;

  /**
   * Get the current proxy port
   *
   * @returns Current listening port
   *
   * @example
   * ```typescript
   * const port = proxy.getPort(); // 8888 (default)
   * ```
   */
  getPort(): number;

  /**
   * Start recording a new cassette
   *
   * @param cassetteName - Name of the cassette (without .json extension)
   * @throws Error if recording cannot be started
   *
   * @example
   * ```typescript
   * proxy.startRecording('github-api-test');
   * // Make HTTP requests...
   * proxy.stopRecording();
   * ```
   */
  startRecording(cassetteName: string): void;

  /**
   * Stop recording and save the cassette to disk
   *
   * @throws Error if no recording is in progress or save fails
   *
   * @example
   * ```typescript
   * proxy.stopRecording();
   * // Cassette saved to ./cassettes/my-test.json
   * ```
   */
  stopRecording(): void;

  /**
   * Replay an existing cassette
   *
   * @param cassetteName - Name of the cassette to replay (without .json extension)
   * @throws Error if cassette doesn't exist or cannot be loaded
   *
   * @example
   * ```typescript
   * proxy.setMode(ProxyMode.Replay);
   * proxy.replay('github-api-test');
   * // HTTP requests will be served from cassette
   * ```
   */
  replay(cassetteName: string): void;

  /**
   * Shutdown the proxy and cleanup resources
   *
   * @example
   * ```typescript
   * proxy.shutdown();
   * ```
   */
  shutdown(): void;
}

/**
 * Get the library version
 *
 * @returns Version string (e.g., "0.0.1")
 *
 * @example
 * ```typescript
 * import { version } from '@taciclei/magneto-serge';
 * console.log('Magneto-Serge version:', version());
 * ```
 */
export function version(): string;

/**
 * Re-export ProxyMode enum
 */
export { ProxyMode as default };
