#!/bin/bash
# Frontend Development Server avec auto-rebuild
# Workaround pour Angular 17 + Vite issue

set -e

echo "🚀 Magnéto-Serge Frontend Development Server"
echo ""
echo "Ce script:"
echo "  1. Build l'application en mode watch (auto-rebuild)"
echo "  2. Sert les fichiers avec http-server + proxy vers backend"
echo ""
echo "⚠️  Workaround: Angular 17 + Vite ne sert pas les fichiers JS correctement avec 'ng serve'"
echo "    Solution: Build + http-server au lieu de Vite dev server"
echo ""

# Vérifier que nous sommes dans le bon répertoire
if [ ! -f "angular.json" ]; then
    echo "❌ Erreur: Ce script doit être exécuté depuis le répertoire frontend/"
    exit 1
fi

# Créer le répertoire de sortie si nécessaire
mkdir -p dist/dev/browser

# Build initial
echo "📦 Build initial..."
npx ng build --configuration=development --output-path=dist/dev

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "✅ Build initial terminé"
echo ""

# Fonction pour rebuild quand les fichiers changent
rebuild_on_change() {
    echo "👁️  Watch mode activé - rebuild automatique sur changement de fichier..."
    npx ng build --watch --configuration=development --output-path=dist/dev &
    WATCH_PID=$!
    echo "Watch PID: $WATCH_PID"
}

# Fonction pour servir les fichiers
serve_files() {
    echo ""
    echo "🌐 Démarrage du serveur HTTP sur http://localhost:4201"
    echo "   Proxy: /api/* -> http://localhost:8889/api/*"
    echo ""
    echo "💡 Ouvrir dans le navigateur: http://localhost:4201"
    echo ""
    cd dist/dev/browser
    npx http-server -p 4201 --proxy http://localhost:8889
}

# Cleanup function
cleanup() {
    echo ""
    echo "🛑 Arrêt du serveur..."
    if [ ! -z "$WATCH_PID" ]; then
        kill $WATCH_PID 2>/dev/null || true
    fi
    exit 0
}

# Trap CTRL+C
trap cleanup INT TERM

# Démarrer watch mode en arrière-plan
rebuild_on_change

# Démarrer le serveur HTTP (bloquant)
serve_files
