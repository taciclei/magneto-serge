# Mode STRICT (Replay Strict)

Le mode STRICT est un mode de replay spécialement conçu pour les environnements CI/CD où vous voulez garantir que tous les appels réseau sont capturés dans vos cassettes et qu'aucune requête ne passe en mode live.

## 🎯 Objectif

En mode STRICT, Magneto refuse de continuer si :
- La cassette demandée n'existe pas
- Une interaction (requête HTTP) n'est pas trouvée dans la cassette

Ce comportement "fail-fast" est idéal pour :
- **Tests CI/CD** : Garantir la reproductibilité complète
- **Tests de régression** : Détecter les nouveaux appels réseau non capturés
- **Tests isolés** : Assurer zéro dépendance externe

## 🚀 Utilisation

### Rust

```rust
use magneto_serge::proxy::{MagnetoProxy, ProxyMode};

// Option 1 : Via ProxyMode
let proxy = MagnetoProxy::new("./cassettes")?
    .with_mode(ProxyMode::ReplayStrict)
    .start()?;

// Option 2 : Via méthode dédiée
let proxy = MagnetoProxy::new("./cassettes")?;
proxy.replay_strict("ma-cassette")?;
```

### Java

```java
import magneto_serge.*;

// Option 1 : Via ProxyMode
MagnetoProxy proxy = new MagnetoProxy("./cassettes");
proxy.setMode(ProxyMode.REPLAY_STRICT);

// Option 2 : Via méthode dédiée
MagnetoProxy proxy = new MagnetoProxy("./cassettes");
boolean success = proxy.replayStrict("ma-cassette");
if (!success) {
    throw new RuntimeException("Failed to start strict replay");
}
```

### JavaScript/TypeScript

```javascript
import { MagnetoProxy, ProxyMode } from 'magneto-serge';

// Option 1 : Via ProxyMode
const proxy = new MagnetoProxy('./cassettes');
proxy.setMode(ProxyMode.ReplayStrict);

// Option 2 : Via méthode dédiée
const proxy = new MagnetoProxy('./cassettes');
const success = proxy.replayStrict('ma-cassette');
if (!success) {
    throw new Error('Failed to start strict replay');
}
```

### Python

```python
from magneto_serge import MagnetoProxy, ProxyMode

# Option 1 : Via ProxyMode
proxy = MagnetoProxy("./cassettes")
proxy.set_mode(ProxyMode.REPLAY_STRICT)

# Option 2 : Via méthode dédiée
proxy = MagnetoProxy("./cassettes")
success = proxy.replay_strict("ma-cassette")
if not success:
    raise RuntimeError("Failed to start strict replay")
```

## 🔍 Comparaison des Modes

| Caractéristique | Mode Replay (Normal) | Mode STRICT |
|----------------|----------------------|-------------|
| Cassette manquante | ⚠️ Warning (log) | ❌ Erreur immédiate |
| Interaction manquante | ⚠️ Warning (log) | ❌ Erreur immédiate |
| Logging | Standard | 🔒 Préfixé STRICT |
| Usage recommandé | Développement | CI/CD, Tests |
| Comportement | Gracieux | Fail-fast |

## 📊 Cas d'Usage

### 1. CI/CD Pipeline

```yaml
# .github/workflows/tests.yml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run tests in STRICT mode
        run: |
          export MATGTO_MODE=replay-strict
          cargo test
```

**Avantages** :
- Détecte immédiatement les nouveaux appels réseau
- Garantit la reproductibilité complète
- Zéro dépendance externe
- Tests rapides (pas de vraies requêtes HTTP)

### 2. Tests de Régression

```rust
#[test]
fn test_api_integration_strict() {
    // En mode STRICT, ce test échouera si une nouvelle requête
    // est ajoutée sans mettre à jour la cassette
    let proxy = MagnetoProxy::new("./cassettes")?
        .with_mode(ProxyMode::ReplayStrict);

    proxy.replay_strict("api-integration-v1")?;

    // Votre code qui fait des appels API
    let result = api_client.get_users();

    // Si une nouvelle requête est faite, le test échoue immédiatement
    assert!(result.is_ok());
}
```

