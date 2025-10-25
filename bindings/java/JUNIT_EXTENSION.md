# JUnit 5 Extension pour Magneto-Serge

Extension JUnit 5 officielle pour **magneto-serge** - enregistrez et rejouez les interactions HTTP/WebSocket dans vos tests JUnit.

## 🚀 Installation

### Maven

```xml
<dependency>
    <groupId>com.magneto</groupId>
    <artifactId>magneto-serge</artifactId>
    <version>0.1.0</version>
    <scope>test</scope>
</dependency>

<dependency>
    <groupId>org.junit.jupiter</groupId>
    <artifactId>junit-jupiter</artifactId>
    <version>5.10.0</version>
    <scope>test</scope>
</dependency>
```

### Gradle

```groovy
testImplementation 'com.magneto:magneto-serge:0.1.0'
testImplementation 'org.junit.jupiter:junit-jupiter:5.10.0'
```

## 📖 Usage Basique

### 1. Activer l'Extension

```java
import com.magneto.MagnetoProxy;
import com.magneto.junit.MagnetoExtension;
import com.magneto.junit.MagnetoExtension.Magneto;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "api-test")
class ApiTest {

    @Test
    void testApiCall(MagnetoProxy magneto) {
        // Le proxy est automatiquement configuré et injecté
        int port = magneto.port();

        // Configurez votre client HTTP pour utiliser le proxy
        // localhost:port

        // Faites vos appels API
    }
}
```

**Première exécution** → Enregistre dans `./test_cassettes/api-test.json`
**Exécutions suivantes** → Rejeu depuis la cassette (aucun appel réseau)

## 🎯 Modes

### Mode Auto (par défaut)

```java
@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "api-test", mode = "auto")
class ApiTest {
    @Test
    void testApi(MagnetoProxy magneto) {
        // Si cassette existe → replay
        // Si cassette manque → record
    }
}
```

### Mode Record (force l'enregistrement)

```bash
# Via variable d'environnement
MAGNETO_MODE=record mvn test

# Via annotation
@Magneto(cassette = "api-test", mode = "record")
```

### Mode Replay (rejeu uniquement)

```bash
MAGNETO_MODE=replay mvn test
```

### Mode Strict (échoue si pas de match)

```java
@Magneto(cassette = "api-test", mode = "strict")
class ApiTest {
    @Test
    void testApi(MagnetoProxy magneto) {
        // Échoue si:
        // - Cassette manquante
        // - Requête non matchée dans la cassette
    }
}
```

## 🔧 Configuration

### Annotation @Magneto

```java
@Magneto(
    cassette = "nom-cassette",        // Nom de la cassette
    mode = "auto",                    // auto|record|replay|strict
    cassetteDir = "./test_cassettes", // Répertoire
    port = 8888,                      // Port du proxy
    scope = Scope.METHOD              // METHOD ou CLASS
)
```

### Scope METHOD (par défaut)

Chaque test a son propre proxy isolé.

```java
@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "api-test", scope = Scope.METHOD)
class ApiTest {
    @Test
    void test1(MagnetoProxy magneto) {
        // Proxy dédié pour ce test
    }

    @Test
    void test2(MagnetoProxy magneto) {
        // Nouveau proxy pour ce test
    }
}
```

### Scope CLASS

Proxy partagé pour toute la classe de tests.

```java
@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "api-test", scope = Scope.CLASS)
class ApiTest {
    @Test
    void test1(MagnetoProxy magneto) {
        // Même proxy pour tous les tests
    }

    @Test
    void test2(MagnetoProxy magneto) {
        // Réutilise le même proxy
    }
}
```

### Variables d'Environnement

```bash
# Mode par défaut
MAGNETO_MODE=auto|record|replay|strict

# Répertoire cassettes
MAGNETO_CASSETTE_DIR=./my_cassettes

# Port proxy
MAGNETO_PORT=9999
```

## 🧪 Exemples

### Tests API REST avec OkHttp

