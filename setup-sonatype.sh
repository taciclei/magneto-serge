#!/bin/bash

# Script pour configurer Sonatype OSSRH
# Usage: ./setup-sonatype.sh

set -e

# Couleurs
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo "🚀 Configuration Sonatype OSSRH pour Maven Central"
echo "===================================================="
echo ""

echo -e "${BLUE}📖 Instructions:${NC}"
echo ""
echo "1. Créer un compte sur: ${YELLOW}https://central.sonatype.org/${NC}"
echo "   - Cliquer sur 'Sign Up' en haut à droite"
echo "   - Utiliser ton email GitHub"
echo "   - Vérifier ton email"
echo ""
echo "2. Créer un namespace: ${YELLOW}io.github.taciclei${NC}"
echo "   - Se connecter à https://central.sonatype.org/"
echo "   - Aller dans 'Namespaces' → 'Add Namespace'"
echo "   - Entrer: io.github.taciclei"
echo "   - Cliquer sur 'Verify Namespace'"
echo ""
echo "3. Vérifier le namespace via GitHub:"
echo "   - Sonatype te demandera de créer un repo public temporaire"
echo "   - Nom du repo: OSSRH-XXXXX (ils te donneront le nom exact)"
echo "   - Créer ce repo vide sur GitHub: ${YELLOW}https://github.com/new${NC}"
echo "   - Attendre la vérification (quelques minutes à 24h max)"
echo ""
echo "======================================"
read -p "As-tu créé ton compte Sonatype? (y/n) " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "👉 Crée d'abord ton compte sur: https://central.sonatype.org/"
    echo "   Puis relance ce script."
    exit 0
fi

echo ""
read -p "Username Sonatype: " OSSRH_USERNAME
read -s -p "Password Sonatype: " OSSRH_PASSWORD
echo ""
echo ""

# Sauvegarder dans .env
echo "# Sonatype OSSRH Configuration (généré le $(date))" >> .env
echo "OSSRH_USERNAME=$OSSRH_USERNAME" >> .env
echo "OSSRH_PASSWORD=$OSSRH_PASSWORD" >> .env
echo ""

echo -e "${GREEN}✓${NC} Credentials sauvegardés dans .env"
echo ""

echo "======================================"
echo "✅ Configuration Sonatype terminée!"
echo "======================================"
echo ""
echo "📋 Prochaines étapes:"
echo ""
echo "1. Vérifier que le namespace est bien vérifié:"
echo "   - Se connecter à https://central.sonatype.org/"
echo "   - Aller dans 'Namespaces'"
echo "   - Vérifier que 'io.github.taciclei' a un ✓ vert"
echo ""
echo "2. Configurer les secrets GitHub:"
echo "   gh secret set OSSRH_USERNAME --body \"$OSSRH_USERNAME\""
echo "   gh secret set OSSRH_PASSWORD --body \"$OSSRH_PASSWORD\""
echo ""
echo "3. Si le namespace n'est pas encore vérifié:"
echo "   - Créer le repo temporaire demandé par Sonatype"
echo "   - Attendre la vérification (max 24h)"
echo ""
