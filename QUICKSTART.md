# 🚀 Guide de Démarrage Rapide - Magneto-Serge

Guide pour démarrer rapidement avec Magneto-Serge en utilisant Docker.

## ⚡ Démarrage Ultra-Rapide (1 minute)

### Option 1 : Docker Compose (Recommandé)

```bash
# 1. Démarrer la stack complète
docker-compose -f docker-compose.alpine.yml up -d

# 2. Ouvrir l'interface web
open http://localhost:4201

# 3. L'interface est prête ! 🎉
```

### Option 2 : Docker Run

```bash
# Démarrer tous les services
docker run -d --name magneto-serge \
  -p 8889:8889 \
  -p 4201:4201 \
  -p 8888:8888 \
  magneto-serge:alpine all

# Ouvrir l'interface
open http://localhost:4201
```

## 🎯 Premiers Pas avec l'Interface

### 1️⃣ Enregistrer Votre Première Session

1. **Ouvrez l'interface** : http://localhost:4201
2. **Créez une cassette** :
   - Cliquez sur "➕ Nouvelle Cassette"
   - Entrez un nom : `mon-premier-test`
3. **Mode automatique** : Le mode "⏺️ Record" est déjà sélectionné
4. **Démarrez** : Cliquez sur "▶️ Démarrer"
5. **Configurez votre application** pour utiliser le proxy :
   ```bash
   export http_proxy=http://localhost:8888
   export https_proxy=http://localhost:8888
   ```
6. **Faites vos requêtes** :
   ```bash
   curl http://api.example.com/users
   curl http://api.example.com/posts
   ```
7. **Arrêtez** : Cliquez sur "⏹️ Arrêter" dans l'interface
8. **Vérifiez** : La cassette apparaît dans la liste avec 2 interactions

### 2️⃣ Rejouer Votre Session

1. **Cliquez** sur "▶️ Utiliser" pour votre cassette
2. **Sélectionnez** le mode "▶️ Replay"
3. **Démarrez** : Cliquez sur "▶️ Démarrer"
4. **Refaites les requêtes** : Elles seront rejouées depuis la cassette !
   ```bash
   curl http://api.example.com/users  # ⚡ Instantané !
   ```

### 3️⃣ Mode Auto (Pour les Tests)

Le mode Auto est parfait pour les tests automatisés :

```bash
# Premier lancement : enregistre
# Lancements suivants : rejoue automatiquement
```

1. Entrez un nom de cassette : `test-integration`
2. Sélectionnez "🔄 Auto"
3. Démarrez
4. Premier run : tout est enregistré
5. Runs suivants : tout est rejoué automatiquement

## 📊 Fonctionnalités de l'Interface

### Barre de Statut
- 🟢 **État** : Actif / Arrêté
- 🎯 **Mode** : Auto / Record / Replay / Passthrough
- 📼 **Cassette** : Nom de la cassette en cours
- 🔄 **Rafraîchir** : Mise à jour manuelle

### Statistiques
- **Requêtes** : Nombre total proxifiées
- **Cassettes** : Nombre disponibles
- **Uptime** : Temps depuis démarrage API
- **Mode Actif** : Mode en cours d'exécution

### Gestion des Cassettes
- **▶️ Utiliser** : Sélectionne pour utilisation
- **🔍 Inspecter** : Affiche détails complets
- **🗑️ Supprimer** : Supprime après confirmation
- **➕ Nouvelle** : Crée une cassette vide

## 🔧 Configuration Avancée

### Changer le Port du Proxy

Par défaut le proxy écoute sur le port 8888. Pour changer :

1. Dans l'interface, modifiez "Port du Proxy"
2. Entrez le nouveau port (ex: 9999)
3. Cliquez sur "Démarrer"

### Modes d'Opération

#### 🔄 Auto (Intelligent)
```
Si cassette existe → Replay
Sinon → Record
```
**Usage** : Tests automatisés, CI/CD

#### ⏺️ Record (Enregistrement)
```
Enregistre toutes les requêtes
Écrase la cassette existante
```
**Usage** : Création/mise à jour de cassettes

#### ▶️ Replay (Lecture)
```
Rejoue depuis la cassette
Erreur si cassette manquante
```
**Usage** : Tests déterministes, démo

#### ➡️ Passthrough (Transparent)
```
Transfère sans enregistrer
Mode proxy classique
```
**Usage** : Debug, monitoring

## 🌐 URLs des Services

| Service | URL | Description |
|---------|-----|-------------|
| **Interface Web** | http://localhost:4201 | Dashboard de gestion complet |
| **API REST** | http://localhost:8889 | API Hydra/JSON-LD |
| **API Health** | http://localhost:8889/health | Vérification santé |
| **Proxy Status** | http://localhost:8889/proxy/status | État du proxy |
| **Proxy** | http://localhost:8888 | Proxy HTTP/HTTPS/WS |

## 📝 Exemples d'Utilisation

