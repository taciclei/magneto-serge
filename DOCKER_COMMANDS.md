# 🐳 Commandes Docker Magneto-Serge

Guide de référence rapide pour toutes les commandes Docker.

## 📦 Construction

### Build Image Alpine

```bash
# Via Makefile (recommandé)
make docker-build-alpine

# Ou directement
docker build -f Dockerfile.alpine -t magneto-serge:alpine -t tabou/magneto-serge:alpine .
```

### Build Image Standard

```bash
make docker-build
# ou
docker build -t magneto-serge:latest .
```

---

## 🚀 Démarrage

### Docker Compose (Recommandé)

```bash
# Démarrer la stack Alpine
make docker-compose-alpine
# ou
docker-compose -f docker-compose.alpine.yml up -d

# Arrêter
docker-compose -f docker-compose.alpine.yml down

# Redémarrer
docker-compose -f docker-compose.alpine.yml restart

# Voir les logs
docker-compose -f docker-compose.alpine.yml logs -f

# Statut
docker-compose -f docker-compose.alpine.yml ps
```

### Docker Run - Tous les Services

```bash
# Via Makefile
make docker-run-alpine

# Ou directement
docker run -d --name magneto-serge \
  -p 8889:8889 \
  -p 3000:3000 \
  -p 4201:4201 \
  -p 8888:8888 \
  -v $(pwd)/cassettes:/app/cassettes \
  magneto-serge:alpine all
```

### Docker Run - Service Spécifique

```bash
# API uniquement
docker run -d --name magneto-api \
  -p 8889:8889 \
  magneto-serge:alpine api

# Proxy uniquement (mode auto)
docker run -d --name magneto-proxy \
  -p 8888:8888 \
  -v $(pwd)/cassettes:/app/cassettes \
  magneto-serge:alpine proxy auto default

# Proxy en mode record
docker run -d --name magneto-proxy \
  -p 8888:8888 \
  -v $(pwd)/cassettes:/app/cassettes \
  magneto-serge:alpine proxy record my-test

# Proxy en mode replay
docker run -d --name magneto-proxy \
  -p 8888:8888 \
  -v $(pwd)/cassettes:/app/cassettes \
  magneto-serge:alpine proxy replay my-test
```

---

## 🔍 Inspection

### Logs

```bash
# Tous les logs
docker logs magneto-serge

# Logs en temps réel
docker logs -f magneto-serge

# 100 dernières lignes
docker logs --tail=100 magneto-serge

# Logs de l'API uniquement
docker logs magneto-serge 2>&1 | grep magneto-api

# Logs du proxy
docker logs magneto-serge 2>&1 | grep magneto-proxy
```

### Statut

```bash
# Statut du container
docker ps | grep magneto-serge

# Statut détaillé
docker inspect magneto-serge

# Health check
docker inspect magneto-serge | grep -A5 Health

# Stats en temps réel
docker stats magneto-serge
```

### Exec dans le Container

```bash
# Shell interactif
docker exec -it magneto-serge sh

# Commande unique
docker exec magneto-serge ls -la /app/cassettes

# Vérifier le binaire
docker exec magneto-serge /app/bin/magneto --version

# Lister les cassettes
docker exec magneto-serge /app/bin/magneto list
```

---

## 🛑 Arrêt et Nettoyage

### Arrêter

```bash
# Arrêter proprement
docker stop magneto-serge

# Arrêt forcé (après 10 secondes)
docker stop -t 10 magneto-serge

# Arrêter et supprimer
docker stop magneto-serge && docker rm magneto-serge
```

### Nettoyage

```bash
# Supprimer le container
docker rm magneto-serge

# Supprimer l'image
docker rmi magneto-serge:alpine

# Nettoyer tout (containers, images, volumes non utilisés)
docker system prune -a --volumes

# Nettoyer uniquement les containers arrêtés
docker container prune
```

---

## 📊 Monitoring

### Vérifier les Services

```bash
# API Health
curl http://localhost:8889/health

# Statut du proxy
curl http://localhost:8889/proxy/status

# Interface web
curl -I http://localhost:4201

# Backend (si fonctionnel)
curl http://localhost:3000
```

### Logs par Service

```bash
# API
docker exec magneto-serge cat /app/logs/api.log

# Backend
docker exec magneto-serge cat /app/logs/backend.log

# Nginx
docker exec magneto-serge cat /app/logs/nginx.log

# Proxy
docker exec magneto-serge cat /app/logs/proxy.log
```

---

## 🔄 Mise à Jour

### Rebuild et Redéploiement

```bash
# 1. Arrêter l'ancien container
docker-compose -f docker-compose.alpine.yml down

# 2. Rebuild l'image
make docker-build-alpine

# 3. Redémarrer
docker-compose -f docker-compose.alpine.yml up -d

# Ou en une commande
docker-compose -f docker-compose.alpine.yml up -d --build
```

