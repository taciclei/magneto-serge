# Modifications des Workflows GitHub Requises

⚠️ **Ces modifications ne peuvent pas être pushées automatiquement car le token OAuth n'a pas le scope `workflow`.**

## Changements à appliquer

### Fichier: `.github/workflows/ci.yml`

**Ligne 86**: Remplacer
```yaml
run: cargo build --bin matgto --features cli --release
```
par
```yaml
run: cargo build --bin magneto --features cli --release
```

**Ligne 89**: Remplacer
```yaml
run: cargo run --bin matgto --features cli -- version
```
par
```yaml
run: cargo run --bin magneto --features cli -- version
```

**Ligne 94**: Remplacer
```yaml
name: matgto-cli-${{ matrix.os }}
```
par
```yaml
name: magneto-cli-${{ matrix.os }}
```

**Ligne 96**: Remplacer
```yaml
target/release/matgto${{ matrix.os == 'windows-latest' && '.exe' || '' }}
```
par
```yaml
target/release/magneto${{ matrix.os == 'windows-latest' && '.exe' || '' }}
```

### Fichier: `.github/workflows/cd.yml`

**Ligne 157**: Remplacer
```yaml
target/release/matgto
```
par
```yaml
target/release/magneto
```

## Comment appliquer ces modifications

### Option 1: Via l'interface GitHub
1. Aller sur https://github.com/taciclei/magneto-serge
2. Éditer les fichiers directement sur GitHub
3. Commit les changements

### Option 2: Via un token avec le scope workflow
```bash
# Créer un nouveau token avec le scope 'workflow' sur GitHub
# https://github.com/settings/tokens
gh auth login --with-token < nouveau_token.txt
git push origin develop
```

### Option 3: Via la Pull Request
Les changements seront automatiquement inclus quand la PR #1 sera mergée dans main/master.

## État actuel

- ✅ Tous les changements sont commités localement
- ✅ Le code, les tests, et la documentation sont à jour
- ⚠️ Seuls les workflows sont en attente de push
