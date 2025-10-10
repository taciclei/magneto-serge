# ☕ matgto-serge - Java Bindings

Bindings Java pour **matgto-serge**, générés via interopérabilité Kotlin/Java.

## 📦 Installation

### Gradle

```gradle
dependencies {
    implementation 'io.github.matgto:matgto-serge:0.1.0'
    implementation 'net.java.dev.jna:jna:5.13.0'
}
```

### Maven

```xml
<dependency>
    <groupId>io.github.matgto</groupId>
    <artifactId>matgto-serge</artifactId>
    <version>0.1.0</version>
</dependency>
<dependency>
    <groupId>net.java.dev.jna</groupId>
    <artifactId>jna</artifactId>
    <version>5.13.0</version>
</dependency>
```

## 🚀 Utilisation Rapide

```java
import io.github.matgto.serge.MatgtoProxy;
import io.github.matgto.serge.MatgtoProxy.Mode;

public class MyTest {
    public void testApi() {
        // Créer un proxy
        MatgtoProxy proxy = new MatgtoProxy("./cassettes");

        // Configurer
        proxy.setPort(8888);
        proxy.setMode(Mode.RECORD);

        // Enregistrer
        if (proxy.startRecording("my_test")) {
            // Faire des requêtes HTTP...
            // Elles seront enregistrées automatiquement

            proxy.stopRecording();
        }

        proxy.shutdown();
    }
}
```

## 🧪 Intégration JUnit 5

```java
import io.github.matgto.serge.MatgtoProxy;
import org.junit.jupiter.api.*;

public class ApiTest {

    private MatgtoProxy proxy;

    @BeforeEach
    void setUp() {
        proxy = new MatgtoProxy("./cassettes");
        proxy.setPort(8888);
    }

    @AfterEach
    void tearDown() {
        proxy.shutdown();
    }

    @Test
    void testApiEndpoint() {
        // Mode enregistrement
        proxy.setMode(MatgtoProxy.Mode.RECORD);
        proxy.startRecording("api_endpoint_test");

        // Faire requête HTTP via proxy
        HttpClient client = HttpClient.newBuilder()
            .proxy(ProxySelector.of(
                new InetSocketAddress("localhost", 8888)
            ))
            .build();

        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create("https://api.example.com/users"))
            .GET()
            .build();

        HttpResponse<String> response = client.send(
            request,
            HttpResponse.BodyHandlers.ofString()
        );

        assertEquals(200, response.statusCode());

        proxy.stopRecording();
    }

    @Test
    void testApiReplay() {
        // Mode replay
        proxy.setMode(MatgtoProxy.Mode.REPLAY);
        proxy.replay("api_endpoint_test");

        // Les requêtes seront rejouées depuis la cassette
        // Pas besoin de connexion réseau!
    }
}
```

## 📖 API

### MatgtoProxy

**Constructeur:**
```java
MatgtoProxy(String cassetteDir)
MatgtoProxy.create(String cassetteDir) // Factory method
```

**Configuration:**
```java
void setPort(int port)
void setMode(Mode mode)

int getPort()
Mode getMode()
```

**Enregistrement:**
```java
boolean startRecording(String cassetteName)
boolean stopRecording()
```

**Replay:**
```java
boolean replay(String cassetteName)
```

**Lifecycle:**
```java
void shutdown()
```

**Statique:**
```java
static String getVersion()
```

### Mode (Enum)

```java
MatgtoProxy.Mode.AUTO        // Auto-détection
MatgtoProxy.Mode.RECORD      // Enregistrement
MatgtoProxy.Mode.REPLAY      // Replay
MatgtoProxy.Mode.PASSTHROUGH // Passthrough sans enregistrement
```

## 🔧 Configuration HttpClient

```java
// Créer HttpClient avec proxy matgto
HttpClient client = HttpClient.newBuilder()
    .proxy(ProxySelector.of(
        new InetSocketAddress("localhost", 8888)
    ))
    .build();

// Pour HTTPS, accepter certificat auto-signé (DEV uniquement!)
SSLContext sslContext = SSLContext.getInstance("TLS");
sslContext.init(null, trustAllCerts, new SecureRandom());

HttpClient client = HttpClient.newBuilder()
    .proxy(ProxySelector.of(new InetSocketAddress("localhost", 8888)))
    .sslContext(sslContext)
    .build();
```

## 📁 Structure des Cassettes

Les cassettes sont sauvegardées en JSON:

```
./cassettes/
├── my_test.json
├── api_endpoint_test.json
└── another_test.json
```

## 🏗️ Build depuis les Sources

```bash
# Compiler
./gradlew build

# Exécuter l'exemple
./gradlew runExample

# Lancer les tests
./gradlew test

# Générer Javadoc
./gradlew javadoc
```

## 🔗 Interopérabilité Kotlin

Les bindings Java utilisent directement les bindings Kotlin générés par UniFFI.
Vous pouvez aussi utiliser directement le code Kotlin si vous préférez:

```kotlin
import uniffi.matgto_serge.*

val proxy = createProxy("./cassettes")
proxy.setPort(8888u)
proxy.setMode(ProxyMode.RECORD)
```

## ⚙️ Dépendances

- **JNA 5.13.0** - Java Native Access (requis)
- **Kotlin stdlib 1.9.20** - Runtime Kotlin (requis)
- **JUnit 5.10.0** - Tests (optionnel)

## 📚 Exemples

Voir le répertoire `examples/` pour plus d'exemples:
- `Example.java` - Utilisation basique
- `MatgtoTest.java` - Tests JUnit complets
- `SpringBootExample.java` - Intégration Spring Boot (à venir)

## 🐛 Troubleshooting

### UnsatisfiedLinkError

Si vous voyez `UnsatisfiedLinkError: libuniffi_matgto_serge.dylib`:

```bash
# Copier la bibliothèque native
cp ../kotlin/libuniffi_matgto_serge.dylib .

# Ou définir java.library.path
java -Djava.library.path=. -jar myapp.jar
```

### Version Mismatch

Assurez-vous que:
- Bindings Java = 0.1.0
- Bindings Kotlin = 0.1.0
- Bibliothèque native = 0.1.0

## 📄 License

MIT OR Apache-2.0

## 🤝 Contributeurs

matgto-serge contributors