---

## 🌐 Réseau

### Configuration du Proxy

```bash
# Variables d'environnement
export http_proxy=http://localhost:8888
export https_proxy=http://localhost:8888
export HTTP_PROXY=http://localhost:8888
export HTTPS_PROXY=http://localhost:8888

# Test
curl http://api.example.com

# Désactiver
unset http_proxy https_proxy HTTP_PROXY HTTPS_PROXY
```

### Réseau Docker

```bash
# Inspecter le réseau
docker network inspect matgto-serge_magneto-network

# Voir les containers sur le réseau
docker network inspect matgto-serge_magneto-network | grep -A10 Containers
```

---

## 💾 Volumes et Données

### Cassettes

```bash
# Lister les cassettes dans le container
docker exec magneto-serge ls -la /app/cassettes

# Copier une cassette depuis le container
docker cp magneto-serge:/app/cassettes/my-test.json ./

# Copier une cassette vers le container
docker cp my-test.json magneto-serge:/app/cassettes/

# Backup des cassettes
docker cp magneto-serge:/app/cassettes ./cassettes-backup
```

### Volumes

```bash
# Lister les volumes
docker volume ls | grep magneto

# Inspecter un volume
docker volume inspect matgto-serge_magneto-logs

# Supprimer les volumes
docker-compose -f docker-compose.alpine.yml down -v
```

---

## 🐛 Dépannage

### Container ne démarre pas

```bash
# Voir les logs d'erreur
docker logs magneto-serge 2>&1 | tail -50

# Vérifier la configuration
docker inspect magneto-serge | grep -A20 Config

# Tester le démarrage en mode interactif
docker run -it --rm magneto-serge:alpine sh
```

### Port déjà utilisé

```bash
# Voir quel processus utilise le port
lsof -i :8889
lsof -i :4201
lsof -i :8888

# Tuer le processus
kill -9 <PID>

# Ou changer le port dans docker-compose.alpine.yml
```

### Problèmes de permissions

```bash
# Vérifier les permissions des cassettes
docker exec magneto-serge ls -la /app/cassettes

# Changer les permissions si nécessaire
docker exec magneto-serge chmod 755 /app/cassettes
docker exec magneto-serge chmod 644 /app/cassettes/*.json
```

### Réinitialiser complètement

```bash
# Arrêter tout
docker-compose -f docker-compose.alpine.yml down -v

# Supprimer l'image
docker rmi magneto-serge:alpine

# Supprimer les cassettes locales
rm -rf cassettes/*

# Rebuild from scratch
make docker-build-alpine

# Redémarrer
make docker-compose-alpine
```

---

## 📤 Publication

### Docker Hub

```bash
# Login
docker login -u tabou

# Push l'image
docker push tabou/magneto-serge:alpine

# Push avec tag spécifique
docker tag magneto-serge:alpine tabou/magneto-serge:v1.0.0
docker push tabou/magneto-serge:v1.0.0

# Push latest
docker tag magneto-serge:alpine tabou/magneto-serge:latest
docker push tabou/magneto-serge:latest
```

### Pull depuis Docker Hub

```bash
# Pull l'image
docker pull tabou/magneto-serge:alpine

# Run directement
docker run -d --name magneto-serge \
  -p 8889:8889 -p 4201:4201 -p 8888:8888 \
  tabou/magneto-serge:alpine all
```

---

## 🎯 Commandes Rapides

### Quick Start

```bash
make docker-compose-alpine && open http://localhost:4201
```

### Quick Stop

```bash
docker-compose -f docker-compose.alpine.yml down
```

### Quick Restart

```bash
docker-compose -f docker-compose.alpine.yml restart
```

### Quick Logs

```bash
docker-compose -f docker-compose.alpine.yml logs -f --tail=100
```

### Quick Health Check

```bash
curl -s http://localhost:8889/health | jq '.data.status'
```

---

## 📋 Checklist de Démarrage

- [ ] Build l'image : `make docker-build-alpine`
- [ ] Démarrer : `make docker-compose-alpine`
- [ ] Vérifier API : `curl http://localhost:8889/health`
- [ ] Ouvrir interface : `open http://localhost:4201`
- [ ] Créer une cassette
- [ ] Démarrer le proxy
- [ ] Configurer `http_proxy`
- [ ] Faire des requêtes
- [ ] Vérifier les cassettes

---

## 🔗 URLs de Référence

- **Interface Web** : http://localhost:4201
- **API REST** : http://localhost:8889
- **API Docs** : http://localhost:8889/docs (si disponible)
- **Health Check** : http://localhost:8889/health
- **Proxy Status** : http://localhost:8889/proxy/status
- **Cassettes List** : http://localhost:8889/cassettes

---

**Dernière mise à jour** : 13 octobre 2025
