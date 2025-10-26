/**
 * Tests for @magneto-serge/jest
 */

import { configure, getConfig, resetConfig, getCurrentCassette } from './index';

describe('@magneto-serge/jest', () => {
  beforeEach(() => {
    resetConfig();
  });

  describe('configuration', () => {
    it('should have default configuration', () => {
      const config = getConfig();
      expect(config.cassetteDir).toBe('__cassettes__');
      expect(config.mode).toBe('auto');
      expect(config.port).toBe(8888);
      expect(config.record).toBe('new_episodes');
      expect(config.verbose).toBe(false);
    });

    it('should allow configuring cassette directory', () => {
      configure({ cassetteDir: 'custom/path' });
      const config = getConfig();
      expect(config.cassetteDir).toBe('custom/path');
    });

    it('should allow configuring mode', () => {
      configure({ mode: 'replay' });
      const config = getConfig();
      expect(config.mode).toBe('replay');
    });

    it('should allow configuring port', () => {
      configure({ port: 9999 });
      const config = getConfig();
      expect(config.port).toBe(9999);
    });

    it('should allow configuring record mode', () => {
      configure({ record: 'all' });
      const config = getConfig();
      expect(config.record).toBe('all');
    });

    it('should allow configuring verbose logging', () => {
      configure({ verbose: true });
      const config = getConfig();
      expect(config.verbose).toBe(true);
    });

    it('should merge configuration options', () => {
      configure({ cassetteDir: 'path1', port: 9000 });
      configure({ mode: 'record' });

      const config = getConfig();
      expect(config.cassetteDir).toBe('path1');
      expect(config.port).toBe(9000);
      expect(config.mode).toBe('record');
    });

    it('should reset configuration to defaults', () => {
      configure({ cassetteDir: 'custom', port: 9999, verbose: true });
      resetConfig();

      const config = getConfig();
      expect(config.cassetteDir).toBe('__cassettes__');
      expect(config.port).toBe(8888);
      expect(config.verbose).toBe(false);
    });
  });

  describe('getCurrentCassette', () => {
    it('should return null when no cassette is active', () => {
      expect(getCurrentCassette()).toBeNull();
    });
  });
});
