# Architecture Technique - matgto-serge

**Version:** 1.0
**Date:** 2025-10-10
**Status:** Design Document

---

## 📐 Vue d'Ensemble Architecturale

matgto-serge est une bibliothèque de test construite sur une architecture proxy MITM (Man-in-the-Middle) qui intercepte le trafic HTTP/HTTPS et WebSocket, l'enregistre dans des "cassettes" réutilisables, et peut rejouer ces interactions de manière déterministe.

### Principes de Design

1. **Performance First** - Écrit en Rust pour maximiser le throughput (>5000 req/s)
2. **Zero-Copy** - Minimiser les allocations mémoire et copies de données
3. **Async/Await** - Tokio pour concurrence maximale
4. **Type Safety** - Exploiter le système de types Rust pour éviter bugs runtime
5. **Multi-Language** - UniFFI pour générer bindings automatiquement
6. **Developer Experience** - API simple, configuration minimale

---

## 🏗️ Architecture en Couches

```
┌─────────────────────────────────────────────────────────────┐
│                    User Application Layer                    │
│  (Java/JavaScript/Python/Ruby/Kotlin/Swift/Go/C#)           │
└────────────────────────┬────────────────────────────────────┘
                         │ UniFFI Bindings
┌────────────────────────▼────────────────────────────────────┐
│                   matgto-serge Public API                    │
│  MatgtoProxy::new() / start_recording() / replay()          │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                      Core Proxy Layer                        │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   HTTP      │  │   HTTPS      │  │  WebSocket   │       │
│  │ Interceptor │  │ Interceptor  │  │ Interceptor  │       │
│  │  (Hudsucker)│  │  (Hudsucker) │  │(tungstenite) │       │
│  └─────────────┘  └──────────────┘  └──────────────┘       │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                   Record/Replay Engine                       │
│  ┌──────────────┐           ┌──────────────┐               │
│  │   Recorder   │           │    Player    │               │
│  │              │           │              │               │
│  │ - Capture    │           │ - Load       │               │
│  │ - Serialize  │           │ - Match      │               │
│  │ - Save       │           │ - Replay     │               │
│  └──────────────┘           └──────────────┘               │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                     Cassette Storage                         │
│  JSON/MessagePack files on disk                             │
│  ./cassettes/my-test.json                                   │
└─────────────────────────────────────────────────────────────┘
```

---

## 🧩 Composants Détaillés

### 1. MatgtoProxy (API Publique)

**Responsabilité :** Interface principale exposée aux développeurs

```rust
pub struct MatgtoProxy {
    cassette_dir: PathBuf,
    proxy_port: u16,
    mode: ProxyMode,
    runtime: Runtime,
}

#[derive(Debug, Clone, Copy)]
pub enum ProxyMode {
    Auto,      // Record si cassette n'existe pas, sinon Replay
    Record,    // Toujours enregistrer (écrase cassette existante)
    Replay,    // Toujours rejouer (erreur si cassette manquante)
    Passthrough, // Proxy transparent sans record/replay
}

impl MatgtoProxy {
    /// Créer nouvelle instance avec répertoire cassettes
    pub fn new(cassette_dir: impl Into<PathBuf>) -> Result<Self>;

    /// Configurer port du proxy (défaut: 8888)
    pub fn with_port(self, port: u16) -> Self;

    /// Définir mode explicite
    pub fn with_mode(self, mode: ProxyMode) -> Self;

    /// Démarrer enregistrement d'une nouvelle cassette
    pub fn start_recording(&mut self, name: &str) -> Result<()>;

    /// Arrêter enregistrement et sauvegarder cassette
    pub fn stop_recording(&mut self) -> Result<()>;

    /// Rejouer cassette existante
    pub fn replay(&mut self, name: &str) -> Result<()>;

    /// Arrêter proxy complètement
    pub fn shutdown(&mut self) -> Result<()>;
}
```

**Design Patterns :**
- **Builder Pattern** : `MatgtoProxy::new().with_port().with_mode()`
- **RAII** : Shutdown automatique au drop
- **Result<T, E>** : Gestion erreurs explicite sans exceptions

