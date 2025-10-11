# 🚀 PUBLISH NOW - Magnéto-Serge v0.1.0

Everything is ready! Follow these steps to publish to all registries.

---

## ✅ Pre-Publication Checklist

- [x] Version 0.1.0 set in all files
- [x] Git tag v0.1.0 created and pushed
- [x] GitHub Release published
- [x] All tests passing (68/68)
- [x] Documentation complete
- [x] TypeScript definitions included

**Status: 🟢 READY TO PUBLISH**

---

## 1️⃣ Crates.io (Rust) - 5 minutes

### Prerequisites
- Email `contact@taciclei.com` verified on https://crates.io
- Cargo token from https://crates.io/settings/tokens

### Steps

```bash
# 1. Login to crates.io (one-time)
cargo login YOUR_CRATES_IO_TOKEN

# 2. Publish
cargo publish

# 3. Verify
cargo search magneto-serge
```

### Expected Output
```
    Updating crates.io index
   Packaging magneto-serge v0.1.0
   Verifying magneto-serge v0.1.0
   Compiling magneto-serge v0.1.0
    Finished dev [optimized + debuginfo] target(s) in 1m 09s
   Uploading magneto-serge v0.1.0
```

### Verify
- https://crates.io/crates/magneto-serge
- Badge should show v0.1.0

---

## 2️⃣ NPM (GitHub Packages) - 3 minutes

### Prerequisites
- GitHub Personal Access Token with `write:packages` scope
- Token from https://github.com/settings/tokens

### Steps

```bash
# 1. Configure authentication (one-time)
echo "//npm.pkg.github.com/:_authToken=YOUR_GITHUB_TOKEN" >> ~/.npmrc
echo "@taciclei:registry=https://npm.pkg.github.com" >> ~/.npmrc

# 2. Build and publish
cd bindings/javascript
npm run build
npm publish

# 3. Verify
npm view @taciclei/magneto-serge
```

### Expected Output
```
npm notice 📦  @taciclei/magneto-serge@0.1.0
npm notice === Tarball Contents ===
npm notice 3.4kB  index.d.ts
npm notice 9.2kB  index.js
npm notice 1.8MB  magneto-serge-node.darwin-arm64.node
npm notice 1.4kB  package.json
npm notice === Tarball Details ===
npm notice name:          @taciclei/magneto-serge
npm notice version:       0.1.0
npm notice Publishing to https://npm.pkg.github.com
+ @taciclei/magneto-serge@0.1.0
```

### Verify
- https://github.com/taciclei/magneto-serge/packages

---

## 3️⃣ Private Packagist (PHP) - 2 minutes

### Prerequisites
- Access to https://packagist.com/orgs/taciclei

### Steps

**Via Web Interface:**

1. Go to https://packagist.com/orgs/taciclei/packages/new
2. Click "Add GitHub Repository"
3. Select repository: `taciclei/magneto-serge`
4. **IMPORTANT:** Set subdirectory: `bindings/php`
5. Click "Add Package"

**Automatic Synchronization:**
- Private Packagist will create a GitHub webhook automatically
- Future releases will sync automatically when you push tags

### Verify
1. Go to https://packagist.com/orgs/taciclei/packages
2. Look for `taciclei/magneto-serge`
3. Version should show 0.1.0

### Test Installation
```bash
composer config repositories.private-packagist composer https://repo.packagist.com/taciclei/
composer config --global --auth http-basic.repo.packagist.com tazouu YOUR_PACKAGIST_TOKEN
composer require taciclei/magneto-serge
```

---

## 📊 Post-Publication Checklist

### Immediate Actions
- [ ] Verify package on crates.io
- [ ] Verify package on GitHub Packages
- [ ] Verify package on Private Packagist
- [ ] Test installation on clean machine

### Update Documentation
- [ ] Add installation badges to README.md
- [ ] Update links in documentation
- [ ] Mark PUBLISHING.md tasks as complete

### Announcements
- [ ] GitHub Discussions - Project announcement
- [ ] Twitter/X - Release announcement
- [ ] Reddit:
  - [ ] r/rust - "Show: Magnéto-Serge v0.1.0 - HTTP/WebSocket record/replay library"
  - [ ] r/javascript - "Magnéto-Serge: VCR for Node.js with TypeScript support"
  - [ ] r/PHP - "Magnéto-Serge: HTTP/WebSocket testing via FFI"
- [ ] Dev.to - Article about the project
- [ ] HackerNews - "Show HN: Magnéto-Serge - Multi-language HTTP/WebSocket VCR"

---

## 🎯 Installation Badges for README

After publication, add these badges:

```markdown
[![Crates.io](https://img.shields.io/crates/v/magneto-serge.svg)](https://crates.io/crates/magneto-serge)
[![npm](https://img.shields.io/npm/v/@taciclei/magneto-serge.svg)](https://github.com/taciclei/magneto-serge/packages)
[![Downloads](https://img.shields.io/crates/d/magneto-serge.svg)](https://crates.io/crates/magneto-serge)
```

---

## 🐛 Troubleshooting

### Crates.io: "email not verified"
1. Check email inbox for `contact@taciclei.com`
2. Click verification link
3. Wait 5 minutes
4. Retry `cargo publish`

### NPM: "authentication failed"
```bash
# Generate new token with write:packages scope
# https://github.com/settings/tokens/new
# Then update ~/.npmrc

npm logout --scope=@taciclei --registry=https://npm.pkg.github.com
npm login --scope=@taciclei --registry=https://npm.pkg.github.com
```

### Private Packagist: "subdirectory required"
- Make sure to specify `bindings/php` as subdirectory
- Without it, composer.json won't be found

---

## ⏱️ Estimated Time

| Task | Time |
|------|------|
| Crates.io | 5 min |
| NPM | 3 min |
| Private Packagist | 2 min |
| Verification | 5 min |
| **TOTAL** | **15 min** |

---

## 🎉 Success Criteria

**You'll know it worked when:**

1. ✅ `cargo search magneto-serge` returns results
2. ✅ `npm view @taciclei/magneto-serge` shows version 0.1.0
3. ✅ https://packagist.com/packages/taciclei/magneto-serge exists
4. ✅ Users can install with:
   - `cargo add magneto-serge`
   - `npm install @taciclei/magneto-serge`
   - `composer require taciclei/magneto-serge`

---

## 📞 Support

If you need help:
- **Documentation:** See PUBLISHING.md for detailed guides
- **Issues:** https://github.com/taciclei/magneto-serge/issues
- **Email:** contact@taciclei.com

---

**Ready to publish? Let's go! 🚀**

Last updated: 2025-10-11
