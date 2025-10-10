#!/usr/bin/env python3
"""
Test des bindings Python UniFFI pour matgto-serge
"""

import sys
import os

# Ajouter le chemin de la bibliothèque
sys.path.insert(0, os.path.dirname(__file__))

def test_import():
    """Test 1: Importer le module"""
    print("Test 1: Import du module matgto_serge...")
    try:
        import matgto_serge
        print("✅ Module importé avec succès")
        return True
    except Exception as e:
        print(f"❌ Erreur d'import: {e}")
        return False

def test_create_proxy():
    """Test 2: Créer une instance de proxy"""
    print("\nTest 2: Création d'un proxy...")
    try:
        from matgto_serge import create_proxy, MatgtoProxy

        # Créer un proxy
        proxy = create_proxy("./test_cassettes")

        if proxy is None:
            print("❌ create_proxy() a retourné None")
            return False

        print(f"✅ Proxy créé: {type(proxy)}")
        return True
    except Exception as e:
        print(f"❌ Erreur: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_proxy_methods():
    """Test 3: Tester les méthodes du proxy"""
    print("\nTest 3: Test des méthodes du proxy...")
    try:
        from matgto_serge import create_proxy, ProxyMode

        proxy = create_proxy("./test_cassettes")
        if proxy is None:
            print("❌ Impossible de créer le proxy")
            return False

        # Test set_port
        proxy.set_port(9999)
        print(f"  Port configuré: {proxy.port()}")
        assert proxy.port() == 9999, "Port incorrect"

        # Test set_mode
        proxy.set_mode(ProxyMode.RECORD)
        print(f"  Mode configuré: {proxy.mode()}")
        assert proxy.mode() == ProxyMode.RECORD, "Mode incorrect"

        # Test version
        from matgto_serge import version
        v = version()
        print(f"  Version: {v}")

        print("✅ Toutes les méthodes fonctionnent")
        return True

    except Exception as e:
        print(f"❌ Erreur: {e}")
        import traceback
        traceback.print_exc()
        return False

def test_recording():
    """Test 4: Tester l'enregistrement"""
    print("\nTest 4: Test de l'enregistrement...")
    try:
        from matgto_serge import create_proxy, ProxyMode

        proxy = create_proxy("./test_cassettes")
        if proxy is None:
            print("❌ Impossible de créer le proxy")
            return False

        # Configurer le proxy
        proxy.set_port(8888)
        proxy.set_mode(ProxyMode.RECORD)

        # Démarrer l'enregistrement
        result = proxy.start_recording("test_session")
        print(f"  start_recording() retourné: {result}")

        # Arrêter l'enregistrement
        result = proxy.stop_recording()
        print(f"  stop_recording() retourné: {result}")

        print("✅ API d'enregistrement fonctionne")
        return True

    except Exception as e:
        print(f"❌ Erreur: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    """Exécuter tous les tests"""
    print("="*60)
    print("🧪 Tests des bindings Python matgto-serge")
    print("="*60)

    tests = [
        test_import,
        test_create_proxy,
        test_proxy_methods,
        test_recording,
    ]

    results = []
    for test_func in tests:
        result = test_func()
        results.append(result)

    print("\n" + "="*60)
    print("📊 Résultats:")
    print("="*60)

    total = len(results)
    passed = sum(results)
    failed = total - passed

    print(f"Total: {total} tests")
    print(f"✅ Réussis: {passed}")
    print(f"❌ Échoués: {failed}")

    if failed == 0:
        print("\n🎉 Tous les tests sont passés!")
        return 0
    else:
        print(f"\n⚠️  {failed} test(s) ont échoué")
        return 1

if __name__ == "__main__":
    sys.exit(main())