---

### 2. HTTP/HTTPS Interceptor (Hudsucker)

**Responsabilité :** Interception transparente du trafic HTTP/HTTPS

```rust
use hudsucker::{
    certificate_authority::RcgenAuthority,
    hyper::{Request, Response, Body},
    HttpContext, HttpHandler, RequestOrResponse,
};

pub struct MatgtoHttpHandler {
    recorder: Arc<Mutex<Recorder>>,
    player: Arc<RwLock<Player>>,
    mode: ProxyMode,
}

#[async_trait]
impl HttpHandler for MatgtoHttpHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        match self.mode {
            ProxyMode::Record => {
                // Forward request, record interaction
                let (parts, body) = req.into_parts();
                let body_bytes = hyper::body::to_bytes(body).await.unwrap();

                // Reconstruct request
                let req_clone = Request::from_parts(parts.clone(), Body::from(body_bytes.clone()));

                // Store for later recording
                self.recorder.lock().await.store_request(parts, body_bytes);

                RequestOrResponse::Request(req_clone)
            }
            ProxyMode::Replay => {
                // Match request against cassette
                let interaction = self.player.read().await.match_request(&req)?;

                // Return mocked response immediately
                RequestOrResponse::Response(interaction.response.into())
            }
            ProxyMode::Auto => {
                // Check if cassette exists, decide mode
                if self.player.read().await.has_cassette() {
                    self.mode = ProxyMode::Replay;
                } else {
                    self.mode = ProxyMode::Record;
                }
                self.handle_request(_ctx, req).await
            }
            ProxyMode::Passthrough => {
                // Transparent proxy
                RequestOrResponse::Request(req)
            }
        }
    }

    async fn handle_response(
        &mut self,
        _ctx: &HttpContext,
        res: Response<Body>,
    ) -> Response<Body> {
        if matches!(self.mode, ProxyMode::Record) {
            // Record response
            let (parts, body) = res.into_parts();
            let body_bytes = hyper::body::to_bytes(body).await.unwrap();

            self.recorder.lock().await.store_response(parts.clone(), body_bytes.clone());

            Response::from_parts(parts, Body::from(body_bytes))
        } else {
            res
        }
    }
}
```

**Caractéristiques Clés :**
- **MITM TLS** : Certificat auto-signé généré avec `rcgen`
- **Zero-Copy** : Minimise clonage de body HTTP
- **Async** : Tokio async/await pour performance
- **Thread-Safe** : Arc<Mutex> pour partage état entre threads

---

### 3. WebSocket Interceptor

**Responsabilité :** Interception bidirectionnelle des messages WebSocket

```rust
use tokio_tungstenite::{
    tungstenite::protocol::Message,
    WebSocketStream,
};

pub struct WebSocketInterceptor {
    url: String,
    recorder: Arc<Mutex<Recorder>>,
    player: Arc<RwLock<Player>>,
    mode: ProxyMode,
}

impl WebSocketInterceptor {
    /// Intercepte connexion WebSocket client
    pub async fn intercept_client(
        &mut self,
        mut client_stream: WebSocketStream<TcpStream>,
    ) -> Result<()> {
        match self.mode {
            ProxyMode::Record => {
                // Établir connexion réelle au serveur
                let (mut server_stream, _) = tokio_tungstenite::connect_async(&self.url).await?;

                // Proxy bidirectionnel avec enregistrement
                loop {
                    tokio::select! {
                        // Client → Serveur
                        Some(msg) = client_stream.next() => {
                            let msg = msg?;
                            self.recorder.lock().await.record_ws_message(
                                Direction::Sent,
                                &msg,
                            );
                            server_stream.send(msg).await?;
                        }

                        // Serveur → Client
                        Some(msg) = server_stream.next() => {
                            let msg = msg?;
                            self.recorder.lock().await.record_ws_message(
                                Direction::Received,
                                &msg,
                            );
                            client_stream.send(msg).await?;
                        }

                        else => break,
                    }
                }
            }

            ProxyMode::Replay => {
                // Simuler serveur depuis cassette
                let messages = self.player.read().await.get_ws_messages();

                for (idx, recorded_msg) in messages.iter().enumerate() {
                    match recorded_msg.direction {
                        Direction::Received => {
                            // Envoyer au client
                            client_stream.send(recorded_msg.to_message()).await?;
                        }
                        Direction::Sent => {
                            // Attendre message client et valider
                            if let Some(client_msg) = client_stream.next().await {
                                let client_msg = client_msg?;
                                if !self.matches_recorded(&client_msg, recorded_msg) {
                                    return Err(Error::SequenceMismatch {
                                        expected: recorded_msg.clone(),
                                        got: client_msg,
                                        index: idx,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            _ => {
                // Passthrough ou Auto
                unimplemented!();
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Sent,      // Client → Serveur
    Received,  // Serveur → Client
}
```

