# Publishing Guide for Magnéto-Serge v0.1.0

Complete guide for publishing to all package registries.

---

## 📦 1. Crates.io (Rust)

### Prerequisites
- ✅ Email verified on crates.io: `contact@taciclei.com`
- ✅ Cargo token set: `CARGO_REGISTRY_TOKEN`

### Check Email Verification

1. Go to https://crates.io/settings/profile
2. Verify `contact@taciclei.com` is confirmed
3. If not, click verification link in email

### Publish

```bash
# Dry run (already tested ✅)
cargo publish --dry-run

# Real publication
cargo publish

# Verify
cargo search magneto-serge
```

### Expected Output

```
Uploading magneto-serge v0.1.0
    Updating crates.io index
   Packaging magneto-serge v0.1.0
   Verifying magneto-serge v0.1.0
    Finished dev [optimized + debuginfo] target(s) in 1m 09s
     Waiting on `magneto-serge` to propagate to index (ctrl-c to wait asynchronously)
```

**Status:** ⏳ Pending email verification

---

## 📦 2. NPM (GitHub Packages)

### Prerequisites
- ✅ GitHub token with `write:packages` scope
- ✅ Package scoped to @taciclei
- ✅ publishConfig in package.json

### Setup Authentication

```bash
# Create/edit ~/.npmrc
echo "//npm.pkg.github.com/:_authToken=YOUR_GITHUB_TOKEN" >> ~/.npmrc
echo "@taciclei:registry=https://npm.pkg.github.com" >> ~/.npmrc
```

**Or use environment variable:**
```bash
export NODE_AUTH_TOKEN=your_github_token
```

### Publish

```bash
cd bindings/javascript

# Build
npm run build

# Test package
npm pack --dry-run

# Publish to GitHub Packages
npm publish
```

### Verify

```bash
npm view @taciclei/magneto-serge
```

### Consumer Installation

```bash
# Configure registry
echo "@taciclei:registry=https://npm.pkg.github.com" >> .npmrc

# Authenticate
npm login --scope=@taciclei --registry=https://npm.pkg.github.com

# Install
npm install @taciclei/magneto-serge
```

**Status:** ✅ Ready to publish

---

## 📦 3. Private Packagist (PHP)

### Prerequisites
- ✅ Private Packagist organization: taciclei
- ✅ composer.json prepared in bindings/php/
- ✅ GitHub repository accessible

### Add Package

**Option 1: Via Web Interface (Recommended)**

1. Go to https://packagist.com/orgs/taciclei/packages/new
2. Select "Add GitHub Repository"
3. Repository: `taciclei/magneto-serge`
4. **Subdirectory:** `bindings/php` ⚠️ IMPORTANT
5. Click "Add Package"

**Option 2: Via Settings**

1. Navigate to Settings → Synchronizations
2. Add synchronization from GitHub
3. Select repository: `taciclei/magneto-serge`
4. Configure path: `bindings/php`
5. Save

### Webhook (Automatic)

Private Packagist will automatically create a GitHub webhook for auto-updates on:
- Push events
- Tag creation

### Verify

1. Go to https://packagist.com/orgs/taciclei/packages
2. Look for `taciclei/magneto-serge`
3. Check version: 0.1.0

### Consumer Installation

```bash
# Configure repository
composer config repositories.private-packagist composer https://repo.packagist.com/taciclei/
composer config repositories.packagist.org false

# Authenticate
composer config --global --auth http-basic.repo.packagist.com tazouu YOUR_TOKEN

# Install
composer require taciclei/magneto-serge
```

**Documentation:** `bindings/php/PACKAGIST.md`

**Status:** ✅ Ready to add

---

## 🔄 Publication Checklist

### Before Publishing

- [x] All tests passing (68/68)
- [x] Version bumped to 0.1.0
- [x] CHANGELOG.md updated
- [x] Git tag v0.1.0 created
- [x] GitHub release published
- [x] Documentation complete

### Crates.io

- [ ] Email verified on crates.io
- [x] `cargo publish --dry-run` successful ✅
- [ ] Execute `cargo publish`
- [ ] Verify on https://crates.io/crates/magneto-serge

### NPM (GitHub Packages)

- [ ] GitHub token configured
- [x] Build successful ✅
- [x] Package tested locally ✅
- [ ] Execute `npm publish`
- [ ] Verify on https://github.com/taciclei/magneto-serge/packages

### Private Packagist

- [ ] Package added to Private Packagist
- [ ] Webhook configured (automatic)
- [ ] Version 0.1.0 detected
- [ ] Test installation: `composer require taciclei/magneto-serge`

---

## 📊 Post-Publication

### Verification Commands

**Crates.io:**
```bash
cargo search magneto-serge
cargo info magneto-serge
```

**NPM:**
```bash
npm view @taciclei/magneto-serge
npm info @taciclei/magneto-serge
```

**Private Packagist:**
```bash
composer show taciclei/magneto-serge --all
```

### Update Documentation

After successful publication:

1. Update README.md with installation badges
2. Update docs with package links
3. Announce on:
   - GitHub Discussions
   - Twitter/X
   - Reddit (r/rust, r/php, r/javascript)
   - Dev.to
   - HackerNews (Show HN)

---

## 🐛 Troubleshooting

### Crates.io: "email not verified"

**Solution:**
1. Check email inbox for `contact@taciclei.com`
2. Click verification link
3. Wait 5 minutes for propagation
4. Retry `cargo publish`

### NPM: "authentication failed"

**Solution:**
```bash
# Check token
npm config get //npm.pkg.github.com/:_authToken

# Re-authenticate
npm login --scope=@taciclei --registry=https://npm.pkg.github.com
```

### Private Packagist: "package not found"

**Solution:**
1. Verify subdirectory path: `bindings/php`
2. Check webhook is active
3. Force manual update in Private Packagist dashboard
4. Verify composer.json is valid

---

## 📝 Version Management

### Future Releases

For version 0.2.0:

```bash
# Update versions
sed -i '' 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml
sed -i '' 's/"version": "0.1.0"/"version": "0.2.0"/' bindings/javascript/package.json
sed -i '' 's/"version": "0.1.0"/"version": "0.2.0"/' bindings/php/composer.json

# Update CHANGELOG
# ... edit CHANGELOG.md ...

# Commit and tag
git add Cargo.toml bindings/*/package.json bindings/*/composer.json CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
git tag v0.2.0
git push origin develop v0.2.0

# Publish (same commands as above)
cargo publish
cd bindings/javascript && npm publish
# Private Packagist updates automatically via webhook
```

---

## 🔗 Registry Links

| Registry | Package | URL |
|----------|---------|-----|
| **Crates.io** | magneto-serge | https://crates.io/crates/magneto-serge |
| **GitHub Packages** | @taciclei/magneto-serge | https://github.com/taciclei/magneto-serge/packages |
| **Private Packagist** | taciclei/magneto-serge | https://packagist.com/packages/taciclei/magneto-serge |

---

## 📧 Support

If you encounter issues during publication:

- **GitHub Issues:** https://github.com/taciclei/magneto-serge/issues
- **Email:** contact@taciclei.com
- **Private Packagist Support:** contact@packagist.com

---

**Last Updated:** 2025-10-11
**Version:** 0.1.0
