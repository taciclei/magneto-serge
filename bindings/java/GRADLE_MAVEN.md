# 🔧 Guide Maven & Gradle - Magnéto-Serge Java/Kotlin

Ce document explique comment utiliser et publier le binding Java/Kotlin avec Maven et Gradle.

## 📦 Installation

### Maven (pom.xml)

```xml
<dependency>
    <groupId>io.github.magneto</groupId>
    <artifactId>magneto-serge</artifactId>
    <version>0.4.0</version>
</dependency>
```

### Gradle Kotlin DSL (build.gradle.kts) - Recommandé

```kotlin
dependencies {
    implementation("io.github.magneto:magneto-serge:0.4.0")
}
```

### Gradle Groovy (build.gradle)

```groovy
dependencies {
    implementation 'io.github.magneto:magneto-serge:0.4.0'
}
```

---

## 🏗️ Build Local

### Avec Gradle (Recommandé)

```bash
cd bindings/java

# Build le projet
./gradlew build

# Exécuter les tests
./gradlew test

# Créer les JARs (sources + javadoc)
./gradlew jar sourcesJar javadocJar

# Exécuter l'exemple
./gradlew runExample

# Publier localement dans Maven local
./gradlew publishToMavenLocal
```

### Avec Maven

```bash
cd bindings/java

# Build le projet
mvn clean install

# Exécuter les tests
mvn test

# Créer le package sans tests
mvn package -DskipTests

# Vérifier la qualité du code
mvn verify
```

---

## 📤 Publication Maven Central

### Prérequis

1. **Compte Sonatype**
   - Créer un compte : https://central.sonatype.org/
   - Créer un namespace `io.github.magneto`
   - Vérifier ownership via GitHub

2. **Clé GPG**
   ```bash
   # Générer une clé GPG
   gpg --gen-key

   # Lister les clés
   gpg --list-keys

   # Publier la clé publique
   gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID

   # Exporter la clé privée
   gpg --armor --export-secret-keys YOUR_KEY_ID > private-key.asc
   ```

3. **Configurer les secrets GitHub**
   ```bash
   # Username Sonatype
   gh secret set OSSRH_USERNAME --body "votre_username"

   # Password Sonatype
   gh secret set OSSRH_PASSWORD --body "votre_password"

   # Clé GPG privée (contenu du fichier private-key.asc)
   gh secret set GPG_PRIVATE_KEY --body "$(cat private-key.asc)"

   # Passphrase GPG
   gh secret set GPG_PASSPHRASE --body "votre_passphrase"
   ```

### Publication avec Gradle

```bash
cd bindings/java

# Publication locale pour tester
./gradlew publishToMavenLocal

# Publication sur Maven Central (avec signature)
export OSSRH_USERNAME=your_username
export OSSRH_PASSWORD=your_password
export GPG_PRIVATE_KEY="$(cat private-key.asc)"
export GPG_PASSPHRASE=your_passphrase

./gradlew publish
```

### Publication avec Maven

```bash
cd bindings/java

# Configuration ~/.m2/settings.xml
cat > ~/.m2/settings.xml << 'EOF'
<settings>
  <servers>
    <server>
      <id>ossrh</id>
      <username>${env.OSSRH_USERNAME}</username>
      <password>${env.OSSRH_PASSWORD}</password>
    </server>
  </servers>

  <profiles>
    <profile>
      <id>ossrh</id>
      <activation>
        <activeByDefault>true</activeByDefault>
      </activation>
      <properties>
        <gpg.executable>gpg</gpg.executable>
        <gpg.passphrase>${env.GPG_PASSPHRASE}</gpg.passphrase>
      </properties>
    </profile>
  </profiles>
</settings>
EOF

# Publication
export OSSRH_USERNAME=your_username
export OSSRH_PASSWORD=your_password
export GPG_PASSPHRASE=your_passphrase

mvn clean deploy -Possrh
```

---

## 🔄 Workflow GitHub Actions

Le workflow `.github/workflows/cd.yml` publie automatiquement sur Maven Central lors d'un tag git.

```bash
# Créer un tag pour déclencher la publication
git tag v0.4.0
git push origin v0.4.0
```

Le workflow :
1. Build la bibliothèque Rust
2. Génère les bindings Kotlin via UniFFI
3. Compile le wrapper Java
4. Signe les artefacts avec GPG
5. Publie sur Maven Central (OSSRH)

---

## 🧪 Tests

### Gradle

```bash
# Exécuter tous les tests
./gradlew test

# Tests avec rapport détaillé
./gradlew test --info

# Tests d'une classe spécifique
./gradlew test --tests MagnetoProxyTest

# Tests en mode continu (watch)
./gradlew test --continuous
```

