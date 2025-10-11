# PHP Private Packagist Setup for Magnéto-Serge

## Package Information

- **Package Name:** `taciclei/magneto-serge`
- **Version:** 0.1.0
- **Type:** Library (PHP FFI bindings to Rust)
- **Repository:** https://github.com/taciclei/magneto-serge

## Private Packagist Configuration

### 1. Add Package to Private Packagist

**Option A: Via GitHub Integration (Recommended)**

1. Go to https://packagist.com/orgs/taciclei/packages/new
2. Select "Add GitHub Repository"
3. Choose `taciclei/magneto-serge`
4. Select subdirectory: `bindings/php`
5. Click "Add Package"

**Option B: Via Git URL**

1. Go to https://packagist.com/orgs/taciclei/packages/new
2. Select "Custom Package"
3. Git URL: `https://github.com/taciclei/magneto-serge.git`
4. Subdirectory: `bindings/php`
5. Click "Add Package"

### 2. Package Settings

```json
{
  "name": "taciclei/magneto-serge",
  "type": "vcs",
  "url": "https://github.com/taciclei/magneto-serge.git",
  "options": {
    "path": "bindings/php"
  }
}
```

### 3. Webhook Configuration (Automatic Updates)

Private Packagist will automatically create a webhook on your GitHub repository to receive push notifications for automatic updates.

**Manual Webhook Setup (if needed):**
- URL: `https://packagist.com/api/update-package?username=taciclei&apiToken=YOUR_TOKEN`
- Content Type: `application/json`
- Events: Push, Create (tags)

### 4. Version Tagging

Private Packagist will automatically detect versions from:
- **Git Tags:** `v0.1.0`, `v0.2.0`, etc.
- **Branches:** `main`, `develop`

For stable releases, always use semantic versioning tags: `v0.1.0`

---

## Consumer Setup (For Users)

### Step 1: Configure Composer Repository

Add the Private Packagist repository to `composer.json`:

```bash
composer config repositories.private-packagist composer https://repo.packagist.com/taciclei/
composer config repositories.packagist.org false
```

**Manual composer.json:**
```json
{
    "repositories": [
        {
            "type": "composer",
            "url": "https://repo.packagist.com/taciclei/"
        },
        {
            "packagist.org": false
        }
    ]
}
```

### Step 2: Setup Authentication

**Global auth (recommended):**
```bash
composer config --global --auth http-basic.repo.packagist.com tazouu YOUR_TOKEN
```

**Project auth.json:**
```bash
composer config --auth http-basic.repo.packagist.com tazouu YOUR_TOKEN
```

**auth.json format:**
```json
{
    "http-basic": {
        "repo.packagist.com": {
            "username": "tazouu",
            "password": "YOUR_TOKEN"
        }
    }
}
```

### Step 3: Install Package

```bash
composer require taciclei/magneto-serge
```

### Step 4: Update composer.lock

After configuring the repository, update the lock file:

```bash
composer update mirrors
```

---

## Usage Example

```php
<?php

require 'vendor/autoload.php';

use Taciclei\MagnetoSerge\MagnetoProxy;
use Taciclei\MagnetoSerge\ProxyMode;

// Create proxy instance
$proxy = new MagnetoProxy('./cassettes');
$proxy->setPort(8888);
$proxy->setMode(ProxyMode::AUTO);

// Start recording
$proxy->startRecording('my-api-test');

// Configure your HTTP client to use proxy http://localhost:8888
// Make your API requests...

// Stop recording
$proxy->stopRecording();
$proxy->shutdown();
```

---

## Requirements

- **PHP:** >= 8.1
- **Extensions:** FFI (Foreign Function Interface)
- **Rust Library:** Compiled `libmagneto_serge.so` (Linux), `.dylib` (macOS), or `.dll` (Windows)

### Installing PHP FFI Extension

**Ubuntu/Debian:**
```bash
sudo apt-get install php-ffi
```

**macOS (Homebrew):**
```bash
brew install php
# FFI is included by default in modern PHP versions
```

**Check if FFI is enabled:**
```bash
php -m | grep FFI
```

---

## CI/CD Integration

### GitHub Actions

```yaml
name: PHP Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup PHP
        uses: shivammathur/setup-php@v2
        with:
          php-version: '8.1'
          extensions: ffi

      - name: Install Composer dependencies
        run: |
          composer config repositories.private-packagist composer https://repo.packagist.com/taciclei/
          composer config --auth http-basic.repo.packagist.com tazouu ${{ secrets.PACKAGIST_TOKEN }}
          composer install

      - name: Run tests
        run: composer test
```

### Environment Variables

Add to GitHub Secrets:
- `PACKAGIST_TOKEN`: Your Private Packagist token

---

## Version Management

### Release New Version

1. **Update version in composer.json:**
   ```json
   {
       "version": "0.2.0"
   }
   ```

2. **Commit and tag:**
   ```bash
   git add bindings/php/composer.json
   git commit -m "chore(php): bump version to 0.2.0"
   git tag v0.2.0
   git push origin v0.2.0
   ```

3. **Private Packagist will automatically:**
   - Detect the new tag
   - Build the package
   - Make it available to consumers

### Version Constraints

**In consumer's composer.json:**
```json
{
    "require": {
        "taciclei/magneto-serge": "^0.1.0"
    }
}
```

**Versions:**
- `^0.1.0` - Any 0.x version >= 0.1.0
- `~0.1.0` - 0.1.x versions only
- `0.1.*` - Same as ~0.1.0
- `>=0.1.0` - Any version >= 0.1.0

---

## Troubleshooting

### Issue: "Package not found"

**Solution:**
1. Verify Private Packagist configuration
2. Check authentication credentials
3. Run `composer clear-cache`
4. Run `composer diagnose`

### Issue: "FFI extension not loaded"

**Solution:**
```bash
# Check if FFI is available
php -m | grep FFI

# Enable in php.ini
extension=ffi

# Restart PHP-FPM
sudo systemctl restart php8.1-fpm
```

### Issue: "Library not found"

**Solution:**
The Rust library needs to be compiled and available:
```bash
# Build Rust library
cargo build --release

# Copy to PHP extension directory (example)
cp target/release/libmagneto_serge.so /usr/lib/magneto-serge/
```

---

## Support

- **Issues:** https://github.com/taciclei/magneto-serge/issues
- **Email:** contact@taciclei.com
- **Documentation:** https://github.com/taciclei/magneto-serge/blob/main/bindings/php/README.md

---

## Private Packagist Links

- **Organization:** https://packagist.com/orgs/taciclei
- **Packages:** https://packagist.com/orgs/taciclei/packages
- **Settings:** https://packagist.com/orgs/taciclei/settings
- **Documentation:** https://packagist.com/docs

---

**Note:** This package uses PHP FFI to call Rust functions. The Rust library must be compiled for your target platform.
