/**
 * @magneto/serge - TypeScript Definitions
 *
 * Multi-language HTTP/WebSocket testing library with record/replay
 */

/**
 * Mode de fonctionnement du proxy
 */
export enum ProxyMode {
    /** Mode automatique: record si cassette n'existe pas, sinon replay */
    AUTO = 0,
    /** Mode enregistrement: enregistre toutes les requêtes */
    RECORD = 1,
    /** Mode replay: rejoue depuis la cassette */
    REPLAY = 2,
    /** Mode passthrough: transparent sans enregistrement */
    PASSTHROUGH = 3
}

/**
 * Classe principale du proxy matgto-serge
 */
export class MagnetoProxy {
    /**
     * Crée une nouvelle instance de proxy
     * @param cassetteDir Répertoire où stocker les cassettes
     * @throws {Error} Si cassetteDir est invalide
     */
    constructor(cassetteDir: string);

    /**
     * Configure le port d'écoute du proxy
     * @param port Port (1-65535)
     * @throws {Error} Si le port est invalide
     */
    setPort(port: number): void;

    /**
     * Configure le mode de fonctionnement
     * @param mode Mode du proxy
     */
    setMode(mode: ProxyMode): void;

    /**
     * Démarre l'enregistrement d'une cassette
     * @param cassetteName Nom de la cassette
     * @returns true si démarré avec succès
     */
    startRecording(cassetteName: string): boolean;

    /**
     * Arrête l'enregistrement en cours
     * @returns true si arrêté avec succès
     */
    stopRecording(): boolean;

    /**
     * Rejoue une cassette enregistrée
     * @param cassetteName Nom de la cassette à rejouer
     * @returns true si le replay a démarré
     */
    replay(cassetteName: string): boolean;

    /**
     * Arrête le proxy proprement
     */
    shutdown(): void;

    /**
     * Obtient le port configuré
     * @returns Port d'écoute
     */
    getPort(): number;

    /**
     * Obtient le mode actuel
     * @returns Mode de fonctionnement
     */
    getMode(): ProxyMode;

    /**
     * Représentation textuelle du proxy
     */
    toString(): string;
}

/**
 * Factory function pour créer un proxy
 * @param cassetteDir Répertoire des cassettes
 * @returns Instance de MagnetoProxy ou null si échec
 */
export function createProxy(cassetteDir: string): MagnetoProxy | null;

/**
 * Obtient la version de matgto-serge
 * @returns Version (ex: "0.1.0")
 */
export function version(): string;

export default MagnetoProxy;
