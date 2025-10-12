// MagnetoXCTestCase.swift - Base class for XCTest avec magneto-serge
// Enregistrement et rejeu automatique des interactions HTTP/WebSocket dans les tests XCTest

import Foundation
import XCTest

/// Configuration pour MagnetoXCTestCase
public struct MagnetoConfiguration {
    /// Répertoire des cassettes
    public var cassetteDir: String

    /// Mode proxy: auto, record, replay, strict
    public var mode: String

    /// Port du proxy
    public var port: UInt16

    /// Nom de la cassette (optionnel, généré depuis le test si vide)
    public var cassetteName: String?

    public init(
        cassetteDir: String = "./test_cassettes",
        mode: String = "auto",
        port: UInt16 = 8888,
        cassetteName: String? = nil
    ) {
        self.cassetteDir = cassetteDir
        self.mode = mode
        self.port = port
        self.cassetteName = cassetteName
    }

    /// Configuration depuis les variables d'environnement
    public static func fromEnvironment() -> MagnetoConfiguration {
        let cassetteDir = ProcessInfo.processInfo.environment["MAGNETO_CASSETTE_DIR"] ?? "./test_cassettes"
        let mode = ProcessInfo.processInfo.environment["MAGNETO_MODE"] ?? "auto"
        let portString = ProcessInfo.processInfo.environment["MAGNETO_PORT"] ?? "8888"
        let port = UInt16(portString) ?? 8888

        return MagnetoConfiguration(
            cassetteDir: cassetteDir,
            mode: mode,
            port: port
        )
    }
}

/// Base class pour les tests XCTest avec magneto-serge
///
/// Usage:
/// ```swift
/// class MyAPITests: MagnetoXCTestCase {
///     override var magnetoConfiguration: MagnetoConfiguration {
///         MagnetoConfiguration(cassetteName: "my-api-tests")
///     }
///
///     func testAPICall() {
///         // Utilisez magneto.urlSessionConfiguration
///         // ou magneto.proxies() pour configurer vos clients HTTP
///     }
/// }
/// ```
open class MagnetoXCTestCase: XCTestCase {

    /// Instance MagnetoProxy pour les tests
    public private(set) var magneto: MagnetoProxy!

    /// Configuration URLSession avec proxy
    public private(set) var urlSessionConfiguration: URLSessionConfiguration!

    /// Configuration magneto (override dans les sous-classes)
    open var magnetoConfiguration: MagnetoConfiguration {
        return MagnetoConfiguration.fromEnvironment()
    }

    /// Scope du proxy: class (partagé) ou test (isolé)
    open var magnetoScope: MagnetoScope {
        return .test
    }

    /// Proxy partagé au niveau de la classe (pour scope .class)
    private static var sharedProxy: MagnetoProxy?
    private static var sharedConfig: URLSessionConfiguration?

    // MARK: - Setup & Teardown

    override open class func setUp() {
        super.setUp()

        // Setup pour scope .class
        // Note: Pas d'accès à self ici, donc on ne peut pas vérifier magnetoScope
    }

    override open class func tearDown() {
        // Cleanup pour scope .class
        if let proxy = sharedProxy {
            proxy.shutdown()
            sharedProxy = nil
            sharedConfig = nil
        }

        super.tearDown()
    }

    override open func setUpWithError() throws {
        try super.setUpWithError()

        let config = magnetoConfiguration

        switch magnetoScope {
        case .class:
            // Utiliser le proxy partagé ou en créer un nouveau
            if let shared = Self.sharedProxy {
                magneto = shared
                urlSessionConfiguration = Self.sharedConfig
            } else {
                try setupProxy(with: config)
                Self.sharedProxy = magneto
                Self.sharedConfig = urlSessionConfiguration
            }

        case .test:
            // Créer un nouveau proxy pour chaque test
            try setupProxy(with: config)
        }
    }

    override open func tearDownWithError() throws {
        // Cleanup uniquement pour scope .test
        if magnetoScope == .test {
            magneto?.shutdown()
            magneto = nil
            urlSessionConfiguration = nil
        }

        try super.tearDownWithError()
    }

    // MARK: - Private Setup

