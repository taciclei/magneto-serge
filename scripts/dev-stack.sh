#!/bin/bash
# Démarrer la stack complète de développement Magnéto-Serge
# Backend Hydra API + Frontend Angular

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🚀 Magnéto-Serge Development Stack"
echo "=================================="
echo ""
echo "Démarrage de:"
echo "  - Backend Hydra API (port 8889)"
echo "  - Frontend Angular (port 4201)"
echo ""

# Vérifier que nous sommes dans le bon répertoire
if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
    echo "❌ Erreur: Impossible de trouver Cargo.toml"
    exit 1
fi

# Fonction pour tuer les processus en arrière-plan à la sortie
cleanup() {
    echo ""
    echo "🛑 Arrêt de tous les serveurs..."
    jobs -p | xargs -r kill 2>/dev/null || true
    exit 0
}

trap cleanup INT TERM

# Démarrer le backend API
echo "📡 Démarrage Backend API..."
cd "$PROJECT_ROOT"
cargo run --example hydra_api_server --features hydra 2>&1 | sed 's/^/[BACKEND] /' &
BACKEND_PID=$!

# Attendre que le backend démarre
echo "⏳ Attente du backend (3 secondes)..."
sleep 3

# Vérifier que le backend est bien démarré
if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo "❌ Le backend n'a pas démarré correctement"
    exit 1
fi

echo "✅ Backend démarré (PID: $BACKEND_PID)"
echo ""

# Démarrer le frontend
echo "🎨 Démarrage Frontend..."
cd "$PROJECT_ROOT/frontend"

# Build initial
echo "📦 Build initial du frontend..."
npx ng build --configuration=development --output-path=dist/dev 2>&1 | sed 's/^/[BUILD] /'

if [ $? -ne 0 ]; then
    echo "❌ Build frontend failed"
    cleanup
    exit 1
fi

echo "✅ Build initial terminé"
echo ""

# Démarrer watch mode pour auto-rebuild
echo "👁️  Activation du watch mode..."
npx ng build --watch --configuration=development --output-path=dist/dev 2>&1 | sed 's/^/[WATCH] /' &
WATCH_PID=$!

# Attendre un peu pour que le watch s'initialise
sleep 2

# Démarrer le serveur HTTP
echo "🌐 Démarrage du serveur HTTP..."
cd dist/dev/browser
npx http-server -p 4201 --proxy http://localhost:8889 2>&1 | sed 's/^/[HTTP] /' &
HTTP_PID=$!

# Attendre que le serveur HTTP démarre
sleep 2

if ! kill -0 $HTTP_PID 2>/dev/null; then
    echo "❌ Le serveur HTTP n'a pas démarré correctement"
    cleanup
    exit 1
fi

echo ""
echo "✅ Stack complète opérationnelle!"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🌐 URLs:"
echo "   Frontend:  http://localhost:4201"
echo "   Backend:   http://localhost:8889/api"
echo ""
echo "📝 Logs:"
echo "   Backend:   /tmp/api-server.log"
echo "   Frontend:  /tmp/frontend-dev-server.log"
echo ""
echo "🔄 Watch mode actif: changements auto-rebuild"
echo ""
echo "⏹️  Appuyer sur CTRL+C pour tout arrêter"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Attendre indéfiniment (les logs s'affichent en temps réel)
wait
