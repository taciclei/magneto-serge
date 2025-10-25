// Example.swift - Exemple d'utilisation des bindings Swift magneto-serge
import Foundation

func printSeparator(_ char: String = "=", length: Int = 60) {
    print(String(repeating: char, count: length))
}

func printHeader(_ title: String) {
    printSeparator()
    print("🧪 \(title)")
    printSeparator()
    print()
}

func main() {
    printHeader("Exemples magneto-serge Swift")

    // Exemple 1: Créer un proxy
    print("Exemple 1: Création d'un proxy")
    printSeparator("-")

    let proxy = MagnetoProxy(cassetteDir: "./test_cassettes")
    print("✓ Proxy créé")

    proxy.setPort(port: 8888)
    print("✓ Port configuré: \(proxy.port())")

    print()

    // Exemple 2: Mode enregistrement
    print("Exemple 2: Mode enregistrement")
    printSeparator("-")

    proxy.setMode(mode: .record)
    print("✓ Mode: \(proxy.mode())")

    let recordSuccess = proxy.startRecording(cassetteName: "example-api")
    print("✓ Enregistrement démarré: \(recordSuccess)")

    print("""

    Maintenant, configurez votre URLSession pour utiliser le proxy:

    let config = URLSessionConfiguration.default
    config.connectionProxyDictionary = [
        kCFNetworkProxiesHTTPEnable: true,
        kCFNetworkProxiesHTTPProxy: "localhost",
        kCFNetworkProxiesHTTPPort: 8888,
        kCFNetworkProxiesHTTPSEnable: true,
        kCFNetworkProxiesHTTPSProxy: "localhost",
        kCFNetworkProxiesHTTPSPort: 8888
    ]

    let session = URLSession(configuration: config)

    Faites vos appels API, puis arrêtez l'enregistrement.
    """)

    print()

    // Exemple 3: Mode rejeu
    print("Exemple 3: Mode rejeu")
    printSeparator("-")

    proxy.setMode(mode: .replay)
    print("✓ Mode: \(proxy.mode())")

    let replaySuccess = proxy.replay(cassetteName: "example-api")
    print("✓ Cassette chargée: \(replaySuccess)")

    print("""

    Les requêtes HTTP seront maintenant servies depuis la cassette,
    sans faire d'appels réseau réels.
    """)

    print()

    // Exemple 4: Mode hybride (auto)
    print("Exemple 4: Mode hybride (auto)")
    printSeparator("-")

    let hybridSuccess = proxy.hybrid(cassetteName: "example-api")
    print("✓ Mode hybride activé: \(hybridSuccess)")

    print("""

    En mode hybride:
    - Si la cassette existe: rejeu automatique
    - Si la cassette n'existe pas: enregistrement automatique

    Parfait pour les tests CI/CD !
    """)

    print()

    // Exemple 5: Mode strict
    print("Exemple 5: Mode rejeu strict")
    printSeparator("-")

    let strictSuccess = proxy.replayStrict(cassetteName: "example-api")
    print("✓ Mode strict activé: \(strictSuccess)")

    print("""

    En mode strict:
    - Toutes les requêtes doivent avoir une correspondance exacte
    - Aucune requête supplémentaire tolérée
    - Idéal pour les tests d'intégration rigoureux
    """)

    print()

    // Exemple 6: Mode once
    print("Exemple 6: Rejeu unique (one-shot)")
    printSeparator("-")

    let onceSuccess = proxy.once(cassetteName: "example-api")
    print("✓ Mode once activé: \(onceSuccess)")

    print("""

    En mode once:
    - Chaque interaction enregistrée ne peut être utilisée qu'une fois
    - Détecte les requêtes dupliquées ou en boucle
    - Utile pour tester les idempotences
    """)

    print()

    // Cleanup
    print("Cleanup")
    printSeparator("-")
    proxy.shutdown()
    print("✓ Proxy arrêté")

    print()
    printSeparator()
    print("✅ Tous les exemples ont été exécutés avec succès!")
    printSeparator()

    print("""

    Pour aller plus loin:

    1. Documentation: https://github.com/taciclei/magneto-serge
    2. Tests: swift test
    3. Build: swift build
    """)
}

// Exemple avec URLSession
func exampleWithURLSession() {
    print("Exemple avec URLSession")
    printSeparator("-")

    let proxy = MagnetoProxy(cassetteDir: "./test_cassettes")
    proxy.setPort(port: 8888)
    proxy.hybrid(cassetteName: "urlsession-test")

    // Configuration URLSession avec proxy
    let config = URLSessionConfiguration.default
    config.connectionProxyDictionary = [
        kCFNetworkProxiesHTTPEnable: true,
        kCFNetworkProxiesHTTPProxy: "localhost",
        kCFNetworkProxiesHTTPPort: 8888,
        kCFNetworkProxiesHTTPSEnable: true,
        kCFNetworkProxiesHTTPSProxy: "localhost",
        kCFNetworkProxiesHTTPSPort: 8888
    ]

    let session = URLSession(configuration: config)

    print("""
    Configuration URLSession avec proxy:

    let url = URL(string: "https://api.example.com/users")!
    let task = session.dataTask(with: url) { data, response, error in
        if let data = data {
            print("Response: \\(String(data: data, encoding: .utf8) ?? "")")
        }
    }
    task.resume()
    """)

    proxy.shutdown()
}

// Exemple pour tests XCTest
func exampleXCTestUsage() {
    print("Exemple XCTest")
    printSeparator("-")

    print("""
    import XCTest
    @testable import YourApp

    class APITests: XCTestCase {
        var proxy: MagnetoProxy!

        override func setUpWithError() throws {
            try super.setUpWithError()

            proxy = MagnetoProxy(cassetteDir: "./test_cassettes")
            proxy.setPort(port: 8888)
            proxy.hybrid(cassetteName: "xctest")

            // Configure URLSession global proxy
            let config = URLSessionConfiguration.default
            config.connectionProxyDictionary = [
                kCFNetworkProxiesHTTPEnable: true,
                kCFNetworkProxiesHTTPProxy: "localhost",
                kCFNetworkProxiesHTTPPort: 8888
            ]
        }

        override func tearDownWithError() throws {
            proxy.shutdown()
            try super.tearDownWithError()
        }

        func testAPICall() throws {
            let expectation = XCTestExpectation(description: "API call")

            // Votre appel API ici
            let url = URL(string: "https://api.example.com/users")!
            let task = URLSession.shared.dataTask(with: url) { data, response, error in
                XCTAssertNotNil(data)
                XCTAssertNil(error)
                expectation.fulfill()
            }
            task.resume()

            wait(for: [expectation], timeout: 5.0)
        }
    }
    """)
}

// Exemple Alamofire
func exampleAlamofireUsage() {
    print("Exemple Alamofire")
    printSeparator("-")

    print("""
    import Alamofire

    // Configuration Session avec proxy
    let config = URLSessionConfiguration.default
    config.connectionProxyDictionary = [
        kCFNetworkProxiesHTTPEnable: true,
        kCFNetworkProxiesHTTPProxy: "localhost",
        kCFNetworkProxiesHTTPPort: 8888
    ]

    let session = Session(configuration: config)

    // Utilisation
    session.request("https://api.example.com/users")
        .validate()
        .responseJSON { response in
            switch response.result {
            case .success(let value):
                print("Success: \\(value)")
            case .failure(let error):
                print("Error: \\(error)")
            }
        }
    """)
}

// Point d'entrée
main()
