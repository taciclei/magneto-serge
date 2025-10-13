# 🔐 Configuration des Secrets GitHub

Guide complet pour configurer les secrets nécessaires à la publication automatique de Magneto-Serge.

---

## 📋 Vue d'ensemble

Le workflow CD (`.github/workflows/cd.yml`) nécessite **9 secrets** pour publier automatiquement sur :
- **crates.io** (Rust)
- **PyPI** (Python)
- **Maven Central** (Java/Kotlin)
- **Docker Hub** (Images Docker)

---

## 🔑 Secrets Requis

| Secret | Service | Description | Obligatoire |
|--------|---------|-------------|-------------|
| `CARGO_REGISTRY_TOKEN` | crates.io | Token API pour publier packages Rust | ✅ Oui |
| `PYPI_TOKEN` | PyPI | Token API pour publier packages Python | ✅ Oui |
| `OSSRH_USERNAME` | Maven Central | Username Sonatype OSSRH | ✅ Oui |
| `OSSRH_PASSWORD` | Maven Central | Password Sonatype OSSRH | ✅ Oui |
| `GPG_PRIVATE_KEY` | Maven Central | Clé privée GPG pour signer artifacts | ✅ Oui |
| `GPG_PASSPHRASE` | Maven Central | Passphrase de la clé GPG | ✅ Oui |
| `DOCKER_USERNAME` | Docker Hub | Username Docker Hub | ⚠️ Optionnel |
| `DOCKER_PASSWORD` | Docker Hub | Password/Token Docker Hub | ⚠️ Optionnel |
| `GITHUB_TOKEN` | GitHub | Token auto-généré par GitHub Actions | ✅ Auto |

> **Note** : `GITHUB_TOKEN` est automatiquement fourni par GitHub Actions, pas besoin de le configurer.

---

## 📝 Instructions Détaillées

### 1️⃣ CARGO_REGISTRY_TOKEN (crates.io)

