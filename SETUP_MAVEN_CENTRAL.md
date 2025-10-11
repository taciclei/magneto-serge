# 🚀 Guide Interactif : Publication sur Maven Central

Ce guide te accompagne pas à pas pour configurer la publication sur Maven Central.

---

## Étape 1 : Créer un compte Sonatype ✅ À FAIRE MAINTENANT

### 1.1 Inscription

1. Ouvrir ce lien : **https://central.sonatype.org/**
2. Cliquer sur **"Sign Up"** en haut à droite
3. Remplir le formulaire :
   - **Email** : ton email GitHub (important pour la vérification)
   - **Username** : choisis un nom d'utilisateur (note-le bien !)
   - **Password** : mot de passe sécurisé (note-le bien !)
4. Vérifier ton email et activer le compte

### 1.2 Créer un Namespace

Un namespace est requis pour publier. Pour notre projet, nous utilisons `io.github.magneto`.

#### Option A : Via GitHub (RECOMMANDÉ - Plus simple)

1. Se connecter à https://central.sonatype.org/
2. Aller dans **"Namespaces"** → **"Add Namespace"**
3. Entrer : `io.github.taciclei`
   - Cela correspond à ton username GitHub
   - Sonatype vérifiera automatiquement que tu es propriétaire du repo
4. Cliquer sur **"Verify Namespace"**
5. Sonatype te demandera de créer un repo public GitHub temporaire :
   - Nom du repo : quelque chose comme `OSSRH-12345` (ils te donneront le nom exact)
   - Créer ce repo vide sur GitHub
   - Attendre la vérification (quelques minutes à 24h max)

#### Option B : Via Domaine (si tu as un domaine)

Si tu possèdes un domaine (ex: `magneto.io`) :
1. Entrer : `io.magneto` comme namespace
2. Ajouter un record DNS TXT pour prouver la propriété
3. Attendre la vérification

### 1.3 Note tes credentials

Une fois le compte créé, note dans un fichier sécurisé :
```
OSSRH_USERNAME=ton_username
OSSRH_PASSWORD=ton_password
```

**⚠️ NE JAMAIS COMMITTER CES CREDENTIALS DANS GIT**

---

## Étape 2 : Générer une clé GPG 🔐 À FAIRE MAINTENANT

Les artefacts Maven Central doivent être signés avec GPG.

### 2.1 Installer GPG (si pas déjà installé)

```bash
# macOS
brew install gnupg

# Linux (Ubuntu/Debian)
sudo apt-get install gnupg

# Vérifier l'installation
gpg --version
```

### 2.2 Générer la clé GPG

```bash
# Générer une nouvelle clé
gpg --full-generate-key
```

