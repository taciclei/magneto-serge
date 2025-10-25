# Session Authentication Best Practices

**Version:** 1.0  
**Date:** 2025-10-24  
**Phase:** 1.1 - Cookie Preservation

---

## 🎯 Problème

Les applications web modernes utilisent principalement deux approches d'authentification:

1. **Session-based (cookies)** - Spring Security, Express Session, Laravel, Django
2. **Token-based (JWT)** - Stateless, pas de cookies serveur

**Magnéto-Serge v0.1** ne préservait pas les cookies entre requêtes, causant l'échec des tests d'authentification session-based.

**Magnéto-Serge v0.2** implémente RFC 6265 complet avec CookieJar pour résoudre ce problème.

---

## 🔍 Anatomie d'un Login Flow

### Flow Session-Based (cookies)

```
1. Client → POST /login (username + password)
2. Server ← Authenticate
3. Server → Create session (store in memory/Redis/DB)
4. Server ← Set-Cookie: JSESSIONID=abc123; Path=/; HttpOnly
5. Client → Store cookie
6. Client → GET /api/account
7. Client → Cookie: JSESSIONID=abc123
8. Server ← Verify session
9. Server → 200 OK {user data}
```

### Flow Token-Based (JWT)

```
1. Client → POST /login (username + password)
2. Server ← Authenticate
3. Server → Generate JWT token
4. Server ← 200 OK {token: "eyJhbG..."}
5. Client → Store token (localStorage/memory)
6. Client → GET /api/account
7. Client → Authorization: Bearer eyJhbG...
8. Server ← Verify JWT signature
9. Server → 200 OK {user data}
```

---

## ❌ Problème avec v0.1 (sans cookies)

### Cassette v1.0 enregistrée:

```json
{
  "version": "1.0",
  "interactions": [
    {
      "request": {"method": "POST", "url": "/login", "body": {"username": "admin"}},
      "response": {"status": 200, "headers": {"set-cookie": "JSESSIONID=abc123"}}
    },
    {
      "request": {"method": "GET", "url": "/api/account"},
      "response": {"status": 401, "body": "Unauthorized"}
    }
  ]
}
```

**Problème:** Le cookie `JSESSIONID` est retourné par `/login` mais **n'est PAS injecté** dans `/api/account` lors du replay → 401 Unauthorized

---

## ✅ Solution avec v0.2 (avec cookies)

### Cassette v2.0 avec cookies:

```json
{
  "version": "2.0",
  "interactions": [
    {
      "request": {"method": "POST", "url": "/login"},
      "response": {"status": 200, "headers": {"set-cookie": "JSESSIONID=abc123; Path=/; HttpOnly"}}
    },
    {
      "request": {"method": "GET", "url": "/api/account", "headers": {"cookie": "JSESSIONID=abc123"}},
      "response": {"status": 200, "body": {"username": "admin"}}
    }
  ],
  "cookies": [
    {
      "name": "JSESSIONID",
      "value": "abc123",
      "domain": null,
      "path": "/",
      "secure": false,
      "http_only": true,
      "same_site": null,
      "created_at": "2025-10-24T10:00:00Z"
    }
  ]
}
```

**Solution:** 
1. Lors du replay de `/login`, CookieJar **extrait** `Set-Cookie: JSESSIONID=abc123`
2. Lors du replay de `/api/account`, CookieJar **injecte** `Cookie: JSESSIONID=abc123`
3. Response: 200 OK ✅

---

## 🔧 Frameworks Session-Based

### Spring Boot + Spring Security (Java)

**Configuration typique:**

```java
@Configuration
public class SecurityConfig {
    @Bean
    public SecurityFilterChain filterChain(HttpSecurity http) {
        http
            .sessionManagement()
                .sessionCreationPolicy(SessionCreationPolicy.IF_REQUIRED)
            .and()
            .formLogin().permitAll();
        return http.build();
    }
}
```

**Cookie généré:** `JSESSIONID`

**Exemple test JUnit 5:**

```java
@SpringBootTest
@EnableMagneto(cassetteDir = "src/test/resources/cassettes")
class AuthenticationTest {

    @Autowired
    private TestRestTemplate restTemplate;

    @Test
    @Cassette("login-flow")
    void shouldAuthenticateAndAccessProtectedResource() {
        // Login
        LoginRequest login = new LoginRequest("admin", "admin");
        ResponseEntity<Void> loginResponse = restTemplate.postForEntity("/api/authenticate", login, Void.class);
        
        assertEquals(200, loginResponse.getStatusCode().value());
        
        // Cookie automatiquement préservé par Magneto v2.0
        
        // Access protected resource
        ResponseEntity<Account> account = restTemplate.getForEntity("/api/account", Account.class);
        assertEquals(200, account.getStatusCode().value());
        assertEquals("admin", account.getBody().getLogin());
    }
}
```