### Exemple 1 : Test d'API REST

```bash
# 1. Démarrer Magneto-Serge
docker-compose -f docker-compose.alpine.yml up -d

# 2. Ouvrir l'interface et créer une cassette "test-jsonplaceholder"
# 3. Mode Record, port 8888, Démarrer

# 4. Faire des requêtes via le proxy
export http_proxy=http://localhost:8888
curl https://jsonplaceholder.typicode.com/posts/1
curl https://jsonplaceholder.typicode.com/users/1

# 5. Arrêter dans l'interface

# 6. Rejouer : Mode Replay, Démarrer
curl https://jsonplaceholder.typicode.com/posts/1  # ⚡ Instantané !
```

### Exemple 2 : Tests d'Intégration

```javascript
// test.js
process.env.HTTP_PROXY = 'http://localhost:8888';

describe('API Tests', () => {
  beforeAll(async () => {
    // Démarrer le proxy via l'API
    await fetch('http://localhost:8889/proxy/start', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        cassette_name: 'integration-tests',
        mode: 'Auto',
        port: 8888
      })
    });
  });

  test('GET /users', async () => {
    const response = await fetch('https://api.example.com/users');
    expect(response.status).toBe(200);
  });

  afterAll(async () => {
    // Arrêter le proxy
    await fetch('http://localhost:8889/proxy/stop', {
      method: 'POST'
    });
  });
});
```

### Exemple 3 : Debug d'Application

```bash
# Mode Passthrough pour debug
# L'interface : Mode Passthrough, Démarrer

# Toutes les requêtes passent par le proxy sans enregistrement
curl http://localhost:8888/api.example.com/debug

# Les logs sont disponibles dans l'interface
docker logs magneto-serge
```

## 🐛 Résolution de Problèmes

### L'interface ne charge pas
```bash
# Vérifier que le container est démarré
docker ps | grep magneto-serge

# Vérifier les logs
docker logs magneto-serge

# Redémarrer
docker-compose -f docker-compose.alpine.yml restart
```

### Le proxy ne démarre pas
```bash
# Vérifier l'API
curl http://localhost:8889/health

# Vérifier les logs de l'API
docker logs magneto-serge | grep magneto-api

# Le port 8888 est-il libre ?
lsof -i :8888
```

### Les cassettes ne s'affichent pas
```bash
# Vérifier le répertoire des cassettes
docker exec magneto-serge ls -la /app/cassettes

# Rafraîchir la liste dans l'interface
# Bouton "🔄 Rafraîchir la Liste"
```

## 📦 Commandes Utiles

### Docker Compose

```bash
# Démarrer
docker-compose -f docker-compose.alpine.yml up -d

# Arrêter
docker-compose -f docker-compose.alpine.yml down

# Voir les logs
docker-compose -f docker-compose.alpine.yml logs -f

# Redémarrer
docker-compose -f docker-compose.alpine.yml restart

# Statut
docker-compose -f docker-compose.alpine.yml ps
```

### Docker Run

```bash
# Démarrer tous les services
docker run -d --name magneto-serge \
  -p 8889:8889 -p 4201:4201 -p 8888:8888 \
  magneto-serge:alpine all

# Arrêter
docker stop magneto-serge && docker rm magneto-serge

# Logs
docker logs -f magneto-serge

# Exec
docker exec -it magneto-serge sh
```

### API Directe

```bash
# Health check
curl http://localhost:8889/health

# Statut du proxy
curl http://localhost:8889/proxy/status

# Lister les cassettes
curl http://localhost:8889/cassettes

# Inspecter une cassette
curl http://localhost:8889/cassettes/mon-test

# Démarrer le proxy
curl -X POST http://localhost:8889/proxy/start \
  -H "Content-Type: application/json" \
  -d '{
    "cassette_name": "test",
    "mode": "Auto",
    "port": 8888
  }'

# Arrêter le proxy
curl -X POST http://localhost:8889/proxy/stop
```

## 🎓 Prochaines Étapes

1. **Explorez la documentation complète** : `docs/DOCKER_ALPINE.md`
2. **Lisez le guide de l'interface** : `docker/frontend/README.md`
3. **Consultez les exemples** : `examples/`
4. **Contribuez** : Ouvrez une issue ou une PR !

## 💡 Astuces

- **Auto-refresh** : L'interface se rafraîchit automatiquement toutes les 5 secondes
- **Raccourcis** : Utilisez les boutons "Utiliser" pour sélectionner rapidement une cassette
- **Inspection** : Double-cliquez sur une cassette pour voir ses détails
- **Mode Auto** : Parfait pour les tests - pas besoin de gérer manuellement
- **Volumes** : Les cassettes sont persistées dans `/app/cassettes`

## 📞 Support

- **Issues** : https://github.com/taciclei/magneto-serge/issues
- **Documentation** : `docs/`
- **Examples** : `examples/`

---

**Bon test avec Magneto-Serge ! 🚀**
