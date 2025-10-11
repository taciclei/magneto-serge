# Compression des Cassettes

La compression des cassettes permet de réduire significativement la taille des fichiers de cassettes, ce qui est particulièrement utile pour les gros tests avec beaucoup de données.

## 🎯 Objectif

Magneto-Serge offre plusieurs formats de cassettes avec différents niveaux de compression :
- **JSON** : Format textuel, facile à lire, taille maximum
- **JSON + Gzip** : Format textuel compressé, taille réduite de 50-90%
- **MessagePack** : Format binaire, 50% plus petit que JSON
- **MessagePack + Gzip** : Format binaire compressé, taille minimale (réduction de 60-95%)

##

 📊 Comparaison des Formats

| Format | Taille relative | Vitesse | Lisible | Cas d'usage |
|--------|----------------|---------|---------|-------------|
| JSON | 100% | Rapide | ✅ Oui | Développement, debugging |
| JSON.GZ | 10-50% | Moyen | ❌ Non | CI/CD avec gros volumes |
| MessagePack | 50% | Très rapide | ❌ Non | Production, performance |
| MessagePack.GZ | 5-40% | Moyen | ❌ Non | Archivage, très gros volumes |

### Exemple de Réduction de Taille

Pour une cassette avec 100 interactions HTTP (600 KB de données):

```
JSON:                1,234 KB  (100%)
JSON.GZ:              156 KB  ( 13%)  ⬇️ 87% de réduction
MessagePack:          652 KB  ( 53%)  ⬇️ 47% de réduction
MessagePack.GZ:        89 KB  (  7%)  ⬇️ 93% de réduction
```

## 🚀 Utilisation

### Rust

```rust
use magneto_serge::cassette::{AsyncCassetteStorage, CassetteFormat};

let storage = AsyncCassetteStorage::new();
let cassette = Cassette::new("my-test".to_string());

// Option 1: JSON compressé
let path = std::path::Path::new("./cassettes/my-test.json.gz");
storage.save_sync(cassette.clone(), path.to_path_buf(), CassetteFormat::JsonGzip).await?;

// Option 2: MessagePack compressé (taille minimale)
let path = std::path::Path::new("./cassettes/my-test.msgpack.gz");
storage.save_sync(cassette, path.to_path_buf(), CassetteFormat::MessagePackGzip).await?;

// Chargement automatique (détecte le format)
let format = detect_format(&path);
let loaded = AsyncCassetteStorage::load_async(&path, format).await?;
```

### Auto-Détection du Format

Magneto détecte automatiquement le format basé sur l'extension du fichier :

```rust
use magneto_serge::cassette::detect_format;

let format = detect_format(Path::new("test.json"));           // CassetteFormat::Json
let format = detect_format(Path::new("test.json.gz"));        // CassetteFormat::JsonGzip
let format = detect_format(Path::new("test.msgpack"));        // CassetteFormat::MessagePack
let format = detect_format(Path::new("test.msgpack.gz"));     // CassetteFormat::MessagePackGzip
```

### Java

```java
import magneto_serge.*;

// Compression pas encore exposée via UniFFI
// Utilisez le CLI ou Rust API directement
```

### JavaScript/TypeScript

```javascript
import { AsyncCassetteStorage, CassetteFormat } from 'magneto-serge';

// Compression pas encore exposée via UniFFI
// Utilisez le CLI ou Rust API directement
```

### Python

```python
from magneto_serge import AsyncCassetteStorage, CassetteFormat

# Compression pas encore exposée via UniFFI
# Utilisez le CLI ou Rust API directement
```

### CLI

```bash
# Enregistrer avec compression JSON
MAGNETO_FORMAT=json.gz magneto record my-cassette

# Enregistrer avec compression MessagePack
MAGNETO_FORMAT=msgpack.gz magneto record my-cassette

# Replay (détection automatique du format)
magneto replay my-cassette

# Inspecter une cassette compressée
magneto inspect my-cassette
```

## 🔧 Configuration

### Via Code (Rust)

```rust
use magneto_serge::cassette::{CassetteFormat, AsyncCassetteStorage};

// Créer le storage
let storage = AsyncCassetteStorage::new();

// Choisir le format explicitement
let format = CassetteFormat::JsonGzip;

// Ou utiliser auto-détection
let path = PathBuf::from("./cassettes/test.json.gz");
let format = detect_format(&path);
```

### Via Variable d'Environnement

```bash
# Format JSON compressé
export MAGNETO_FORMAT=json.gz

# Format MessagePack compressé
export MAGNETO_FORMAT=msgpack.gz

# Format JSON (par défaut)
export MAGNETO_FORMAT=json
```

### Via Fichier de Configuration

```toml
# magneto.toml
[magneto]
cassette_dir = "./cassettes"
format = "msgpack.gz"  # json | json.gz | msgpack | msgpack.gz
```

## 📈 Performance

### Temps de Compression