**Répondre aux questions :**
1. Type de clé : `(1) RSA and RSA` (par défaut)
2. Taille de clé : `4096` (recommandé)
3. Durée de validité : `0` (pas d'expiration) ou `2y` (2 ans)
4. Confirmer : `y`
5. **Nom réel** : `Magneto Serge` (ou ton nom)
6. **Email** : ton email GitHub (IMPORTANT : utiliser le même que Sonatype)
7. **Commentaire** : `Maven Central signing key` (optionnel)
8. **Passphrase** : choisis un mot de passe fort (NOTE-LE BIEN !)

### 2.3 Lister et identifier la clé

```bash
# Lister les clés générées
gpg --list-keys
```

**Exemple de sortie :**
```
pub   rsa4096 2025-10-10 [SC]
      ABCD1234EFGH5678IJKL9012MNOP3456QRST7890
uid           [ultimate] Magneto Serge <email@example.com>
sub   rsa4096 2025-10-10 [E]
```

**Note le KEY_ID** : `ABCD1234EFGH5678IJKL9012MNOP3456QRST7890` (la longue chaîne)

### 2.4 Publier la clé publique sur les serveurs de clés

Les serveurs Maven Central vérifient que ta clé publique est disponible.

```bash
# Remplace YOUR_KEY_ID par ton KEY_ID réel
gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID

# Alternative : publier sur keys.openpgp.org aussi
gpg --keyserver keys.openpgp.org --send-keys YOUR_KEY_ID
```

**Attendre quelques minutes** pour que la clé se propage sur les serveurs.

### 2.5 Exporter la clé privée

```bash
# Remplace YOUR_KEY_ID par ton KEY_ID réel
gpg --armor --export-secret-keys YOUR_KEY_ID > private-key.asc
```

**⚠️ ATTENTION : Ce fichier contient ta clé PRIVÉE**
- Ne JAMAIS le committer dans Git
- Le stocker dans un endroit sécurisé
- Le supprimer après configuration des secrets GitHub

### 2.6 Note tes informations GPG

Dans ton fichier sécurisé, ajouter :
```
GPG_KEY_ID=ABCD1234EFGH5678IJKL9012MNOP3456QRST7890
GPG_PASSPHRASE=ta_passphrase_gpg
```

---

## Étape 3 : Configurer GitHub Secrets 🔒 À FAIRE MAINTENANT

Maintenant, on va ajouter tous les secrets dans GitHub pour que le workflow CD puisse publier automatiquement.

### 3.1 Via GitHub CLI (recommandé)

```bash
cd /Users/sga/projects/matgto-serge

# 1. OSSRH Username
gh secret set OSSRH_USERNAME --body "ton_username_sonatype"

# 2. OSSRH Password
gh secret set OSSRH_PASSWORD --body "ton_password_sonatype"

# 3. GPG Private Key (contenu du fichier private-key.asc)
gh secret set GPG_PRIVATE_KEY --body "$(cat private-key.asc)"

# 4. GPG Passphrase
gh secret set GPG_PASSPHRASE --body "ta_passphrase_gpg"

# Vérifier que les secrets sont bien configurés
gh secret list
```

### 3.2 Via Interface Web GitHub (alternative)

1. Aller sur : https://github.com/taciclei/magneto-serge/settings/secrets/actions
2. Cliquer sur **"New repository secret"**
3. Ajouter ces 4 secrets un par un :

   **Secret 1:**
   - Name: `OSSRH_USERNAME`
   - Value: `ton_username_sonatype`

   **Secret 2:**
   - Name: `OSSRH_PASSWORD`
   - Value: `ton_password_sonatype`

   **Secret 3:**
   - Name: `GPG_PRIVATE_KEY`
   - Value: (copier tout le contenu de `private-key.asc`)

   **Secret 4:**
   - Name: `GPG_PASSPHRASE`
   - Value: `ta_passphrase_gpg`

---

## Étape 4 : Nettoyer les fichiers sensibles 🧹 IMPORTANT

Une fois les secrets configurés dans GitHub, **supprimer le fichier de clé privée** :

```bash
# Supprimer la clé privée exportée (elle est maintenant dans GitHub Secrets)
rm private-key.asc

# Vérifier qu'elle n'est plus là
ls -la | grep private-key
```

---

## Étape 5 : Test de Publication Locale 🧪 À FAIRE

Avant de publier sur Maven Central, testons localement.

### 5.1 Build le projet Rust

```bash
cd /Users/sga/projects/matgto-serge

# Build la bibliothèque native
cargo build --release

# Copier la lib dans le binding Java
cp target/release/libmagneto_serge.dylib bindings/java/lib/ || true
cp target/release/libmagneto_serge.so bindings/java/lib/ || true
```

### 5.2 Générer les bindings Kotlin

```bash
# Générer les bindings Kotlin via UniFFI
cargo run --features=uniffi/cli --bin uniffi-bindgen generate \
  src/magneto_serge.udl \
  --language kotlin \
  --out-dir bindings/kotlin/generated
```

### 5.3 Test build Gradle

```bash
cd bindings/java

# Test build
./gradlew build

# Publier dans Maven local (~/.m2/repository)
./gradlew publishToMavenLocal
```

Si tout réussit, tu devrais voir :
```
BUILD SUCCESSFUL in XXs
```

### 5.4 Vérifier la publication locale

```bash
# Vérifier que le package est dans Maven local
ls -la ~/.m2/repository/io/github/magneto/magneto-serge/0.4.0/
```

Tu devrais voir :
- `magneto-serge-0.4.0.jar`
- `magneto-serge-0.4.0-sources.jar`
- `magneto-serge-0.4.0-javadoc.jar`
- `magneto-serge-0.4.0.pom`

---

## Étape 6 : Publication sur Maven Central 🚀 FINALE

Une fois que tout est configuré et testé, on peut publier sur Maven Central.

### Option A : Via Workflow GitHub (RECOMMANDÉ)

```bash
cd /Users/sga/projects/matgto-serge

# Merger develop dans main
git checkout main
git merge develop
git push origin main

# Créer le tag de release
git tag v0.4.0
git push origin v0.4.0
```

Le workflow `.github/workflows/cd.yml` va automatiquement :
1. ✅ Build la lib Rust
2. ✅ Générer les bindings
3. ✅ Compiler le Java
4. ✅ Signer avec GPG
5. ✅ Publier sur Maven Central
6. ✅ Créer GitHub Release

### Option B : Publication manuelle (pour tester)

```bash
cd bindings/java

# Configurer les variables d'environnement
export OSSRH_USERNAME=ton_username
export OSSRH_PASSWORD=ton_password
export GPG_PASSPHRASE=ta_passphrase

# Publier via Gradle
./gradlew publish

# OU via Maven
mvn clean deploy -Possrh
```

---

## Étape 7 : Vérification 🔍 APRÈS PUBLICATION

### 7.1 Vérifier sur Sonatype OSSRH

1. Se connecter à https://s01.oss.sonatype.org/
2. Aller dans **"Staging Repositories"**
3. Chercher `iogithubmagneto-XXXX`
4. Vérifier que le repository est :
   - ✅ Closed (fermé automatiquement)
   - ✅ Released (publié automatiquement)

### 7.2 Attendre la synchronisation

Maven Central synchronise depuis OSSRH toutes les ~2 heures.

Attendre 2-4 heures, puis vérifier :
- https://central.sonatype.com/artifact/io.github.magneto/magneto-serge
- https://repo1.maven.org/maven2/io/github/magneto/magneto-serge/

### 7.3 Tester l'installation

Créer un projet test :

**build.gradle.kts :**
```kotlin
dependencies {
    implementation("io.github.magneto:magneto-serge:0.4.0")
}
```

Puis :
```bash
./gradlew build
```

Si ça télécharge et build, **c'est réussi ! 🎉**

---

## 📋 Checklist Complète

- [ ] Compte Sonatype créé et email vérifié
- [ ] Namespace `io.github.taciclei` créé et vérifié
- [ ] Clé GPG générée (4096 bits)
- [ ] Clé publique GPG publiée sur keyserver.ubuntu.com
- [ ] Clé privée GPG exportée dans private-key.asc
- [ ] Secret GitHub `OSSRH_USERNAME` configuré
- [ ] Secret GitHub `OSSRH_PASSWORD` configuré
- [ ] Secret GitHub `GPG_PRIVATE_KEY` configuré
- [ ] Secret GitHub `GPG_PASSPHRASE` configuré
- [ ] Fichier private-key.asc supprimé
- [ ] Build local Gradle réussi
- [ ] Publication Maven local réussie
- [ ] Tag v0.4.0 créé et poussé
- [ ] Workflow CD GitHub réussi
- [ ] Package visible sur Maven Central

---

## 🆘 Aide et Support

### En cas de problème

1. **Namespace non vérifié** : Attendre 24h max, contacter support Sonatype si plus long
2. **Erreur GPG** : Vérifier que la clé est bien publiée avec `gpg --keyserver keyserver.ubuntu.com --recv-keys YOUR_KEY_ID`
3. **401 Unauthorized** : Vérifier username/password Sonatype
4. **Signature failed** : Vérifier GPG_PASSPHRASE

### Liens utiles

- **Sonatype Support** : https://central.sonatype.org/publish/publish-guide/
- **GPG Signing** : https://central.sonatype.org/publish/requirements/gpg/
- **OSSRH Dashboard** : https://s01.oss.sonatype.org/

---

## 🎯 Prochaines étapes après publication

Une fois sur Maven Central, mettre à jour :
- [ ] README.md avec instructions d'installation Maven/Gradle
- [ ] ROADMAP.md : marquer "Publication Maven Central" comme ✅
- [ ] Créer un blog post / annonce
- [ ] Tweeter / partager sur les réseaux sociaux

---

**Dernière mise à jour** : 2025-10-10

**Status** : 🔄 Configuration en cours

Bonne chance ! 🚀
