# Magneto-Serge Kotlin Bindings

Bindings Kotlin pour **magneto-serge**, générés avec UniFFI.

## 🚀 Installation

### Gradle (build.gradle.kts)

```kotlin
dependencies {
    implementation(files("libs/magneto-serge-kotlin.jar"))
}
```

### Bibliothèque native

La bibliothèque native `libuniffi_magneto_serge.dylib` (macOS) ou `.so` (Linux) doit être dans le classpath.

```bash
# macOS
export DYLD_LIBRARY_PATH=/path/to/bindings/kotlin:$DYLD_LIBRARY_PATH

# Linux
export LD_LIBRARY_PATH=/path/to/bindings/kotlin:$LD_LIBRARY_PATH
```

## 📖 Usage Basique

```kotlin
import uniffi.magneto_serge.*

fun main() {
    // Créer un proxy
    val proxy = MagnetoProxy("./cassettes")
    proxy.setPort(8888u)

    // Mode enregistrement
    proxy.setMode(ProxyMode.RECORD)
    proxy.startRecording("api-test")

    // Configurez votre client HTTP pour utiliser le proxy
    // localhost:8888

    // Mode rejeu
    proxy.setMode(ProxyMode.REPLAY)
    proxy.replay("api-test")

    // Cleanup
    proxy.shutdown()
}
```

## 🎯 Modes Disponibles

### ProxyMode Enum

```kotlin
enum class ProxyMode {
    RECORD,      // Enregistrement
    REPLAY,      // Rejeu
    PASSTHROUGH  // Passthrough (no recording)
}
```

### Mode Enregistrement

```kotlin
proxy.setMode(ProxyMode.RECORD)
proxy.startRecording("cassette-name")

// Faites vos appels API
// ...

// Arrêt: appeler shutdown() ou changer de mode
```

### Mode Rejeu

```kotlin
proxy.setMode(ProxyMode.REPLAY)
val success = proxy.replay("cassette-name")

if (success) {
    // Cassette chargée, les appels seront servis depuis la cassette
}
```

### Mode Hybride (Auto)

```kotlin
val success = proxy.hybrid("cassette-name")

// Si cassette existe → rejeu
// Si cassette manque → enregistrement
```

### Mode Strict

```kotlin
val success = proxy.replayStrict("cassette-name")

// Échoue si:
// - Cassette manquante
// - Requête non matchée
```

### Mode Once (rejeu unique)

```kotlin
val success = proxy.once("cassette-name")

// Chaque interaction ne peut être rejouée qu'une fois
```

## 🔧 Intégration avec Frameworks

### OkHttp

```kotlin
import okhttp3.*
import java.net.Proxy
import java.net.InetSocketAddress

// Configuration proxy
val proxyConfig = Proxy(
    Proxy.Type.HTTP,
    InetSocketAddress("localhost", 8888)
)

val client = OkHttpClient.Builder()
    .proxy(proxyConfig)
    .build()

// Utilisation
val request = Request.Builder()
    .url("https://api.example.com/users")
    .build()

val response = client.newCall(request).execute()
```

### Ktor Client

```kotlin
import io.ktor.client.*
import io.ktor.client.engine.cio.*
import io.ktor.client.request.*

val client = HttpClient(CIO) {
    engine {
        proxy = ProxyBuilder.http("http://localhost:8888")
    }
}

val response = client.get("https://api.example.com/users")
```

### Retrofit

```kotlin
import retrofit2.Retrofit
import okhttp3.OkHttpClient
import java.net.Proxy
import java.net.InetSocketAddress

val proxyConfig = Proxy(
    Proxy.Type.HTTP,
    InetSocketAddress("localhost", 8888)
)

val client = OkHttpClient.Builder()
    .proxy(proxyConfig)
    .build()

val retrofit = Retrofit.Builder()
    .baseUrl("https://api.example.com")
    .client(client)
    .build()
```

## 🧪 Tests avec JUnit 5

### Extension JUnit

```kotlin
import org.junit.jupiter.api.extension.*
import uniffi.magneto_serge.*

class MagnetoExtension : BeforeAllCallback, AfterAllCallback {
    companion object {
        private lateinit var proxy: MagnetoProxy
    }

    override fun beforeAll(context: ExtensionContext) {
        proxy = MagnetoProxy("./test_cassettes")
        proxy.setPort(8888u)

        val cassetteName = context.displayName.replace(" ", "-")
        proxy.hybrid(cassetteName)

        // Configure system proxy
        System.setProperty("http.proxyHost", "localhost")
        System.setProperty("http.proxyPort", "8888")
        System.setProperty("https.proxyHost", "localhost")
        System.setProperty("https.proxyPort", "8888")
    }

    override fun afterAll(context: ExtensionContext) {
        proxy.shutdown()
    }
}
```

