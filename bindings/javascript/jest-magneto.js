// jest-magneto.js - Plugin Jest pour magneto-serge
// Enregistrement et rejeu automatique des interactions HTTP/WebSocket dans les tests Jest

const { MagnetoProxy, ProxyMode } = require('./index');
const path = require('path');
const fs = require('fs');

/**
 * Configuration globale du plugin
 */
const defaultConfig = {
  cassetteDir: './test_cassettes',
  mode: 'auto', // auto | record | replay | strict
  port: 8888,
  disabled: false,
};

let globalConfig = { ...defaultConfig };
let globalProxy = null;

/**
 * Setup global pour Jest
 * @param {object} jestConfig - Configuration Jest
 */
function setup(jestConfig = {}) {
  const magnetoConfig = jestConfig.magneto || {};
  globalConfig = { ...defaultConfig, ...magnetoConfig };

  // Créer le répertoire de cassettes si nécessaire
  if (!fs.existsSync(globalConfig.cassetteDir)) {
    fs.mkdirSync(globalConfig.cassetteDir, { recursive: true });
  }
}

/**
 * Teardown global pour Jest
 */
function teardown() {
  if (globalProxy) {
    globalProxy.shutdown();
    globalProxy = null;
  }
}

/**
 * Fixture magneto pour les tests
 * @param {string} cassetteName - Nom de la cassette (optionnel)
 * @param {object} options - Options de configuration
 * @returns {object} Instance MagnetoProxy configurée
 */
function useMagneto(cassetteName = null, options = {}) {
  if (globalConfig.disabled) {
    return null;
  }

  const config = { ...globalConfig, ...options };
  const proxy = new MagnetoProxy(config.cassetteDir);
  proxy.setPort(config.port);

  // Déterminer le nom de la cassette
  const finalCassetteName = cassetteName || expect.getState().currentTestName.replace(/\s+/g, '-');

  // Configurer le mode
  switch (config.mode) {
    case 'record':
      proxy.setMode(ProxyMode.RECORD);
      proxy.startRecording(finalCassetteName);
      break;
    case 'replay':
      proxy.setMode(ProxyMode.REPLAY);
      proxy.replay(finalCassetteName);
      break;
    case 'strict':
      proxy.replayStrict(finalCassetteName);
      break;
    case 'auto':
    default:
      proxy.hybrid(finalCassetteName);
      break;
  }

  return proxy;
}

/**
 * Helper pour obtenir la configuration proxy pour axios/fetch
 * @param {MagnetoProxy} proxy - Instance proxy
 * @returns {object} Configuration proxy
 */
function getProxyConfig(proxy) {
  const port = proxy.port();
  return {
    host: 'localhost',
    port: port,
    protocol: 'http',
  };
}

/**
 * Helper pour obtenir la configuration proxy pour node-fetch
 * @param {MagnetoProxy} proxy - Instance proxy
 * @returns {string} URL du proxy
 */
function getProxyUrl(proxy) {
  const port = proxy.port();
  return `http://localhost:${port}`;
}

/**
 * Matcher Jest personnalisé pour vérifier les cassettes
 */
expect.extend({
  toHaveCassette(cassetteName) {
    const cassettePath = path.join(globalConfig.cassetteDir, `${cassetteName}.json`);
    const exists = fs.existsSync(cassettePath);

    return {
      pass: exists,
      message: () => exists
        ? `Expected cassette "${cassetteName}" not to exist, but it does`
        : `Expected cassette "${cassetteName}" to exist, but it doesn't`,
    };
  },
});

/**
 * Hook beforeAll global (optionnel)
 */
beforeAll(() => {
  // Créer le répertoire de cassettes
  if (!fs.existsSync(globalConfig.cassetteDir)) {
    fs.mkdirSync(globalConfig.cassetteDir, { recursive: true });
  }
});

/**
 * Hook afterAll global (optionnel)
 */
afterAll(() => {
  if (globalProxy) {
    globalProxy.shutdown();
    globalProxy = null;
  }
});

module.exports = {
  setup,
  teardown,
  useMagneto,
  getProxyConfig,
  getProxyUrl,
};
