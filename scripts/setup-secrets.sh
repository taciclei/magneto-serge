#!/usr/bin/env bash
# Script interactif pour aider à configurer les secrets GitHub

set -e

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Emojis
CHECK="✅"
CROSS="❌"
WARNING="⚠️"
INFO="ℹ️"
LOCK="🔐"
ROCKET="🚀"

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  ${LOCK} Magneto-Serge - Configuration des Secrets      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${CYAN}Ce script vous guide pour configurer les secrets GitHub${NC}"
echo -e "${CYAN}nécessaires à la publication automatique.${NC}"
echo ""

# Fonction pour afficher un titre de section
section() {
    echo ""
    echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
    echo ""
}

# Fonction pour afficher une étape
step() {
    echo -e "${CYAN}➜ $1${NC}"
}

# Fonction pour afficher un succès
success() {
    echo -e "${GREEN}${CHECK} $1${NC}"
}

# Fonction pour afficher un avertissement
warn() {
    echo -e "${YELLOW}${WARNING} $1${NC}"
}

# Fonction pour afficher une erreur
error() {
    echo -e "${RED}${CROSS} $1${NC}"
}

# Fonction pour afficher une info
info() {
    echo -e "${CYAN}${INFO} $1${NC}"
}

# Vérifier que nous sommes dans le bon répertoire
if [ ! -f "Cargo.toml" ]; then
    error "Ce script doit être exécuté depuis la racine du projet magneto-serge"
    exit 1
fi

# Vérifier que gh CLI est installé
if ! command -v gh &> /dev/null; then
    error "GitHub CLI (gh) n'est pas installé"
    echo ""
    echo "Installation:"
    echo "  macOS: brew install gh"
    echo "  Linux: https://github.com/cli/cli/blob/trunk/docs/install_linux.md"
    exit 1
fi

# Vérifier l'authentification GitHub
if ! gh auth status &> /dev/null; then
    error "Vous devez vous authentifier avec GitHub CLI"
    echo ""
    step "Lancez: gh auth login"
    exit 1
fi

success "GitHub CLI configuré"

# Vérifier les permissions
REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner 2>/dev/null || echo "")
if [ -z "$REPO" ]; then
    error "Impossible de détecter le repository GitHub"
    exit 1
fi

success "Repository détecté: $REPO"

section "📋 Vue d'ensemble"

echo "Secrets nécessaires pour la publication automatique:"
echo ""
echo "  1. ${GREEN}CARGO_REGISTRY_TOKEN${NC}   - crates.io (Rust)"
echo "  2. ${GREEN}PYPI_TOKEN${NC}             - PyPI (Python)"
echo "  3. ${GREEN}OSSRH_USERNAME${NC}         - Maven Central (Java/Kotlin)"
echo "  4. ${GREEN}OSSRH_PASSWORD${NC}         - Maven Central (Java/Kotlin)"
echo "  5. ${GREEN}GPG_PRIVATE_KEY${NC}        - Maven Central (signature)"
echo "  6. ${GREEN}GPG_PASSPHRASE${NC}         - Maven Central (signature)"
echo "  7. ${YELLOW}DOCKER_USERNAME${NC}        - Docker Hub (optionnel)"
echo "  8. ${YELLOW}DOCKER_PASSWORD${NC}        - Docker Hub (optionnel)"
echo ""

# Demander confirmation
read -p "Voulez-vous configurer les secrets maintenant ? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    info "Configuration annulée. Consultez docs/SECRETS_SETUP.md pour plus d'infos."
    exit 0
fi

section "1️⃣  CARGO_REGISTRY_TOKEN (crates.io)"

echo "Ce token permet de publier des packages Rust sur crates.io"
echo ""
step "Génération du token:"
echo "  1. Allez sur: ${CYAN}https://crates.io/settings/tokens${NC}"
echo "  2. Cliquez sur 'New Token'"
echo "  3. Nom: ${CYAN}magneto-serge-cd${NC}"
echo "  4. Copiez le token généré"
echo ""

read -p "Avez-vous votre token crates.io ? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    read -s -p "Collez le token (caché): " CARGO_TOKEN
    echo ""

    if [ -z "$CARGO_TOKEN" ]; then
        warn "Token vide, ignoré"
    else
        gh secret set CARGO_REGISTRY_TOKEN --body "$CARGO_TOKEN" --repo "$REPO"
        success "CARGO_REGISTRY_TOKEN configuré"
    fi
else
    warn "Passé - Vous pouvez le configurer plus tard"
fi

section "2️⃣  PYPI_TOKEN (PyPI)"

