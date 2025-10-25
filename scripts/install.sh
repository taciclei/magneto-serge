#!/bin/bash

# Magneto-Serge Installation Script
# Version: 2.2.0
# Description: Complete installation and setup for Magneto-Serge

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_header() {
    echo ""
    echo -e "${BLUE}╔═══════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║   Magneto-Serge v2.2.0 Installation Script   ║${NC}"
    echo -e "${BLUE}╚═══════════════════════════════════════════════╝${NC}"
    echo ""
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to check Rust installation
check_rust() {
    print_info "Checking Rust installation..."

    if ! command_exists rustc; then
        print_warning "Rust not found. Installing rustup..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        print_success "Rust installed successfully"
    else
        local rust_version=$(rustc --version | awk '{print $2}')
        print_success "Rust $rust_version found"

        # Check if version is >= 1.75
        local required_version="1.75.0"
        if ! printf '%s\n%s\n' "$required_version" "$rust_version" | sort -V -C; then
            print_warning "Rust version $rust_version is older than required $required_version"
            print_info "Updating Rust..."
            rustup update
        fi
    fi
}

# Function to check Git installation
check_git() {
    print_info "Checking Git installation..."

    if ! command_exists git; then
        print_error "Git not found. Please install Git first."
        exit 1
    else
        local git_version=$(git --version | awk '{print $3}')
        print_success "Git $git_version found"
    fi
}

# Function to check optional dependencies
check_optional_deps() {
    print_info "Checking optional dependencies..."

    local deps_found=0

    # Check Python
    if command_exists python3; then
        local python_version=$(python3 --version | awk '{print $2}')
        print_success "Python $python_version found (for Python bindings)"
        deps_found=$((deps_found + 1))
    else
        print_warning "Python 3 not found (optional - needed for Python bindings)"
    fi

    # Check Node.js
    if command_exists node; then
        local node_version=$(node --version)
        print_success "Node.js $node_version found (for JavaScript bindings)"
        deps_found=$((deps_found + 1))
    else
        print_warning "Node.js not found (optional - needed for JavaScript bindings)"
    fi

    # Check Java
    if command_exists java; then
        local java_version=$(java -version 2>&1 | head -n 1 | awk -F '"' '{print $2}')
        print_success "Java $java_version found (for Java/Kotlin bindings)"
        deps_found=$((deps_found + 1))
    else
        print_warning "Java not found (optional - needed for Java/Kotlin bindings)"
    fi

    # Check Docker
    if command_exists docker; then
        local docker_version=$(docker --version | awk '{print $3}' | sed 's/,//')
        print_success "Docker $docker_version found (for containerized deployment)"
        deps_found=$((deps_found + 1))
    else
        print_warning "Docker not found (optional - needed for containerized deployment)"
    fi

    echo ""
    print_info "Found $deps_found/4 optional dependencies"
}

# Function to build the project
build_project() {
    print_info "Building Magneto-Serge..."

    # Check dependencies first
    if [ -x "./scripts/check-deps.sh" ]; then
        ./scripts/check-deps.sh
    fi

    # Build in release mode
    cargo build --release --all-features

    print_success "Build completed successfully"
}

# Function to run tests
run_tests() {
    print_info "Running tests..."

    cargo test --all-features

    print_success "All tests passed"
}

# Function to install git hooks
install_git_hooks() {
    print_info "Installing Git hooks..."

    if [ -x "./scripts/install-git-hooks.sh" ]; then
        ./scripts/install-git-hooks.sh
        print_success "Git hooks installed"
    else
        print_warning "Git hooks script not found"
    fi
}

# Function to create necessary directories
create_directories() {
    print_info "Creating necessary directories..."

    mkdir -p cassettes
    mkdir -p logs
    mkdir -p config

    print_success "Directories created"
}

# Function to install the CLI binary
install_cli() {
    print_info "Installing magneto CLI binary..."

    # Check if binary exists
    if [ ! -f "target/release/magneto" ]; then
        print_error "Binary not found. Please build the project first."
        return 1
    fi

    # Ask user where to install
    local install_dir="${HOME}/.local/bin"

    # Create install directory if it doesn't exist
    mkdir -p "$install_dir"

    # Copy binary
    cp target/release/magneto "$install_dir/"
    chmod +x "$install_dir/magneto"

    # Check if install_dir is in PATH
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        print_warning "$install_dir is not in your PATH"
        print_info "Add this line to your ~/.bashrc or ~/.zshrc:"
        echo ""
        echo "  export PATH=\"\$PATH:$install_dir\""
        echo ""
    else
        print_success "magneto CLI installed to $install_dir"
    fi
}

# Function to generate CA certificate
generate_ca_cert() {
    print_info "Generating MITM CA certificate..."

    if [ -f "magneto-ca.pem" ]; then
        print_warning "CA certificate already exists. Skipping generation."
        return 0
    fi

    # Run magneto init to generate certificate
    if command_exists magneto; then
        magneto init
        print_success "CA certificate generated"
    else
        print_warning "magneto CLI not found. Certificate will be generated on first run."
    fi
}

# Function to install CA certificate (macOS only)
install_ca_cert_macos() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        print_info "Would you like to install the CA certificate to macOS trust store?"
        read -p "This requires sudo access (y/n): " -n 1 -r
        echo

        if [[ $REPLY =~ ^[Yy]$ ]]; then
            if [ -f "magneto-ca.pem" ]; then
                sudo security add-trusted-cert -d -r trustRoot \
                    -k /Library/Keychains/System.keychain magneto-ca.pem
                print_success "CA certificate installed to macOS trust store"
            else
                print_warning "CA certificate not found. Run 'magneto init' first."
            fi
        fi
    fi
}

