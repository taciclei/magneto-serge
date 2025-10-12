# Magneto-Serge Swift Bindings

Bindings Swift pour **magneto-serge**, générés avec UniFFI.

## 🚀 Installation

### Swift Package Manager

```swift
// Package.swift
dependencies: [
    .package(url: "https://github.com/taciclei/magneto-serge.git", from: "0.1.0")
]
```

### Manuel

1. Copier `magneto_serge.swift` dans votre projet
2. Copier `libuniffi_magneto_serge.dylib` (macOS) ou `.so` (Linux)
3. Ajouter le module map

## 📖 Usage Basique

```swift
import Foundation

// Créer un proxy
let proxy = MagnetoProxy(cassetteDir: "./cassettes")
proxy.setPort(port: 8888)

// Mode enregistrement
proxy.setMode(mode: .record)
proxy.startRecording(cassetteName: "api-test")

// Configurez URLSession pour utiliser le proxy
// localhost:8888

// Mode rejeu
proxy.setMode(mode: .replay)
proxy.replay(cassetteName: "api-test")

// Cleanup
proxy.shutdown()
```

## 🎯 Modes Disponibles

### ProxyMode Enum

```swift
enum ProxyMode {
    case record      // Enregistrement
    case replay      // Rejeu
    case passthrough // Passthrough (pas d'enregistrement)
}
```

### Mode Enregistrement

```swift
proxy.setMode(mode: .record)
proxy.startRecording(cassetteName: "cassette-name")

// Faites vos appels API
// ...
```

### Mode Rejeu

```swift
proxy.setMode(mode: .replay)
let success = proxy.replay(cassetteName: "cassette-name")

if success {
    // Cassette chargée
}
```

### Mode Hybride (Auto)

```swift
let success = proxy.hybrid(cassetteName: "cassette-name")

// Si cassette existe → rejeu
// Si cassette manque → enregistrement
```

### Mode Strict

```swift
let success = proxy.replayStrict(cassetteName: "cassette-name")

// Échoue si:
// - Cassette manquante
// - Requête non matchée
```

### Mode Once

```swift
let success = proxy.once(cassetteName: "cassette-name")

// Chaque interaction ne peut être rejouée qu'une fois
```

## 🔧 Intégration avec URLSession

### Configuration Proxy

```swift
import Foundation

let proxy = MagnetoProxy(cassetteDir: "./test_cassettes")
proxy.setPort(port: 8888)
proxy.hybrid(cassetteName: "api-test")

// Configuration URLSession
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

// Utilisation
let url = URL(string: "https://api.example.com/users")!
let task = session.dataTask(with: url) { data, response, error in
    if let data = data {
        print("Response: \(String(data: data, encoding: .utf8) ?? "")")
    }
}
task.resume()
```

## 🧪 Tests avec XCTest

### Test Class

```swift
import XCTest
@testable import YourApp

class APITests: XCTestCase {
    var proxy: MagnetoProxy!
    var session: URLSession!

    override func setUpWithError() throws {
        try super.setUpWithError()

        // Setup proxy
        proxy = MagnetoProxy(cassetteDir: "./test_cassettes")
        proxy.setPort(port: 8888)
        proxy.hybrid(cassetteName: "api-tests")

        // Setup URLSession avec proxy
        let config = URLSessionConfiguration.default
        config.connectionProxyDictionary = [
            kCFNetworkProxiesHTTPEnable: true,
            kCFNetworkProxiesHTTPProxy: "localhost",
            kCFNetworkProxiesHTTPPort: 8888,
            kCFNetworkProxiesHTTPSEnable: true,
            kCFNetworkProxiesHTTPSProxy: "localhost",
            kCFNetworkProxiesHTTPSPort: 8888
        ]
        session = URLSession(configuration: config)
    }

    override func tearDownWithError() throws {
        proxy.shutdown()
        try super.tearDownWithError()
    }

    func testAPICall() throws {
        let expectation = XCTestExpectation(description: "API call")

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

        wait(for: [expectation], timeout: 5.0)
    }

    func testPOSTRequest() throws {
        let expectation = XCTestExpectation(description: "POST request")

        let url = URL(string: "https://api.example.com/users")!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let userData = ["name": "Alice", "email": "alice@example.com"]
        request.httpBody = try? JSONSerialization.data(withJSONObject: userData)

        let task = session.dataTask(with: request) { data, response, error in
            XCTAssertNotNil(data)
            XCTAssertNil(error)
            expectation.fulfill()
        }
        task.resume()

        wait(for: [expectation], timeout: 5.0)
    }
}
```

### Test individuel avec cassette spécifique

