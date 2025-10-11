# 📦 Guide de Publication - Magnéto-Serge

Ce document explique comment publier magneto-serge sur les différents registres de packages.

## 🚀 Publication Automatique (Recommandé)

La publication est automatisée via GitHub Actions. Il suffit de créer un tag git pour déclencher le workflow CD.

### Prérequis

Configurer les secrets GitHub (Settings → Secrets and variables → Actions) :

- ✅ `CARGO_REGISTRY_TOKEN` - Token crates.io (configuré)
- ⏳ `NPM_TOKEN` - Token npm
- ⏳ `PYPI_TOKEN` - Token PyPI
- ⏳ `OSSRH_USERNAME` + `OSSRH_PASSWORD` - Maven Central
- ⏳ `GPG_PRIVATE_KEY` + `GPG_PASSPHRASE` - Signature Maven
- ⏳ `DOCKER_USERNAME` + `DOCKER_PASSWORD` - Docker Hub

### Processus de Release

```bash
# 1. Vérifier que develop est à jour et CI est verte
git checkout develop
git pull origin develop

# 2. Merger develop dans main
git checkout main
git merge develop
git push origin main

# 3. Créer le tag de version
git tag v0.4.0
git push origin v0.4.0

# 4. Le workflow CD se déclenche automatiquement et publie sur :
# - crates.io (Rust)
# - NPM (JavaScript) - si NPM_TOKEN configuré
# - PyPI (Python) - si PYPI_TOKEN configuré
# - Maven Central (Java/Kotlin) - si credentials configurés
# - Docker Hub - si DOCKER credentials configurés
# - GitHub Releases (automatique)
```

## 📦 Registres de Packages

### 1. Crates.io (Rust) ✅ PRÊT

**Status**: Token configuré, prêt à publier

```bash
# Publication manuelle (si besoin)
cargo publish --token $CARGO_REGISTRY_TOKEN
```

**URL**: https://crates.io/crates/magneto-serge

### 2. NPM (JavaScript) ⏳ EN ATTENTE

**Status**: Token manquant

**Configuration requise**:
1. Créer compte NPM : https://www.npmjs.com/signup
2. Créer token : https://www.npmjs.com/settings/tokens (Type: Automation)
3. Ajouter secret GitHub : `NPM_TOKEN`

```bash
# Publication manuelle
cd bindings/javascript
npm publish --access public
```

**URL future**: https://www.npmjs.com/package/@magneto/serge

### 3. PyPI (Python) ⏳ EN ATTENTE

**Status**: Token manquant

**Configuration requise**:
1. Créer compte PyPI : https://pypi.org/account/register/
2. Vérifier email
3. Créer API token : https://pypi.org/manage/account/token/
4. Ajouter secret GitHub : `PYPI_TOKEN`

```bash
# Publication manuelle
cd bindings/python
pip install maturin twine
maturin build --release
twine upload target/wheels/*.whl
```

**URL future**: https://pypi.org/project/magneto-serge/

### 4. Maven Central (Java/Kotlin) ⏳ EN ATTENTE

**Status**: Compte et GPG key manquants

**Configuration requise** (plus complexe):
1. Créer compte Sonatype : https://central.sonatype.org/
2. Créer namespace (ex: `io.github.magneto`)
3. Vérifier ownership du namespace (GitHub ou domaine)
4. Générer clé GPG :
   ```bash
   gpg --gen-key
   gpg --list-keys
   gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID
   gpg --armor --export-secret-keys YOUR_KEY_ID > private-key.asc
   ```
5. Ajouter secrets GitHub :
   - `OSSRH_USERNAME`
   - `OSSRH_PASSWORD`
   - `GPG_PRIVATE_KEY` (contenu de private-key.asc)
   - `GPG_PASSPHRASE`

```bash
# Publication manuelle
cd bindings/java
mvn clean deploy -Possrh
```

**URL future**: https://central.sonatype.com/artifact/io.github.magneto/magneto-serge

### 5. Docker Hub ⏳ EN ATTENTE

**Status**: Token manquant

**Configuration requise**:
1. Créer compte Docker Hub : https://hub.docker.com/signup
2. Créer access token : https://hub.docker.com/settings/security
3. Ajouter secrets GitHub :
   - `DOCKER_USERNAME`
   - `DOCKER_PASSWORD`

```bash
# Publication manuelle
docker buildx build --platform linux/amd64,linux/arm64 \
  -t YOUR_USERNAME/magneto-serge:v0.4.0 \
  -t YOUR_USERNAME/magneto-serge:latest \
  --push .
```

**URL future**: https://hub.docker.com/r/YOUR_USERNAME/magneto-serge

## 🔄 Workflow CD

Le workflow `.github/workflows/cd.yml` gère automatiquement :

1. **Build multi-plateformes** (Ubuntu, macOS, Windows)
2. **Publication parallèle** sur tous les registres configurés
3. **Création GitHub Release** avec binaires
4. **Build Docker multi-arch** (amd64, arm64)

### Statut actuel

| Registre | Secret | Statut | Priorité |
|----------|--------|--------|----------|
| crates.io | `CARGO_REGISTRY_TOKEN` | ✅ Configuré | Haute |
| NPM | `NPM_TOKEN` | ⏳ Manquant | Haute |
| PyPI | `PYPI_TOKEN` | ⏳ Manquant | Haute |
| Maven Central | `OSSRH_*` + `GPG_*` | ⏳ Manquant | Moyenne |
| Docker Hub | `DOCKER_*` | ⏳ Manquant | Basse |

## 📝 Checklist Avant Publication

- [x] CI/CD verte (12/12 jobs success)
- [x] Tests Rust passent (43/43)
- [x] Documentation à jour
- [x] CHANGELOG.md créé
- [x] Version bumped dans Cargo.toml
- [x] ROADMAP.md à jour
- [x] Token crates.io configuré
- [ ] Tokens NPM/PyPI configurés
- [ ] Compte Maven Central créé
- [ ] GPG key générée

## 🎯 Prochaines Étapes

### Priorité Haute (Cette semaine)
1. Configurer NPM token → Publication JavaScript
2. Configurer PyPI token → Publication Python
3. Créer tag v0.4.0 → Publier sur crates.io

### Priorité Moyenne (Prochaines semaines)
4. Configurer Maven Central → Publication Java/Kotlin
5. Configurer Docker Hub → Image Docker officielle

### Priorité Basse (Optionnel)
6. Swift Package Manager (déjà disponible via GitHub)
7. RubyGems (si binding Ruby ajouté)
8. Packagist (PHP) - Composer peut déjà utiliser GitHub

## 📚 Ressources

- [Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Publishing npm packages](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry)
- [Publishing on PyPI](https://packaging.python.org/en/latest/tutorials/packaging-projects/)
- [Publishing to Maven Central](https://central.sonatype.org/publish/publish-guide/)
- [Docker Hub documentation](https://docs.docker.com/docker-hub/)

---

*Dernière mise à jour : 2025-10-10*