**Challenges WebSocket :**
- **Bidirectionalité** : Messages client/serveur entrelacés
- **Timing** : Respecter ordre chronologique en replay
- **Validation** : Vérifier séquence messages client correspond
- **Close Frames** : Gérer fermeture propre connexion

---

### 4. Recorder (Enregistrement)

**Responsabilité :** Capturer interactions et les sérialiser en cassette

```rust
pub struct Recorder {
    cassette_name: String,
    interactions: Vec<Interaction>,
    current_request: Option<RecordedRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Interaction {
    #[serde(flatten)]
    pub kind: InteractionKind,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum InteractionKind {
    Http {
        request: HttpRequest,
        response: HttpResponse,
    },
    WebSocket {
        url: String,
        messages: Vec<WebSocketMessage>,
        close_frame: Option<CloseFrame>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebSocketMessage {
    pub direction: Direction,
    pub timestamp_ms: u64,  // Timestamp relatif depuis connexion
    #[serde(flatten)]
    pub payload: MessagePayload,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "msg_type")]
pub enum MessagePayload {
    Text { data: String },
    Binary { data: Vec<u8> },
    Ping { data: Vec<u8> },
    Pong { data: Vec<u8> },
}

impl Recorder {
    pub fn new(cassette_name: String) -> Self {
        Self {
            cassette_name,
            interactions: Vec::new(),
            current_request: None,
        }
    }

    /// Enregistrer requête HTTP (partie 1/2)
    pub fn store_request(&mut self, parts: http::request::Parts, body: Bytes) {
        self.current_request = Some(RecordedRequest {
            method: parts.method.to_string(),
            url: parts.uri.to_string(),
            headers: parts.headers.into_iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                .collect(),
            body: if body.is_empty() { None } else { Some(body.to_vec()) },
        });
    }

    /// Enregistrer réponse HTTP (partie 2/2)
    pub fn store_response(&mut self, parts: http::response::Parts, body: Bytes) {
        if let Some(request) = self.current_request.take() {
            self.interactions.push(Interaction {
                kind: InteractionKind::Http {
                    request,
                    response: HttpResponse {
                        status: parts.status.as_u16(),
                        headers: parts.headers.into_iter()
                            .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                            .collect(),
                        body: if body.is_empty() { None } else { Some(body.to_vec()) },
                    },
                },
                recorded_at: Utc::now(),
            });
        }
    }

    /// Enregistrer message WebSocket
    pub fn record_ws_message(&mut self, direction: Direction, msg: &Message) {
        // Logique enregistrement WebSocket
    }

    /// Sauvegarder cassette sur disque
    pub fn save(&self, cassette_dir: &Path) -> Result<()> {
        let cassette = Cassette {
            version: "1.0".to_string(),
            name: self.cassette_name.clone(),
            recorded_at: Utc::now(),
            interactions: self.interactions.clone(),
        };

        let path = cassette_dir.join(format!("{}.json", self.cassette_name));
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &cassette)?;

        Ok(())
    }
}
```

**Optimisations :**
- **Lazy Serialization** : Sérialisation uniquement à `save()`
- **Streaming Large Bodies** : Option compression gzip
- **Header Filtering** : Ignorer headers dynamiques (Date, User-Agent)

