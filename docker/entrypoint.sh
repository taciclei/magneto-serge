#!/bin/bash
# ============================================================================
# Magneto-Serge Docker Entrypoint
# ============================================================================
# Gère le démarrage des services selon la commande

set -e

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Magneto-Serge Docker - Alpine Multi-Service          ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Fonction d'affichage
info() {
    echo -e "${GREEN}✓${NC} $1"
}

warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
}

# Vérifier les variables d'environnement
info "Configuration:"
echo "  API Port:      ${API_PORT}"
echo "  Backend Port:  ${BACKEND_PORT}"
echo "  Frontend Port: ${FRONTEND_PORT}"
echo "  Proxy Port:    ${PROXY_PORT}"
echo "  Cassette Dir:  ${CASSETTE_DIR}"
echo ""

# Créer les répertoires nécessaires
mkdir -p "${CASSETTE_DIR}"
mkdir -p /app/logs

# Vérifier que le binaire magneto existe
if [ ! -f "/app/bin/magneto" ]; then
    error "Magneto binary not found at /app/bin/magneto"
    exit 1
fi

info "Magneto binary found: $(magneto --version 2>&1 || echo 'version unavailable')"

# Fonction pour démarrer tous les services
start_all_services() {
    info "Starting all services with supervisord..."
    exec /usr/bin/supervisord -c /etc/supervisord.conf
}

# Fonction pour démarrer uniquement l'API
start_api_only() {
    info "Starting Magneto API only on port ${API_PORT}..."
    exec /app/bin/magneto api --host 0.0.0.0 --port "${API_PORT}"
}

# Fonction pour démarrer le proxy
start_proxy_only() {
    local mode="${1:-auto}"
    local cassette="${2:-default}"
    info "Starting Magneto Proxy in ${mode} mode with cassette '${cassette}'..."
    exec /app/bin/magneto "${mode}" "${cassette}" --port "${PROXY_PORT}"
}

# Fonction pour démarrer le backend seulement
start_backend_only() {
    info "Starting Node.js backend only on port ${BACKEND_PORT}..."
    cd /app/backend
    export MAGNETO_API_URL="http://localhost:${API_PORT}"
    exec node src/server.js
}

# Fonction pour initialiser Magneto
init_magneto() {
    info "Initializing Magneto configuration..."
    /app/bin/magneto init
    info "Configuration initialized in ${CASSETTE_DIR}"
}

# Fonction pour lister les cassettes
list_cassettes() {
    info "Listing cassettes in ${CASSETTE_DIR}:"
    /app/bin/magneto list
}

# Fonction d'aide
show_help() {
    cat << EOF
${BLUE}Magneto-Serge Docker Image - Usage${NC}

${GREEN}COMMANDES DISPONIBLES:${NC}

  ${YELLOW}all${NC}                - Démarre tous les services (API + Backend + Frontend)
  ${YELLOW}api${NC}                - Démarre uniquement l'API REST (port ${API_PORT})
  ${YELLOW}backend${NC}            - Démarre uniquement le backend Node.js (port ${BACKEND_PORT})
  ${YELLOW}proxy [mode] [name]${NC} - Démarre le proxy HTTP/HTTPS/WebSocket
                       Modes: auto, record, replay, passthrough
                       Exemple: docker run magneto-serge proxy record test

  ${YELLOW}init${NC}               - Initialise la configuration Magneto
  ${YELLOW}list${NC}               - Liste les cassettes disponibles
  ${YELLOW}inspect <name>${NC}     - Affiche le contenu d'une cassette
  ${YELLOW}delete <name>${NC}      - Supprime une cassette
  ${YELLOW}version${NC}            - Affiche la version

${GREEN}EXEMPLES:${NC}

  # Démarrer tous les services
  docker run -p 8889:8889 -p 3000:3000 -p 4201:4201 magneto-serge all

  # Démarrer uniquement l'API
  docker run -p 8889:8889 magneto-serge api

  # Démarrer le proxy en mode auto
  docker run -p 8888:8888 magneto-serge proxy auto test

  # Lister les cassettes
  docker run -v ./cassettes:/app/cassettes magneto-serge list

${GREEN}PORTS:${NC}

  ${API_PORT}  - API REST Magneto-Serge (Hydra/JSON-LD)
  ${BACKEND_PORT} - Backend Node.js (Express + Alcaeus)
  ${FRONTEND_PORT} - Interface Angular
  ${PROXY_PORT} - Proxy HTTP/HTTPS/WebSocket

${GREEN}VOLUMES:${NC}

  /app/cassettes - Stockage des cassettes (persistant)

${GREEN}VARIABLES D'ENVIRONNEMENT:${NC}

  API_PORT=${API_PORT}
  BACKEND_PORT=${BACKEND_PORT}
  FRONTEND_PORT=${FRONTEND_PORT}
  PROXY_PORT=${PROXY_PORT}
  CASSETTE_DIR=${CASSETTE_DIR}
  RUST_LOG=${RUST_LOG}

EOF
}

# Parser la commande
case "${1:-all}" in
    all)
        start_all_services
        ;;

    api)
        start_api_only
        ;;

    backend)
        start_backend_only
        ;;

    proxy)
        mode="${2:-auto}"
        cassette="${3:-default}"
        start_proxy_only "$mode" "$cassette"
        ;;

    init)
        init_magneto
        ;;

    list)
        list_cassettes
        ;;

    inspect)
        if [ -z "$2" ]; then
            error "Usage: docker run magneto-serge inspect <cassette-name>"
            exit 1
        fi
        /app/bin/magneto inspect "$2"
        ;;

    delete)
        if [ -z "$2" ]; then
            error "Usage: docker run magneto-serge delete <cassette-name>"
            exit 1
        fi
        /app/bin/magneto delete "$2"
        ;;

    version|--version|-v)
        /app/bin/magneto version
        ;;

    help|--help|-h)
        show_help
        ;;

    *)
        error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
