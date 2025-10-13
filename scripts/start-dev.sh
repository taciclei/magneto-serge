#!/usr/bin/env bash
# Script pour démarrer la stack complète Magneto-Serge dans tmux

set -e

SESSION="magneto-dev"

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Magneto-Serge - Démarrage automatique (tmux)         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Vérifier que tmux est installé
if ! command -v tmux &> /dev/null; then
    echo -e "${RED}❌ tmux n'est pas installé${NC}"
    echo ""
    echo "Installation:"
    echo "  macOS: brew install tmux"
    echo "  Linux: sudo apt install tmux"
    exit 1
fi

# Tuer la session existante si elle existe
tmux kill-session -t $SESSION 2>/dev/null || true

echo -e "${GREEN}✓ Création de la session tmux '$SESSION'${NC}"
echo ""

# Créer une nouvelle session tmux
tmux new-session -d -s $SESSION -n "magneto"

# Fenêtre 1: API Magneto-Serge
echo -e "${BLUE}→ Fenêtre 1: API Magneto-Serge (port 8889)${NC}"
tmux send-keys -t $SESSION:0 "cd $(pwd)" C-m
tmux send-keys -t $SESSION:0 "echo '═══════════════════════════════════════════'" C-m
tmux send-keys -t $SESSION:0 "echo '  API Magneto-Serge (port 8889)'" C-m
tmux send-keys -t $SESSION:0 "echo '═══════════════════════════════════════════'" C-m
tmux send-keys -t $SESSION:0 "echo ''" C-m
tmux send-keys -t $SESSION:0 "sleep 1 && make run-api" C-m

# Fenêtre 2: Backend Node.js
echo -e "${BLUE}→ Fenêtre 2: Backend Node.js (port 3000)${NC}"
tmux new-window -t $SESSION:1 -n "backend"
tmux send-keys -t $SESSION:1 "cd $(pwd)" C-m
tmux send-keys -t $SESSION:1 "echo '═══════════════════════════════════════════'" C-m
tmux send-keys -t $SESSION:1 "echo '  Backend Node.js (port 3000)'" C-m
tmux send-keys -t $SESSION:1 "echo '═══════════════════════════════════════════'" C-m
tmux send-keys -t $SESSION:1 "echo ''" C-m
tmux send-keys -t $SESSION:1 "echo 'Attente de l API Magneto...'" C-m
tmux send-keys -t $SESSION:1 "sleep 3 && make run-backend" C-m

# Fenêtre 3: Client Angular
echo -e "${BLUE}→ Fenêtre 3: Client Angular (port 4201)${NC}"
tmux new-window -t $SESSION:2 -n "client"
tmux send-keys -t $SESSION:2 "cd $(pwd)" C-m
tmux send-keys -t $SESSION:2 "echo '═══════════════════════════════════════════'" C-m
tmux send-keys -t $SESSION:2 "echo '  Client Angular Simple (port 4201)'" C-m
tmux send-keys -t $SESSION:2 "echo '═══════════════════════════════════════════'" C-m
tmux send-keys -t $SESSION:2 "echo ''" C-m
tmux send-keys -t $SESSION:2 "echo 'Attente du backend...'" C-m
tmux send-keys -t $SESSION:2 "sleep 5 && make run-client-simple" C-m

# Fenêtre 4: Terminal libre pour commandes
echo -e "${BLUE}→ Fenêtre 4: Terminal (commandes)${NC}"
tmux new-window -t $SESSION:3 -n "terminal"
tmux send-keys -t $SESSION:3 "cd $(pwd)" C-m
tmux send-keys -t $SESSION:3 "clear" C-m
tmux send-keys -t $SESSION:3 "echo '═══════════════════════════════════════════'" C-m
tmux send-keys -t $SESSION:3 "echo '  Terminal de commandes'" C-m
tmux send-keys -t $SESSION:3 "echo '═══════════════════════════════════════════'" C-m
tmux send-keys -t $SESSION:3 "echo ''" C-m
tmux send-keys -t $SESSION:3 "echo 'Services en cours de démarrage...'" C-m
tmux send-keys -t $SESSION:3 "echo ''" C-m
tmux send-keys -t $SESSION:3 "sleep 8 && make status" C-m

# Sélectionner la première fenêtre
tmux select-window -t $SESSION:0

echo ""
echo -e "${GREEN}✓ Stack démarrée dans tmux${NC}"
echo ""
echo -e "${YELLOW}Navigation tmux:${NC}"
echo "  Ctrl+B puis 0/1/2/3 - Changer de fenêtre"
echo "  Ctrl+B puis d       - Détacher (services continuent)"
echo "  Ctrl+B puis &       - Fermer la fenêtre courante"
echo "  tmux attach -t $SESSION - Se rattacher"
echo "  tmux kill-session -t $SESSION - Tout arrêter"
echo ""
echo -e "${YELLOW}Services disponibles (dans ~10 secondes):${NC}"
echo "  • API:     http://localhost:8889"
echo "  • Backend: http://localhost:3000"
echo "  • Client:  http://localhost:4201"
echo ""
echo -e "${BLUE}Attachement à la session tmux...${NC}"
sleep 2

# Attacher à la session
tmux attach-session -t $SESSION
