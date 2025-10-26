#!/bin/bash
# Magneto-Serge installation script
# Usage: curl -sSL https://raw.githubusercontent.com/taciclei/magneto-serge/main/install.sh | bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧲 Magneto-Serge Installer${NC}"
echo ""

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    OS_TYPE="linux"
    ;;
  Darwin)
    OS_TYPE="macos"
    ;;
  *)
    echo -e "${RED}❌ Unsupported operating system: $OS${NC}"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)
    ARCH_TYPE="amd64"
    ;;
  aarch64|arm64)
    ARCH_TYPE="arm64"
    ;;
  *)
    echo -e "${RED}❌ Unsupported architecture: $ARCH${NC}"
    exit 1
    ;;
esac

echo -e "${GREEN}✓${NC} Detected: $OS_TYPE-$ARCH_TYPE"

# Get latest release version
echo ""
echo -e "${BLUE}📥 Fetching latest release...${NC}"
VERSION=$(curl -s https://api.github.com/repos/taciclei/magneto-serge/releases/latest | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
  echo -e "${RED}❌ Failed to fetch latest version${NC}"
  exit 1
fi

echo -e "${GREEN}✓${NC} Latest version: v$VERSION"

# Construct download URL
BINARY_NAME="magneto-$OS_TYPE-$ARCH_TYPE"
if [ "$OS_TYPE" = "linux" ]; then
  ARCHIVE_NAME="${BINARY_NAME}.tar.gz"
  DOWNLOAD_URL="https://github.com/taciclei/magneto-serge/releases/download/v${VERSION}/${ARCHIVE_NAME}"
else
  ARCHIVE_NAME="${BINARY_NAME}.tar.gz"
  DOWNLOAD_URL="https://github.com/taciclei/magneto-serge/releases/download/v${VERSION}/${ARCHIVE_NAME}"
fi

# Create temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download
echo ""
echo -e "${BLUE}⬇️  Downloading from:${NC}"
echo "  $DOWNLOAD_URL"
curl -L -o "$ARCHIVE_NAME" "$DOWNLOAD_URL"

if [ ! -f "$ARCHIVE_NAME" ]; then
  echo -e "${RED}❌ Download failed${NC}"
  exit 1
fi

echo -e "${GREEN}✓${NC} Downloaded"

# Extract
echo ""
echo -e "${BLUE}📦 Extracting...${NC}"
if [ "$OS_TYPE" = "linux" ]; then
  tar xzf "$ARCHIVE_NAME"
else
  tar xzf "$ARCHIVE_NAME"
fi

# Install
echo ""
echo -e "${BLUE}📥 Installing...${NC}"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

if [ -f "magneto" ]; then
  chmod +x magneto
  mv magneto "$INSTALL_DIR/"
  echo -e "${GREEN}✓${NC} Installed to: $INSTALL_DIR/magneto"
else
  echo -e "${RED}❌ Binary not found in archive${NC}"
  exit 1
fi

# Cleanup
cd /
rm -rf "$TMP_DIR"

# Check if in PATH
echo ""
if echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo -e "${GREEN}✓${NC} $INSTALL_DIR is in your PATH"
else
  echo -e "${YELLOW}⚠️  $INSTALL_DIR is NOT in your PATH${NC}"
  echo ""
  echo "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
  echo ""
  echo -e "  ${BLUE}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
  echo ""
fi

# Success
echo ""
echo -e "${GREEN}✅ Installation complete!${NC}"
echo ""
echo "Try it out:"
echo ""
echo -e "  ${BLUE}magneto --version${NC}"
echo -e "  ${BLUE}magneto init${NC}"
echo ""
echo "Documentation: https://github.com/taciclei/magneto-serge"
echo ""