echo "Ce token permet de publier des packages Python sur PyPI"
echo ""
step "Génération du token:"
echo "  1. Allez sur: ${CYAN}https://pypi.org/manage/account/token/${NC}"
echo "  2. Activez 2FA si ce n'est pas déjà fait"
echo "  3. Cliquez sur 'Add API token'"
echo "  4. Token name: ${CYAN}magneto-serge-cd${NC}"
echo "  5. Scope: ${CYAN}Entire account${NC}"
echo "  6. Copiez le token (commence par ${CYAN}pypi-${NC})"
echo ""

read -p "Avez-vous votre token PyPI ? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    read -s -p "Collez le token (caché): " PYPI_TOKEN
    echo ""

    if [ -z "$PYPI_TOKEN" ]; then
        warn "Token vide, ignoré"
    elif [[ ! $PYPI_TOKEN =~ ^pypi- ]]; then
        error "Le token doit commencer par 'pypi-'"
    else
        gh secret set PYPI_TOKEN --body "$PYPI_TOKEN" --repo "$REPO"
        success "PYPI_TOKEN configuré"
    fi
else
    warn "Passé - Vous pouvez le configurer plus tard"
fi

section "3️⃣  Maven Central (OSSRH)"

echo "Maven Central nécessite 4 secrets (le plus complexe)"
echo ""
step "Configuration requise:"
echo "  1. Compte Sonatype OSSRH: ${CYAN}https://issues.sonatype.org${NC}"
echo "  2. Ticket pour réclamer le namespace ${CYAN}io.github.$USER${NC}"
echo "  3. Clé GPG pour signer les artifacts"
echo ""
warn "Cette configuration est complexe et prend 1-2 jours (attente approbation)"
echo ""
info "Consultez ${CYAN}docs/SECRETS_SETUP.md${NC} pour le guide détaillé Maven Central"
echo ""

read -p "Avez-vous déjà un compte OSSRH approuvé ? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then

    # OSSRH Username
    read -p "Username OSSRH: " OSSRH_USER
    if [ -n "$OSSRH_USER" ]; then
        gh secret set OSSRH_USERNAME --body "$OSSRH_USER" --repo "$REPO"
        success "OSSRH_USERNAME configuré"
    fi

    # OSSRH Password
    read -s -p "Password OSSRH (caché): " OSSRH_PASS
    echo ""
    if [ -n "$OSSRH_PASS" ]; then
        gh secret set OSSRH_PASSWORD --body "$OSSRH_PASS" --repo "$REPO"
        success "OSSRH_PASSWORD configuré"
    fi

    # GPG
    echo ""
    step "Configuration GPG"

    read -p "Avez-vous une clé GPG ? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then

        # Lister les clés
        echo ""
        info "Vos clés GPG:"
        gpg --list-secret-keys --keyid-format=long 2>/dev/null || echo "Aucune clé trouvée"
        echo ""

        read -p "Key ID (ex: ABCD1234EFGH5678): " GPG_KEY_ID

        if [ -n "$GPG_KEY_ID" ]; then
            # Exporter la clé
            GPG_KEY=$(gpg --armor --export-secret-keys "$GPG_KEY_ID" 2>/dev/null)

            if [ -n "$GPG_KEY" ]; then
                gh secret set GPG_PRIVATE_KEY --body "$GPG_KEY" --repo "$REPO"
                success "GPG_PRIVATE_KEY configuré"

                # Passphrase
                read -s -p "GPG Passphrase (caché): " GPG_PASS
                echo ""
                if [ -n "$GPG_PASS" ]; then
                    gh secret set GPG_PASSPHRASE --body "$GPG_PASS" --repo "$REPO"
                    success "GPG_PASSPHRASE configuré"
                fi
            else
                error "Impossible d'exporter la clé GPG $GPG_KEY_ID"
            fi
        fi

    else
        warn "GPG non configuré"
        echo ""
        info "Pour générer une clé GPG:"
        echo "  ${CYAN}gpg --full-generate-key${NC}"
        echo "  Puis: ${CYAN}gpg --keyserver keyserver.ubuntu.com --send-keys KEY_ID${NC}"
    fi

else
    warn "Maven Central non configuré"
    echo ""
    info "Étapes pour configurer Maven Central:"
    echo "  1. Créer un compte: ${CYAN}https://issues.sonatype.org/secure/Signup!default.jspa${NC}"
    echo "  2. Créer un ticket pour réclamer le namespace"
    echo "  3. Attendre l'approbation (1-2 jours)"
    echo "  4. Relancer ce script"
fi

section "4️⃣  Docker Hub (optionnel)"

echo "Docker Hub permet de publier des images Docker"
echo ""
warn "Cette étape est ${YELLOW}optionnelle${NC}"
echo ""

