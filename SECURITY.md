# Security Policy

## Supported Versions

Les versions suivantes de Magneto-Serge reçoivent actuellement des mises à jour de sécurité :

| Version | Support Sécurité |
| ------- | --------------- |
| 0.1.x   | ✅ Supporté     |
| < 0.1   | ❌ Non supporté |

## Reporting a Vulnerability

La sécurité de Magneto-Serge est prise très au sérieux. Si vous découvrez une vulnérabilité de sécurité, nous vous remercions de nous aider à protéger la communauté en la signalant de manière responsable.

### Comment Signaler une Vulnérabilité

**⚠️ NE PAS créer d'issue publique pour les vulnérabilités de sécurité.**

#### Option 1 : GitHub Security Advisory (Recommandé)

1. Allez sur https://github.com/taciclei/magneto-serge/security/advisories
2. Cliquez sur "Report a vulnerability"
3. Remplissez le formulaire avec les détails de la vulnérabilité
4. Soumettez le rapport

#### Option 2 : Email Privé

Si vous ne pouvez pas utiliser GitHub Security Advisories, créez une issue GitHub avec le label "security" et marquez-la comme confidentielle, ou contactez les mainteneurs directement.

### Informations à Inclure

Pour nous aider à mieux comprendre et résoudre le problème rapidement, veuillez inclure :

1. **Type de vulnérabilité** (ex: injection, XSS, CSRF, etc.)
2. **Composant affecté** (ex: proxy HTTP, WebSocket, CLI, bindings)
3. **Version affectée** (ex: 0.1.0)
4. **Description détaillée** du problème
5. **Steps to Reproduce** (étapes pour reproduire)
6. **Proof of Concept** (PoC) si disponible
7. **Impact potentiel** de la vulnérabilité
8. **Suggestions de correction** si vous en avez

### Exemple de Rapport

```markdown
**Type**: Buffer Overflow

**Composant**: WebSocket message handler

**Version**: 0.1.0

**Description**: Un buffer overflow peut se produire lors du traitement de messages
WebSocket de grande taille (>16MB), permettant potentiellement l'exécution de code
arbitraire.

**Steps to Reproduce**:
1. Démarrer le proxy en mode record
2. Envoyer un message WebSocket de 20MB
3. Observer le crash avec segmentation fault

**PoC**: [Lien vers code de démonstration]

**Impact**: Remote Code Execution (RCE) potentiel

**Suggestion**: Ajouter une limite de taille de message et valider la longueur
avant traitement.
```

## Processus de Réponse

### Timeline

1. **Accusé de réception** : Sous 48 heures
2. **Évaluation initiale** : Sous 5 jours ouvrables
3. **Développement du correctif** : Selon la gravité (voir ci-dessous)
4. **Publication du correctif** : Coordonnée avec le rapporteur
5. **Divulgation publique** : Après publication du correctif

### Gravité et Timeline de Correctif

| Gravité | Timeline de Correctif | Exemple |
|---------|----------------------|---------|
| **Critique** | 7 jours | RCE, injection SQL, authentication bypass |
| **Haute** | 30 jours | XSS, CSRF, information disclosure |
| **Moyenne** | 90 jours | DoS, path traversal |
| **Basse** | 180 jours | Information leak mineur, misconfiguration |

### Notification

Nous vous tiendrons informé :
- De la progression de notre investigation
- De nos plans pour corriger la vulnérabilité
- De la date de publication prévue
- Du crédit pour la découverte (si vous le souhaitez)

## Divulgation Coordonnée

Nous suivons le principe de **divulgation coordonnée** :

1. Nous travaillons avec vous pour comprendre et corriger le problème
2. Nous publions un correctif
3. Nous publions un advisory de sécurité
4. Vous pouvez publiquement discuter de la vulnérabilité après notre advisory

## Récompenses

Actuellement, nous n'offrons pas de programme de bug bounty formel. Cependant :

- ✅ **Crédit public** dans nos release notes et security advisories
- ✅ **Mention** dans notre fichier CONTRIBUTORS.md
- ✅ **Notre gratitude** et reconnaissance de la communauté

## Scope

### Dans le Scope

Les vulnérabilités affectant :
- **Core Rust library** (src/*)
- **Bindings multi-langages** (bindings/*)
- **CLI tool** (src/cli/*)
- **Dépendances critiques** avec impact direct

### Hors du Scope

- **Vulnérabilités dans les dépendances** déjà signalées en amont
- **Issues de configuration** documentées
- **Attaques DoS** requérant des ressources excessives
- **Attaques de social engineering**
- **Vulnérabilités dans les exemples/tests** non utilisés en production

## Meilleures Pratiques de Sécurité

### Pour les Utilisateurs

1. **Toujours utiliser la dernière version** avec les correctifs de sécurité
2. **Filtrer les données sensibles** des cassettes enregistrées
3. **Ne pas committer de cassettes** contenant des credentials
4. **Utiliser HTTPS** pour les connexions au proxy
5. **Limiter l'accès au proxy** (localhost uniquement par défaut)
6. **Auditer régulièrement** vos cassettes pour fuites de données

### Pour les Développeurs

1. **Input validation** : Valider toutes les entrées utilisateur
2. **Output encoding** : Encoder les sorties pour prévenir injection
3. **Dependency audits** : Utiliser `cargo audit` régulièrement
4. **Least privilege** : Minimiser les permissions requises
5. **Secure defaults** : Configuration sécurisée par défaut
6. **Tests de sécurité** : Inclure des tests pour vulnérabilités connues

## Dépendances de Sécurité

Nous utilisons plusieurs outils pour maintenir la sécurité :

- **cargo-audit** : Audit des dépendances Rust
- **Dependabot** : Mises à jour automatiques de dépendances
- **GitHub Security Advisories** : Notifications de vulnérabilités
- **Clippy** : Linting pour détecter les patterns dangereux

## Vulnérabilités Connues

### Actuelles

Aucune vulnérabilité connue pour le moment.

### Historique

Aucun advisory de sécurité publié pour le moment.

Consultez https://github.com/taciclei/magneto-serge/security/advisories pour la liste à jour.

## Sécurité du Proxy MITM

### Certificats TLS

Magneto-Serge génère un certificat CA auto-signé pour l'interception HTTPS (MITM). **Implications de sécurité** :

⚠️ **Le certificat CA doit être gardé privé**
- Ne pas committer le certificat dans Git
- Restreindre les permissions du fichier (chmod 600)
- Régénérer régulièrement

⚠️ **Installation du CA est requise**
- L'installation du CA dans le système expose à des attaques MITM
- Utiliser uniquement pour les tests/développement
- Ne JAMAIS installer le CA en production

⚠️ **Risques**
- Un attaquant avec accès au CA peut intercepter votre trafic HTTPS
- Le CA devrait être traité comme une clé privée sensible

### Recommandations

1. **Environnements de test isolés** uniquement
2. **Désinstaller le CA** après les tests
3. **Ne pas partager le CA** avec d'autres
4. **Régénérer le CA** périodiquement
5. **Utiliser un CA dédié** par projet/environnement

## Contact

Pour toute question de sécurité non urgente :
- **GitHub Discussions** : https://github.com/taciclei/magneto-serge/discussions
- **GitHub Issues** : https://github.com/taciclei/magneto-serge/issues (pour questions publiques uniquement)

---

## Références

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [GitHub Security Best Practices](https://docs.github.com/en/code-security)

---

**Dernière mise à jour** : 2025-10-12

🦀 Magneto-Serge - Built with Rust for maximum performance and safety