---

### 5. Player (Replay)

**Responsabilité :** Charger cassette et matcher requêtes pour replay

```rust
pub struct Player {
    cassette: Option<Cassette>,
    interactions_index: HashMap<RequestSignature, usize>,
    replay_count: HashMap<usize, usize>,  // Compteur pour requêtes multiples
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct RequestSignature {
    pub method: String,
    pub url: String,
    pub body_hash: Option<u64>,  // Hash du body pour matching
}

impl Player {
    pub fn load(cassette_dir: &Path, name: &str) -> Result<Self> {
        let path = cassette_dir.join(format!("{}.json", name));
        let file = File::open(path)?;
        let cassette: Cassette = serde_json::from_reader(file)?;

        // Indexer interactions pour lookup rapide
        let mut interactions_index = HashMap::new();
        for (idx, interaction) in cassette.interactions.iter().enumerate() {
            if let InteractionKind::Http { request, .. } = &interaction.kind {
                let signature = RequestSignature {
                    method: request.method.clone(),
                    url: request.url.clone(),
                    body_hash: request.body.as_ref().map(|b| {
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        b.hash(&mut hasher);
                        hasher.finish()
                    }),
                };
                interactions_index.insert(signature, idx);
            }
        }

        Ok(Self {
            cassette: Some(cassette),
            interactions_index,
            replay_count: HashMap::new(),
        })
    }

    /// Matcher requête contre cassette
    pub fn match_request(&mut self, req: &Request<Body>) -> Result<&Interaction> {
        let signature = RequestSignature {
            method: req.method().to_string(),
            url: req.uri().to_string(),
            body_hash: None,  // Simplification, body vide ici
        };

        let idx = self.interactions_index.get(&signature)
            .ok_or(Error::NoMatchingInteraction { signature })?;

        // Incrémenter compteur replay
        *self.replay_count.entry(*idx).or_insert(0) += 1;

        self.cassette.as_ref().unwrap().interactions.get(*idx)
            .ok_or(Error::InternalIndexError)
    }

    /// Récupérer tous messages WebSocket d'une cassette
    pub fn get_ws_messages(&self) -> Vec<&WebSocketMessage> {
        self.cassette.as_ref()
            .map(|c| c.interactions.iter()
                .filter_map(|i| match &i.kind {
                    InteractionKind::WebSocket { messages, .. } => Some(messages.as_slice()),
                    _ => None,
                })
                .flatten()
                .collect())
            .unwrap_or_default()
    }
}
```

**Stratégies de Matching :**
1. **Exact Match** : Méthode + URL + Body hash (défaut)
2. **Partial Match** : Ignorer query params spécifiques
3. **Regex Match** : URL pattern matching
4. **Custom Matcher** : Callback utilisateur

---

### 6. Cassette Format

**Responsabilité :** Format de stockage portable et lisible

```json
{
  "version": "1.0",
  "name": "api-users-test",
  "recorded_at": "2025-10-10T14:30:00Z",
  "interactions": [
    {
      "type": "Http",
      "recorded_at": "2025-10-10T14:30:01Z",
      "request": {
        "method": "GET",
        "url": "https://api.example.com/users?page=1",
        "headers": {
          "Accept": "application/json",
          "Authorization": "[FILTERED]"
        },
        "body": null
      },
      "response": {
        "status": 200,
        "headers": {
          "Content-Type": "application/json"
        },
        "body": [123, 34, 117, 115, 101, 114, 115, 34, 58, ...]
      }
    },
    {
      "type": "WebSocket",
      "recorded_at": "2025-10-10T14:30:05Z",
      "url": "wss://api.example.com/updates",
      "messages": [
        {
          "direction": "Sent",
          "timestamp_ms": 0,
          "msg_type": "Text",
          "data": "{\"action\":\"subscribe\",\"channel\":\"notifications\"}"
        },
        {
          "direction": "Received",
          "timestamp_ms": 120,
          "msg_type": "Text",
          "data": "{\"event\":\"connected\",\"connection_id\":\"abc123\"}"
        },
        {
          "direction": "Received",
          "timestamp_ms": 5340,
          "msg_type": "Binary",
          "data": [0x89, 0x50, 0x4E, 0x47, ...]
        }
      ],
      "close_frame": {
        "code": 1000,
        "reason": "Normal closure"
      }
    }
  ]
}
```

