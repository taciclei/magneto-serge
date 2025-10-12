# XCTest Integration pour Magneto-Serge

Intégration XCTest officielle pour **magneto-serge** - enregistrez et rejouez les interactions HTTP/WebSocket dans vos tests XCTest (iOS et macOS).

## 🚀 Installation

### Swift Package Manager

```swift
// Package.swift
dependencies: [
    .package(url: "https://github.com/taciclei/magneto-serge.git", from: "0.1.0")
]
```

### Xcode

1. File → Add Package Dependencies
2. Entrez l'URL: `https://github.com/taciclei/magneto-serge.git`
3. Ajoutez `MagnetoXCTestCase.swift` à votre target de tests

## 📖 Usage Basique

### 1. Hériter de MagnetoXCTestCase

```swift
import XCTest

class APITests: MagnetoXCTestCase {

    override var magnetoConfiguration: MagnetoConfiguration {
        MagnetoConfiguration(cassetteName: "api-tests")
    }

    func testAPICall() {
        // Le proxy est automatiquement configuré
        let session = createURLSession()

        let expectation = self.expectation(description: "API call")

        let url = URL(string: "https://api.example.com/users")!
        let task = session.dataTask(with: url) { data, response, error in
            XCTAssertNotNil(data)
            XCTAssertNil(error)

            if let httpResponse = response as? HTTPURLResponse {
                XCTAssertEqual(httpResponse.statusCode, 200)
            }

            expectation.fulfill()
        }
        task.resume()

        waitForExpectations(timeout: 5.0)
    }
}
```

**Première exécution** → Enregistre dans `./test_cassettes/api-tests.json`
**Exécutions suivantes** → Rejeu depuis la cassette (aucun appel réseau)

## 🎯 Modes

### Mode Auto (par défaut)

```swift
class APITests: MagnetoXCTestCase {
    override var magnetoConfiguration: MagnetoConfiguration {
        MagnetoConfiguration(mode: "auto")
    }

    func testApi() {
        // Si cassette existe → replay
        // Si cassette manque → record
    }
}
```

### Mode Record (force l'enregistrement)

```bash
# Via variable d'environnement
export MAGNETO_MODE=record
xcodebuild test -scheme MyApp

# Via configuration
override var magnetoConfiguration: MagnetoConfiguration {
    MagnetoConfiguration(mode: "record")
}
```

### Mode Replay (rejeu uniquement)

```bash
export MAGNETO_MODE=replay
xcodebuild test -scheme MyApp
```

### Mode Strict (échoue si pas de match)

```swift
override var magnetoConfiguration: MagnetoConfiguration {
    MagnetoConfiguration(mode: "strict")
}
// Échoue si:
// - Cassette manquante
// - Requête non matchée dans la cassette
```

## 🔧 Configuration

### MagnetoConfiguration

```swift
MagnetoConfiguration(
    cassetteDir: "./test_cassettes",  // Répertoire cassettes
    mode: "auto",                     // auto|record|replay|strict
    port: 8888,                       // Port du proxy
    cassetteName: "api-tests"         // Nom cassette (optionnel)
)
```

### Scope: Test vs Class

#### Scope .test (par défaut)

Chaque test a son propre proxy isolé.

```swift
class APITests: MagnetoXCTestCase {
    override var magnetoScope: MagnetoScope {
        return .test
    }

    func test1() {
        // Proxy dédié pour ce test
    }

    func test2() {
        // Nouveau proxy pour ce test
    }
}
```

#### Scope .class

Proxy partagé pour toute la classe de tests.

```swift
class APITests: MagnetoXCTestCase {
    override var magnetoScope: MagnetoScope {
        return .class
    }

    func test1() {
        // Même proxy pour tous les tests
    }

    func test2() {
        // Réutilise le même proxy
    }
}
```

### Variables d'Environnement

```bash
# Mode par défaut
export MAGNETO_MODE=auto|record|replay|strict

# Répertoire cassettes
export MAGNETO_CASSETTE_DIR=./my_cassettes

# Port proxy
export MAGNETO_PORT=9999
```

Dans Xcode:
1. Product → Scheme → Edit Scheme
2. Test → Arguments → Environment Variables
3. Ajouter: `MAGNETO_MODE` = `record`

## 🧪 Exemples

### Tests API REST