```swift
func testSpecificAPI() throws {
    // Créer proxy pour ce test uniquement
    let testProxy = MagnetoProxy(cassetteDir: "./test_cassettes")
    testProxy.setPort(port: 9999)  // Port différent
    testProxy.replayStrict(cassetteName: "specific-test")

    defer {
        testProxy.shutdown()
    }

    // Configuration URLSession pour ce test
    let config = URLSessionConfiguration.default
    config.connectionProxyDictionary = [
        kCFNetworkProxiesHTTPProxy: "localhost",
        kCFNetworkProxiesHTTPPort: 9999
    ]
    let testSession = URLSession(configuration: config)

    // Votre test...
}
```

## 🔗 Intégration Alamofire

```swift
import Alamofire

// Configuration Session Alamofire avec proxy
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
    .responseDecodable(of: [User].self) { response in
        switch response.result {
        case .success(let users):
            print("Users: \(users)")
        case .failure(let error):
            print("Error: \(error)")
        }
    }
```

## 📚 API Reference

### Classe `MagnetoProxy`

#### Initializer

```swift
init(cassetteDir: String)
```

- `cassetteDir`: Répertoire de stockage des cassettes

#### Méthodes de Configuration

```swift
func setPort(port: UInt16)
func port() -> UInt16

func setMode(mode: ProxyMode)
func mode() -> ProxyMode
```

#### Méthodes d'Enregistrement/Rejeu

```swift
func startRecording(cassetteName: String) -> Bool
func replay(cassetteName: String) -> Bool
func replayStrict(cassetteName: String) -> Bool
func hybrid(cassetteName: String) -> Bool
func once(cassetteName: String) -> Bool
func stopHybrid() -> Bool
func shutdown()
```

## 🔍 Exemples

### Exemple Complet

Voir `Example.swift` pour des exemples détaillés :
- Mode enregistrement
- Mode rejeu
- Mode hybride
- Mode strict
- Mode once
- Intégration URLSession
- Tests XCTest
- Intégration Alamofire

### Exécuter l'exemple

```bash
# Compiler
swiftc -import-objc-header magneto_sergeFFI.h Example.swift magneto_serge.swift -o example

# Exécuter
./example
```

## 🍎 Support iOS

### Configuration pour iOS

```swift
import UIKit

class NetworkManager {
    static let shared = NetworkManager()
    private var proxy: MagnetoProxy?

    func setupProxy(cassetteDir: String) {
        let documentsPath = FileManager.default.urls(
            for: .documentDirectory,
            in: .userDomainMask
        )[0].path

        let cassettePath = "\(documentsPath)/cassettes"
        proxy = MagnetoProxy(cassetteDir: cassettePath)
        proxy?.setPort(port: 8888)
        proxy?.hybrid(cassetteName: "ios-app")
    }

    func getURLSession() -> URLSession {
        let config = URLSessionConfiguration.default
        config.connectionProxyDictionary = [
            kCFNetworkProxiesHTTPEnable: true,
            kCFNetworkProxiesHTTPProxy: "localhost",
            kCFNetworkProxiesHTTPPort: 8888
        ]
        return URLSession(configuration: config)
    }

    func cleanup() {
        proxy?.shutdown()
    }
}
```

### UITests

```swift
import XCTest

class AppUITests: XCTestCase {
    var app: XCUIApplication!

    override func setUpWithError() throws {
        try super.setUpWithError()

        app = XCUIApplication()
        app.launchEnvironment["MAGNETO_MODE"] = "replay"
        app.launchEnvironment["MAGNETO_CASSETTE"] = "ui-tests"
        app.launch()
    }

    func testUserFlow() {
        // UI tests avec rejeu automatique des appels API
    }
}
```

## 🐛 Troubleshooting

### Erreur: "Library not loaded"

**Cause**: Bibliothèque native non trouvée.

**Solution macOS**:
```bash
export DYLD_LIBRARY_PATH=$(pwd):$DYLD_LIBRARY_PATH
```

**Solution iOS**: Copier libuniffi_magneto_serge.dylib dans le bundle

### Module not found

**Cause**: Module map incorrect.

**Solution**: Vérifier que `magneto_sergeFFI.modulemap` est présent et correct.

### Proxy ne démarre pas

**Cause**: Port déjà utilisé.

**Solution**:
```swift
proxy.setPort(port: 9999)  // Utiliser un port différent
```

## 📦 Build

### Avec Swift Package Manager

```bash
swift build
swift test
```

### Xcode

1. Ajouter les fichiers au projet
2. Ajouter la bibliothèque aux Frameworks
3. Configurer Build Settings

## 🌟 Fonctionnalités

- ✅ Enregistrement HTTP/HTTPS
- ✅ Enregistrement WebSocket
- ✅ Rejeu déterministe
- ✅ Mode hybride (auto)
- ✅ Mode strict
- ✅ Support iOS 13+
- ✅ Support macOS 10.15+
- ✅ Thread-safe

## 📄 Licence

MIT OR Apache-2.0

## 🔗 Ressources

- [Documentation principale](../../README.md)
- [Architecture](../../docs/ARCHITECTURE.md)
- [Exemples](../../docs/EXAMPLES.md)
- [ROADMAP](../../docs/ROADMAP.md)