```java
import com.magneto.MagnetoProxy;
import com.magneto.junit.MagnetoExtension;
import com.magneto.junit.MagnetoExtension.Magneto;
import okhttp3.OkHttpClient;
import okhttp3.Request;
import okhttp3.Response;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.net.InetSocketAddress;
import java.net.Proxy;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "github-api")
class GitHubApiTest {

    @Test
    void shouldFetchRepository(MagnetoProxy magneto) throws Exception {
        // Configurer OkHttp avec le proxy
        Proxy proxy = new Proxy(
            Proxy.Type.HTTP,
            new InetSocketAddress("localhost", magneto.port())
        );

        OkHttpClient client = new OkHttpClient.Builder()
            .proxy(proxy)
            .build();

        // Faire l'appel API
        Request request = new Request.Builder()
            .url("https://api.github.com/repos/taciclei/magneto-serge")
            .build();

        try (Response response = client.newCall(request).execute()) {
            assertEquals(200, response.code());
            assertTrue(response.body().string().contains("magneto-serge"));
        }
    }
}
```

### Tests POST avec JSON

```java
@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "create-user")
class UserApiTest {

    @Test
    void shouldCreateUser(MagnetoProxy magneto) throws Exception {
        Proxy proxy = new Proxy(
            Proxy.Type.HTTP,
            new InetSocketAddress("localhost", magneto.port())
        );

        OkHttpClient client = new OkHttpClient.Builder()
            .proxy(proxy)
            .build();

        String json = "{\"name\":\"Alice\",\"email\":\"alice@example.com\"}";
        RequestBody body = RequestBody.create(json, MediaType.parse("application/json"));

        Request request = new Request.Builder()
            .url("https://api.example.com/users")
            .post(body)
            .build();

        try (Response response = client.newCall(request).execute()) {
            assertEquals(201, response.code());
        }
    }
}
```

### Tests avec Apache HttpClient

```java
import org.apache.hc.client5.http.classic.methods.HttpGet;
import org.apache.hc.client5.http.impl.classic.CloseableHttpClient;
import org.apache.hc.client5.http.impl.classic.HttpClients;
import org.apache.hc.core5.http.HttpHost;

@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "apache-http")
class ApacheHttpTest {

    @Test
    void shouldFetchWithApacheHttp(MagnetoProxy magneto) throws Exception {
        HttpHost proxy = new HttpHost("localhost", magneto.port());

        try (CloseableHttpClient client = HttpClients.custom()
                .setProxy(proxy)
                .build()) {

            HttpGet request = new HttpGet("https://api.example.com/data");
            client.execute(request, response -> {
                assertEquals(200, response.getCode());
                return null;
            });
        }
    }
}
```

### Tests paramétrés

```java
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

@ExtendWith(MagnetoExtension.class)
class UserApiTest {

    @ParameterizedTest
    @ValueSource(ints = {1, 2, 3})
    @Magneto(cassette = "users")
    void shouldFetchUser(int userId, MagnetoProxy magneto) throws Exception {
        // Test avec plusieurs utilisateurs
        String url = "https://api.example.com/users/" + userId;
        // ...
    }
}
```

### Fixture Personnalisée

```java
class BaseApiTest {
    protected OkHttpClient createClient(MagnetoProxy magneto) {
        Proxy proxy = new Proxy(
            Proxy.Type.HTTP,
            new InetSocketAddress("localhost", magneto.port())
        );

        return new OkHttpClient.Builder()
            .proxy(proxy)
            .addInterceptor(chain -> {
                Request request = chain.request().newBuilder()
                    .addHeader("Authorization", "Bearer token")
                    .build();
                return chain.proceed(request);
            })
            .build();
    }
}

@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "protected-api")
class ProtectedApiTest extends BaseApiTest {

    @Test
    void shouldAccessProtectedEndpoint(MagnetoProxy magneto) throws Exception {
        OkHttpClient client = createClient(magneto);
        // ...
    }
}
```

### Configuration Proxy Système

