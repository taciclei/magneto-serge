/**
 * @magneto-serge/jest - Jest integration for Magnéto-Serge
 *
 * Provides automatic cassette management for Jest tests, similar to VCR for RSpec.
 */

import { MagnetoProxy } from 'magneto-serge';
import type { Config } from '@jest/types';

/**
 * Cassette options for test configuration
 */
export interface CassetteOptions {
  /** Cassette name (auto-generated from test name if not provided) */
  name?: string;
  /** Recording mode: 'auto', 'record', 'replay', 'passthrough' */
  mode?: 'auto' | 'record' | 'replay' | 'passthrough';
  /** Cassette directory (overrides global config) */
  cassetteDir?: string;
  /** Proxy port (overrides global config) */
  port?: number;
  /** Record mode (VCR-compatible): 'new_episodes', 'once', 'all', 'none' */
  record?: 'new_episodes' | 'once' | 'all' | 'none';
}

/**
 * Global configuration for @magneto-serge/jest
 */
export interface MagnetoJestConfig {
  /** Directory where cassettes are stored */
  cassetteDir?: string;
  /** Default recording mode */
  mode?: 'auto' | 'record' | 'replay' | 'passthrough';
  /** Default proxy port */
  port?: number;
  /** Default record mode (VCR-compatible) */
  record?: 'new_episodes' | 'once' | 'all' | 'none';
  /** Whether to enable verbose logging */
  verbose?: boolean;
}

// Global configuration
let globalConfig: MagnetoJestConfig = {
  cassetteDir: '__cassettes__',
  mode: 'auto',
  port: 8888,
  record: 'new_episodes',
  verbose: false,
};

// Active proxy instance (if any)
let activeProxy: MagnetoProxy | null = null;
let activeCassetteName: string | null = null;

/**
 * Configure global settings for @magneto-serge/jest
 *
 * @example
 * ```typescript
 * import { configure } from '@magneto-serge/jest';
 *
 * configure({
 *   cassetteDir: 'tests/fixtures/cassettes',
 *   mode: 'auto',
 *   port: 8888,
 * });
 * ```
 */
export function configure(config: MagnetoJestConfig): void {
  globalConfig = { ...globalConfig, ...config };
}

/**
 * Get current global configuration
 */
export function getConfig(): Readonly<MagnetoJestConfig> {
  return { ...globalConfig };
}

/**
 * Reset configuration to defaults (mainly for testing)
 */
export function resetConfig(): void {
  globalConfig = {
    cassetteDir: '__cassettes__',
    mode: 'auto',
    port: 8888,
    record: 'new_episodes',
    verbose: false,
  };
}

/**
 * Translate VCR record mode to Magneto mode
 */
function translateRecordMode(record?: 'new_episodes' | 'once' | 'all' | 'none'): 'auto' | 'record' | 'replay' {
  switch (record) {
    case 'new_episodes':
      return 'auto';
    case 'once':
      return 'replay';
    case 'all':
      return 'record';
    case 'none':
      return 'replay';
    default:
      return 'auto';
  }
}

/**
 * Generate cassette name from test context
 */
function generateCassetteName(): string {
  // Get current test name from Jest's expect.getState()
  const state = (expect as any).getState();

  if (!state || !state.currentTestName) {
    return 'default_cassette';
  }

  // Sanitize test name for filename
  const testName = state.currentTestName
    .replace(/[^\w\s\-]/g, '_') // Replace special chars with underscore
    .replace(/\s+/g, '_')       // Replace spaces with underscore
    .replace(/_+/g, '_')        // Collapse multiple underscores
    .toLowerCase();

  return testName;
}

/**
 * Start a cassette for the current test
 *
 * @param options - Cassette options
 * @returns Promise that resolves when cassette is active
 */
async function startCassette(options: CassetteOptions = {}): Promise<void> {
  // Merge options with global config
  const cassetteDir = options.cassetteDir || globalConfig.cassetteDir || '__cassettes__';
  const port = options.port || globalConfig.port || 8888;

  // Determine mode
  let mode: 'auto' | 'record' | 'replay' | 'passthrough';
  if (options.mode) {
    mode = options.mode;
  } else if (options.record) {
    mode = translateRecordMode(options.record);
  } else if (globalConfig.mode) {
    mode = globalConfig.mode;
  } else {
    mode = 'auto';
  }

  // Generate or use provided cassette name
  const cassetteName = options.name || generateCassetteName();
  activeCassetteName = cassetteName;

  if (globalConfig.verbose) {
    console.log(`[magneto-serge] Starting cassette: ${cassetteName} (mode: ${mode})`);
  }

  // Create proxy instance
  activeProxy = new MagnetoProxy(cassetteDir);

  // Start proxy in appropriate mode
  switch (mode) {
    case 'auto':
      await activeProxy.auto(cassetteName);
      break;
    case 'record':
      await activeProxy.record(cassetteName);
      break;
    case 'replay':
      await activeProxy.replay(cassetteName);
      break;
    case 'passthrough':
      await activeProxy.passthrough();
      break;
  }
}

