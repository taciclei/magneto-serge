package io.github.magneto.serge;

import uniffi.matgto_serge.ProxyMode;

/**
 * Java wrapper pour matgto-serge
 *
 * Cette classe wraps les bindings Kotlin générés par UniFFI
 * pour une utilisation native depuis Java.
 *
 * @author matgto-serge contributors
 * @version 0.1.0
 */
public class MagnetoProxy {

    private final uniffi.matgto_serge.MagnetoProxy kotlinProxy;

    /**
     * Mode du proxy
     */
    public enum Mode {
        AUTO(ProxyMode.AUTO),
        RECORD(ProxyMode.RECORD),
        REPLAY(ProxyMode.REPLAY),
        PASSTHROUGH(ProxyMode.PASSTHROUGH);

        private final ProxyMode kotlinMode;

        Mode(ProxyMode kotlinMode) {
            this.kotlinMode = kotlinMode;
        }

        ProxyMode toKotlin() {
            return kotlinMode;
        }

        static Mode fromKotlin(ProxyMode kotlinMode) {
            for (Mode mode : values()) {
                if (mode.kotlinMode == kotlinMode) {
                    return mode;
                }
            }
            throw new IllegalArgumentException("Unknown ProxyMode: " + kotlinMode);
        }
    }

    /**
     * Crée un nouveau proxy MagnetoProxy
     *
     * @param cassetteDir Répertoire des cassettes
     * @throws IllegalStateException si la création échoue
     */
    public MagnetoProxy(String cassetteDir) {
        uniffi.matgto_serge.MagnetoProxy proxy =
            uniffi.matgto_serge.MatgtoSergeKt.createProxy(cassetteDir);

        if (proxy == null) {
            throw new IllegalStateException(
                "Failed to create MagnetoProxy with cassetteDir: " + cassetteDir
            );
        }

        this.kotlinProxy = proxy;
    }

    /**
     * Configure le port du proxy
     *
     * @param port Port d'écoute (ex: 8888)
     */
    public void setPort(int port) {
        if (port < 1 || port > 65535) {
            throw new IllegalArgumentException("Port must be between 1 and 65535");
        }
        kotlinProxy.setPort(port);
    }

    /**
     * Configure le mode du proxy
     *
     * @param mode Mode de fonctionnement (AUTO, RECORD, REPLAY, PASSTHROUGH)
     */
    public void setMode(Mode mode) {
        kotlinProxy.setMode(mode.toKotlin());
    }

    /**
     * Démarre l'enregistrement d'une cassette
     *
     * @param cassetteName Nom de la cassette
     * @return true si l'enregistrement a démarré, false sinon
     */
    public boolean startRecording(String cassetteName) {
        return kotlinProxy.startRecording(cassetteName);
    }

    /**
     * Arrête l'enregistrement en cours
     *
     * @return true si arrêté avec succès, false sinon
     */
    public boolean stopRecording() {
        return kotlinProxy.stopRecording();
    }

    /**
     * Rejoue une cassette enregistrée
     *
     * @param cassetteName Nom de la cassette à rejouer
     * @return true si le replay a démarré, false sinon
     */
    public boolean replay(String cassetteName) {
        return kotlinProxy.replay(cassetteName);
    }

    /**
     * Arrête le proxy
     */
    public void shutdown() {
        kotlinProxy.shutdown();
    }

    /**
     * Obtient le port configuré
     *
     * @return Port d'écoute
     */
    public int getPort() {
        return kotlinProxy.port();
    }

    /**
     * Obtient le mode actuel
     *
     * @return Mode de fonctionnement
     */
    public Mode getMode() {
        return Mode.fromKotlin(kotlinProxy.mode());
    }

    /**
     * Obtient la version de matgto-serge
     *
     * @return Version (ex: "0.1.0")
     */
    public static String getVersion() {
        return uniffi.matgto_serge.MatgtoSergeKt.version();
    }

    /**
     * Factory method pour créer un proxy
     *
     * @param cassetteDir Répertoire des cassettes
     * @return Instance de MagnetoProxy ou null si échec
     */
    public static MagnetoProxy create(String cassetteDir) {
        try {
            return new MagnetoProxy(cassetteDir);
        } catch (IllegalStateException e) {
            return null;
        }
    }

    @Override
    public String toString() {
        return "MagnetoProxy{" +
               "port=" + getPort() +
               ", mode=" + getMode() +
               ", version=" + getVersion() +
               '}';
    }
}
