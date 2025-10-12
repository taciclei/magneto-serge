// Example.kt - Exemple d'utilisation des bindings Kotlin magneto-serge
package magneto.example

import uniffi.magneto_serge.*

fun main() {
    println("=".repeat(60))
    println("🧪 Exemple magneto-serge Kotlin")
    println("=".repeat(60))
    println()

    // Exemple 1: Créer un proxy
    println("Exemple 1: Création d'un proxy")
    println("-".repeat(60))

    val proxy = MagnetoProxy("./test_cassettes")
    println("✓ Proxy créé")

    proxy.setPort(8888u)
    println("✓ Port configuré: ${proxy.port()}")

    println()

    // Exemple 2: Mode enregistrement
    println("Exemple 2: Mode enregistrement")
    println("-".repeat(60))

    proxy.setMode(ProxyMode.RECORD)
    println("✓ Mode: ${proxy.mode()}")

    val recordSuccess = proxy.startRecording("example-api")
    println("✓ Enregistrement démarré: $recordSuccess")

    println("""
    Maintenant, configurez votre application pour utiliser le proxy:

    System.setProperty("http.proxyHost", "localhost")
    System.setProperty("http.proxyPort", "8888")
    System.setProperty("https.proxyHost", "localhost")
    System.setProperty("https.proxyPort", "8888")

    Faites vos appels API, puis arrêtez l'enregistrement.
    """.trimIndent())

    println()

    // Exemple 3: Mode rejeu
    println("Exemple 3: Mode rejeu")
    println("-".repeat(60))

    proxy.setMode(ProxyMode.REPLAY)
    println("✓ Mode: ${proxy.mode()}")

    val replaySuccess = proxy.replay("example-api")
    println("✓ Cassette chargée: $replaySuccess")

    println("""
    Les requêtes HTTP seront maintenant servies depuis la cassette,
    sans faire d'appels réseau réels.
    """.trimIndent())

    println()

    // Exemple 4: Mode hybride (auto)
    println("Exemple 4: Mode hybride (auto)")
    println("-".repeat(60))

    val hybridSuccess = proxy.hybrid("example-api")
    println("✓ Mode hybride activé: $hybridSuccess")

    println("""
    En mode hybride:
    - Si la cassette existe: rejeu automatique
    - Si la cassette n'existe pas: enregistrement automatique

    Parfait pour les tests CI/CD !
    """.trimIndent())

    println()

    // Exemple 5: Mode strict
    println("Exemple 5: Mode rejeu strict")
    println("-".repeat(60))

    val strictSuccess = proxy.replayStrict("example-api")
    println("✓ Mode strict activé: $strictSuccess")

    println("""
    En mode strict:
    - Toutes les requêtes doivent avoir une correspondance exacte
    - Aucune requête supplémentaire tolérée
    - Idéal pour les tests d'intégration rigoureux
    """.trimIndent())

    println()

    // Exemple 6: Mode once (rejeu unique)
    println("Exemple 6: Rejeu unique (one-shot)")
    println("-".repeat(60))

    val onceSuccess = proxy.once("example-api")
    println("✓ Mode once activé: $onceSuccess")

    println("""
    En mode once:
    - Chaque interaction enregistrée ne peut être utilisée qu'une fois
    - Détecte les requêtes dupliquées ou en boucle
    - Utile pour tester les idempotences
    """.trimIndent())

    println()

    // Cleanup
    println("Cleanup")
    println("-".repeat(60))
    proxy.shutdown()
    println("✓ Proxy arrêté")

    println()
    println("=".repeat(60))
    println("✅ Tous les exemples ont été exécutés avec succès!")
    println("=".repeat(60))

    println("""

    Pour aller plus loin:

    1. Documentation: https://github.com/taciclei/magneto-serge
    2. Tests: ./gradlew test
    3. Build: ./gradlew build
    """.trimIndent())
}

// Exemple avec OkHttp
fun exampleWithOkHttp() {
    println("Exemple avec OkHttp")
    println("-".repeat(60))

    val proxy = MagnetoProxy("./test_cassettes")
    proxy.setPort(8888u)
    proxy.hybrid("okhttp-test")

    // Configuration OkHttp avec proxy
    val proxyConfig = java.net.Proxy(
        java.net.Proxy.Type.HTTP,
        java.net.InetSocketAddress("localhost", 8888)
    )

    println("""
    Configuration OkHttp:

    val client = OkHttpClient.Builder()
        .proxy(proxyConfig)
        .build()

    val request = Request.Builder()
        .url("https://api.example.com/users")
        .build()

    val response = client.newCall(request).execute()
    """.trimIndent())

    proxy.shutdown()
}

// Exemple pour tests unitaires JUnit
fun exampleJUnitUsage() {
    println("Exemple JUnit")
    println("-".repeat(60))

    println("""
    import org.junit.jupiter.api.*
    import uniffi.magneto_serge.*

    class ApiTest {
        companion object {
            private lateinit var proxy: MagnetoProxy

            @BeforeAll
            @JvmStatic
            fun setupProxy() {
                proxy = MagnetoProxy("./test_cassettes")
                proxy.setPort(8888u)
                proxy.hybrid("junit-test")
            }

            @AfterAll
            @JvmStatic
            fun teardownProxy() {
                proxy.shutdown()
            }
        }

        @Test
        fun testApiCall() {
            // Vos tests HTTP ici
            // Les appels passeront par le proxy magneto
        }
    }
    """.trimIndent())
}
