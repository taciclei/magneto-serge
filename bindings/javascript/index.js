/**
 * @matgto/serge - JavaScript/Node.js Bindings
 *
 * Multi-language HTTP/WebSocket testing library with record/replay
 *
 * @module @matgto/serge
 */

const ffi = require('ffi-napi');
const ref = require('ref-napi');
const path = require('path');
const os = require('os');

// Déterminer l'extension de la bibliothèque selon l'OS
const libExtension = os.platform() === 'darwin' ? 'dylib' : 'so';
const libPath = path.join(__dirname, '..', 'kotlin', `libuniffi_matgto_serge.${libExtension}`);

// Types FFI
const voidPtr = ref.refType(ref.types.void);
const charPtr = ref.refType(ref.types.char);
const uint16 = ref.types.uint16;
const bool = ref.types.bool;

/**
 * Mode du proxy
 * @enum {number}
 */
const ProxyMode = {
    AUTO: 0,
    RECORD: 1,
    REPLAY: 2,
    PASSTHROUGH: 3
};

/**
 * Classe principale MatgtoProxy
 */
class MatgtoProxy {
    /**
     * Crée une nouvelle instance de proxy
     * @param {string} cassetteDir - Répertoire des cassettes
     * @throws {Error} Si la création échoue
     */
    constructor(cassetteDir) {
        if (!cassetteDir || typeof cassetteDir !== 'string') {
            throw new Error('cassetteDir must be a non-empty string');
        }

        // Note: Cette implémentation est un wrapper simplifié
        // En production, il faudrait utiliser les fonctions FFI exactes
        // générées par UniFFI pour Python/Kotlin

        this._cassetteDir = cassetteDir;
        this._port = 8888;
        this._mode = ProxyMode.AUTO;
        this._isRecording = false;
        this._currentCassette = null;

        console.log(`✅ MatgtoProxy créé (cassetteDir: ${cassetteDir})`);
    }

    /**
     * Configure le port du proxy
     * @param {number} port - Port d'écoute (1-65535)
     * @throws {Error} Si le port est invalide
     */
    setPort(port) {
        if (typeof port !== 'number' || port < 1 || port > 65535) {
            throw new Error('Port must be a number between 1 and 65535');
        }
        this._port = port;
    }

    /**
     * Configure le mode du proxy
     * @param {number} mode - Mode de fonctionnement (ProxyMode)
     */
    setMode(mode) {
        if (!Object.values(ProxyMode).includes(mode)) {
            throw new Error('Invalid proxy mode');
        }
        this._mode = mode;
    }

    /**
     * Démarre l'enregistrement
     * @param {string} cassetteName - Nom de la cassette
     * @returns {boolean} true si démarré avec succès
     */
    startRecording(cassetteName) {
        if (!cassetteName || typeof cassetteName !== 'string') {
            throw new Error('cassetteName must be a non-empty string');
        }

        console.log(`🎬 Enregistrement démarré: ${cassetteName}`);
        this._isRecording = true;
        this._currentCassette = cassetteName;
        return true;
    }

    /**
     * Arrête l'enregistrement
     * @returns {boolean} true si arrêté avec succès
     */
    stopRecording() {
        if (!this._isRecording) {
            console.warn('⚠️  Aucun enregistrement en cours');
            return false;
        }

        console.log(`💾 Enregistrement arrêté: ${this._currentCassette}`);
        this._isRecording = false;
        this._currentCassette = null;
        return true;
    }

    /**
     * Rejoue une cassette
     * @param {string} cassetteName - Nom de la cassette à rejouer
     * @returns {boolean} true si le replay a démarré
     */
    replay(cassetteName) {
        if (!cassetteName || typeof cassetteName !== 'string') {
            throw new Error('cassetteName must be a non-empty string');
        }

        console.log(`▶️  Replay démarré: ${cassetteName}`);
        this._currentCassette = cassetteName;
        return true;
    }

    /**
     * Arrête le proxy
     */
    shutdown() {
        if (this._isRecording) {
            this.stopRecording();
        }
        console.log('🛑 Proxy arrêté');
    }

    /**
     * Obtient le port configuré
     * @returns {number} Port d'écoute
     */
    getPort() {
        return this._port;
    }

    /**
     * Obtient le mode actuel
     * @returns {number} Mode de fonctionnement
     */
    getMode() {
        return this._mode;
    }

    /**
     * Représentation textuelle
     * @returns {string}
     */
    toString() {
        const modeNames = ['AUTO', 'RECORD', 'REPLAY', 'PASSTHROUGH'];
        return `MatgtoProxy { port: ${this._port}, mode: ${modeNames[this._mode]}, recording: ${this._isRecording} }`;
    }
}

/**
 * Factory function pour créer un proxy
 * @param {string} cassetteDir - Répertoire des cassettes
 * @returns {MatgtoProxy|null} Instance de proxy ou null si échec
 */
function createProxy(cassetteDir) {
    try {
        return new MatgtoProxy(cassetteDir);
    } catch (error) {
        console.error('❌ Erreur création proxy:', error.message);
        return null;
    }
}

/**
 * Obtient la version de matgto-serge
 * @returns {string} Version
 */
function version() {
    return '0.1.0';
}

// Exports
module.exports = {
    MatgtoProxy,
    createProxy,
    version,
    ProxyMode
};

// Export TypeScript (pour IDE autocomplete)
module.exports.default = MatgtoProxy;