Pour 100 interactions (600 KB):

| Opération | JSON | JSON.GZ | MessagePack | MessagePack.GZ |
|-----------|------|---------|-------------|----------------|
| Sérialisation | 2.5ms | 8.2ms | 1.1ms | 4.8ms |
| Désérialisation | 3.1ms | 7.9ms | 0.8ms | 4.2ms |
| Taille disque | 1.2 MB | 156 KB | 652 KB | 89 KB |

### Recommandations par Usage

#### Développement Local
```toml
format = "json"  # Facile à inspecter et debugger
```

#### Tests CI/CD
```toml
format = "msgpack"  # Rapide + économie d'espace
```

#### Archivage Long Terme
```toml
format = "msgpack.gz"  # Taille minimale
```

#### Très Gros Volumes (>10 MB)
```toml
format = "json.gz"  # Balance taille/vitesse
```

## 🎓 Bonnes Pratiques

### ✅ DO

1. **Utiliser compression en CI/CD** : Réduit le temps de téléchargement des artefacts
2. **MessagePack pour production** : Meilleur compromis vitesse/taille
3. **JSON pour debug** : Plus facile à inspecter avec un éditeur de texte
4. **Nommer les fichiers clairement** : Utilisez les bonnes extensions (.json.gz, .msgpack.gz)
5. **Benchmarker votre cas d'usage** : La compression est plus efficace sur gros volumes

### ❌ DON'T

1. **Ne pas compresser de très petites cassettes** : Overhead de gzip peut augmenter la taille
2. **Ne pas éditer manuellement les fichiers .gz** : Utilisez `gunzip` puis réenregistrez
3. **Ne pas mélanger les formats** : Standardisez dans votre projet
4. **Ne pas commit les deux formats** : Choisissez un format par cassette

## 🔍 Inspection des Cassettes Compressées

### Via CLI

```bash
# Inspecter automatiquement (décompresse à la volée)
magneto inspect my-cassette

# Liste toutes les cassettes (tous formats)
magneto list
```

### Via gunzip

```bash
# Décompresser temporairement
gunzip -c cassettes/my-test.json.gz | jq .

# Convertir en JSON non compressé
gunzip cassettes/my-test.json.gz
# Crée cassettes/my-test.json
```

### Via Rust API

```rust
use magneto_serge::cassette::{AsyncCassetteStorage, CassetteFormat};

// Charger n'importe quel format
let path = PathBuf::from("cassettes/test.json.gz");
let format = detect_format(&path);
let cassette = AsyncCassetteStorage::load_async(&path, format).await?;

// Afficher le contenu
println!("Cassette: {}", cassette.name);
println!("Interactions: {}", cassette.interactions.len());

for (i, interaction) in cassette.interactions.iter().enumerate() {
    println!("  #{}: {:?}", i, interaction);
}
```

## 🔄 Migration de Format

### JSON → JSON.GZ

```rust
use magneto_serge::cassette::{AsyncCassetteStorage, CassetteFormat};

let storage = AsyncCassetteStorage::new();

// Charger JSON
let json_path = PathBuf::from("cassettes/test.json");
let cassette = AsyncCassetteStorage::load_async(&json_path, CassetteFormat::Json).await?;

// Sauver en JSON.GZ
let gz_path = PathBuf::from("cassettes/test.json.gz");
storage.save_sync(cassette, gz_path, CassetteFormat::JsonGzip).await?;

// Supprimer l'ancien
std::fs::remove_file(json_path)?;
```

### JSON → MessagePack.GZ

```rust
// Charger JSON
let json_path = PathBuf::from("cassettes/test.json");
let cassette = AsyncCassetteStorage::load_async(&json_path, CassetteFormat::Json).await?;

// Sauver en MessagePack.GZ
let msgpack_gz_path = PathBuf::from("cassettes/test.msgpack.gz");
storage.save_sync(cassette, msgpack_gz_path, CassetteFormat::MessagePackGzip).await?;
```

### Script de Migration en Masse

```bash
#!/bin/bash
# migrate-to-compressed.sh

for file in cassettes/*.json; do
    name=$(basename "$file" .json)

    # Charger et resauver avec compression
    magneto inspect "$name" > /tmp/temp.json
    gzip -c /tmp/temp.json > "cassettes/$name.json.gz"

    # Vérifier que ça marche
    if magneto inspect "$name" > /dev/null 2>&1; then
        rm "$file"  # Supprimer l'ancien
        echo "✓ Migrated $name"
    else
        echo "✗ Failed $name"
        rm "cassettes/$name.json.gz"
    fi
done
```

## 🐛 Dépannage

### Problème : Fichier .gz plus gros que l'original

**Cause** : Cassette très petite (<1 KB), overhead de gzip

**Solution** : Utiliser JSON non compressé ou MessagePack pour petites cassettes

