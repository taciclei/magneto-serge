#!/usr/bin/env python3
"""
Exemple d'utilisation des bindings Python magneto-serge

Montre comment utiliser le proxy magneto-serge pour enregistrer et rejouer
des interactions HTTP/WebSocket.
"""

import sys
import os

# Ajouter le répertoire au path
sys.path.insert(0, os.path.dirname(__file__))

import magneto_serge

def example_record():
    """
    Exemple 1: Enregistrer des interactions HTTP
    """
    print("=" * 60)
    print("Exemple 1: Enregistrement d'interactions HTTP")
    print("=" * 60)

    # Créer un proxy en mode enregistrement
    proxy = magneto_serge.MagnetoProxy("./cassettes")
    proxy.set_port(8888)
    proxy.set_mode(magneto_serge.ProxyMode.RECORD)

    print(f"✓ Proxy créé sur le port {proxy.port()}")
    print(f"✓ Mode: {proxy.mode()}")

    # Démarrer l'enregistrement
    success = proxy.start_recording("example-api")
    print(f"✓ Enregistrement démarré: {success}")

    print("""
    Maintenant, configurez votre application pour utiliser le proxy:

    export HTTP_PROXY=http://localhost:8888
    export HTTPS_PROXY=http://localhost:8888

    Faites vos appels API, puis arrêtez l'enregistrement.
    """)

def example_replay():
    """
    Exemple 2: Rejouer des interactions depuis une cassette
    """
    print("=" * 60)
    print("Exemple 2: Rejeu d'interactions depuis cassette")
    print("=" * 60)

    # Créer un proxy en mode rejeu
    proxy = magneto_serge.MagnetoProxy("./cassettes")
    proxy.set_port(8888)
    proxy.set_mode(magneto_serge.ProxyMode.REPLAY)

    print(f"✓ Proxy créé sur le port {proxy.port()}")
    print(f"✓ Mode: {proxy.mode()}")

    # Charger et rejouer une cassette
    success = proxy.replay("example-api")
    print(f"✓ Cassette chargée: {success}")

    print("""
    Les requêtes HTTP seront maintenant servies depuis la cassette,
    sans faire d'appels réseau réels.
    """)

def example_hybrid():
    """
    Exemple 3: Mode hybride (auto)
    """
    print("=" * 60)
    print("Exemple 3: Mode hybride (auto)")
    print("=" * 60)

    # Mode hybride: enregistre si la cassette n'existe pas, rejoue sinon
    proxy = magneto_serge.MagnetoProxy("./cassettes")
    proxy.set_port(8888)

    success = proxy.hybrid("example-api")
    print(f"✓ Mode hybride activé: {success}")

    print("""
    En mode hybride:
    - Si la cassette existe: rejeu automatique
    - Si la cassette n'existe pas: enregistrement automatique

    Parfait pour les tests CI/CD !
    """)

def example_replay_strict():
    """
    Exemple 4: Rejeu strict
    """
    print("=" * 60)
    print("Exemple 4: Rejeu strict (pour tests)")
    print("=" * 60)

    # Mode strict: échoue si une requête ne correspond à aucune interaction
    proxy = magneto_serge.MagnetoProxy("./cassettes")
    proxy.set_port(8888)

    success = proxy.replay_strict("example-api")
    print(f"✓ Mode strict activé: {success}")

    print("""
    En mode strict:
    - Toutes les requêtes doivent avoir une correspondance exacte
    - Aucune requête supplémentaire tolérée
    - Idéal pour les tests d'intégration rigoureux
    """)

def example_once():
    """
    Exemple 5: Rejeu unique (one-shot)
    """
    print("=" * 60)
    print("Exemple 5: Rejeu unique (one-shot)")
    print("=" * 60)

    # Mode once: chaque interaction ne peut être rejouée qu'une fois
    proxy = magneto_serge.MagnetoProxy("./cassettes")
    proxy.set_port(8888)

    success = proxy.once("example-api")
    print(f"✓ Mode once activé: {success}")

    print("""
    En mode once:
    - Chaque interaction enregistrée ne peut être utilisée qu'une fois
    - Détecte les requêtes dupliquées ou en boucle
    - Utile pour tester les idempotences
    """)

def main():
    """
    Fonction principale - exécute tous les exemples
    """
    print("\n" + "=" * 60)
    print("🧪 Exemples d'utilisation de magneto-serge Python")
    print("=" * 60 + "\n")

    # Exemple 1: Enregistrement
    example_record()
    print()

    # Exemple 2: Rejeu
    example_replay()
    print()

    # Exemple 3: Mode hybride
    example_hybrid()
    print()

    # Exemple 4: Rejeu strict
    example_replay_strict()
    print()

    # Exemple 5: Rejeu unique
    example_once()
    print()

    print("=" * 60)
    print("✅ Tous les exemples ont été exécutés avec succès!")
    print("=" * 60)

    print("""
    Pour aller plus loin:

    1. Documentation: https://github.com/taciclei/magneto-serge
    2. Tests: python3 test_magneto_bindings.py
    3. CLI: magneto --help
    """)

if __name__ == "__main__":
    main()