### 3. Tests Isolés (Offline)

```rust
#[test]
fn test_offline_mode() {
    // Parfait pour tester sur un laptop sans connexion
    let proxy = MagnetoProxy::new("./cassettes")?;

    // Mode STRICT garantit aucun appel réseau réel
    proxy.replay_strict("offline-cassette")?;

    // Tous les appels sont rejoués depuis la cassette
    let result = external_api.fetch_data();
    assert_eq!(result.status, 200);
}
```

## 🔧 Configuration

### Via Code

```rust
use magneto_serge::proxy::{MagnetoProxy, ProxyMode};

let proxy = MagnetoProxy::new("./cassettes")?
    .with_port(8888)
    .with_mode(ProxyMode::ReplayStrict);
```

### Via Variable d'Environnement

```bash
export MATGTO_MODE=replay-strict
cargo test
```

### Via Fichier de Configuration

```toml
# matgto.toml
[matgto]
mode = "replay-strict"
cassette_dir = "./cassettes"
```

## 📝 Logs et Debugging

### Logs en Mode STRICT

Le mode STRICT ajoute des préfixes 🔒 pour faciliter l'identification :

```
INFO  🔒 Loaded cassette 'api-test' in STRICT mode with 10 interactions
DEBUG 🔒 STRICT MODE: Found interaction #5 for GET https://api.example.com/users
ERROR 🔒 STRICT MODE: No matching interaction found for POST https://api.example.com/posts
ERROR 💡 Available interactions in cassette: 10
```

### Activer les Logs de Debug

```bash
export RUST_LOG=magneto_serge=debug
cargo test
```

### Exemple de Sortie

```
   Compiling magneto-serge v0.1.0
    Finished test [optimized + debuginfo] target(s) in 2.34s
     Running tests/integration_test.rs

running 1 test
INFO  🔒 Loaded cassette 'api-test' in STRICT mode with 10 interactions
DEBUG 🔒 STRICT MODE: Found interaction #0 for GET https://api.example.com/users
DEBUG 🔒 STRICT MODE: Found interaction #1 for GET https://api.example.com/posts
test test_api_integration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## ❌ Gestion des Erreurs

### Cassette Manquante

```rust
let result = proxy.replay_strict("nonexistent");

// Error: CassetteNotFound { name: "nonexistent" }
assert!(result.is_err());
```

**Message d'erreur** :
```
Error: Cassette not found: nonexistent
```

### Interaction Manquante

```rust
proxy.replay_strict("ma-cassette")?;

// Nouvelle requête non capturée dans la cassette
let result = http_client.post("https://api.example.com/new-endpoint");

// Error: NoMatchingInteraction { method: "POST", url: "https://api.example.com/new-endpoint" }
```

**Message d'erreur** :
```
ERROR 🔒 STRICT MODE: No matching interaction found for POST https://api.example.com/new-endpoint
ERROR 💡 Available interactions in cassette: 10
Error: No matching interaction for POST https://api.example.com/new-endpoint
```

## 🔄 Workflow Recommandé

### 1. Développement Local (Mode Auto)

```rust
// Enregistre automatiquement les nouvelles interactions
let proxy = MagnetoProxy::new("./cassettes")?
    .with_mode(ProxyMode::Auto);

proxy.auto("dev-cassette")?;
```

### 2. Mise à Jour des Cassettes (Mode Record)

```bash
# Réenregistrer une cassette
export MATGTO_MODE=record
cargo test test_api_integration

# Vérifier les changements
git diff cassettes/dev-cassette.json
```

### 3. CI/CD (Mode STRICT)

```yaml
# GitHub Actions
- name: Run tests in STRICT mode
  env:
    MATGTO_MODE: replay-strict
  run: cargo test
