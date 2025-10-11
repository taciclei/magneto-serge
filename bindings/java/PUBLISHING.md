# 📦 Publishing matgto-serge to Maven Central

## Prerequisites

### 1. Sonatype Account
- Create account at https://s01.oss.sonatype.org/
- Request access to `io.github.magneto` group

### 2. GPG Key
```bash
# Generate GPG key
gpg --gen-key

# List keys
gpg --list-keys

# Export public key to key server
gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID
```

### 3. Maven Settings
Add to `~/.m2/settings.xml`:
```xml
<settings>
  <servers>
    <server>
      <id>ossrh</id>
      <username>YOUR_SONATYPE_USERNAME</username>
      <password>YOUR_SONATYPE_PASSWORD</password>
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
        <gpg.passphrase>YOUR_GPG_PASSPHRASE</gpg.passphrase>
      </properties>
    </profile>
  </profiles>
</settings>
```

## Publishing Steps

### 1. Prepare Release

```bash
cd bindings/java

# Verify package
mvn clean verify

# Build locally
mvn clean install
```

### 2. Deploy to Maven Central

```bash
# Deploy snapshot (for testing)
mvn clean deploy

# Deploy release
mvn clean deploy -P release
```

### 3. Release via Nexus UI

1. Go to https://s01.oss.sonatype.org/
2. Login with Sonatype credentials
3. Navigate to "Staging Repositories"
4. Find `io.github.magneto-serge-xxx` repository
5. Click "Close" button
6. Wait for validation (5-10 minutes)
7. Click "Release" button

### 4. Verify Publication

After ~2 hours, the package will be available:
- https://central.sonatype.com/artifact/io.github.magneto/serge
- https://mvnrepository.com/artifact/io.github.magneto/serge

## Gradle Alternative

Update `bindings/java/build.gradle`:

```gradle
plugins {
    id 'java'
    id 'maven-publish'
    id 'signing'
}

group = 'io.github.magneto'
version = '0.1.0'

publishing {
    publications {
        mavenJava(MavenPublication) {
            from components.java

            pom {
                name = 'matgto-serge'
                description = 'Multi-language HTTP/WebSocket testing library'
                url = 'https://github.com/matgto/serge'

                licenses {
                    license {
                        name = 'MIT License'
                        url = 'https://opensource.org/licenses/MIT'
                    }
                    license {
                        name = 'Apache License, Version 2.0'
                        url = 'https://www.apache.org/licenses/LICENSE-2.0.txt'
                    }
                }

                developers {
                    developer {
                        name = 'matgto-serge contributors'
                        email = 'noreply@github.com'
                    }
                }

                scm {
                    connection = 'scm:git:git://github.com/matgto/serge.git'
                    developerConnection = 'scm:git:ssh://github.com/matgto/serge.git'
                    url = 'https://github.com/matgto/serge'
                }
            }
        }
    }

    repositories {
        maven {
            name = 'sonatype'
            url = version.endsWith('SNAPSHOT')
                ? 'https://s01.oss.sonatype.org/content/repositories/snapshots/'
                : 'https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/'

            credentials {
                username = project.findProperty('ossrhUsername') ?: System.getenv('OSSRH_USERNAME')
                password = project.findProperty('ossrhPassword') ?: System.getenv('OSSRH_PASSWORD')
            }
        }
    }
}

signing {
    sign publishing.publications.mavenJava
}
```

Then publish with Gradle:
```bash
./gradlew publish
```

## Troubleshooting

### GPG Signing Fails
```bash
# Ensure GPG agent is running
gpgconf --kill gpg-agent
gpgconf --launch gpg-agent

# Test signing
echo "test" | gpg --clearsign
```

### Authentication Failed
- Verify credentials in `~/.m2/settings.xml`
- Check Sonatype account status
- Ensure group ownership is confirmed

### Deployment Rejected
- Verify POM completeness (name, description, url, scm, developers, licenses)
- Check artifact signing
- Validate group ownership

## Resources

- [Maven Central Guide](https://central.sonatype.org/publish/publish-guide/)
- [Sonatype OSSRH](https://central.sonatype.org/publish/publish-maven/)
- [GPG Guide](https://central.sonatype.org/publish/requirements/gpg/)

## Quick Publish (Automated)

Create `.github/workflows/publish-maven.yml`:

```yaml
name: Publish to Maven Central

on:
  release:
    types: [published]

jobs:
  publish:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Set up JDK 11
        uses: actions/setup-java@v3
        with:
          java-version: '11'
          distribution: 'temurin'
          server-id: ossrh
          server-username: MAVEN_USERNAME
          server-password: MAVEN_PASSWORD
          gpg-private-key: ${{ secrets.GPG_PRIVATE_KEY }}
          gpg-passphrase: GPG_PASSPHRASE

      - name: Publish to Maven Central
        run: |
          cd bindings/java
          mvn clean deploy -P release
        env:
          MAVEN_USERNAME: ${{ secrets.OSSRH_USERNAME }}
          MAVEN_PASSWORD: ${{ secrets.OSSRH_PASSWORD }}
          GPG_PASSPHRASE: ${{ secrets.GPG_PASSPHRASE }}
```

Configure GitHub secrets:
- `OSSRH_USERNAME`
- `OSSRH_PASSWORD`
- `GPG_PRIVATE_KEY`
- `GPG_PASSPHRASE`
