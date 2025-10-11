#!/bin/bash
# generate-python-bindings.sh
# Script pour générer les bindings Python magneto-serge avec UniFFI

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BINDINGS_DIR="$PROJECT_ROOT/bindings/python"

echo "========================================"
echo "  Génération Bindings Python UniFFI"
echo "========================================"

# Étape 1: Compiler uniffi-bindgen si nécessaire
echo "Étape 1: Compilation de uniffi-bindgen..."
cargo build --bin uniffi-bindgen --features uniffi/cli
echo "✓ uniffi-bindgen compilé"

# Étape 2: Compiler la bibliothèque Rust en mode release
echo ""
echo "Étape 2: Compilation de la bibliothèque magneto_serge..."
cargo build --lib --release
echo "✓ Bibliothèque compilée"

# Étape 3: Générer les bindings Python depuis le fichier UDL
echo ""
echo "Étape 3: Génération des bindings Python..."
"$PROJECT_ROOT/target/debug/uniffi-bindgen" generate \
    "$PROJECT_ROOT/src/magneto_serge.udl" \
    --language python \
    --out-dir "$BINDINGS_DIR"
echo "✓ Fichier magneto_serge.py généré"

# Étape 4: Copier la bibliothèque partagée
echo ""
echo "Étape 4: Copie de la bibliothèque partagée..."

# Déterminer l'extension selon l'OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    LIB_EXT="dylib"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    LIB_EXT="so"
else
    echo "❌ OS non supporté: $OSTYPE"
    exit 1
fi

cp "$PROJECT_ROOT/target/release/libmagneto_serge.$LIB_EXT" \
   "$BINDINGS_DIR/libuniffi_magneto_serge.$LIB_EXT"
echo "✓ Bibliothèque copiée: libuniffi_magneto_serge.$LIB_EXT"

# Étape 5: Vérifier l'import Python
echo ""
echo "Étape 5: Vérification de l'import Python..."
cd "$BINDINGS_DIR"
python3 -c "import magneto_serge; print(f'✓ Module importé: {magneto_serge.__name__}')"

# Étape 6: Afficher les classes disponibles
echo ""
echo "Étape 6: Classes Python disponibles:"
python3 -c "
import magneto_serge
import inspect

for name, obj in inspect.getmembers(magneto_serge):
    if inspect.isclass(obj) and not name.startswith('_'):
        print(f'  - {name}')
"

echo ""
echo "========================================"
echo "✅ Bindings Python générés avec succès!"
echo "========================================"
echo ""
echo "Fichiers générés:"
echo "  - $BINDINGS_DIR/magneto_serge.py"
echo "  - $BINDINGS_DIR/libuniffi_magneto_serge.$LIB_EXT"
echo ""
echo "Tests disponibles:"
echo "  python3 $BINDINGS_DIR/test_magneto_bindings.py"
echo "  python3 $BINDINGS_DIR/example_magneto.py"
echo ""