```rust
if std::fs::metadata(&path)?.len() < 1024 {
    // Petite cassette, pas de compression
    format = CassetteFormat::Json;
} else {
    // Grosse cassette, compression efficace
    format = CassetteFormat::JsonGzip;
}
```

### Problème : Erreur "invalid gzip header"

**Cause** : Fichier corrompu ou pas réellement compressé

**Solution** : Vérifier le fichier

```bash
# Tester si c'est vraiment du gzip
file cassettes/test.json.gz

# Essayer de décompresser
gunzip -t cassettes/test.json.gz

# Si corrompu, réenregistrer
rm cassettes/test.json.gz
magneto record test
```

### Problème : Ralentissement en CI/CD

**Cause** : Trop de compression CPU-intensive

**Solution** : Utiliser MessagePack sans compression

```toml
# magneto.toml
[magneto]
format = "msgpack"  # Rapide sans compression
```

## 🔐 Compression et Sécurité

La compression **ne chiffre pas** les données. Pour des données sensibles :

```rust
use magneto_serge::filters::FilterPresets;

// Filtrer les données sensibles AVANT compression
let filters = FilterPresets::security();
let mut recorder = Recorder::new_with_filters("secure".to_string(), filters);

// Enregistrer
recorder.record_http(request, response);

// Sauver compressé (données déjà filtrées)
recorder.save_compressed("./cassettes")?;
```

Voir [FILTERS.md](./FILTERS.md) pour plus d'informations sur le filtrage.

## 📊 Benchmarks

### Test de Compression

```bash
# Exécuter les benchmarks
cargo bench --bench serialization_optim

# Résultats attendus (cassette 100 interactions):
# JSON:              2.5ms sérialisation, 1.2 MB
# JSON.GZ:           8.2ms sérialisation, 156 KB (87% compression)
# MessagePack:       1.1ms sérialisation, 652 KB (47% compression)
# MessagePack.GZ:    4.8ms sérialisation, 89 KB (93% compression)
```

### Benchmark Personnalisé

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_compression(c: &mut Criterion) {
    let cassette = create_large_cassette(); // Votre cassette

    c.bench_function("json_gz_save", |b| {
        b.iter(|| {
            storage.save_sync(
                black_box(cassette.clone()),
                path.clone(),
                CassetteFormat::JsonGzip
            )
        });
    });
}

criterion_group!(benches, benchmark_compression);
criterion_main!(benches);
```

## 🔗 Ressources Connexes

- **[OPTIMIZATIONS.md](./OPTIMIZATIONS.md)** : Optimisations de performance (MessagePack, async I/O)
- **[FILTERS.md](./FILTERS.md)** : Filtrage des données sensibles
- **[STRICT_MODE.md](./STRICT_MODE.md)** : Mode STRICT pour CI/CD
- **[BENCHMARKS.md](./BENCHMARKS.md)** : Résultats complets des benchmarks

## 🔬 Détails Techniques

### Algorithme de Compression

Magneto utilise **gzip** (flate2) avec niveau de compression par défaut (6) :
- Niveau 1-3 : Rapide, compression modérée
- Niveau 6 (défaut) : Balance vitesse/compression
- Niveau 9 : Lent, compression maximale

### Format de Fichier

```
.json.gz:
├── Magic bytes: 1F 8B (gzip)
├── Compression method: 08 (DEFLATE)
└── Compressed JSON data

.msgpack.gz:
├── Magic bytes: 1F 8B (gzip)
├── Compression method: 08 (DEFLATE)
└── Compressed MessagePack data
```

### Compatibilité

- **Rust** : ✅ Full support
- **Python** : ⏳ À venir (UniFFI binding)
- **Java** : ⏳ À venir (UniFFI binding)
- **JavaScript** : ⏳ À venir (UniFFI binding)
- **CLI** : ✅ Full support

## 📄 FAQ

### Q: Quelle compression choisir ?

**R**: Dépend de votre cas d'usage :
- **Développement** : JSON (lisible)
- **CI/CD** : MessagePack (rapide)
- **Archivage** : MessagePack.GZ (petit)
- **Gros volumes (>10MB)** : JSON.GZ ou MessagePack.GZ

### Q: La compression ralentit-elle les tests ?

**R**: Pour cassettes <1MB : impact minimal (<10ms)
Pour cassettes >10MB : gain de temps I/O compense largement le CPU de compression

### Q: Puis-je éditer manuellement une cassette .gz ?

**R**: Non recommandé. Décompressez d'abord :
```bash
gunzip cassettes/test.json.gz
# Éditer test.json
gzip cassettes/test.json
```

### Q: Puis-je mélanger formats dans un même projet ?

**R**: Techniquement oui, mais déconseillé. Standardisez sur un format pour cohérence.

### Q: Comment réduire encore plus la taille ?

**R**: Utilisez les filtres pour supprimer les données inutiles :
```rust
let filters = FilterPresets::small_bodies(1000); // Max 1KB par body
```

---

*Dernière mise à jour : 2025-10-11*