/**
 * Stop the active cassette
 */
async function stopCassette(): Promise<void> {
  if (activeProxy) {
    if (globalConfig.verbose) {
      console.log(`[magneto-serge] Stopping cassette: ${activeCassetteName}`);
    }

    await activeProxy.stop();
    activeProxy = null;
    activeCassetteName = null;
  }
}

/**
 * Get the currently active cassette name
 */
export function getCurrentCassette(): string | null {
  return activeCassetteName;
}

/**
 * Wrapper function for tests with automatic cassette management
 *
 * @example
 * ```typescript
 * import { magnetoTest } from '@magneto-serge/jest';
 *
 * magnetoTest('should fetch users', async () => {
 *   const response = await fetch('https://api.example.com/users');
 *   expect(response.status).toBe(200);
 * });
 *
 * // With custom options
 * magnetoTest('custom cassette', { name: 'shared', mode: 'replay' }, async () => {
 *   // Test code...
 * });
 * ```
 */
export function magnetoTest(
  name: string,
  optionsOrFn: CassetteOptions | (() => void | Promise<void>),
  maybeFn?: () => void | Promise<void>
): void {
  let options: CassetteOptions = {};
  let testFn: () => void | Promise<void>;

  // Parse arguments
  if (typeof optionsOrFn === 'function') {
    testFn = optionsOrFn;
  } else {
    options = optionsOrFn;
    testFn = maybeFn!;
  }

  // Wrap test function with cassette management
  test(name, async () => {
    try {
      await startCassette({ ...options, name: options.name || name });
      await testFn();
    } finally {
      await stopCassette();
    }
  });
}

/**
 * Wrapper function for describe blocks with automatic cassette management
 *
 * @example
 * ```typescript
 * import { magnetoDescribe } from '@magneto-serge/jest';
 *
 * magnetoDescribe('API Tests', { cassetteDir: 'api_cassettes' }, () => {
 *   test('fetches users', async () => {
 *     // Cassette auto-managed
 *   });
 * });
 * ```
 */
export function magnetoDescribe(
  name: string,
  optionsOrFn: CassetteOptions | (() => void),
  maybeFn?: () => void
): void {
  let options: CassetteOptions = {};
  let suiteFn: () => void;

  // Parse arguments
  if (typeof optionsOrFn === 'function') {
    suiteFn = optionsOrFn;
  } else {
    options = optionsOrFn;
    suiteFn = maybeFn!;
  }

  describe(name, () => {
    beforeEach(async () => {
      await startCassette(options);
    });

    afterEach(async () => {
      await stopCassette();
    });

    suiteFn();
  });
}

/**
 * Manual cassette control - use within a test
 *
 * @example
 * ```typescript
 * import { useCassette } from '@magneto-serge/jest';
 *
 * test('manual cassette control', async () => {
 *   await useCassette('my_cassette', async () => {
 *     const response = await fetch('https://api.example.com/data');
 *     expect(response.status).toBe(200);
 *   });
 * });
 * ```
 */
export async function useCassette(
  nameOrOptions: string | CassetteOptions,
  fn: () => void | Promise<void>
): Promise<void> {
  const options = typeof nameOrOptions === 'string'
    ? { name: nameOrOptions }
    : nameOrOptions;

  try {
    await startCassette(options);
    await fn();
  } finally {
    await stopCassette();
  }
}

/**
 * Setup function to configure Jest environment (optional)
 * Add to your Jest setup file (setupFilesAfterEnv)
 */
export function setupMagneto(config?: MagnetoJestConfig): void {
  if (config) {
    configure(config);
  }

  // Ensure cleanup after each test
  afterEach(async () => {
    if (activeProxy) {
      await stopCassette();
    }
  });
}

// Default export
export default {
  configure,
  getConfig,
  resetConfig,
  magnetoTest,
  magnetoDescribe,
  useCassette,
  setupMagneto,
  getCurrentCassette,
};