### Maven

```bash
# Exécuter tous les tests
mvn test

# Tests d'une classe spécifique
mvn test -Dtest=MagnetoProxyTest

# Tests avec rapport Surefire
mvn surefire-report:report
```

---

## 📁 Structure du Projet

```
bindings/java/
├── build.gradle.kts          # Build Gradle moderne (Kotlin DSL)
├── build.gradle              # Build Gradle legacy (Groovy)
├── pom.xml                   # Build Maven
├── settings.gradle.kts       # Configuration Gradle
├── gradle.properties         # Propriétés Gradle
├── src/
│   ├── main/
│   │   └── java/
│   │       └── io/github/magneto/serge/
│   │           ├── MagnetoProxy.java
│   │           ├── ProxyMode.java
│   │           └── examples/
│   │               └── Example.java
│   └── test/
│       └── java/
│           └── io/github/magneto/serge/
│               └── MagnetoProxyTest.java
└── lib/
    └── libmagneto_serge.{so,dylib}  # Bibliothèque native

bindings/kotlin/
├── build.gradle.kts          # Build Gradle pour Kotlin
├── settings.gradle.kts
├── gradle.properties
└── generated/                # Bindings Kotlin générés par UniFFI
```

---

## 🛠️ Tâches Gradle Disponibles

### Build
- `./gradlew build` - Build complet avec tests
- `./gradlew assemble` - Build sans tests
- `./gradlew clean` - Nettoyer le build

### Tests
- `./gradlew test` - Exécuter les tests
- `./gradlew check` - Vérifier le code (tests + lint)

### JARs
- `./gradlew jar` - Créer le JAR principal
- `./gradlew sourcesJar` - Créer le JAR des sources
- `./gradlew javadocJar` - Créer le JAR de la javadoc

### Publication
- `./gradlew publishToMavenLocal` - Publier dans Maven local (~/.m2)
- `./gradlew publish` - Publier sur Maven Central

### Exemples
- `./gradlew runExample` - Exécuter l'exemple

### Wrapper
- `./gradlew wrapper` - Mettre à jour le wrapper Gradle

---

## 🛠️ Tâches Maven Disponibles

### Build
- `mvn compile` - Compiler le code
- `mvn package` - Créer le JAR
- `mvn install` - Installer dans Maven local
- `mvn clean` - Nettoyer le build

### Tests
- `mvn test` - Exécuter les tests
- `mvn verify` - Vérifier le code

### Publication
- `mvn deploy` - Déployer sur Maven Central
- `mvn deploy -Possrh` - Déployer avec profil OSSRH

### Documentation
- `mvn javadoc:javadoc` - Générer la javadoc
- `mvn site` - Générer le site Maven

---

## 📋 Checklist Publication

- [ ] Compte Sonatype créé et namespace vérifié
- [ ] Clé GPG générée et publiée
- [ ] Secrets GitHub configurés (OSSRH_USERNAME, OSSRH_PASSWORD, GPG_PRIVATE_KEY, GPG_PASSPHRASE)
- [ ] Version bumped dans build.gradle.kts et pom.xml
- [ ] Tests passent (`./gradlew test` ou `mvn test`)
- [ ] Build réussit (`./gradlew build` ou `mvn package`)
- [ ] Bindings Kotlin générés dans `bindings/kotlin/generated/`
- [ ] Bibliothèque native présente dans `lib/`

---

## 🔗 Ressources

- [Maven Central Publishing Guide](https://central.sonatype.org/publish/publish-guide/)
- [Gradle Publishing Guide](https://docs.gradle.org/current/userguide/publishing_maven.html)
- [GPG Signing Guide](https://central.sonatype.org/publish/requirements/gpg/)
- [Sonatype OSSRH](https://s01.oss.sonatype.org/)

---

## 🐛 Troubleshooting

### Erreur : "No such file: libmagneto_serge.so"

Assurez-vous que la bibliothèque native est copiée :

```bash
# macOS
cp ../../target/release/libmagneto_serge.dylib lib/

# Linux
cp ../../target/release/libmagneto_serge.so lib/
```

### Erreur GPG : "No secret key"

Vérifiez que la clé GPG est bien importée :

```bash
gpg --list-secret-keys
```

Si vide, importer votre clé privée :

```bash
gpg --import private-key.asc
```

### Erreur Maven : "Unauthorized"

Vérifiez vos credentials dans `~/.m2/settings.xml` ou les variables d'environnement :

```bash
echo $OSSRH_USERNAME
echo $OSSRH_PASSWORD
```

---

*Dernière mise à jour : 2025-10-10*
