/**
 * Exemple de tests Jest avec matgto-serge
 */

const { MagnetoProxy, createProxy, version, ProxyMode } = require('../index');
const https = require('https');
const http = require('http');

describe('MagnetoProxy', () => {
    let proxy;

    beforeEach(() => {
        proxy = createProxy('./test_cassettes');
    });

    afterEach(() => {
        if (proxy) {
            proxy.shutdown();
        }
    });

    describe('Creation', () => {
        test('should create proxy successfully', () => {
            expect(proxy).toBeTruthy();
            expect(proxy).toBeInstanceOf(MagnetoProxy);
        });

        test('should create proxy with factory function', () => {
            const newProxy = createProxy('./cassettes');
            expect(newProxy).toBeTruthy();
            newProxy.shutdown();
        });

        test('should return null for invalid cassette dir', () => {
            const invalidProxy = createProxy('');
            expect(invalidProxy).toBeNull();
        });
    });

    describe('Configuration', () => {
        test('should set port correctly', () => {
            proxy.setPort(9999);
            expect(proxy.getPort()).toBe(9999);
        });

        test('should throw error for invalid port', () => {
            expect(() => proxy.setPort(0)).toThrow();
            expect(() => proxy.setPort(70000)).toThrow();
            expect(() => proxy.setPort('8888')).toThrow();
        });

        test('should set mode correctly', () => {
            proxy.setMode(ProxyMode.RECORD);
            expect(proxy.getMode()).toBe(ProxyMode.RECORD);

            proxy.setMode(ProxyMode.REPLAY);
            expect(proxy.getMode()).toBe(ProxyMode.REPLAY);

            proxy.setMode(ProxyMode.AUTO);
            expect(proxy.getMode()).toBe(ProxyMode.AUTO);

            proxy.setMode(ProxyMode.PASSTHROUGH);
            expect(proxy.getMode()).toBe(ProxyMode.PASSTHROUGH);
        });
    });

    describe('Recording', () => {
        test('should start recording', () => {
            proxy.setMode(ProxyMode.RECORD);
            const result = proxy.startRecording('test_cassette');
            expect(result).toBe(true);
        });

        test('should stop recording', () => {
            proxy.startRecording('test_cassette');
            const result = proxy.stopRecording();
            expect(result).toBe(true);
        });

        test('should return false when stopping without recording', () => {
            const result = proxy.stopRecording();
            expect(result).toBe(false);
        });
    });

    describe('Replay', () => {
        test('should start replay', () => {
            proxy.setMode(ProxyMode.REPLAY);
            const result = proxy.replay('test_cassette');
            expect(result).toBe(true);
        });

        test('should throw error for invalid cassette name', () => {
            expect(() => proxy.replay('')).toThrow();
        });
    });

    describe('Version', () => {
        test('should return version', () => {
            const v = version();
            expect(v).toBeTruthy();
            expect(typeof v).toBe('string');
            expect(v).toBe('0.1.0');
        });
    });

    describe('toString', () => {
        test('should have meaningful toString', () => {
            proxy.setPort(8888);
            proxy.setMode(ProxyMode.RECORD);

            const str = proxy.toString();
            expect(str).toContain('8888');
            expect(str).toContain('RECORD');
        });
    });
});

describe('HTTP Integration', () => {
    let proxy;

    beforeEach(() => {
        proxy = createProxy('./test_cassettes');
        proxy.setPort(8888);
    });

    afterEach(() => {
        if (proxy) {
            proxy.shutdown();
        }
    });

    test('should configure for HTTP requests', () => {
        proxy.setMode(ProxyMode.RECORD);
        proxy.startRecording('http_test');

        // Configuration du proxy pour les requêtes HTTP
        const proxyConfig = {
            host: 'localhost',
            port: proxy.getPort()
        };

        expect(proxyConfig.port).toBe(8888);

        proxy.stopRecording();
    });

    test('full record and replay cycle', () => {
        const cassetteName = 'full_cycle_test';

        // Phase 1: Record
        proxy.setMode(ProxyMode.RECORD);
        expect(proxy.startRecording(cassetteName)).toBe(true);
        // ... faire des requêtes HTTP ...
        expect(proxy.stopRecording()).toBe(true);

        // Phase 2: Replay
        proxy.setMode(ProxyMode.REPLAY);
        expect(proxy.replay(cassetteName)).toBe(true);
        // ... rejouer les requêtes ...
    });
});