**Choix de Design :**
- **JSON** pour lisibilité et debugging
- **Body binaire** : Array d'octets (alternative: Base64 string)
- **Timestamps relatifs** pour WebSocket (reproductibilité)
- **Version field** pour migration future

**Alternative MessagePack :**
```toml
# Cargo.toml
[features]
msgpack = ["rmp-serde"]

# Pour cassettes volumineuses (>10 MB)
# Compression ~3-5x, désérialisation ~2x plus rapide
```

---

## 🔧 UniFFI Bindings Architecture

**Responsabilité :** Générer code multi-langage depuis Rust

### Interface Definition (UDL)

```
// matgto_serge.udl
namespace matgto_serge {
    MatgtoProxy new_proxy(string cassette_dir);
};

enum ProxyMode {
    "Auto",
    "Record",
    "Replay",
    "Passthrough",
};

interface MatgtoProxy {
    constructor(string cassette_dir);

    [Self=ByArc]
    MatgtoProxy with_port(u16 port);

    [Self=ByArc]
    MatgtoProxy with_mode(ProxyMode mode);

    [Throws=MatgtoError]
    void start_recording(string cassette_name);

    [Throws=MatgtoError]
    void stop_recording();

    [Throws=MatgtoError]
    void replay(string cassette_name);

    void shutdown();
};

[Error]
enum MatgtoError {
    "CassetteNotFound",
    "NoMatchingInteraction",
    "RecordingFailed",
    "ProxyStartFailed",
};
```

### Code Généré Automatiquement

**Java :**
```java
// uniffi/matgto_serge/MatgtoProxy.java (généré)
package com.matgto.serge;

public class MatgtoProxy implements AutoCloseable {
    private long handle;

    public MatgtoProxy(String cassetteDir) {
        this.handle = _UniFFILib.INSTANCE.matgto_serge_MatgtoProxy_new(cassetteDir);
    }

    public MatgtoProxy withPort(short port) {
        long newHandle = _UniFFILib.INSTANCE.matgto_serge_MatgtoProxy_with_port(this.handle, port);
        return new MatgtoProxy(newHandle);
    }

    public void startRecording(String cassetteName) throws MatgtoException {
        _UniFFILib.INSTANCE.matgto_serge_MatgtoProxy_start_recording(this.handle, cassetteName);
    }

    @Override
    public void close() {
        _UniFFILib.INSTANCE.matgto_serge_MatgtoProxy_destroy(this.handle);
    }
}
```

**JavaScript (N-API) :**
```javascript
// uniffi/matgto_serge.js (généré)
const nativeBinding = require('./native/matgto_serge.node');

class MatgtoProxy {
    constructor(cassetteDir) {
        this._handle = nativeBinding.MatgtoProxy_new(cassetteDir);
    }

    withPort(port) {
        const newHandle = nativeBinding.MatgtoProxy_with_port(this._handle, port);
        return MatgtoProxy._fromHandle(newHandle);
    }

    startRecording(cassetteName) {
        nativeBinding.MatgtoProxy_start_recording(this._handle, cassetteName);
    }

    [Symbol.dispose]() {
        nativeBinding.MatgtoProxy_destroy(this._handle);
    }
}

module.exports = { MatgtoProxy };
```

