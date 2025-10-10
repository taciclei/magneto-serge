/**
 * Exemple basique d'utilisation de matgto-serge en JavaScript
 */

const { MatgtoProxy, createProxy, version, ProxyMode } = require('../index');

console.log('='.repeat(60));
console.log('🧪 Exemple matgto-serge JavaScript');
console.log('='.repeat(60));
console.log();

// Exemple 1: Utilisation basique
console.log('📝 Exemple 1: Utilisation basique');
console.log('-'.repeat(60));

const proxy = createProxy('./cassettes');

if (!proxy) {
    console.error('❌ Impossible de créer le proxy');
    process.exit(1);
}

console.log('✅ Proxy créé:', proxy.toString());

// Configurer le port
proxy.setPort(8888);
console.log('  Port configuré:', proxy.getPort());

// Configurer le mode
proxy.setMode(ProxyMode.RECORD);
console.log('  Mode configuré:', proxy.getMode());

// Afficher la version
console.log('  Version:', version());

console.log();

// Exemple 2: Enregistrement
console.log('📝 Exemple 2: Enregistrement et Replay');
console.log('-'.repeat(60));

// Démarrer l'enregistrement
if (proxy.startRecording('javascript_test')) {
    console.log('🎬 Enregistrement démarré');

    // Simuler des requêtes HTTP
    // (En production, vous utiliseriez fetch ou axios via le proxy)
    console.log('  📡 Simulation de requêtes HTTP...');

    // Arrêter l'enregistrement
    if (proxy.stopRecording()) {
        console.log('💾 Enregistrement arrêté et cassette sauvegardée');
    }
}

console.log();

// Exemple 3: Replay
console.log('📝 Exemple 3: Replay');
console.log('-'.repeat(60));

proxy.setMode(ProxyMode.REPLAY);

if (proxy.replay('javascript_test')) {
    console.log('▶️  Replay démarré');
    console.log('  Les requêtes seront rejouées depuis la cassette');
}

console.log();

// Shutdown
proxy.shutdown();

console.log('✅ Tous les exemples terminés !');
