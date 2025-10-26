# Magneto-Serge Homebrew Formula

This directory contains the Homebrew formula for installing Magneto-Serge on macOS and Linux.

## Installation

### Option 1: Via Homebrew Tap (Recommended)

```bash
# Add the tap
brew tap taciclei/tap https://github.com/taciclei/magneto-serge

# Install
brew install magneto-serge
```

### Option 2: Direct Formula

```bash
brew install https://raw.githubusercontent.com/taciclei/magneto-serge/main/Formula/magneto-serge.rb
```

### Option 3: From Source

```bash
# Clone the repository
git clone https://github.com/taciclei/magneto-serge.git
cd magneto-serge

# Install with Homebrew
brew install --build-from-source Formula/magneto-serge.rb
```

## Usage

After installation:

```bash
# Initialize configuration
magneto init

# Record interactions
magneto record my-test --port 8888

# List cassettes
magneto list

# Replay
magneto replay my-test --port 8888
```

## HTTPS Interception Setup

For HTTPS proxying, you need to trust the CA certificate:

```bash
# macOS
security add-trusted-cert -d -r trustRoot \
  -k ~/Library/Keychains/login.keychain \
  ~/.magneto/ca/magneto-ca.pem

# Linux
sudo cp ~/.magneto/ca/magneto-ca.pem /usr/local/share/ca-certificates/
sudo update-ca-certificates
```

## Updating

```bash
brew update
brew upgrade magneto-serge
```

## Uninstalling

```bash
brew uninstall magneto-serge
brew untap taciclei/tap  # Optional: remove tap
```

## Development

To test the formula locally:

```bash
# Audit formula
brew audit --strict Formula/magneto-serge.rb

# Test installation
brew install --build-from-source Formula/magneto-serge.rb

# Run tests
brew test magneto-serge
```

## Troubleshooting

### Formula not found

If `brew install magneto-serge` fails:

1. Ensure the tap is added: `brew tap taciclei/tap`
2. Update Homebrew: `brew update`
3. Try the direct formula URL

### Binary not running

If the binary doesn't run after installation:

```bash
# Check installation
which magneto
magneto --version

# Verify formula
brew info magneto-serge
```

### Permission issues

On macOS, if you get a security warning:

```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine $(which magneto)
```

## Links

- **Homepage**: https://github.com/taciclei/magneto-serge
- **Issues**: https://github.com/taciclei/magneto-serge/issues
- **Releases**: https://github.com/taciclei/magneto-serge/releases
