# 🐳 Magneto-Serge - Docker Image Alpine

Image Docker optimisée basée sur Alpine Linux incluant l'écosystème complet Magneto-Serge.

---

## 📦 Contenu de l'Image

L'image inclut :

- ✅ **CLI Magneto-Serge** (Rust) - Binaire statique
- ✅ **API REST** (port 8889) - Hydra/JSON-LD + OpenAPI 3.0
- ✅ **Interface Web de Gestion** (port 4201) - Dashboard interactif complet
  - Contrôle du proxy (démarrer/arrêter)
  - Sélection des modes (Auto/Record/Replay/Passthrough)
  - Gestion des cassettes (liste, inspection, suppression)
  - Statistiques en temps réel
  - Interface responsive et moderne
- ✅ **Proxy HTTP/HTTPS/WebSocket** (port 8888) - Configurable

**Taille de l'image** : ~462 MB (Alpine Linux avec Node.js)

## 🎨 Interface Web

L'interface web propose les fonctionnalités suivantes :

### 📊 Vue d'ensemble
- **Barre de statut** : État du proxy en temps réel, mode actif, cassette en cours
- **Statistiques** : Nombre de requêtes, cassettes disponibles, uptime de l'API
- **Rafraîchissement automatique** : Mise à jour toutes les 5 secondes

### ⚙️ Contrôle du Proxy
- **Sélection du mode** : 4 modes disponibles (Auto, Record, Replay, Passthrough)
- **Configuration** : Nom de cassette personnalisable, port du proxy configurable
- **Boutons de contrôle** : Démarrer/Arrêter le proxy avec feedback visuel

### 📼 Gestion des Cassettes
- **Liste complète** : Toutes les cassettes avec métadonnées
- **Actions par cassette** :
  - ▶️ Utiliser : Sélectionne la cassette pour le prochain démarrage
  - 🔍 Inspecter : Affiche les détails et interactions de la cassette
  - 🗑️ Supprimer : Supprime la cassette après confirmation
- **Création** : Bouton pour créer une nouvelle cassette

### 🎨 Design
- **Thème moderne** : Gradient violet/bleu avec cartes blanches
- **Responsive** : S'adapte aux différentes tailles d'écran
- **Animations** : Transitions fluides et feedback visuel
- **Alertes** : Notifications pour les actions réussies ou les erreurs

---

## 🚀 Quick Start

### Option 1 : Docker Compose (Recommandé)

```bash
# Démarrer la stack complète
docker-compose -f docker-compose.alpine.yml up -d

# Vérifier le statut
docker-compose -f docker-compose.alpine.yml ps

# Voir les logs
docker-compose -f docker-compose.alpine.yml logs -f

# Arrêter
docker-compose -f docker-compose.alpine.yml down
```

**Services disponibles** :
- API : http://localhost:8889
- Backend : http://localhost:3000
- Interface : http://localhost:4201
- Proxy : http://localhost:8888

### Option 2 : Docker Run

```bash
# Build l'image
docker build -f Dockerfile.alpine -t magneto-serge:alpine .

# Démarrer tous les services
docker run -d \
  --name magneto-serge \
  -p 8889:8889 \
  -p 3000:3000 \
  -p 4201:4201 \
  -p 8888:8888 \
  -v $(pwd)/cassettes:/app/cassettes \
  magneto-serge:alpine all
```

---

## 🎛️ Modes de Démarrage

### Mode 1 : Tous les Services (Défaut)

Démarre API + Backend + Frontend en un seul container.

```bash
docker run -p 8889:8889 -p 3000:3000 -p 4201:4201 magneto-serge:alpine all
```

**Services démarrés** :
- ✅ API REST (8889)
- ✅ Backend Node.js (3000)
- ✅ Interface Angular (4201) via Nginx

### Mode 2 : API Seulement

```bash
docker run -p 8889:8889 magneto-serge:alpine api
```

### Mode 3 : Proxy Standalone

```bash
# Mode auto
docker run -p 8888:8888 -v ./cassettes:/app/cassettes \
  magneto-serge:alpine proxy auto test

# Mode record
docker run -p 8888:8888 -v ./cassettes:/app/cassettes \
  magneto-serge:alpine proxy record my-api-test
```

---

## 🌐 Redistribution du Réseau

### Scénario : Proxy avec Port Forwarding

```bash
# Proxy écoute sur 8888 et redistribue vers l'API cible
docker run -d \
  --name magneto-proxy \
  -p 8888:8888 \
  -v ./cassettes:/app/cassettes \
  magneto-serge:alpine proxy auto test

# Configurer votre application pour utiliser le proxy
export http_proxy=http://localhost:8888
export https_proxy=http://localhost:8888

# Toutes les requêtes HTTP/HTTPS passent par le proxy
curl http://api.example.com
```

---

## 📚 Documentation Complète

Pour la documentation complète, voir les fichiers suivants qui vont être créés :
- `Dockerfile.alpine` - Image Docker multi-stage
- `docker-compose.alpine.yml` - Orchestration complète
- `docker/` - Fichiers de configuration

---

**Dernière mise à jour** : 13 octobre 2025
