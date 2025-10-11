#!/bin/bash
#
# Script de génération des bindings UniFFI pour Python, Kotlin et Swift
#
set -e

echo "🔗 Génération des bindings UniFFI pour matgto-serge"
echo ""

# Couleurs
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Vérifier que la bibliothèque est compilée
if [ ! -f "target/release/libmatgto_serge.dylib" ] && [ ! -f "target/release/libmatgto_serge.so" ]; then
    echo -e "${RED}❌ Erreur: Bibliothèque non trouvée${NC}"
    echo "Exécutez d'abord: cargo build --release --lib"
    exit 1
fi

echo -e "${GREEN}✅ Bibliothèque trouvée${NC}"
echo ""

# Détecter la bibliothèque dynamique
if [ -f "target/release/libmatgto_serge.dylib" ]; then
    LIB_FILE="target/release/libmatgto_serge.dylib"
elif [ -f "target/release/libmatgto_serge.so" ]; then
    LIB_FILE="target/release/libmatgto_serge.so"
else
    echo -e "${RED}❌ Bibliothèque introuvable${NC}"
    exit 1
fi

echo "📚 Bibliothèque: $LIB_FILE"
echo "📝 UDL: src/magneto_serge.udl"
echo ""

# Créer les répertoires de sortie
mkdir -p bindings/python
mkdir -p bindings/kotlin
mkdir -p bindings/swift

# Fonction pour générer les bindings
generate_bindings() {
    local lang=$1
    local out_dir="bindings/$lang"

    echo -e "${YELLOW}📦 Génération des bindings $lang...${NC}"

    # Utiliser Python pour générer les bindings avec uniffi
    python3 <<EOF
import subprocess
import sys

try:
    # Tenter d'utiliser uniffi-bindgen via Python
    result = subprocess.run([
        "python3", "-m", "uniffi_bindgen",
        "generate",
        "src/magneto_serge.udl",
        "--language", "$lang",
        "--out-dir", "$out_dir"
    ], capture_output=True, text=True)

    if result.returncode != 0:
        # Fallback: utiliser cargo run avec uniffi
        print("⚠️  uniffi-bindgen Python non disponible, utilisation de cargo...")
        result = subprocess.run([
            "cargo", "run", "--release", "--bin", "uniffi-bindgen", "--",
            "generate",
            "src/magneto_serge.udl",
            "--language", "$lang",
            "--out-dir", "$out_dir"
        ], capture_output=True, text=True)

    if result.returncode == 0:
        print("✅ Bindings $lang générés avec succès")
        sys.exit(0)
    else:
        print(f"❌ Erreur: {result.stderr}")
        sys.exit(1)

except Exception as e:
    print(f"❌ Exception: {e}")
    sys.exit(1)
EOF

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Bindings $lang générés dans $out_dir${NC}"
    else
        echo -e "${RED}❌ Échec de la génération des bindings $lang${NC}"
        echo ""
        echo "💡 Solution alternative:"
        echo "   Installez uniffi-bindgen manuellement et réessayez"
        return 1
    fi

    echo ""
}

# Méthode alternative: utiliser directement le module Rust
echo -e "${YELLOW}🦀 Méthode alternative: Génération via Rust...${NC}"
echo ""

# Créer un générateur Rust temporaire
cat > /tmp/gen_bindings.rs <<'RUST'
use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let udl_file = crate_dir.join("src/magneto_serge.udl");

    let languages = ["python", "kotlin", "swift"];

    for lang in &languages {
        let out_dir = crate_dir.join("bindings").join(lang);
        std::fs::create_dir_all(&out_dir).unwrap();

        println!("📦 Generating {} bindings...", lang);
        println!("   UDL: {}", udl_file.display());
        println!("   Out: {}", out_dir.display());

        // Utiliser uniffi_bindgen API directement
        // Note: Nécessite d'ajouter uniffi_bindgen aux dépendances
        match uniffi_bindgen::generate_bindings(
            &udl_file,
            None, // config
            &[lang.to_string()],
            &out_dir,
            false // try_format_code
        ) {
            Ok(_) => println!("✅ {} OK", lang),
            Err(e) => eprintln!("❌ {} Error: {}", lang, e),
        }
    }
}
RUST

echo -e "${YELLOW}📝 Info: Pour une génération automatique complète:${NC}"
echo ""
echo "Option 1: Installer uniffi-bindgen-cli (recommandé)"
echo "  pip install uniffi-bindgen"
echo ""
echo "Option 2: Utiliser directement depuis Python"
echo "  python3 -c 'from uniffi_bindgen import generate; generate(...)'"
echo ""
echo "Option 3: Génération manuelle avec build.rs (déjà configuré)"
echo "  Les bindings seront générés automatiquement lors de cargo build"
echo ""

echo -e "${GREEN}✨ Script terminé${NC}"
echo ""
echo "📂 Vérifiez les répertoires:"
echo "   - bindings/python/"
echo "   - bindings/kotlin/"
echo "   - bindings/swift/"