**Python :**
```python
# uniffi/matgto_serge.py (généré)
from ._matgto_serge import ffi, lib

class MatgtoProxy:
    def __init__(self, cassette_dir: str):
        self._handle = lib.matgto_serge_MatgtoProxy_new(
            cassette_dir.encode('utf-8')
        )

    def with_port(self, port: int) -> 'MatgtoProxy':
        new_handle = lib.matgto_serge_MatgtoProxy_with_port(self._handle, port)
        return MatgtoProxy._from_handle(new_handle)

    def start_recording(self, cassette_name: str) -> None:
        lib.matgto_serge_MatgtoProxy_start_recording(
            self._handle,
            cassette_name.encode('utf-8')
        )

    def __del__(self):
        lib.matgto_serge_MatgtoProxy_destroy(self._handle)
```

---

## 🚀 Performance Considerations

### Benchmarks Cibles

| Métrique | Cible | Stratégie |
|----------|-------|-----------|
| **HTTP Throughput** | 5000+ req/s | Zero-copy, async I/O |
| **WebSocket Throughput** | 10k+ msg/s | Tokio channels, batch processing |
| **Latence Proxy** | < 1ms p50 | Minimal allocations, pooling |
| **Mémoire** | < 50 MB | Streaming bodies, compact cassettes |
| **Startup Time** | < 100ms | Lazy loading cassettes |

### Optimisations Clés

1. **Zero-Copy HTTP Bodies**
   ```rust
   // ❌ Mauvais : Clone complet du body
   let body_vec = hyper::body::to_bytes(body).await?.to_vec();

   // ✅ Bon : Bytes (Arc sous le capot)
   let body_bytes = hyper::body::to_bytes(body).await?;
   ```

2. **Connection Pooling**
   ```rust
   use hyper::client::HttpConnector;
   use hyper_rustls::HttpsConnector;

   let https = HttpsConnector::with_native_roots();
   let client = Client::builder()
       .pool_max_idle_per_host(100)
       .build::<_, hyper::Body>(https);
   ```

3. **Cassette Indexing**
   ```rust
   // Index HashMap pour O(1) lookup au lieu de O(n) scan
   let mut index: HashMap<RequestSignature, usize> = HashMap::new();
   for (idx, interaction) in cassette.interactions.iter().enumerate() {
       index.insert(interaction.signature(), idx);
   }
   ```

4. **Async Batching (WebSocket)**
   ```rust
   // Buffer messages WebSocket pour batch writes
   let mut buffer = Vec::with_capacity(100);
   let mut flush_interval = tokio::time::interval(Duration::from_millis(10));

   loop {
       tokio::select! {
           msg = rx.recv() => buffer.push(msg),
           _ = flush_interval.tick() => {
               if !buffer.is_empty() {
                   recorder.batch_record(&buffer).await;
                   buffer.clear();
               }
           }
       }
   }
   ```

---

## 🔒 Sécurité

### Certificat MITM

**Génération automatique :**
```rust
use rcgen::{CertificateParams, DistinguishedName};

pub fn generate_ca_certificate() -> Result<(Certificate, PrivateKey)> {
    let mut params = CertificateParams::new(vec!["matgto-serge-ca".to_string()]);
    params.distinguished_name = DistinguishedName::new();
    params.distinguished_name.push(CN, "matgto-serge Certificate Authority");
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);

    let cert = Certificate::from_params(params)?;
    let pem = cert.serialize_pem()?;
    let key = cert.serialize_private_key_pem();

    Ok((pem, key))
}
```

**Installation Trust Store :**
```rust
#[cfg(target_os = "macos")]
fn install_certificate_macos(cert_path: &Path) -> Result<()> {
    Command::new("security")
        .args(&["add-trusted-cert", "-d", "-r", "trustRoot", "-k", "/Library/Keychains/System.keychain"])
        .arg(cert_path)
        .status()?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn install_certificate_linux(cert_path: &Path) -> Result<()> {
    fs::copy(cert_path, "/usr/local/share/ca-certificates/matgto-serge.crt")?;
    Command::new("update-ca-certificates").status()?;
    Ok(())
}
```

### Filtrage Données Sensibles

```rust
const SENSITIVE_HEADERS: &[&str] = &[
    "authorization",
    "cookie",
    "set-cookie",
    "proxy-authorization",
    "x-api-key",
];

fn filter_headers(headers: &mut HashMap<String, String>) {
    for sensitive in SENSITIVE_HEADERS {
        if let Some(value) = headers.get_mut(*sensitive) {
            *value = "[FILTERED]".to_string();
        }
    }
}
```