**Service** : Publication packages Rust sur [crates.io](https://crates.io)

#### Étapes :

1. **Se connecter à crates.io**
   - Va sur https://crates.io
   - Connecte-toi avec ton compte GitHub

2. **Générer un token API**
   - Va dans **Account Settings** : https://crates.io/settings/tokens
   - Clique sur **New Token**
   - Nom : `magneto-serge-cd`
   - Permissions : **Publish** (par défaut)
   - Clique sur **Generate Token**

3. **Copier le token**
   ```
   Exemple : crates_io_XXXXXXXXXXXXXXXXXXXXXXXXXXXXX
   ```
   ⚠️ **IMPORTANT** : Le token n'est affiché qu'une seule fois !

4. **Ajouter le secret GitHub**
   - Va sur https://github.com/taciclei/magneto-serge/settings/secrets/actions
   - Clique sur **New repository secret**
   - Name : `CARGO_REGISTRY_TOKEN`
   - Value : Colle le token copié
   - Clique sur **Add secret**

#### Test local (optionnel) :
```bash
# Vérifier que le token fonctionne
export CARGO_REGISTRY_TOKEN="crates_io_XXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
cargo publish --dry-run
```

---

### 2️⃣ PYPI_TOKEN (PyPI)

**Service** : Publication packages Python sur [PyPI](https://pypi.org)

#### Étapes :

1. **Créer un compte PyPI** (si nécessaire)
   - Va sur https://pypi.org/account/register/

2. **Activer 2FA** (Two-Factor Authentication)
   - Va dans **Account settings** : https://pypi.org/manage/account/
   - Section **Two factor authentication**
   - Configure 2FA (obligatoire pour les tokens API)

3. **Générer un token API**
   - Va dans **Account settings** → **API tokens** : https://pypi.org/manage/account/token/
   - Clique sur **Add API token**
   - Token name : `magneto-serge-cd`
   - Scope : **Entire account** (pour le premier package)
   - Clique sur **Add token**

4. **Copier le token**
   ```
   Exemple : pypi-AgEIcHlwaS5vcmc...
   ```
   ⚠️ **IMPORTANT** : Le token commence par `pypi-` et n'est affiché qu'une fois !

5. **Ajouter le secret GitHub**
   - Name : `PYPI_TOKEN`
   - Value : Colle le token complet (avec `pypi-`)

#### Après la première publication :
Tu peux créer un token **scoped** (limité au package) :
- Scope : **Project: magneto-serge**
- Plus sécurisé

#### Test local (optionnel) :
```bash
# Installer twine
pip install twine

# Tester l'upload (dry-run)
export TWINE_USERNAME="__token__"
export TWINE_PASSWORD="pypi-ton-token-ici"
cd bindings/python
maturin build --release
twine check target/wheels/*.whl
```

---

### 3️⃣ Maven Central (OSSRH_USERNAME, OSSRH_PASSWORD, GPG_PRIVATE_KEY, GPG_PASSPHRASE)

**Service** : Publication packages Java/Kotlin sur [Maven Central](https://central.sonatype.com)

⚠️ **C'est le plus complexe à configurer** (4 secrets + setup initial)

#### Étape A : Créer un compte Sonatype OSSRH

1. **S'inscrire sur Sonatype OSSRH**
   - Va sur https://issues.sonatype.org/secure/Signup!default.jspa
   - Crée un compte Jira
   - Username : `taciclei` (par exemple)
   - Email : ton email

2. **Créer un ticket pour réclamer le namespace**
   - Va sur https://issues.sonatype.org/secure/CreateIssue.jspa?issuetype=21&pid=10134
   - Project : **Community Support - Open Source Project Repository Hosting (OSSRH)**
   - Issue Type : **New Project**
   - Summary : `Request for io.github.taciclei namespace`
   - Group Id : `io.github.taciclei`
   - Project URL : `https://github.com/taciclei/magneto-serge`
   - SCM URL : `https://github.com/taciclei/magneto-serge.git`
   - Username : `taciclei`
   - Description :
     ```
     I would like to publish my Rust library with Java/Kotlin bindings to Maven Central.
     Project: magneto-serge (HTTP/WebSocket proxy for testing)
     GitHub: https://github.com/taciclei/magneto-serge
     ```

3. **Vérifier la propriété du namespace**
   - Sonatype va te demander de prouver que tu possèdes le GitHub
   - Méthode : Créer un repo public temporaire `https://github.com/taciclei/OSSRH-xxxxx` (numéro du ticket)
   - OU ajouter un commentaire sur le ticket depuis ton compte GitHub

4. **Attendre l'approbation** (1-2 jours ouvrés)

#### Étape B : Créer une clé GPG

Maven Central **exige** que tous les artifacts soient signés avec GPG.

1. **Installer GPG**
   ```bash
   # macOS
   brew install gnupg

   # Linux
   sudo apt install gnupg
   ```

2. **Générer une clé GPG**
   ```bash
   gpg --full-generate-key
   ```
   - Type : **RSA and RSA** (1)
   - Key size : **4096** bits
   - Validity : **0** (n'expire jamais) ou **2y** (2 ans)
   - Real name : `Magneto-Serge CI`
   - Email : ton email
   - Passphrase : Choisis une passphrase forte (tu en auras besoin !)

3. **Lister les clés**
   ```bash
   gpg --list-secret-keys --keyid-format=long
   ```
   Sortie :
   ```
   sec   rsa4096/ABCD1234EFGH5678 2025-10-13 [SC]
         1234567890ABCDEF1234567890ABCDEF12345678
   uid                 [ultimate] Magneto-Serge CI <ton@email.com>
   ssb   rsa4096/IJKL9012MNOP3456 2025-10-13 [E]
   ```
   Note le **key ID** : `ABCD1234EFGH5678`

4. **Exporter la clé privée**
   ```bash
   gpg --armor --export-secret-keys ABCD1234EFGH5678 > gpg-private.key
   ```
   ⚠️ **ATTENTION** : Ce fichier contient ta clé privée, garde-le sécurisé !

5. **Publier la clé publique sur un keyserver**
   ```bash
   gpg --keyserver keyserver.ubuntu.com --send-keys ABCD1234EFGH5678
   ```
   OU
   ```bash
   gpg --keyserver keys.openpgp.org --send-keys ABCD1234EFGH5678
   ```

6. **Vérifier la publication**
   ```bash
   gpg --keyserver keyserver.ubuntu.com --recv-keys ABCD1234EFGH5678
   ```

#### Étape C : Ajouter les 4 secrets Maven Central

1. **OSSRH_USERNAME**
   - Value : Ton username Sonatype (par exemple `taciclei`)

2. **OSSRH_PASSWORD**
   - Value : Ton password Sonatype

3. **GPG_PRIVATE_KEY**
   - Value : Contenu complet du fichier `gpg-private.key`
   ```bash
   cat gpg-private.key
   ```
   - Copie TOUT le contenu (de `-----BEGIN PGP PRIVATE KEY BLOCK-----` à `-----END PGP PRIVATE KEY BLOCK-----`)

4. **GPG_PASSPHRASE**
   - Value : La passphrase que tu as choisie lors de la génération de la clé

#### Test local (optionnel) :
```bash
# Configurer Maven settings.xml
mkdir -p ~/.m2
cat > ~/.m2/settings.xml <<EOF
<settings>
  <servers>
    <server>
      <id>ossrh</id>
      <username>ton-username</username>
      <password>ton-password</password>
    </server>
  </servers>
</settings>
EOF

# Tester le build et la signature
cd bindings/java
mvn clean install -Possrh
```

#### Références Maven Central :
- Guide officiel : https://central.sonatype.org/publish/publish-guide/
- Requirements : https://central.sonatype.org/publish/requirements/

---

### 4️⃣ Docker Hub (DOCKER_USERNAME, DOCKER_PASSWORD)

**Service** : Publication images Docker sur [Docker Hub](https://hub.docker.com)

⚠️ **Optionnel** - Si tu ne veux pas publier d'images Docker, tu peux :
1. Ne pas configurer ces secrets
2. Supprimer le job `build-docker` du workflow CD

#### Étapes :

1. **Créer un compte Docker Hub** (si nécessaire)
   - Va sur https://hub.docker.com/signup

2. **Créer un Access Token**
   - Va dans **Account Settings** → **Security** : https://hub.docker.com/settings/security
   - Clique sur **New Access Token**
   - Description : `magneto-serge-cd`
   - Access permissions : **Read, Write, Delete**
   - Clique sur **Generate**

3. **Copier le token**
   ```
   Exemple : dckr_pat_1234567890abcdefghijklmno
   ```

4. **Ajouter les secrets GitHub**
   - **DOCKER_USERNAME**
     - Value : Ton username Docker Hub (par exemple `taciclei`)

   - **DOCKER_PASSWORD**
     - Value : Le token copié

#### Test local (optionnel) :
```bash
# Login
echo "ton-token" | docker login -u taciclei --password-stdin

# Build et push
docker build -t taciclei/magneto-serge:test .
docker push taciclei/magneto-serge:test
```

---

## ✅ Vérification de la Configuration

Une fois tous les secrets configurés, vérifie sur GitHub :

1. **Aller dans les secrets**
   https://github.com/taciclei/magneto-serge/settings/secrets/actions

2. **Vérifier la liste**
   Tu dois voir :
   ```
   ✅ CARGO_REGISTRY_TOKEN
   ✅ PYPI_TOKEN
   ✅ OSSRH_USERNAME
   ✅ OSSRH_PASSWORD
   ✅ GPG_PRIVATE_KEY
   ✅ GPG_PASSPHRASE
   ✅ DOCKER_USERNAME (optionnel)
   ✅ DOCKER_PASSWORD (optionnel)
   ```

3. **GITHUB_TOKEN**
   - Pas besoin de configuration
   - Automatiquement fourni par GitHub Actions

---

## 🚀 Déclencher la Publication

Une fois tous les secrets configurés :

### Option 1 : Via Git Tag (recommandé)

```bash
# Créer et pusher un tag de version
git tag v0.1.0
git push origin v0.1.0
```

Le workflow CD se déclenchera automatiquement et publiera sur :
- ✅ crates.io
- ✅ PyPI
- ✅ Maven Central
- ✅ Docker Hub (si configuré)
- ✅ GitHub Releases

### Option 2 : Via GitHub Web UI

1. Va sur https://github.com/taciclei/magneto-serge/releases/new
2. Choose a tag : `v0.1.0` (créer un nouveau tag)
3. Release title : `Magneto-Serge v0.1.0`
4. Description : Copie le contenu de `docs/RELEASE_NOTES.md`
5. Clique sur **Publish release**

---

## 🐛 Dépannage

### Erreur : "401 Unauthorized" (crates.io)
- ❌ Token invalide ou expiré
- ✅ Régénère un nouveau token sur https://crates.io/settings/tokens

### Erreur : "403 Forbidden" (PyPI)
- ❌ Token invalide ou scope insuffisant
- ✅ Crée un token avec scope "Entire account"

### Erreur : "401 Unauthorized" (Maven Central)
- ❌ Username/password incorrect
- ✅ Vérifie tes credentials Sonatype OSSRH

### Erreur : "GPG signature verification failed"
- ❌ Clé GPG non publiée sur keyserver
- ✅ Publie ta clé : `gpg --keyserver keyserver.ubuntu.com --send-keys KEY_ID`

### Erreur : "Namespace not approved" (Maven Central)
- ❌ Le ticket OSSRH n'est pas encore approuvé
- ✅ Attends l'approbation (1-2 jours ouvrés)

### Erreur : "docker login failed"
- ❌ Token Docker Hub invalide
- ✅ Régénère un Access Token sur https://hub.docker.com/settings/security

---

## 📚 Ressources

- **crates.io** : https://doc.rust-lang.org/cargo/reference/publishing.html
- **PyPI** : https://packaging.python.org/tutorials/packaging-projects/
- **Maven Central** : https://central.sonatype.org/publish/publish-guide/
- **Docker Hub** : https://docs.docker.com/docker-hub/access-tokens/
- **GitHub Actions Secrets** : https://docs.github.com/en/actions/security-guides/encrypted-secrets

---

## 🔒 Sécurité

- ✅ **Jamais** commit les tokens dans le code
- ✅ **Toujours** utiliser les GitHub Secrets
- ✅ Régénère les tokens si compromis
- ✅ Utilise des tokens **scoped** (permissions minimales)
- ✅ Active 2FA sur tous les comptes
- ✅ Révoque les tokens inutilisés

---

**Dernière mise à jour** : 2025-10-13
**Version** : v0.5.0