    private func setupProxy(with config: MagnetoConfiguration) throws {
        // Créer le répertoire de cassettes si nécessaire
        let fileManager = FileManager.default
        let cassetteURL = URL(fileURLWithPath: config.cassetteDir)

        if !fileManager.fileExists(atPath: cassetteURL.path) {
            try fileManager.createDirectory(
                at: cassetteURL,
                withIntermediateDirectories: true
            )
        }

        // Créer le proxy
        magneto = MagnetoProxy(cassetteDir: config.cassetteDir)
        magneto.setPort(port: config.port)

        // Déterminer le nom de la cassette
        let cassetteName = determineCassetteName(from: config)

        // Configurer le mode
        configureMode(proxy: magneto, mode: config.mode, cassetteName: cassetteName)

        // Créer la configuration URLSession
        urlSessionConfiguration = createURLSessionConfiguration(port: config.port)
    }

    private func determineCassetteName(from config: MagnetoConfiguration) -> String {
        if let name = config.cassetteName, !name.isEmpty {
            return name
        }

        // Générer depuis le nom du test
        let className = String(describing: type(of: self))
        let testName = self.name
            .replacingOccurrences(of: "-[", with: "")
            .replacingOccurrences(of: "]", with: "")
            .replacingOccurrences(of: " ", with: "-")

        return "\(className)-\(testName)"
    }

    private func configureMode(proxy: MagnetoProxy, mode: String, cassetteName: String) {
        switch mode.lowercased() {
        case "record":
            proxy.setMode(mode: .record)
            _ = proxy.startRecording(cassetteName: cassetteName)

        case "replay":
            proxy.setMode(mode: .replay)
            _ = proxy.replay(cassetteName: cassetteName)

        case "strict":
            _ = proxy.replayStrict(cassetteName: cassetteName)

        case "auto":
            fallthrough
        default:
            _ = proxy.hybrid(cassetteName: cassetteName)
        }
    }

    private func createURLSessionConfiguration(port: UInt16) -> URLSessionConfiguration {
        let config = URLSessionConfiguration.default
        config.connectionProxyDictionary = [
            kCFNetworkProxiesHTTPEnable: true,
            kCFNetworkProxiesHTTPProxy: "localhost",
            kCFNetworkProxiesHTTPPort: port,
            kCFNetworkProxiesHTTPSEnable: true,
            kCFNetworkProxiesHTTPSProxy: "localhost",
            kCFNetworkProxiesHTTPSPort: port
        ]
        return config
    }

    // MARK: - Helper Methods

    /// Crée un URLSession configuré avec le proxy magneto
    public func createURLSession() -> URLSession {
        return URLSession(configuration: urlSessionConfiguration)
    }

    /// Retourne la configuration proxy pour un client HTTP personnalisé
    public func proxyConfiguration() -> [AnyHashable: Any] {
        return urlSessionConfiguration.connectionProxyDictionary ?? [:]
    }
}

/// Scope du proxy magneto
public enum MagnetoScope {
    /// Proxy partagé pour toute la classe de tests
    case `class`

    /// Proxy isolé pour chaque test
    case test
}

/// Extension pour faciliter l'utilisation avec XCTestExpectation
extension MagnetoXCTestCase {

    /// Helper pour faire un appel HTTP avec gestion automatique de l'expectation
    public func performHTTPRequest(
        _ request: URLRequest,
        timeout: TimeInterval = 10.0,
        completion: @escaping (Data?, URLResponse?, Error?) -> Void
    ) {
        let expectation = self.expectation(description: "HTTP request")

        let session = createURLSession()
        let task = session.dataTask(with: request) { data, response, error in
            completion(data, response, error)
            expectation.fulfill()
        }
        task.resume()

        waitForExpectations(timeout: timeout)
    }

    /// Helper pour faire un appel HTTP GET simple
    public func performGET(
        _ urlString: String,
        timeout: TimeInterval = 10.0,
        completion: @escaping (Data?, HTTPURLResponse?, Error?) -> Void
    ) {
        guard let url = URL(string: urlString) else {
            XCTFail("Invalid URL: \(urlString)")
            return
        }

        let request = URLRequest(url: url)
        performHTTPRequest(request, timeout: timeout) { data, response, error in
            completion(data, response as? HTTPURLResponse, error)
        }
    }

    /// Helper pour faire un appel HTTP POST avec JSON
    public func performPOST(
        _ urlString: String,
        json: [String: Any],
        timeout: TimeInterval = 10.0,
        completion: @escaping (Data?, HTTPURLResponse?, Error?) -> Void
    ) {
        guard let url = URL(string: urlString) else {
            XCTFail("Invalid URL: \(urlString)")
            return
        }

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        do {
            request.httpBody = try JSONSerialization.data(withJSONObject: json)
        } catch {
            XCTFail("Failed to serialize JSON: \(error)")
            return
        }

        performHTTPRequest(request, timeout: timeout) { data, response, error in
            completion(data, response as? HTTPURLResponse, error)
        }
    }
}
