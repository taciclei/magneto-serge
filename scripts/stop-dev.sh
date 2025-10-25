#!/usr/bin/env bash
# Script pour arrêter la stack Magneto-Serge

set -e

SESSION="magneto-dev"

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Magneto-Serge - Arrêt des services                    ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Tuer la session tmux
if tmux has-session -t $SESSION 2>/dev/null; then
    echo -e "${YELLOW}→ Arrêt de la session tmux '$SESSION'${NC}"
    tmux kill-session -t $SESSION
    echo -e "${GREEN}✓ Session tmux arrêtée${NC}"
else
    echo -e "${YELLOW}⚠ Aucune session tmux active${NC}"
fi

# Tuer les processus Node.js éventuels
echo ""
echo -e "${YELLOW}→ Nettoyage des processus Node.js...${NC}"
pkill -f "ng serve" 2>/dev/null && echo -e "${GREEN}✓ Angular stoppé${NC}" || echo -e "${YELLOW}  (Angular non actif)${NC}"
pkill -f "node.*nodejs-backend" 2>/dev/null && echo -e "${GREEN}✓ Backend Node.js stoppé${NC}" || echo -e "${YELLOW}  (Backend non actif)${NC}"

# Tuer les processus Rust éventuels
echo ""
echo -e "${YELLOW}→ Nettoyage des processus Rust...${NC}"
pkill -f "magneto.*api" 2>/dev/null && echo -e "${GREEN}✓ API Magneto stoppée${NC}" || echo -e "${YELLOW}  (API non active)${NC}"

echo ""
echo -e "${GREEN}✓ Tous les services sont arrêtés${NC}"
echo ""
echo -e "${YELLOW}Pour redémarrer:${NC}"
echo "  make dev"
echo "  ou"
echo "  ./scripts/start-dev.sh"
