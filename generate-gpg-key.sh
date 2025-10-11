#!/bin/bash

# Script pour générer une clé GPG pour Maven Central
# Usage: ./generate-gpg-key.sh

set -e

echo "🔐 Génération d'une clé GPG pour Maven Central"
echo "================================================"
echo ""

# Couleurs
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Demander les informations
echo -e "${YELLOW}Informations pour la clé GPG:${NC}"
read -p "Nom réel (ex: Magneto Serge): " GPG_NAME
read -p "Email (utiliser ton email GitHub): " GPG_EMAIL
read -s -p "Passphrase (mot de passe de la clé): " GPG_PASSPHRASE
echo ""
read -s -p "Confirmer la passphrase: " GPG_PASSPHRASE_CONFIRM
echo ""

if [ "$GPG_PASSPHRASE" != "$GPG_PASSPHRASE_CONFIRM" ]; then
    echo "❌ Les passphrases ne correspondent pas!"
    exit 1
fi

echo ""
echo -e "${GREEN}✓${NC} Génération de la clé GPG..."

# Générer la clé GPG de manière non-interactive
gpg --batch --gen-key <<EOF
%no-protection
Key-Type: RSA
Key-Length: 4096
Subkey-Type: RSA
Subkey-Length: 4096
Name-Real: $GPG_NAME
Name-Email: $GPG_EMAIL
Expire-Date: 0
Passphrase: $GPG_PASSPHRASE
EOF

echo -e "${GREEN}✓${NC} Clé GPG générée!"
echo ""

# Récupérer le KEY_ID
KEY_ID=$(gpg --list-keys --keyid-format LONG "$GPG_EMAIL" | grep pub | awk '{print $2}' | cut -d'/' -f2)

echo "📋 Informations de la clé:"
echo "=========================="
echo "KEY_ID: $KEY_ID"
echo "Email: $GPG_EMAIL"
echo ""

# Publier la clé publique
echo -e "${GREEN}✓${NC} Publication de la clé publique sur keyserver.ubuntu.com..."
gpg --keyserver keyserver.ubuntu.com --send-keys "$KEY_ID"

echo -e "${GREEN}✓${NC} Publication de la clé publique sur keys.openpgp.org..."
gpg --keyserver keys.openpgp.org --send-keys "$KEY_ID"

echo ""
echo -e "${GREEN}✓${NC} Clé publique publiée!"
echo ""

# Exporter la clé privée
echo -e "${GREEN}✓${NC} Export de la clé privée..."
gpg --armor --export-secret-keys "$KEY_ID" > private-key.asc

echo -e "${GREEN}✓${NC} Clé privée exportée dans: private-key.asc"
echo ""

# Sauvegarder les informations dans .env
echo "# GPG Configuration (généré le $(date))" >> .env
echo "GPG_KEY_ID=$KEY_ID" >> .env
echo "GPG_PASSPHRASE=$GPG_PASSPHRASE" >> .env
echo ""

echo -e "${GREEN}✓${NC} Informations sauvegardées dans .env"
echo ""

echo "======================================"
echo "✅ Configuration GPG terminée!"
echo "======================================"
echo ""
echo "📋 Prochaines étapes:"
echo ""
echo "1. Configurer les secrets GitHub:"
echo "   gh secret set GPG_PRIVATE_KEY --body \"\$(cat private-key.asc)\""
echo "   gh secret set GPG_PASSPHRASE --body \"$GPG_PASSPHRASE\""
echo ""
echo "2. Supprimer le fichier de clé privée (après config GitHub):"
echo "   rm private-key.asc"
echo ""
echo "3. Vérifier que la clé est bien publiée (attendre 2-5 min):"
echo "   gpg --keyserver keyserver.ubuntu.com --recv-keys $KEY_ID"
echo ""
echo "⚠️  IMPORTANT: La clé privée (private-key.asc) contient des informations sensibles!"
echo "    Supprimer ce fichier après avoir configuré GitHub Secrets."
echo ""
