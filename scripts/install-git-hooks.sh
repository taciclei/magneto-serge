#!/bin/bash
# Install Git hooks for magneto-serge project
# Run this script from the project root: ./scripts/install-git-hooks.sh

set -e

echo "🔧 Installing Git hooks for magneto-serge..."

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Install pre-push hook
echo "📝 Installing pre-push hook (runs linter before push)..."
cat > .git/hooks/pre-push << 'EOF'
#!/bin/sh
# Git pre-push hook that runs Rust linting
# This ensures code quality before pushing to remote

echo "🔍 Running Rust linting before push..."

# Run cargo fmt --check (check formatting)
echo "  📝 Checking code formatting (cargo fmt --check)..."
if ! cargo fmt --check; then
    echo "❌ Code formatting check failed!"
    echo "💡 Run 'cargo fmt' to fix formatting issues"
    exit 1
fi
echo "  ✅ Code formatting OK"

# Run cargo clippy (linting)
echo "  🔧 Running clippy linter (cargo clippy --all-features --all-targets -- -D warnings)..."
if ! cargo clippy --all-features --all-targets -- -D warnings; then
    echo "❌ Clippy linting failed!"
    echo "💡 Fix the warnings and try again"
    exit 1
fi
echo "  ✅ Clippy linting OK"

echo "✅ All linting checks passed! Proceeding with push..."
exit 0
EOF

# Make hook executable
chmod +x .git/hooks/pre-push

echo "✅ Git hooks installed successfully!"
echo ""
echo "Installed hooks:"
echo "  - pre-push: Runs cargo fmt --check and cargo clippy before pushing"
echo ""
echo "To skip the pre-push hook temporarily, use: git push --no-verify"