---

### Express.js + express-session (JavaScript)

**Configuration typique:**

```javascript
const session = require('express-session');

app.use(session({
  secret: 'keyboard cat',
  resave: false,
  saveUninitialized: false,
  cookie: { 
    secure: false, // true if HTTPS
    httpOnly: true,
    maxAge: 24 * 60 * 60 * 1000 // 24h
  }
}));

app.post('/login', (req, res) => {
  req.session.userId = user.id;
  res.json({ success: true });
});

app.get('/api/me', (req, res) => {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Unauthorized' });
  }
  res.json({ userId: req.session.userId });
});
```

**Cookie généré:** `connect.sid`

**Exemple test Jest:**

```javascript
const { useMagneto } = require('@taciclei/magneto-jest');

describe('Authentication', () => {
  const magneto = useMagneto({ cassetteDir: './cassettes' });

  test('should login and access protected route', async () => {
    magneto.cassette('login-flow');

    // Login
    const loginRes = await fetch('http://localhost:3000/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username: 'admin', password: 'admin' }),
      agent: magneto.httpAgent
    });

    expect(loginRes.status).toBe(200);

    // Cookie automatiquement préservé par Magneto v2.0

    // Access protected
    const meRes = await fetch('http://localhost:3000/api/me', {
      agent: magneto.httpAgent
    });

    expect(meRes.status).toBe(200);
    const user = await meRes.json();
    expect(user.userId).toBe(1);
  });
});
```

---

### Django + session middleware (Python)

**Configuration typique:**

```python
# settings.py
MIDDLEWARE = [
    'django.contrib.sessions.middleware.SessionMiddleware',
    'django.contrib.auth.middleware.AuthenticationMiddleware',
]

SESSION_ENGINE = 'django.contrib.sessions.backends.db'
SESSION_COOKIE_NAME = 'sessionid'
SESSION_COOKIE_HTTPONLY = True
```

**Cookie généré:** `sessionid`

**Exemple test pytest:**

```python
import pytest
from magneto_pytest import magneto

@pytest.mark.magneto(cassette='login-flow')
def test_authentication(magneto):
    import requests

    # Login
    login_res = requests.post(
        'http://localhost:8000/api/login',
        json={'username': 'admin', 'password': 'admin'}
    )
    
    assert login_res.status_code == 200
    
    # Cookie automatiquement préservé par Magneto v2.0
    
    # Access protected
    me_res = requests.get('http://localhost:8000/api/me')
    
    assert me_res.status_code == 200
    assert me_res.json()['username'] == 'admin'
```

---

### Laravel + session (PHP)

**Configuration typique:**

```php
// config/session.php
return [
    'driver' => 'file',
    'lifetime' => 120,
    'expire_on_close' => false,
    'cookie' => 'laravel_session',
    'http_only' => true,
];
```

**Cookie généré:** `laravel_session`

**Exemple test PHPUnit:**

```php
use TaciClei\Magneto\PHPUnit\UsesMagneto;

class AuthenticationTest extends TestCase
{
    use UsesMagneto;

    protected $cassetteDir = 'tests/cassettes';

    public function testLoginFlow()
    {
        $this->useCassette('login-flow');

        // Login
        $response = $this->post('/api/login', [
            'email' => 'admin@example.com',
            'password' => 'password',
        ]);

        $response->assertStatus(200);

        // Cookie automatiquement préservé par Magneto v2.0

        // Access protected
        $response = $this->get('/api/user');

        $response->assertStatus(200);
        $response->assertJson(['email' => 'admin@example.com']);
    }
}
```

---

## 🔐 Cookies Sécurisés

### Attributs Importants

| Attribut | Description | Exemple |
|----------|-------------|---------|
| **HttpOnly** | Pas accessible via JavaScript (XSS protection) | `HttpOnly` |
| **Secure** | HTTPS uniquement | `Secure` |
| **SameSite** | Protection CSRF | `SameSite=Strict` |
| **Domain** | Scope domaine | `Domain=.example.com` |
| **Path** | Scope chemin | `Path=/api` |
| **Max-Age** | Durée de vie (secondes) | `Max-Age=3600` |
| **Expires** | Date expiration absolue | `Expires=Wed, 21 Oct 2025 07:28:00 GMT` |

### Exemple Cookie Sécurisé

```
Set-Cookie: session=abc123; 
            Path=/; 
            Domain=.example.com; 
            Secure; 
            HttpOnly; 
            SameSite=Strict; 
            Max-Age=86400
```

**Magnéto-Serge v2.0** respecte tous ces attributs lors du matching (domain, path, secure).

---

## 🐛 Troubleshooting

### Problème: Cookie pas préservé

**Symptômes:**
- Login réussit (200 OK)
- Requête suivante échoue (401 Unauthorized)

