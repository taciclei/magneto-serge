# Release Guide

This guide explains how to create a new release of Magneto-Serge with pre-built binaries and Homebrew formula updates.

## Prerequisites

- [ ] All features merged to `develop`
- [ ] All tests passing on `develop`
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Documentation updated

## Release Process

### 1. Prepare Release Branch

```bash
# Checkout develop
git checkout develop
git pull origin develop

# Create release branch
VERSION="0.3.0"  # Update this
git checkout -b release/v${VERSION}

# Update version in Cargo.toml
sed -i '' "s/version = \".*\"/version = \"${VERSION}\"/" Cargo.toml

# Update CHANGELOG.md
# Edit CHANGELOG.md manually with release notes

# Commit version bump
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to v${VERSION}"
git push origin release/v${VERSION}
```

### 2. Create Pull Request to Main

```bash
# Create PR to main
gh pr create \
  --base main \
  --head release/v${VERSION} \
  --title "Release v${VERSION}" \
  --body "$(cat <<EOF
## Release v${VERSION}

### Changes
- See [CHANGELOG.md](CHANGELOG.md#v${VERSION//.})

### Checklist
- [x] Version bumped in Cargo.toml
- [x] CHANGELOG.md updated
- [x] Tests passing
- [x] Documentation updated

### Post-Merge Actions
After merging:
1. Tag will be created: v${VERSION}
2. Release workflow will build binaries
3. GitHub Release will be created
4. Homebrew formula will be updated
5. Merge main back to develop
EOF
)"
```

### 3. Merge and Tag

```bash
# After PR approval and merge
git checkout main
git pull origin main

# Create and push tag
git tag -a v${VERSION} -m "Release v${VERSION}"
git push origin v${VERSION}
```

### 4. Automated Release Workflow

The GitHub Actions workflow `.github/workflows/release.yml` will automatically:

1. **Build binaries** for all platforms:
   - macOS Intel (`x86_64-apple-darwin`)
   - macOS Apple Silicon (`aarch64-apple-darwin`)
   - Linux x86_64 (`x86_64-unknown-linux-gnu`)
   - Linux ARM64 (`aarch64-unknown-linux-gnu`)
   - Windows x86_64 (`x86_64-pc-windows-msvc`)

2. **Create archives**:
   - `.tar.gz` for Unix-like systems
   - `.zip` for Windows

3. **Generate checksums**:
   - SHA256 for each archive

4. **Create GitHub Release**:
   - Title: "Magneto-Serge vX.Y.Z"
   - Assets: All binaries + checksums
   - Release notes: Auto-generated

5. **Update Homebrew formula**:
   - Replace SHA256 placeholders
   - Update version number
   - Commit and push to repository

### 5. Verify Release

```bash
# Check GitHub Release
gh release view v${VERSION}

# Download and test binary
PLATFORM="macos-arm64"  # Or: macos-amd64, linux-amd64, etc.
curl -LO "https://github.com/taciclei/magneto-serge/releases/download/v${VERSION}/magneto-${PLATFORM}.tar.gz"
tar xzf magneto-${PLATFORM}.tar.gz
./magneto --version
./magneto init
```

### 6. Test Homebrew Installation

```bash
# Update Homebrew
brew update

# Test installation
brew install taciclei/tap/magneto-serge

# Verify
magneto --version

# Cleanup
brew uninstall magneto-serge
```

### 7. Merge Main Back to Develop

```bash
git checkout develop
git merge main
git push origin develop
```

## Manual Release (Fallback)

If the automated workflow fails, you can manually create a release:

### Build Binaries Locally

```bash
# macOS ARM64 (M1/M2/M3)
cargo build --release --bin magneto --features cli --target aarch64-apple-darwin

# macOS Intel
cargo build --release --bin magneto --features cli --target x86_64-apple-darwin

# Linux x86_64
cargo build --release --bin magneto --features cli --target x86_64-unknown-linux-gnu
```

### Create Archives

```bash
# macOS ARM64
cd target/aarch64-apple-darwin/release
tar czf magneto-macos-arm64.tar.gz magneto
shasum -a 256 magneto-macos-arm64.tar.gz > magneto-macos-arm64.tar.gz.sha256

# Repeat for other platforms...
```

### Create GitHub Release Manually

```bash
# Create release
gh release create v${VERSION} \
  --title "Magneto-Serge v${VERSION}" \
  --notes "See CHANGELOG.md for details" \
  magneto-macos-arm64.tar.gz \
  magneto-macos-arm64.tar.gz.sha256 \
  # ... add all other binaries
```

### Update Homebrew Formula Manually

```bash
# Get SHA256 for macOS binaries
ARM64_SHA=$(shasum -a 256 magneto-macos-arm64.tar.gz | awk '{print $1}')
X86_64_SHA=$(shasum -a 256 magneto-macos-amd64.tar.gz | awk '{print $1}')

# Update Formula/magneto-serge.rb
sed -i '' "s/PLACEHOLDER_ARM64_SHA256/${ARM64_SHA}/" Formula/magneto-serge.rb
sed -i '' "s/PLACEHOLDER_X86_64_SHA256/${X86_64_SHA}/" Formula/magneto-serge.rb
sed -i '' "s/version \".*\"/version \"${VERSION}\"/" Formula/magneto-serge.rb

# Commit and push
git add Formula/magneto-serge.rb
git commit -m "chore: update Homebrew formula to v${VERSION}"
git push origin main
```

## Troubleshooting

### Workflow Fails on Cross-Compilation

If ARM64 Linux build fails:

```bash
# Install cross-compilation tools
sudo apt-get install gcc-aarch64-linux-gnu

# Build with cargo
cargo build --release --target aarch64-unknown-linux-gnu
```

### SHA256 Mismatch in Homebrew

If users report SHA256 mismatch:

1. Download the binary from GitHub Releases
2. Verify SHA256: `shasum -a 256 magneto-macos-arm64.tar.gz`
3. Update formula with correct SHA256
4. Commit and push

### Binary Not Executable

If binary doesn't run on macOS:

```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine magneto

# Make executable
chmod +x magneto
```

## Post-Release Checklist

- [ ] GitHub Release created with all binaries
- [ ] Homebrew formula updated with correct SHA256s
- [ ] Installation tested via Homebrew
- [ ] Installation tested via curl script
- [ ] Documentation updated (if needed)
- [ ] Announcement posted (if major release)
- [ ] Social media update (optional)

## Version Numbering (SemVer)

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.2.0): New features, backward compatible
- **PATCH** (0.2.1): Bug fixes, backward compatible

Current: **v0.2.0**

## Next Release

Planned: **v0.3.0** (date TBD)

Features:
- Advanced matching strategies (regex, JSON path)
- WebSocket replay with timing
- Additional language bindings (Python, Kotlin, Swift)

See [ROADMAP.md](ROADMAP.md) for details.

---

*Last updated: 2025-10-25*
