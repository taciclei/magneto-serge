# 📦 Publishing @magneto/serge to NPM

## Prerequisites

### 1. NPM Account
```bash
# Create account at https://www.npmjs.com/signup
# Or login
npm login
```

### 2. Package Scope
```bash
# Request access to @magneto scope (if needed)
# Or use your own scope: @your-username/serge
```

## Publishing Steps

### 1. Prepare Package

```bash
cd bindings/javascript

# Install dependencies
npm install

# Run tests
npm test

# Build (if needed)
npm run build
```

### 2. Update package.json

Ensure `package.json` is complete:
```json
{
  "name": "@magneto/serge",
  "version": "0.1.0",
  "description": "Multi-language HTTP/WebSocket testing library with record/replay",
  "main": "index.js",
  "types": "index.d.ts",
  "files": [
    "index.js",
    "index.d.ts",
    "lib/",
    "examples/",
    "README.md",
    "LICENSE-MIT",
    "LICENSE-APACHE"
  ],
  "scripts": {
    "test": "jest",
    "prepublishOnly": "npm test"
  },
  "keywords": [
    "testing",
    "http",
    "websocket",
    "proxy",
    "vcr",
    "record",
    "replay",
    "mock"
  ],
  "author": "matgto-serge contributors",
  "license": "(MIT OR Apache-2.0)",
  "repository": {
    "type": "git",
    "url": "https://github.com/matgto/serge.git",
    "directory": "bindings/javascript"
  },
  "bugs": {
    "url": "https://github.com/matgto/serge/issues"
  },
  "homepage": "https://github.com/matgto/serge#readme",
  "dependencies": {
    "ffi-napi": "^4.0.3",
    "ref-napi": "^3.0.3"
  },
  "devDependencies": {
    "jest": "^29.7.0",
    "@types/node": "^20.10.0"
  },
  "engines": {
    "node": ">=14.0.0"
  }
}
```

### 3. Test Package Locally

```bash
# Create tarball
npm pack

# Install locally in another project
npm install /path/to/matgto-serge-0.1.0.tgz

# Test the installed package
```

### 4. Publish to NPM

#### Public Package (Recommended)
```bash
# Publish as public package
npm publish --access public

# For scoped packages
npm publish --access public
```

#### Private Package
```bash
# Requires paid NPM account
npm publish --access restricted
```

### 5. Verify Publication

Check package page:
- https://www.npmjs.com/package/@magneto/serge

Test installation:
```bash
npm install @magneto/serge
```

## Version Management

### Semantic Versioning

```bash
# Patch release (0.1.0 → 0.1.1)
npm version patch

# Minor release (0.1.0 → 0.2.0)
npm version minor

# Major release (0.1.0 → 1.0.0)
npm version major

# Pre-release (0.1.0 → 0.1.1-beta.0)
npm version prerelease --preid=beta
```

### Beta/Alpha Releases

```bash
# Publish beta version
npm publish --tag beta

# Install beta version
npm install @magneto/serge@beta

# Promote beta to latest
npm dist-tag add @magneto/serge@0.2.0-beta.1 latest
```

## Automation with GitHub Actions

Create `.github/workflows/publish-npm.yml`:

```yaml
name: Publish to NPM

on:
  release:
    types: [published]

jobs:
  publish:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Set up Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'

      - name: Install dependencies
        run: |
          cd bindings/javascript
          npm ci

      - name: Run tests
        run: |
          cd bindings/javascript
          npm test

      - name: Publish to NPM
        run: |
          cd bindings/javascript
          npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

Configure GitHub secret:
- `NPM_TOKEN` - Generate at https://www.npmjs.com/settings/YOUR_USERNAME/tokens

## Best Practices

### 1. Use .npmignore

Create `.npmignore`:
```
# Development files
*.test.js
*.spec.js
test/
tests/
coverage/
.nyc_output/

# Build artifacts
*.tgz
node_modules/

# IDE
.vscode/
.idea/
*.swp
*.swo

# Git
.git/
.gitignore
.gitattributes
```

### 2. Include Required Files

Ensure these are published:
- `index.js` (main entry)
- `index.d.ts` (TypeScript definitions)
- `README.md` (documentation)
- `LICENSE-MIT` and `LICENSE-APACHE` (licenses)
- `package.json` (metadata)

### 3. Test Before Publishing

```bash
# Dry run
npm publish --dry-run

# Check what will be published
npm pack --dry-run
```

### 4. Add Badges to README

```markdown
[![npm version](https://img.shields.io/npm/v/@magneto/serge.svg)](https://www.npmjs.com/package/@magneto/serge)
[![npm downloads](https://img.shields.io/npm/dm/@magneto/serge.svg)](https://www.npmjs.com/package/@magneto/serge)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
```

## Troubleshooting

### Package Name Conflict
```bash
# Change package name in package.json
# Use your own scope: @your-username/serge
```

### Authentication Failed
```bash
# Verify login
npm whoami

# Re-login
npm logout
npm login
```

### Publish Rejected
- Ensure package name is available
- Verify scope ownership
- Check version not already published
- Validate package.json

### 2FA Required
```bash
# Generate OTP code from authenticator app
npm publish --otp=123456
```

## Unpublishing (Use with Caution)

```bash
# Unpublish specific version (within 72 hours)
npm unpublish @magneto/serge@0.1.0

# Deprecate instead (recommended)
npm deprecate @magneto/serge@0.1.0 "Please upgrade to 0.2.0"
```

## Resources

- [NPM Publishing Guide](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry)
- [NPM CLI Documentation](https://docs.npmjs.com/cli)
- [Semantic Versioning](https://semver.org/)

## Quick Publish Checklist

- [ ] Update version in `package.json`
- [ ] Update `CHANGELOG.md`
- [ ] Run tests (`npm test`)
- [ ] Build package (`npm run build`)
- [ ] Test package locally (`npm pack` + install)
- [ ] Login to NPM (`npm login`)
- [ ] Publish (`npm publish --access public`)
- [ ] Verify on npmjs.com
- [ ] Test installation (`npm install @magneto/serge`)
- [ ] Create GitHub release
- [ ] Update documentation

## Post-Publication

### 1. Create Git Tag
```bash
git tag v0.1.0
git push origin v0.1.0
```

### 2. Create GitHub Release
- Go to https://github.com/matgto/serge/releases/new
- Select tag `v0.1.0`
- Add release notes
- Publish release

### 3. Update Documentation
- Update main README.md with new version
- Update CHANGELOG.md
- Update bindings documentation
