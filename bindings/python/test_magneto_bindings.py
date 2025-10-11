#!/usr/bin/env python3
"""
Test des bindings Python magneto-serge générés avec UniFFI
"""

import sys
import os

# Ajouter le répertoire au path
sys.path.insert(0, os.path.dirname(__file__))

def test_import():
    """Test 1: Import du module magneto_serge"""
    print("Test 1: Import du module magneto_serge...")
    try:
        import magneto_serge
        print(f"✅ Module importé avec succès")
        return magneto_serge
    except Exception as e:
        print(f"❌ Erreur d'import: {e}")
        return None

def test_proxy_creation(magneto):
    """Test 2: Création d'un proxy"""
    print("\nTest 2: Création d'un proxy...")
    try:
        proxy = magneto.MagnetoProxy("./test_cassettes")
        print(f"✅ Proxy créé: {type(proxy)}")
        return proxy
    except Exception as e:
        print(f"❌ Erreur de création: {e}")
        return None

def test_proxy_methods(proxy):
    """Test 3: Test des méthodes du proxy"""
    print("\nTest 3: Test des méthodes du proxy...")
    try:
        # Test port
        proxy.set_port(9999)
        port = proxy.port()
        print(f"  Port configuré: {port}")

        # Test mode
        import magneto_serge
        proxy.set_mode(magneto_serge.ProxyMode.RECORD)
        mode = proxy.mode()
        print(f"  Mode configuré: {mode}")

        print("✅ Toutes les méthodes fonctionnent")
        return True
    except Exception as e:
        print(f"❌ Erreur des méthodes: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_recording(proxy):
    """Test 4: Test de l'enregistrement"""
    print("\nTest 4: Test de l'enregistrement...")
    try:
        result = proxy.start_recording("test-cassette")
        print(f"  start_recording() retourné: {result}")

        # Note: stop_recording n'existe peut-être pas dans les nouveaux bindings
        print("✅ API d'enregistrement fonctionne")
        return True
    except Exception as e:
        print(f"❌ Erreur d'enregistrement: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    print("=" * 60)
    print("🧪 Tests des bindings Python magneto-serge (UniFFI)")
    print("=" * 60)

    # Test 1: Import
    magneto = test_import()
    if not magneto:
        return 1

    # Test 2: Création proxy
    proxy = test_proxy_creation(magneto)
    if not proxy:
        return 1

    # Test 3: Méthodes
    if not test_proxy_methods(proxy):
        return 1

    # Test 4: Enregistrement
    if not test_recording(proxy):
        return 1

    print("\n" + "=" * 60)
    print("🎉 Tous les tests sont passés!")
    print("=" * 60)
    return 0

if __name__ == "__main__":
    sys.exit(main())