# Function to show installation summary
show_summary() {
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║          Installation Complete! 🎉            ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
    echo ""

    print_info "Next steps:"
    echo ""
    echo "  1. Test the CLI:"
    echo "     $ magneto --version"
    echo ""
    echo "  2. Initialize configuration:"
    echo "     $ magneto init"
    echo ""
    echo "  3. Record your first cassette:"
    echo "     $ magneto record my-first-test"
    echo ""
    echo "  4. Read the quickstart guide:"
    echo "     $ cat QUICKSTART.md"
    echo ""
    echo "  5. Check the examples:"
    echo "     $ ls examples/"
    echo ""

    print_info "Useful resources:"
    echo "  - Documentation: README.md"
    echo "  - Architecture: docs/ARCHITECTURE.md"
    echo "  - Contributing: CONTRIBUTING.md"
    echo "  - Changelog: CHANGELOG.md"
    echo ""
}

# Main installation flow
main() {
    print_header

    # Parse command line arguments
    SKIP_TESTS=false
    INSTALL_CLI=true
    QUICK_MODE=false

    for arg in "$@"; do
        case $arg in
            --skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            --no-cli)
                INSTALL_CLI=false
                shift
                ;;
            --quick)
                QUICK_MODE=true
                shift
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --skip-tests    Skip running tests"
                echo "  --no-cli        Don't install CLI binary"
                echo "  --quick         Quick mode (skip optional steps)"
                echo "  --help          Show this help message"
                echo ""
                exit 0
                ;;
        esac
    done

    # Step 1: Check dependencies
    check_git
    check_rust

    if [ "$QUICK_MODE" = false ]; then
        check_optional_deps
    fi

    # Step 2: Create directories
    create_directories

    # Step 3: Build project
    build_project

    # Step 4: Run tests (optional)
    if [ "$SKIP_TESTS" = false ]; then
        run_tests
    else
        print_warning "Skipping tests (--skip-tests flag)"
    fi

    # Step 5: Install Git hooks
    if [ "$QUICK_MODE" = false ]; then
        install_git_hooks
    fi

    # Step 6: Install CLI binary
    if [ "$INSTALL_CLI" = true ]; then
        install_cli
    else
        print_warning "Skipping CLI installation (--no-cli flag)"
    fi

    # Step 7: Generate CA certificate (optional)
    if [ "$QUICK_MODE" = false ]; then
        generate_ca_cert
        install_ca_cert_macos
    fi

    # Step 8: Show summary
    show_summary
}

# Run main function
main "$@"