**Diagnostic:**

```bash
# Inspecter cassette
cat cassettes/my-test.json | jq '.cookies'

# Devrait afficher:
[
  {
    "name": "JSESSIONID",
    "value": "...",
    ...
  }
]

# Si vide: cassette v1.0 ou extraction échouée
```

**Solutions:**

1. **Vérifier version cassette:**
   ```bash
   cat cassettes/my-test.json | jq '.version'
   # Devrait être "2.0"
   ```

2. **Re-record en v2.0:**
   ```bash
   rm cassettes/my-test.json
   USE_MOCK=true MOCK_MODE=record npm test
   ```

3. **Migrer v1.0 → v2.0:**
   ```bash
   magneto cassette migrate my-test --to-version 2.0
   ```

---

### Problème: Cookie expiré lors du replay

**Symptômes:**
- Test passe juste après record
- Test échoue 24h après (401 Unauthorized)

**Cause:** Cookie a `Max-Age=86400` (24h) et est expiré lors du replay

**Solutions:**

1. **Désactiver vérification expiration (dev only):**
   ```rust
   // Dans Player
   cookie.max_age = None; // Ignorer expiration
   ```

2. **Re-record régulièrement:**
   ```bash
   # Re-record si cassette >7 jours
   find cassettes/ -mtime +7 -name "*.json" -exec rm {} \;
   ```

3. **Utiliser cookies sans expiration en test:**
   ```javascript
   // Backend: session config pour tests
   if (process.env.NODE_ENV === 'test') {
     session.cookie.maxAge = null; // Pas d'expiration
   }
   ```

---

### Problème: XSRF-TOKEN requis mais absent

**Symptômes:**
- Login OK
- POST /api/create échoue: 403 Forbidden (XSRF validation)

**Cause:** XSRF token pas extrait et réinjecté

**Solution:** XSRF token généralement dans cookie ET header

```javascript
// Spring Security génère:
Set-Cookie: XSRF-TOKEN=xyz789

// Client doit renvoyer:
Cookie: XSRF-TOKEN=xyz789
X-XSRF-TOKEN: xyz789  // Header aussi !
```

**Fix:** Extraire XSRF du cookie et l'injecter dans header:

```rust
// Dans Player, après injection Cookie header
if let Some(xsrf_cookie) = self.cookie_jar.get_cookie("XSRF-TOKEN") {
    request.headers.insert("x-xsrf-token".to_string(), xsrf_cookie.value.clone());
}
```

---

## 📊 Comparaison Session vs JWT

| Aspect | Session (Cookies) | JWT (Token) |
|--------|-------------------|-------------|
| **Storage** | Server (RAM/DB/Redis) | Client (localStorage) |
| **State** | Stateful | Stateless |
| **Revocation** | Facile (delete session) | Difficile (blacklist) |
| **Performance** | DB lookup | Signature verification |
| **Horizontal scaling** | Difficile (sticky sessions) | Facile (stateless) |
| **Security** | HttpOnly protège XSS | Vulnérable si localStorage |
| **Magneto v0.1** | ❌ Ne marche pas | ✅ Marche |
| **Magneto v0.2** | ✅ Marche | ✅ Marche |

---

## 🎓 Best Practices

### 1. Utiliser v2.0 pour tous nouveaux tests

```bash
# Vérifier version avant commit
find cassettes/ -name "*.json" -exec sh -c \
  'jq -r .version "$1" | grep -q "2.0" || echo "⚠️  $1 is v1.0"' _ {} \;
```

### 2. Re-record régulièrement

```bash
# CI/CD: warning si cassette >30 jours
- name: Check cassette age
  run: |
    find cassettes/ -mtime +30 -name "*.json" | while read f; do
      echo "::warning file=$f::Cassette older than 30 days"
    done
```

### 3. Gitignore cookies sensibles

Si vos cookies contiennent tokens secrets, filtrer avant commit:

```bash
# .gitattributes
cassettes/*.json filter=remove-cookies

# .git/config
[filter "remove-cookies"]
  clean = jq 'del(.cookies[] | select(.name == \"SECRET_TOKEN\"))'
```

### 4. Documenter dépendances session

```markdown
<!-- README.md -->
## Testing

Tests E2E utilisent Magnéto-Serge v2.0 avec cookies.

**Important:** Ces tests dépendent de:
- Session cookies (JSESSIONID)
- XSRF protection activée
- Re-record si backend change auth logic
```

---

## 📚 Ressources

- **RFC 6265:** HTTP State Management Mechanism
- **OWASP:** Session Management Cheat Sheet
- **Magneto Docs:** `/docs/CASSETTE_OPTIMIZATION.md`
- **Roadmap:** `/docs/ROADMAP_IMPROVEMENTS.md`

---

**Version:** 1.0  
**Dernière mise à jour:** 2025-10-24