```swift
import XCTest

class GitHubAPITests: MagnetoXCTestCase {

    override var magnetoConfiguration: MagnetoConfiguration {
        MagnetoConfiguration(cassetteName: "github-api")
    }

    func testFetchRepository() {
        let expectation = self.expectation(description: "Fetch repository")

        let url = URL(string: "https://api.github.com/repos/taciclei/magneto-serge")!
        let session = createURLSession()

        let task = session.dataTask(with: url) { data, response, error in
            XCTAssertNotNil(data)
            XCTAssertNil(error)

            if let data = data,
               let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any] {
                XCTAssertEqual(json["name"] as? String, "magneto-serge")
            }

            expectation.fulfill()
        }
        task.resume()

        waitForExpectations(timeout: 5.0)
    }
}
```

### Tests POST avec JSON

```swift
class UserAPITests: MagnetoXCTestCase {

    override var magnetoConfiguration: MagnetoConfiguration {
        MagnetoConfiguration(cassetteName: "create-user")
    }

    func testCreateUser() {
        performPOST(
            "https://api.example.com/users",
            json: ["name": "Alice", "email": "alice@example.com"]
        ) { data, response, error in
            XCTAssertNotNil(data)
            XCTAssertNil(error)
            XCTAssertEqual(response?.statusCode, 201)
        }
    }
}
```

### Helper Methods

```swift
class APITests: MagnetoXCTestCase {

    // Helper GET simple
    func testSimpleGET() {
        performGET("https://api.example.com/data") { data, response, error in
            XCTAssertNotNil(data)
            XCTAssertEqual(response?.statusCode, 200)
        }
    }

    // Helper POST avec JSON
    func testSimplePOST() {
        performPOST(
            "https://api.example.com/users",
            json: ["name": "Bob"]
        ) { data, response, error in
            XCTAssertEqual(response?.statusCode, 201)
        }
    }

    // Requête personnalisée
    func testCustomRequest() {
        var request = URLRequest(url: URL(string: "https://api.example.com")!)
        request.httpMethod = "PUT"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        performHTTPRequest(request) { data, response, error in
            XCTAssertNotNil(data)
        }
    }
}
```

### Tests avec Alamofire

```swift
import Alamofire
import XCTest

class AlamofireTests: MagnetoXCTestCase {

    override var magnetoConfiguration: MagnetoConfiguration {
        MagnetoConfiguration(cassetteName: "alamofire-api")
    }

    func testAlamofireRequest() {
        let expectation = self.expectation(description: "Alamofire request")

        // Créer Session Alamofire avec proxy
        let session = Session(configuration: urlSessionConfiguration)

        session.request("https://api.example.com/users")
            .validate()
            .responseJSON { response in
                switch response.result {
                case .success(let value):
                    print("Response: \(value)")
                    XCTAssertNotNil(value)
                case .failure(let error):
                    XCTFail("Request failed: \(error)")
                }
                expectation.fulfill()
            }

        waitForExpectations(timeout: 5.0)
    }
}
```

### Fixture Personnalisée

```swift
class BaseAPITest: MagnetoXCTestCase {

    var apiClient: URLSession!

    override func setUpWithError() throws {
        try super.setUpWithError()

        // Créer un client HTTP personnalisé
        apiClient = URLSession(configuration: urlSessionConfiguration)
    }

    override func tearDownWithError() throws {
        apiClient?.invalidateAndCancel()
        apiClient = nil
        try super.tearDownWithError()
    }

    func performAuthenticatedRequest(_ url: URL, completion: @escaping (Data?, URLResponse?, Error?) -> Void) {
        var request = URLRequest(url: url)
        request.setValue("Bearer token", forHTTPHeaderField: "Authorization")

        let task = apiClient.dataTask(with: request, completionHandler: completion)
        task.resume()
    }
}

class ProtectedAPITests: BaseAPITest {

    override var magnetoConfiguration: MagnetoConfiguration {
        MagnetoConfiguration(cassetteName: "protected-api")
    }

    func testProtectedEndpoint() {
        let expectation = self.expectation(description: "Protected endpoint")

        let url = URL(string: "https://api.example.com/protected")!
        performAuthenticatedRequest(url) { data, response, error in
            XCTAssertNotNil(data)
            expectation.fulfill()
        }

        waitForExpectations(timeout: 5.0)
    }
}
```

### Tests iOS avec UITests

