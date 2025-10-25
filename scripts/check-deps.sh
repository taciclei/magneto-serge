#!/usr/bin/env bash
# Script pour vérifier que toutes les dépendances sont installées

set -e

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Magneto-Serge - Vérification des dépendances         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

MISSING=0

# Vérifier Rust
echo -n "Rust (rustc):     "
if command -v rustc &> /dev/null; then
    VERSION=$(rustc --version | awk '{print $2}')
    echo -e "${GREEN}✓ $VERSION${NC}"
else
    echo -e "${RED}✗ Non installé${NC}"
    echo -e "${YELLOW}  → https://rustup.rs/${NC}"
    MISSING=1
fi

# Vérifier Cargo
echo -n "Cargo:            "
if command -v cargo &> /dev/null; then
    VERSION=$(cargo --version | awk '{print $2}')
    echo -e "${GREEN}✓ $VERSION${NC}"
else
    echo -e "${RED}✗ Non installé${NC}"
    MISSING=1
fi

# Vérifier Node.js
echo -n "Node.js:          "
if command -v node &> /dev/null; then
    VERSION=$(node --version)
    echo -e "${GREEN}✓ $VERSION${NC}"
else
    echo -e "${RED}✗ Non installé${NC}"
    echo -e "${YELLOW}  → https://nodejs.org/${NC}"
    MISSING=1
fi

# Vérifier NPM
echo -n "NPM:              "
if command -v npm &> /dev/null; then
    VERSION=$(npm --version)
    echo -e "${GREEN}✓ $VERSION${NC}"
else
    echo -e "${RED}✗ Non installé${NC}"
    MISSING=1
fi

# Vérifier tmux (optionnel)
echo -n "tmux (optionnel): "
if command -v tmux &> /dev/null; then
    VERSION=$(tmux -V | awk '{print $2}')
    echo -e "${GREEN}✓ $VERSION${NC}"
else
    echo -e "${YELLOW}✗ Non installé (optionnel)${NC}"
    echo -e "${YELLOW}  macOS: brew install tmux${NC}"
    echo -e "${YELLOW}  Linux: sudo apt install tmux${NC}"
fi

# Vérifier Make
echo -n "Make:             "
if command -v make &> /dev/null; then
    VERSION=$(make --version | head -n1 | awk '{print $3}')
    echo -e "${GREEN}✓ $VERSION${NC}"
else
    echo -e "${RED}✗ Non installé${NC}"
    MISSING=1
fi

# Vérifier Git
echo -n "Git:              "
if command -v git &> /dev/null; then
    VERSION=$(git --version | awk '{print $3}')
    echo -e "${GREEN}✓ $VERSION${NC}"
else
    echo -e "${RED}✗ Non installé${NC}"
    MISSING=1
fi

echo ""

if [ $MISSING -eq 0 ]; then
    echo -e "${GREEN}✓ Toutes les dépendances obligatoires sont installées${NC}"
    echo ""
    echo -e "${YELLOW}Prochaines étapes:${NC}"
    echo "  make install  # Installer les dépendances des projets"
    echo "  make quick    # Installation + build rapide"
    echo "  make dev      # Démarrer la stack complète"
    exit 0
else
    echo -e "${RED}✗ Des dépendances obligatoires sont manquantes${NC}"
    echo -e "${YELLOW}Installez-les et relancez ce script${NC}"
    exit 1
fi
