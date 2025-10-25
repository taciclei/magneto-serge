---
name: Bug Report
about: Signaler un bug pour nous aider à améliorer Magneto-Serge
title: '[BUG] '
labels: bug
assignees: ''
---

## Description du Bug

<!-- Description claire et concise du bug -->

## Steps to Reproduce

<!-- Étapes pour reproduire le comportement -->

1. Initialiser le proxy avec '...'
2. Appeler la méthode '...'
3. Observer l'erreur

## Comportement Attendu

<!-- Description claire de ce que vous attendiez -->

## Comportement Actuel

<!-- Ce qui se passe réellement -->

## Environnement

**OS et Version**:
- [ ] Linux (distro et version: )
- [ ] macOS (version: )
- [ ] Windows (version: )

**Versions**:
- Magneto-Serge version: [ex: 0.1.0]
- Rust version (si applicable): [ex: 1.75.0]
- Langage binding utilisé: [Rust/Python/Kotlin/Swift/Java/JavaScript]
- Version binding (si applicable): [ex: Python 3.11]

## Logs et Erreurs

<!-- Incluez les logs pertinents, messages d'erreur, stack traces -->

```
Collez les logs ici
```

## Configuration

<!-- Configuration magneto.toml ou code de configuration -->

```toml
# magneto.toml
[magneto]
cassette_dir = "./cassettes"
proxy_port = 8888
```

## Code de Reproduction

<!-- Code minimal pour reproduire le bug -->

```rust
// Exemple Rust
use magneto_serge::{MagnetoProxy, ProxyMode};

let mut proxy = MagnetoProxy::new("./cassettes");
proxy.set_port(8888);
// ... code qui cause le bug
```

ou

```python
# Exemple Python
from magneto_serge import MagnetoProxy

proxy = MagnetoProxy("./cassettes")
proxy.set_port(8888)
# ... code qui cause le bug
```

## Screenshots

<!-- Si applicable, ajoutez des captures d'écran -->

## Contexte Additionnel

<!-- Toute autre information utile sur le problème -->

## Possible Solution

<!-- Si vous avez une idée de comment corriger le bug, partagez-la ici -->

## Checklist

- [ ] J'ai vérifié que ce bug n'est pas déjà signalé dans les issues existantes
- [ ] J'ai testé avec la dernière version de Magneto-Serge
- [ ] J'ai fourni un exemple de code minimal pour reproduire le bug
- [ ] J'ai inclus les logs/erreurs pertinents
- [ ] J'ai fourni les informations sur mon environnement