```

## 🎓 Bonnes Pratiques

### ✅ DO

1. **Utiliser STRICT en CI/CD** : Garantit la reproductibilité
2. **Committer les cassettes** : Les cassettes font partie du code
3. **Nommer les cassettes clairement** : `api-users-list`, `auth-login`, etc.
4. **Activer les logs en cas d'échec** : `RUST_LOG=magneto_serge=debug`
5. **Mettre à jour les cassettes régulièrement** : Mode Record pour capturer les changements

### ❌ DON'T

1. **Ne pas utiliser STRICT en développement** : Trop restrictif, utilisez Auto
2. **Ne pas ignorer les erreurs** : Si STRICT échoue, c'est qu'il manque une interaction
3. **Ne pas modifier manuellement les cassettes** : Réenregistrez avec Record
4. **Ne pas committer des cassettes avec des secrets** : Utilisez les filtres (voir FILTERS.md)

## 🔗 Ressources Connexes

- **[FILTERS.md](./FILTERS.md)** : Filtrage des données sensibles dans les cassettes
- **[CLAUDE.md](./CLAUDE.md)** : Instructions pour Claude Code
- **[ROADMAP.md](./ROADMAP.md)** : Feuille de route du projet
- **[README.md](./README.md)** : Documentation principale

## 🐛 Dépannage

### Problème : Tests échouent en mode STRICT

**Symptôme** :
```
Error: No matching interaction for GET https://api.example.com/users
```

**Solutions** :
1. Vérifier que la cassette existe : `ls cassettes/`
2. Réenregistrer la cassette : `MATGTO_MODE=record cargo test`
3. Vérifier les logs : `RUST_LOG=magneto_serge=debug cargo test`

### Problème : Cassette trop ancienne

**Symptôme** : API a changé, cassette obsolète

**Solution** :
```bash
# Supprimer l'ancienne cassette
rm cassettes/old-cassette.json

# Réenregistrer
MATGTO_MODE=record cargo test test_name

# Vérifier les changements
git diff cassettes/
```

### Problème : Performances en CI

**Symptôme** : Tests lents en CI

**Solution** : Le mode STRICT devrait être rapide (pas de vraies requêtes HTTP). Si lent :
1. Vérifier que le mode est bien STRICT : logs doivent afficher 🔒
2. Vérifier que les cassettes sont bien chargées : `cassettes/` doit exister
3. Profiler : `cargo build --release && cargo test --release`

## 📈 Métriques et Performance

### Comparaison de Performance

| Métrique | Mode Live | Mode Replay | Mode STRICT |
|---------|-----------|-------------|-------------|
| Latence | 100-500ms | <1ms | <1ms |
| Throughput | 10-50 req/s | 5000+ req/s | 5000+ req/s |
| Reproductibilité | ❌ Variable | ✅ 100% | ✅ 100% |
| Isolation | ❌ Dépend réseau | ✅ Complet | ✅ Complet |
| Fail-fast | ❌ Non | ⚠️ Warnings | ✅ Erreurs |

### Benchmarks

```bash
# Exécuter les benchmarks
cargo bench

# Résultats attendus (mode STRICT) :
# - Chargement cassette : ~500µs
# - Matching interaction : ~100ns
# - Replay complet : ~1ms
```

## 🔐 Sécurité

Le mode STRICT est **plus sûr** car :

1. **Aucun appel réseau réel** : Impossible de fuiter des données
2. **Cassettes vérifiées** : Toutes les interactions sont pré-capturées
3. **Fail-fast** : Détecte immédiatement les comportements inattendus

### Combinaison avec Filtres

```rust
use magneto_serge::filters::FilterPresets;

// Cassette avec filtres de sécurité
let filters = FilterPresets::security();
let mut recorder = Recorder::new_with_filters("secure".to_string(), filters);

// Enregistrement
recorder.record_http(request, response);
recorder.save("./cassettes")?;

// Replay en mode STRICT (données filtrées)
proxy.replay_strict("secure")?;
```

Voir [FILTERS.md](./FILTERS.md) pour plus de détails sur le filtrage.

---

*Dernière mise à jour : 2025-10-11*
