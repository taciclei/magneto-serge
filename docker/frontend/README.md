# Interface Web Magneto-Serge

Interface de gestion complète pour le proxy HTTP/HTTPS/WebSocket Magneto-Serge.

## 🎯 Fonctionnalités

### Dashboard Principal
- **Barre de statut en temps réel** : Affiche l'état du proxy, le mode actif et la cassette en cours
- **Rafraîchissement automatique** : Mise à jour automatique toutes les 5 secondes
- **Design moderne** : Interface responsive avec gradient violet/bleu

### Contrôle du Proxy

#### Modes Disponibles
1. **🔄 Auto** - Enregistre si la cassette n'existe pas, sinon rejoue
2. **⏺️ Record** - Enregistre toutes les requêtes dans une cassette
3. **▶️ Replay** - Rejoue les interactions depuis une cassette
4. **➡️ Passthrough** - Transfère les requêtes sans enregistrement

#### Configuration
- **Nom de cassette** : Personnalisable, par défaut "default"
- **Port du proxy** : Configurable, par défaut 8888
- **Boutons de contrôle** :
  - ▶️ Démarrer : Lance le proxy avec la configuration actuelle
  - ⏹️ Arrêter : Arrête le proxy en cours d'exécution

### Gestion des Cassettes

#### Liste des Cassettes
Affiche toutes les cassettes disponibles avec :
- Nom de la cassette
- Nombre d'interactions enregistrées
- Taille du fichier

#### Actions Disponibles
- **▶️ Utiliser** : Sélectionne la cassette pour le prochain démarrage
- **🔍 Inspecter** : Affiche les détails de la cassette :
  - Version
  - Date d'enregistrement
  - Liste des interactions (méthode HTTP, URL)
- **🗑️ Supprimer** : Supprime la cassette après confirmation

#### Création de Cassettes
- Bouton "➕ Nouvelle Cassette" : Crée une cassette vide prête à enregistrer
- Bascule automatiquement en mode Record

### Statistiques

Panneau de statistiques en temps réel :
- **Requêtes** : Nombre total de requêtes proxifiées
- **Cassettes** : Nombre de cassettes disponibles
- **Uptime API** : Temps depuis le démarrage de l'API
- **Mode Actif** : Mode du proxy actuellement en cours

## 🚀 Utilisation

### Accès
```
http://localhost:4201
```

### Workflow Typique

#### 1. Enregistrer une Nouvelle Session
1. Cliquez sur "➕ Nouvelle Cassette"
2. Entrez un nom (ex: "test-api-users")
3. Sélectionnez le mode "⏺️ Record"
4. Cliquez sur "▶️ Démarrer"
5. Configurez votre application pour utiliser le proxy (port 8888)
6. Effectuez vos requêtes
7. Cliquez sur "⏹️ Arrêter" quand terminé

#### 2. Rejouer une Session
1. Cliquez sur "🔄 Rafraîchir la Liste"
2. Trouvez votre cassette dans la liste
3. Cliquez sur "▶️ Utiliser" pour la sélectionner
4. Sélectionnez le mode "▶️ Replay"
5. Cliquez sur "▶️ Démarrer"
6. Vos requêtes seront rejouées depuis la cassette

#### 3. Mode Auto (Recommandé pour les Tests)
1. Entrez un nom de cassette
2. Sélectionnez le mode "🔄 Auto"
3. Cliquez sur "▶️ Démarrer"
4. Premier lancement : enregistre
5. Lancements suivants : rejoue automatiquement

## 🔧 API Endpoints Utilisés

L'interface communique avec ces endpoints de l'API :

### Health Check
```
GET /health
```
Vérifie l'état de l'API et récupère l'uptime.

### Statut du Proxy
```
GET /proxy/status
```
Récupère l'état actuel du proxy :
- `running` : boolean
- `mode` : string (Auto, Record, Replay, Passthrough)
- `port` : number
- `cassette` : string | null
- `interactions_count` : number

### Démarrer le Proxy
```
POST /proxy/start
Content-Type: application/json

{
  "cassette_name": "string",
  "mode": "Auto" | "Record" | "Replay" | "Passthrough",
  "port": number
}
```

### Arrêter le Proxy
```
POST /proxy/stop
```

### Lister les Cassettes
```
GET /cassettes
```
Retourne la liste de toutes les cassettes disponibles.

### Détails d'une Cassette
```
GET /cassettes/{name}
```
Récupère les détails complets d'une cassette spécifique.

### Supprimer une Cassette
```
DELETE /cassettes/{name}
```
Supprime définitivement une cassette.

## 🎨 Design

### Palette de Couleurs
- **Principal** : Gradient violet (#667eea → #764ba2)
- **Succès** : Vert (#4CAF50)
- **Danger** : Rouge (#f44336)
- **Warning** : Orange (#ff9800)
- **Info** : Bleu (#2196F3)

### Composants UI
- **Cards** : Cartes blanches avec ombres douces
- **Buttons** : Boutons colorés avec effet hover et animation
- **Badges** : États visuels avec codes couleur
- **Forms** : Champs de saisie avec focus bleu
- **Alerts** : Notifications temporaires (5 secondes)

### Responsive Design
- Desktop : Grille 2-3 colonnes
- Tablet : Grille adaptative
- Mobile : Une seule colonne

## 🔄 Rafraîchissement Automatique

L'interface actualise automatiquement :
- **Statut du proxy** : Toutes les 5 secondes
- **Statistiques** : Toutes les 5 secondes
- **Liste des cassettes** : Sur demande (bouton Rafraîchir)

## 🛠️ Technologies

- **HTML5** : Structure sémantique
- **CSS3** : Styles modernes avec Flexbox/Grid
- **Vanilla JavaScript** : Pas de framework, performance optimale
- **Fetch API** : Communication avec l'API REST
- **JSON** : Format d'échange de données

## 📱 Compatibilité

- ✅ Chrome/Edge 90+
- ✅ Firefox 88+
- ✅ Safari 14+
- ✅ Opera 76+

## 🐛 Dépannage

### L'interface ne se charge pas
- Vérifiez que Nginx est démarré : `docker logs <container>`
- Vérifiez le port 4201 : `curl http://localhost:4201`

### Les statistiques n'apparaissent pas
- Vérifiez que l'API est accessible : `curl http://localhost:8889/health`
- Ouvrez la console du navigateur (F12) pour voir les erreurs

### Les cassettes ne s'affichent pas
- Cliquez sur "Rafraîchir la Liste"
- Vérifiez les logs de l'API : `docker logs <container> | grep magneto-api`
- Vérifiez que le répertoire `/app/cassettes` existe

### CORS Errors
L'API doit autoriser les requêtes depuis `http://localhost:4201`.
Vérifiez la configuration CORS de l'API.

## 📝 Notes

- Interface en français 🇫🇷
- Mode sombre non disponible (à venir)
- Pas de gestion d'authentification (réseau local uniquement)
- Toutes les données sont stockées côté serveur

## 🚀 Améliorations Futures

- [ ] Mode sombre
- [ ] Graphiques de statistiques (Chart.js)
- [ ] Export de cassettes
- [ ] Import de cassettes
- [ ] Filtrage et recherche de cassettes
- [ ] Comparaison de cassettes
- [ ] Logs en temps réel
- [ ] Notifications WebSocket pour les mises à jour

---

**Version** : 1.0.0
**Date** : 13 octobre 2025
**Auteur** : Magneto-Serge Team