### Utilisation dans les tests

```kotlin
import org.junit.jupiter.api.*
import org.junit.jupiter.api.extension.ExtendWith

@ExtendWith(MagnetoExtension::class)
class ApiTest {

    @Test
    fun testGetUsers() {
        // Les appels HTTP passeront par magneto
        val response = // ... votre appel HTTP

        assert(response.isSuccessful)
    }

    @Test
    fun testCreateUser() {
        // Chaque test utilise le même proxy magneto
        // mais peut avoir sa propre cassette
    }
}
```

### Annotation personnalisée

```kotlin
import org.junit.jupiter.api.extension.ExtendWith
import kotlin.annotation.AnnotationTarget.*

@Target(FUNCTION, CLASS)
@Retention(AnnotationRetention.RUNTIME)
@ExtendWith(MagnetoExtension::class)
annotation class Magneto(
    val cassette: String = "",
    val mode: String = "auto", // auto, record, replay, strict
    val port: Int = 8888
)
```

Usage:

```kotlin
@Magneto(cassette = "github-api", mode = "strict")
@Test
fun testGitHubApi() {
    // Test avec cassette stricte
}
```

## 📚 API Reference

### Classe `MagnetoProxy`

#### Constructeur

```kotlin
MagnetoProxy(cassetteDir: String)
```

- `cassetteDir`: Répertoire de stockage des cassettes

#### Méthodes de Configuration

```kotlin
fun setPort(port: UShort)
fun port(): UShort

fun setMode(mode: ProxyMode)
fun mode(): ProxyMode
```

#### Méthodes d'Enregistrement/Rejeu

```kotlin
fun startRecording(cassetteName: String): Boolean
fun replay(cassetteName: String): Boolean
fun replayStrict(cassetteName: String): Boolean
fun hybrid(cassetteName: String): Boolean
fun once(cassetteName: String): Boolean
fun stopHybrid(): Boolean
fun shutdown()
```

## 🔍 Exemples

### Exemple Complet

Voir `Example.kt` pour des exemples détaillés :
- Mode enregistrement
- Mode rejeu
- Mode hybride
- Mode strict
- Mode once
- Intégration OkHttp
- Tests JUnit

### Exécuter l'exemple

```bash
# Compiler
kotlinc -cp uniffi/magneto_serge Example.kt -include-runtime -d example.jar

# Exécuter
java -Djava.library.path=. -jar example.jar
```

## 🐛 Troubleshooting

### Erreur: "java.lang.UnsatisfiedLinkError"

**Cause**: Bibliothèque native non trouvée.

**Solution**:
```bash
# macOS
export DYLD_LIBRARY_PATH=$(pwd):$DYLD_LIBRARY_PATH

# Linux
export LD_LIBRARY_PATH=$(pwd):$LD_LIBRARY_PATH
```

### Erreur: "Cannot find magneto_serge"

**Cause**: Classes Kotlin non dans le classpath.

**Solution**:
```bash
kotlinc -cp uniffi/magneto_serge YourApp.kt
```

### Proxy ne démarre pas

**Cause**: Port déjà utilisé.

**Solution**:
```kotlin
proxy.setPort(9999u)  // Utiliser un port différent
```

## 📦 Build

### Avec Gradle

```bash
./gradlew build
```

### Créer JAR

```bash
./gradlew jar
```

### Tests

```bash
./gradlew test
```

## 🌟 Fonctionnalités

- ✅ Enregistrement HTTP/HTTPS
- ✅ Enregistrement WebSocket
- ✅ Rejeu déterministe
- ✅ Mode hybride (auto)
- ✅ Mode strict
- ✅ Support multi-thread
- ✅ Compatible JVM 8+
- ✅ Compatible Android

## 📄 Licence

MIT OR Apache-2.0

## 🔗 Ressources

- [Documentation principale](../../README.md)
- [Architecture](../../docs/ARCHITECTURE.md)
- [Exemples](../../docs/EXAMPLES.md)
- [ROADMAP](../../docs/ROADMAP.md)
