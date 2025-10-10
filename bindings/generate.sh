#!/usr/bin/env bash
# Generate UniFFI bindings for all target languages

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Building matgto-serge library...${NC}"
cargo build --release

echo -e "${BLUE}📦 Generating UniFFI bindings...${NC}"

# Get the library path
LIB_PATH="../target/release/libmatgto_serge.dylib"  # macOS
if [ ! -f "$LIB_PATH" ]; then
    LIB_PATH="../target/release/libmatgto_serge.so"  # Linux
fi
if [ ! -f "$LIB_PATH" ]; then
    LIB_PATH="../target/release/matgto_serge.dll"  # Windows
fi

UDL_FILE="../src/magneto_serge.udl"

# Generate Python bindings
echo -e "${GREEN}🐍 Generating Python bindings...${NC}"
cargo run --features=uniffi/cli --bin uniffi-bindgen generate \
    --library "$LIB_PATH" \
    --language python \
    --out-dir python/ \
    "$UDL_FILE" || echo "Note: uniffi-bindgen CLI may need to be installed separately"

# Generate Kotlin bindings
echo -e "${GREEN}🤖 Generating Kotlin bindings...${NC}"
cargo run --features=uniffi/cli --bin uniffi-bindgen generate \
    --library "$LIB_PATH" \
    --language kotlin \
    --out-dir kotlin/ \
    "$UDL_FILE" || echo "Note: uniffi-bindgen CLI may need to be installed separately"

# Generate Swift bindings
echo -e "${GREEN}🍎 Generating Swift bindings...${NC}"
cargo run --features=uniffi/cli --bin uniffi-bindgen generate \
    --library "$LIB_PATH" \
    --language swift \
    --out-dir swift/ \
    "$UDL_FILE" || echo "Note: uniffi-bindgen CLI may need to be installed separately"

echo -e "${GREEN}✅ Bindings generation complete!${NC}"
echo ""
echo "Generated bindings in:"
echo "  - python/"
echo "  - kotlin/"
echo "  - swift/"