```java
import com.magneto.junit.MagnetoExtension;

@ExtendWith(MagnetoExtension.class)
@Magneto(cassette = "system-proxy")
class SystemProxyTest {

    @Test
    void shouldUseSystemProxy(MagnetoProxy magneto) {
        // Configurer le proxy système
        MagnetoExtension.configureSystemProxy(magneto);

        try {
            // Les appels HTTP utiliseront automatiquement le proxy
            // ...
        } finally {
            MagnetoExtension.clearSystemProxy();
        }
    }
}
```

## 🔄 Workflow Typique

### 1. Développement (premier run)

```bash
# Enregistre toutes les interactions
MAGNETO_MODE=record mvn test
```

Cassettes créées dans `./test_cassettes/`

### 2. CI/CD (tests rapides)

```bash
# Rejeu uniquement, échoue si cassette manquante
MAGNETO_MODE=strict mvn test
```

Aucun appel réseau → tests ultra-rapides ⚡

### 3. Mise à jour API

```bash
# Ré-enregistre une cassette spécifique
MAGNETO_MODE=record mvn test -Dtest=ApiTest#testUsers
```

### 4. Debugging

```bash
# Mode auto pour développement
mvn test

# Logs détaillés
mvn test -X
```

## ⚙️ API Reference

### Annotation `@Magneto`

| Paramètre | Type | Default | Description |
|-----------|------|---------|-------------|
| `cassette` | String | "" | Nom de la cassette |
| `mode` | String | "auto" | Mode: auto, record, replay, strict |
| `cassetteDir` | String | "./test_cassettes" | Répertoire cassettes |
| `port` | int | 8888 | Port du proxy |
| `scope` | Scope | METHOD | Scope du proxy |

### Enum `Scope`

- `METHOD`: Proxy isolé par méthode de test
- `CLASS`: Proxy partagé pour toute la classe

### Méthodes Statiques

```java
// Configurer le proxy système
MagnetoExtension.configureSystemProxy(magneto);

// Réinitialiser le proxy système
MagnetoExtension.clearSystemProxy();
```

## 🐛 Troubleshooting

### Erreur: "magneto-serge not found"

```bash
# Vérifier les dépendances
mvn dependency:tree | grep magneto
```

### Proxy ne démarre pas

```java
// Utiliser un port différent
@Magneto(cassette = "api-test", port = 9999)
```

### Cassettes non créées

```bash
# Vérifier le répertoire
ls -la test_cassettes/

# Forcer mode record
MAGNETO_MODE=record mvn test
```

### Mode strict échoue

```bash
# Vérifier que la cassette existe
ls test_cassettes/nom-cassette.json

# Ou passer en mode auto
MAGNETO_MODE=auto mvn test
```

## 🎓 Best Practices

### 1. Une cassette par test

```java
@Magneto(cassette = "specific-test", scope = Scope.METHOD)
// Cassette dédiée = meilleure isolation
```

### 2. Gitignorer les cassettes sensibles

```gitignore
# .gitignore
test_cassettes/*-secret.json
test_cassettes/*-auth.json
```

### 3. Versionner les cassettes stables

```bash
git add test_cassettes/stable-api-*.json
git commit -m "test: add stable API cassettes"
```

### 4. CI/CD en mode strict

```yaml
# .github/workflows/test.yml
- name: Run tests
  run: mvn test
  env:
    MAGNETO_MODE: strict
```

### 5. Documentation des cassettes

```java
/**
 * Test API GitHub v3
 *
 * Cassette: github-api-v3.json
 * Enregistré: 2025-10-12
 * Endpoint: https://api.github.com/repos/...
 */
@Magneto(cassette = "github-api-v3")
@Test
void testGitHubApi(MagnetoProxy magneto) {
    // ...
}
```

## 🔗 Ressources

- [Documentation magneto-serge](../../README.md)
- [Bindings Java](./README.md)
- [Exemples](./src/test/java/)
- [JUnit 5 User Guide](https://junit.org/junit5/docs/current/user-guide/)

## 📄 Licence

MIT OR Apache-2.0