read -p "Voulez-vous configurer Docker Hub ? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then

    step "Génération du token:"
    echo "  1. Allez sur: ${CYAN}https://hub.docker.com/settings/security${NC}"
    echo "  2. Cliquez sur 'New Access Token'"
    echo "  3. Description: ${CYAN}magneto-serge-cd${NC}"
    echo "  4. Permissions: ${CYAN}Read, Write, Delete${NC}"
    echo "  5. Copiez le token"
    echo ""

    read -p "Username Docker Hub: " DOCKER_USER
    if [ -n "$DOCKER_USER" ]; then
        gh secret set DOCKER_USERNAME --body "$DOCKER_USER" --repo "$REPO"
        success "DOCKER_USERNAME configuré"
    fi

    read -s -p "Token Docker Hub (caché): " DOCKER_TOKEN
    echo ""
    if [ -n "$DOCKER_TOKEN" ]; then
        gh secret set DOCKER_PASSWORD --body "$DOCKER_TOKEN" --repo "$REPO"
        success "DOCKER_PASSWORD configuré"
    fi

else
    warn "Docker Hub non configuré (optionnel)"
fi

section "${ROCKET} Résumé"

echo "Vérification des secrets configurés:"
echo ""

# Vérifier chaque secret
check_secret() {
    local secret=$1
    if gh secret list --repo "$REPO" 2>/dev/null | grep -q "^$secret"; then
        echo -e "  ${GREEN}${CHECK} $secret${NC}"
        return 0
    else
        echo -e "  ${RED}${CROSS} $secret${NC}"
        return 1
    fi
}

REQUIRED_COUNT=0
OPTIONAL_COUNT=0

# Secrets obligatoires
if check_secret "CARGO_REGISTRY_TOKEN"; then ((REQUIRED_COUNT++)); fi
if check_secret "PYPI_TOKEN"; then ((REQUIRED_COUNT++)); fi
if check_secret "OSSRH_USERNAME"; then ((REQUIRED_COUNT++)); fi
if check_secret "OSSRH_PASSWORD"; then ((REQUIRED_COUNT++)); fi
if check_secret "GPG_PRIVATE_KEY"; then ((REQUIRED_COUNT++)); fi
if check_secret "GPG_PASSPHRASE"; then ((REQUIRED_COUNT++)); fi

echo ""
echo "Secrets optionnels:"
if check_secret "DOCKER_USERNAME"; then ((OPTIONAL_COUNT++)); fi
if check_secret "DOCKER_PASSWORD"; then ((OPTIONAL_COUNT++)); fi

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════${NC}"
echo ""

if [ $REQUIRED_COUNT -eq 6 ]; then
    success "Tous les secrets obligatoires sont configurés ! (6/6)"
    echo ""
    echo -e "${GREEN}${ROCKET} Vous êtes prêt pour la publication automatique !${NC}"
    echo ""
    echo "Pour déclencher la publication:"
    echo "  ${CYAN}git tag v0.1.0${NC}"
    echo "  ${CYAN}git push origin v0.1.0${NC}"
    echo ""
else
    warn "Secrets configurés: $REQUIRED_COUNT/6 obligatoires"
    echo ""
    echo "Secrets manquants:"
    [ $(gh secret list --repo "$REPO" 2>/dev/null | grep -c "CARGO_REGISTRY_TOKEN") -eq 0 ] && echo "  - CARGO_REGISTRY_TOKEN"
    [ $(gh secret list --repo "$REPO" 2>/dev/null | grep -c "PYPI_TOKEN") -eq 0 ] && echo "  - PYPI_TOKEN"
    [ $(gh secret list --repo "$REPO" 2>/dev/null | grep -c "OSSRH_USERNAME") -eq 0 ] && echo "  - OSSRH_USERNAME"
    [ $(gh secret list --repo "$REPO" 2>/dev/null | grep -c "OSSRH_PASSWORD") -eq 0 ] && echo "  - OSSRH_PASSWORD"
    [ $(gh secret list --repo "$REPO" 2>/dev/null | grep -c "GPG_PRIVATE_KEY") -eq 0 ] && echo "  - GPG_PRIVATE_KEY"
    [ $(gh secret list --repo "$REPO" 2>/dev/null | grep -c "GPG_PASSPHRASE") -eq 0 ] && echo "  - GPG_PASSPHRASE"
    echo ""
    info "Consultez ${CYAN}docs/SECRETS_SETUP.md${NC} pour configurer les secrets manquants"
fi

if [ $OPTIONAL_COUNT -eq 2 ]; then
    success "Secrets optionnels configurés: $OPTIONAL_COUNT/2"
fi

echo ""
info "Pour voir tous les secrets configurés:"
echo "  ${CYAN}gh secret list --repo $REPO${NC}"
echo ""
info "Pour supprimer un secret:"
echo "  ${CYAN}gh secret remove SECRET_NAME --repo $REPO${NC}"
echo ""