---

## 📊 Monitoring & Debugging

### Logs Structurés (tracing)

```rust
use tracing::{info, debug, warn, instrument};

#[instrument(skip(self))]
async fn handle_request(&mut self, req: Request<Body>) -> Result<Response<Body>> {
    debug!(
        method = %req.method(),
        uri = %req.uri(),
        "Intercepting HTTP request"
    );

    let start = Instant::now();
    let res = self.forward_request(req).await?;
    let elapsed = start.elapsed();

    info!(
        status = res.status().as_u16(),
        duration_ms = elapsed.as_millis(),
        "Request completed"
    );

    Ok(res)
}
```

### Métriques Prometheus (optionnel)

```rust
use prometheus::{IntCounter, Histogram, register_int_counter, register_histogram};

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: IntCounter = register_int_counter!(
        "matgto_http_requests_total",
        "Total HTTP requests intercepted"
    ).unwrap();

    static ref REQUEST_DURATION: Histogram = register_histogram!(
        "matgto_request_duration_seconds",
        "HTTP request duration in seconds"
    ).unwrap();
}

// Dans handle_request
HTTP_REQUESTS_TOTAL.inc();
let _timer = REQUEST_DURATION.start_timer();
```

---

## 🧪 Testing Strategy

### Tests Unitaires
- Recorder serialization/deserialization
- Player request matching logic
- Cassette format validation

### Tests Intégration
- HTTP proxy E2E (record + replay)
- WebSocket proxy E2E
- Multi-interaction scenarios

### Tests Bindings
- Java JUnit tests
- JavaScript Jest tests
- Python pytest tests

### Property-Based Testing (proptest)
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_cassette_roundtrip(
        interactions in prop::collection::vec(arb_interaction(), 1..100)
    ) {
        let cassette = Cassette { interactions, ..Default::default() };
        let json = serde_json::to_string(&cassette)?;
        let deserialized: Cassette = serde_json::from_str(&json)?;
        prop_assert_eq!(cassette, deserialized);
    }
}
```

---

## 🔄 Lifecycle d'une Requête

### Mode Record

```
1. Client → matgto-serge proxy (port 8888)
2. Proxy intercepte requête HTTP
3. Recorder.store_request(req)
4. Proxy forward → serveur réel
5. Serveur répond
6. Proxy intercepte réponse
7. Recorder.store_response(res)
8. Proxy retourne réponse → client
9. À la fin : Recorder.save() → cassette.json
```

### Mode Replay

```
1. Client → matgto-serge proxy (port 8888)
2. Proxy intercepte requête HTTP
3. Player.match_request(req) → lookup cassette
4. Si match trouvé :
   - Retourner réponse immédiatement (pas de réseau)
5. Si pas de match (mode strict) :
   - Erreur NoMatchingInteraction
6. Si pas de match (mode permissif) :
   - Fallback mode Record + warning
```

---

## 📝 Conclusions Architecturales

### Forces
- ✅ Performance Rust (10-100x Ruby)
- ✅ Type safety & memory safety
- ✅ UniFFI = maintenance minimale bindings
- ✅ Async I/O mature (Tokio)
- ✅ Écosystème proxy robuste (Hudsucker)

### Faiblesses
- ⚠️ Complexité installation certificat MITM
- ⚠️ UniFFI encore jeune (< 1.0)
- ⚠️ Debugging bindings peut être difficile
- ⚠️ Binary size (~5-10 MB)

### Alternatives Considérées
- **VCR (Ruby)** : Trop lent, Ruby uniquement
- **Polly (JavaScript)** : Pas de WebSocket, JS uniquement
- **WireMock (Java)** : Pas record automatique, config manuelle
- **Go implementation** : Moins mûr pour bindings multi-langage

---

**Dernière mise à jour :** 2025-10-10
**Auteur :** Architecture Team
**Version :** 1.0-draft
