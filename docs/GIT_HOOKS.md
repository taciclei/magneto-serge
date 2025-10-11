# Git Hooks

Ce document décrit les hooks Git configurés pour le projet magneto-serge.

## Hooks Disponibles

### Pre-Push Hook

Le hook `pre-push` s'exécute automatiquement avant chaque `git push` et vérifie la qualité du code.

**Vérifications effectuées:**
1. **Formatage du code** (`cargo fmt --check`)
   - Vérifie que tout le code Rust est correctement formaté
   - Échoue si des fichiers ne sont pas formatés selon `rustfmt`

2. **Linting avec Clippy** (`cargo clippy --all-features --all-targets -- -D warnings`)
   - Vérifie la qualité du code avec le linter Clippy
   - Échoue si des warnings sont détectés
   - Couvre tous les features et tous les targets (lib, bins, tests, benches)

**Avantages:**
- ✅ Garantit la qualité du code avant le push
- ✅ Évite de polluer l'historique avec du code mal formaté
- ✅ Détecte les problèmes avant la CI/CD
- ✅ Réduit le temps d'itération en détectant les erreurs localement

## Installation

### Installation Automatique

Pour installer tous les hooks Git du projet:

\`\`\`bash
./scripts/install-git-hooks.sh
\`\`\`

### Installation Manuelle

Le hook pre-push est déjà créé dans `.git/hooks/pre-push` si vous avez cloné ce dépôt.

Si ce n'est pas le cas, exécutez:

\`\`\`bash
cp scripts/install-git-hooks.sh .
./install-git-hooks.sh
\`\`\`

## Utilisation

### Utilisation Normale

Les hooks s'exécutent automatiquement:

\`\`\`bash
git push origin develop
# 🔍 Running Rust linting before push...
#   📝 Checking code formatting (cargo fmt --check)...
#   ✅ Code formatting OK
#   🔧 Running clippy linter...
#   ✅ Clippy linting OK
# ✅ All linting checks passed! Proceeding with push...
\`\`\`

### Corriger les Erreurs de Formatage

Si le formatage échoue:

\`\`\`bash
cargo fmt
git add -u
git commit --amend --no-edit
git push origin develop
\`\`\`

### Corriger les Warnings Clippy

Si Clippy détecte des warnings:

\`\`\`bash
# Voir les warnings détaillés
cargo clippy --all-features --all-targets

# Corriger manuellement les warnings
# Puis:
git add -u
git commit --amend --no-edit
git push origin develop
\`\`\`

### Ignorer Temporairement les Hooks

⚠️ **À utiliser avec précaution!**

Si vous devez vraiment pousser sans exécuter les hooks:

\`\`\`bash
git push --no-verify origin develop
\`\`\`

**Note:** Cela n'est pas recommandé car la CI/CD rejettera probablement le code.

## Commandes de Linting Locales

Vous pouvez exécuter les mêmes vérifications manuellement:

### Formatage

\`\`\`bash
# Vérifier le formatage (sans modifier)
cargo fmt --check

# Formater automatiquement
cargo fmt
\`\`\`

### Clippy

\`\`\`bash
# Exécuter clippy avec tous les warnings comme erreurs
cargo clippy --all-features --all-targets -- -D warnings

# Clippy avec suggestions de fix automatiques
cargo clippy --fix --all-features --all-targets
\`\`\`

## Troubleshooting

### Le hook ne s'exécute pas

Vérifiez que le hook est exécutable:

\`\`\`bash
chmod +x .git/hooks/pre-push
\`\`\`

### Le hook échoue avec "cargo: command not found"

Assurez-vous que Rust et Cargo sont installés et dans le PATH:

\`\`\`bash
cargo --version
rustup show
\`\`\`

### Le hook est trop lent

Le linting complet peut prendre quelques secondes. Si c'est trop long:

1. Utilisez `cargo clippy` localement pendant le développement
2. Le hook ne s'exécute qu'au push, pas à chaque commit

### Désactiver Définitivement les Hooks

Si vous ne voulez vraiment pas utiliser les hooks:

\`\`\`bash
rm .git/hooks/pre-push
\`\`\`

## Configuration du Projet

### rustfmt

La configuration `rustfmt` est dans `.rustfmt.toml` (ou `rustfmt.toml`).

### clippy

Les configurations Clippy sont dans `Cargo.toml` ou `.cargo/config.toml`.

Exemple de lints désactivés si nécessaire:

\`\`\`toml
[lints.rust]
# Désactiver certains lints si besoin
\`\`\`

## CI/CD Integration

Les mêmes vérifications sont exécutées dans la CI/CD (GitHub Actions):

- `.github/workflows/ci.yml` - Job "lint"
  - `cargo fmt --check`
  - `cargo clippy --all-features --all-targets -- -D warnings`

Les hooks permettent de détecter ces erreurs **avant** le push, économisant du temps.

## Contribuer

Si vous ajoutez de nouveaux hooks:

1. Ajoutez le script dans `scripts/install-git-hooks.sh`
2. Documentez-le dans ce fichier
3. Mettez à jour le README si nécessaire

---

**Dernière mise à jour:** 2025-10-11
**Auteur:** Claude Code