```swift
import XCTest

class AppUITests: XCTestCase {

    var app: XCUIApplication!

    override func setUpWithError() throws {
        try super.setUpWithError()

        continueAfterFailure = false

        app = XCUIApplication()

        // Configurer magneto via launch environment
        app.launchEnvironment["MAGNETO_MODE"] = "replay"
        app.launchEnvironment["MAGNETO_CASSETTE"] = "ui-tests"
        app.launchEnvironment["MAGNETO_CASSETTE_DIR"] = "./test_cassettes"

        app.launch()
    }

    func testUserFlow() {
        // UI tests avec rejeu automatique des appels API
        app.buttons["Login"].tap()
        // ...
    }
}
```

## 🔄 Workflow Typique

### 1. Développement (premier run)

```bash
# Enregistre toutes les interactions
export MAGNETO_MODE=record
xcodebuild test -scheme MyApp
```

Cassettes créées dans `./test_cassettes/`

### 2. CI/CD (tests rapides)

```bash
# Rejeu uniquement, échoue si cassette manquante
export MAGNETO_MODE=strict
xcodebuild test -scheme MyApp
```

Aucun appel réseau → tests ultra-rapides ⚡

### 3. Mise à jour API

```bash
# Ré-enregistre une cassette spécifique
export MAGNETO_MODE=record
xcodebuild test -scheme MyApp -only-testing:MyAppTests/APITests/testUsers
```

### 4. Debugging

```bash
# Mode auto pour développement
xcodebuild test -scheme MyApp

# Logs détaillés
xcodebuild test -scheme MyApp -verbose
```

## ⚙️ API Reference

### MagnetoXCTestCase

Base class pour les tests avec magneto-serge.

**Propriétés**:
- `magneto: MagnetoProxy` - Instance proxy
- `urlSessionConfiguration: URLSessionConfiguration` - Configuration URLSession avec proxy

**Méthodes Override**:
```swift
// Configuration magneto
override var magnetoConfiguration: MagnetoConfiguration { ... }

// Scope du proxy (test ou class)
override var magnetoScope: MagnetoScope { ... }
```

**Helper Methods**:
```swift
// Créer URLSession avec proxy
func createURLSession() -> URLSession

// Configuration proxy pour client custom
func proxyConfiguration() -> [AnyHashable: Any]

// Helpers pour appels HTTP
func performHTTPRequest(_:timeout:completion:)
func performGET(_:timeout:completion:)
func performPOST(_:json:timeout:completion:)
```

### MagnetoConfiguration

```swift
struct MagnetoConfiguration {
    var cassetteDir: String             // "./test_cassettes"
    var mode: String                    // "auto"|"record"|"replay"|"strict"
    var port: UInt16                    // 8888
    var cassetteName: String?           // Optionnel

    static func fromEnvironment() -> MagnetoConfiguration
}
```

### MagnetoScope

```swift
enum MagnetoScope {
    case test   // Proxy isolé par test
    case class  // Proxy partagé pour la classe
}
```

## 🐛 Troubleshooting

### Erreur: "magneto-serge not found"

```bash
# Vérifier l'installation
swift package show-dependencies
```

### Proxy ne démarre pas

```swift
// Utiliser un port différent
override var magnetoConfiguration: MagnetoConfiguration {
    MagnetoConfiguration(port: 9999)
}
```

### Cassettes non créées

```bash
# Vérifier le répertoire
ls -la test_cassettes/

# Forcer mode record
export MAGNETO_MODE=record
xcodebuild test
```

### Mode strict échoue

```bash
# Vérifier que la cassette existe
ls test_cassettes/nom-cassette.json

# Ou passer en mode auto
export MAGNETO_MODE=auto
```

## 🎓 Best Practices

### 1. Une cassette par test

```swift
override var magnetoScope: MagnetoScope {
    return .test
}
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
  run: xcodebuild test -scheme MyApp
  env:
    MAGNETO_MODE: strict
```

### 5. Documentation des cassettes

```swift
/// Test API GitHub v3
///
/// Cassette: github-api-v3.json
/// Enregistré: 2025-10-12
/// Endpoint: https://api.github.com/repos/...
func testGitHubAPI() {
    // ...
}
```

## 🍎 Support Plateformes

- ✅ iOS 13.0+
- ✅ macOS 10.15+
- ✅ tvOS 13.0+
- ✅ watchOS 6.0+ (limité)
- ✅ Xcode 13.0+
- ✅ Swift 5.5+

## 🔗 Ressources

- [Documentation magneto-serge](../../README.md)
- [Bindings Swift](./README.md)
- [Exemples](./Example.swift)
- [XCTest Documentation](https://developer.apple.com/documentation/xctest)

## 📄 Licence

MIT OR Apache-2.0
