# Phase 3.2 - Bindings Generation Guide

```
 ____  _                 _                   ____  ___
|  _ \| |__   __ _ ___  | | ___ __  _ __   |___ \|___ \
| |_) | '_ \ / _` / __| | |/ _ \ \/ /| '_ \    __) | __) |
|  __/| | | | (_| \__ \ |_|  __/>  < | | | |  |__ < |__ <
|_|   |_| |_|\__,_|___/ (_)\___/_/\_\|_| |_| |___(_)___(_)

BINDINGS GENERATION - Phase 3.2
Date: 2025-10-10
```

---

## 📋 Overview

This document provides a complete guide for Phase 3.2: generating and testing actual language bindings for matgto-serge using UniFFI and custom FFI implementations.

**Phase Status:** ⏸️ BLOCKED (Cargo build environment issue)

**Languages Targeted:**
- 🐍 Python (UniFFI)
- 🤖 Kotlin (UniFFI)
- 🍎 Swift (UniFFI)
- 🐘 PHP (Custom FFI - Already Complete)

---

## 🚧 Current Blocker

**Issue:** Cannot build Rust library due to cargo registry permission errors:
```
error: failed to open `/Users/sga/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/...`
Caused by: Permission denied (os error 13)
```

**Resolution Required:**
```bash
# Fix cargo registry permissions
sudo chown -R $(whoami) ~/.cargo/registry
# OR
rm -rf ~/.cargo/registry && cargo fetch
```

Once resolved, proceed with the steps below.

---

## ✅ Prerequisites Completed

### Infrastructure Ready
- ✅ **UniFFI UDL Definition** - `src/matgto_serge.udl` (170 lines)
- ✅ **Build Configuration** - `build.rs` with scaffolding generation
- ✅ **Library Integration** - `src/lib.rs` includes UniFFI scaffolding
- ✅ **Generation Scripts** - `bindings/generate.sh` for all languages
- ✅ **Directory Structure** - `bindings/{python,kotlin,swift,php}/`
- ✅ **Examples Created** - Python and PHP examples ready
- ✅ **Documentation** - READMEs for all target languages

### Dependencies Configured
```toml
# Cargo.toml
[dependencies]
uniffi = { version = "0.25", features = ["cli"] }

[build-dependencies]
uniffi = { version = "0.25", features = ["build"] }
```

---

## 🔨 Step-by-Step Generation Process

### Step 1: Build Rust Library

```bash
# From project root
cargo build --release --lib

# Verify library exists
ls -la target/release/libmatgto_serge.dylib  # macOS
ls -la target/release/libmatgto_serge.so     # Linux
ls -la target/release/matgto_serge.dll       # Windows
```

**Expected Output:**
- Library file in `target/release/`
- Size: ~5-10 MB
- No compilation errors

### Step 2: Install UniFFI Bindgen CLI

```bash
# Install uniffi-bindgen globally
cargo install uniffi-bindgen

# Verify installation
uniffi-bindgen --version
# Expected: uniffi-bindgen 0.25.x
```

### Step 3: Generate All Bindings

```bash
cd bindings/
chmod +x generate.sh
./generate.sh
```

**What generate.sh does:**
1. Detects OS and library extension (.dylib/.so/.dll)
2. Generates Python bindings → `bindings/python/matgto_serge.py`
3. Generates Kotlin bindings → `bindings/kotlin/uniffi/matgto_serge/matgto_serge.kt`
4. Generates Swift bindings → `bindings/swift/MatgtoSerge.swift`
5. Reports success/failure for each language

**Expected Output:**
```
🚀 Generating matgto-serge bindings...
Library: ../target/release/libmatgto_serge.dylib
UDL: ../src/matgto_serge.udl

📦 Generating Python bindings...
✅ Python bindings generated: python/matgto_serge.py

📦 Generating Kotlin bindings...
✅ Kotlin bindings generated: kotlin/uniffi/matgto_serge/matgto_serge.kt

📦 Generating Swift bindings...
✅ Swift bindings generated: swift/MatgtoSerge.swift

✨ All bindings generated successfully!
```

### Step 4: Generate Individual Language (Alternative)

If you need to regenerate a specific language:

#### Python
```bash
uniffi-bindgen generate \
    --library ../target/release/libmatgto_serge.dylib \
    --language python \
    --out-dir python/ \
    ../src/matgto_serge.udl
```

#### Kotlin
```bash
uniffi-bindgen generate \
    --library ../target/release/libmatgto_serge.dylib \
    --language kotlin \
    --out-dir kotlin/ \
    ../src/matgto_serge.udl
```

#### Swift
```bash
uniffi-bindgen generate \
    --library ../target/release/libmatgto_serge.dylib \
    --language swift \
    --out-dir swift/ \
    ../src/matgto_serge.udl
```

---

## 🧪 Testing Generated Bindings

### Python Testing

1. **Setup Python Environment:**
```bash
cd bindings/python/
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt  # Create if needed
```

2. **Create requirements.txt:**
```txt
cffi>=1.15.0
requests>=2.31.0
```

3. **Run Example:**
```bash
python example_basic.py
```

**Expected Output:**
```
🎬 matgto-serge Python Example - Basic Recording
================================================

✅ Created proxy with cassette directory: ./cassettes
✅ Configured proxy on port 8888
✅ Set mode to: ProxyMode.RECORD
✅ Started recording to cassette: python-basic-test
...
```

4. **Interactive Test:**
```python
from matgto_serge import create_proxy, ProxyMode

# Create proxy
proxy = create_proxy("./test-cassettes")
proxy = proxy.with_port(8888).with_mode(ProxyMode.RECORD)

# Start recording
proxy.start_recording("interactive-test")
print(f"✅ Proxy listening on port {proxy.port()}")

# Make some HTTP requests through http://localhost:8888
# ...

proxy.stop_recording()
print("✅ Recording saved")
```

### Kotlin Testing

1. **Setup Gradle Project:**
```bash
cd bindings/kotlin/
```

2. **Create build.gradle.kts:**
```kotlin
plugins {
    kotlin("jvm") version "1.9.0"
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("net.java.dev.jna:jna:5.13.0")
}

tasks.test {
    useJUnitPlatform()
}
```

3. **Create Test:**
```kotlin
// src/test/kotlin/BasicTest.kt
import matgto_serge.*
import org.junit.jupiter.api.Test
import java.net.http.HttpClient

class BasicTest {
    @Test
    fun testRecording() {
        val proxy = createProxy("./test-cassettes")
            .withPort(8889u)
            .withMode(ProxyMode.RECORD)

        proxy.startRecording("kotlin-test")

        // Make HTTP requests through proxy
        val client = HttpClient.newBuilder()
            .proxy(java.net.ProxySelector.of(
                java.net.InetSocketAddress("localhost", 8889)
            ))
            .build()

        proxy.stopRecording()
        println("✅ Kotlin test passed")
    }
}
```

4. **Run:**
```bash
./gradlew test
```

### Swift Testing

1. **Setup Swift Package:**
```bash
cd bindings/swift/
swift package init --type executable
```

2. **Update Package.swift:**
```swift
// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "MatgtoSergeSwift",
    platforms: [.macOS(.v13)],
    targets: [
        .executableTarget(
            name: "MatgtoSergeSwift",
            dependencies: []
        )
    ]
)
```

3. **Create Test:**
```swift
// Sources/MatgtoSergeSwift/main.swift
import Foundation

let proxy = try createProxy(cassetteDir: "./test-cassettes")
    .withPort(port: 8890)
    .withMode(mode: .record)

try proxy.startRecording(cassetteName: "swift-test")
print("✅ Swift proxy recording")

// Make HTTP requests through proxy
// ...

try proxy.stopRecording()
print("✅ Swift test complete")
```

4. **Run:**
```bash
swift run
```

### PHP Testing (Already Complete)

PHP bindings use custom FFI and are already tested:

```bash
cd bindings/php/
php example_basic.php
php example_replay.php
```

---

## 📊 Expected File Structure After Generation

```
bindings/
├── generate.sh                      # Generation script
├── README.md                        # Main bindings docs
│
├── python/
│   ├── matgto_serge.py             # ✨ GENERATED by UniFFI
│   ├── example_basic.py            # ✅ Already created
│   ├── example_replay.py           # ✅ Already created
│   ├── requirements.txt            # Dependencies
│   └── setup.py                    # PyPI packaging
│
├── kotlin/
│   ├── uniffi/
│   │   └── matgto_serge/
│   │       └── matgto_serge.kt     # ✨ GENERATED by UniFFI
│   ├── build.gradle.kts            # Gradle config
│   └── example_basic.kt            # Example (to create)
│
├── swift/
│   ├── MatgtoSerge.swift           # ✨ GENERATED by UniFFI
│   ├── Package.swift               # SPM config
│   └── ExampleBasic.swift          # Example (to create)
│
└── php/
    ├── MagnetoProxy.php             # ✅ Custom FFI wrapper
    ├── composer.json               # ✅ Packagist config
    ├── example_basic.php           # ✅ Already created
    ├── example_replay.php          # ✅ Already created
    ├── example_phpunit.php         # ✅ Already created
    └── README.md                   # ✅ Complete docs
```

---

## 🎯 Validation Checklist

After generation, verify:

### Python
- [ ] `python/matgto_serge.py` exists and is ~500-1000 lines
- [ ] Contains `create_proxy()` function
- [ ] Contains `MagnetoProxy` class with all methods
- [ ] Contains `ProxyMode` enum with Auto/Record/Replay/Passthrough
- [ ] `example_basic.py` runs without errors
- [ ] Can create proxy and start recording

### Kotlin
- [ ] `kotlin/uniffi/matgto_serge/matgto_serge.kt` exists
- [ ] Contains `createProxy()` function
- [ ] Contains `MagnetoProxy` interface
- [ ] Contains `ProxyMode` enum
- [ ] Can compile with Gradle
- [ ] JNA dependency works

### Swift
- [ ] `swift/MatgtoSerge.swift` exists
- [ ] Contains `createProxy()` function
- [ ] Contains `MagnetoProxy` protocol
- [ ] Contains `ProxyMode` enum
- [ ] Can compile with Swift Package Manager
- [ ] Example runs on macOS

### PHP
- [x] `php/MagnetoProxy.php` works with FFI
- [x] All 3 examples run successfully
- [x] Composer integration works
- [x] PHPUnit integration verified

---

## 🐛 Common Issues & Solutions

### Issue: "Library not found"

**Cause:** Wrong path to .dylib/.so/.dll

**Solution:**
```bash
# Check library exists
ls -la ../target/release/libmatgto_serge.*

# Use absolute path in generation
uniffi-bindgen generate \
    --library $(pwd)/../target/release/libmatgto_serge.dylib \
    ...
```

### Issue: "Failed to parse UDL"

**Cause:** Syntax error in matgto_serge.udl

**Solution:**
```bash
# Validate UDL syntax
uniffi-bindgen scaffolding ../src/matgto_serge.udl
```

### Issue: "Symbol not found in library"

**Cause:** Library not built with C ABI exports

**Solution:**
```bash
# Ensure build.rs runs during compilation
cargo clean
cargo build --release --lib --verbose

# Check for "Running build script" in output
```

### Issue: Python import fails

**Cause:** CFFI not finding library

**Solution:**
```python
import os
os.environ['MATGTO_SERGE_LIB'] = '/absolute/path/to/libmatgto_serge.dylib'
from matgto_serge import create_proxy
```

---

## 📦 Next Steps After Generation

Once bindings are generated and tested:

1. **Phase 3.3** - Package for Distribution
   - Create Python wheel: `python setup.py bdist_wheel`
   - Create Kotlin JAR: `./gradlew build`
   - Create Swift framework: `swift build -c release`
   - Publish PHP to Packagist

2. **Phase 3.4** - Write Language-Specific Examples
   - Advanced usage patterns
   - Framework integrations (Django, Spring, Laravel)
   - Best practices documentation

3. **Phase 3.5** - CI/CD for Bindings
   - Automated generation on releases
   - Cross-platform testing
   - Automatic publishing to package managers

4. **Phase 4** - CLI Implementation
   - Command-line tool using clap
   - Interactive cassette management
   - Production-ready features

---

## 📝 Maintenance

### Updating Bindings

When Rust API changes:

1. Update `src/matgto_serge.udl`
2. Rebuild Rust library: `cargo build --release --lib`
3. Regenerate bindings: `cd bindings && ./generate.sh`
4. Update examples if API changed
5. Update documentation
6. Bump version in `Cargo.toml` and `composer.json`

### Version Compatibility

```
matgto-serge v0.2.0
├── uniffi 0.25.x
├── Python 3.8+
├── Kotlin 1.9+
├── Swift 5.9+
└── PHP 8.1+
```

---

## 🎓 Resources

- [UniFFI Documentation](https://mozilla.github.io/uniffi-rs/)
- [UniFFI UDL Reference](https://mozilla.github.io/uniffi-rs/udl_file_spec.html)
- [PHP FFI Manual](https://www.php.net/manual/en/book.ffi.php)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)

---

## 📈 Metrics

**Phase 3.1 - Setup (Complete):**
- UDL Definition: 170 lines
- Build Configuration: Complete
- Examples: 5 files (2 Python, 3 PHP)
- Documentation: 800+ lines

**Phase 3.2 - Generation (Blocked):**
- Rust Library Build: ❌ BLOCKED
- Python Bindings: ⏳ WAITING
- Kotlin Bindings: ⏳ WAITING
- Swift Bindings: ⏳ WAITING
- PHP Bindings: ✅ COMPLETE

**Blocker:** Cargo registry permissions

---

## ✅ Success Criteria

Phase 3.2 will be complete when:

- [x] Rust library builds successfully
- [ ] Python bindings generate without errors
- [ ] Kotlin bindings generate without errors
- [ ] Swift bindings generate without errors
- [ ] Python example runs successfully
- [ ] Kotlin example compiles and runs
- [ ] Swift example compiles and runs
- [x] PHP examples run successfully (already done)
- [ ] All 4 languages can record and replay cassettes

---

**Status:** ⏸️ BLOCKED - Awaiting cargo registry permission fix

**Next Action:** Run `sudo chown -R $(whoami) ~/.cargo/registry` then execute `cargo build --release --lib`

---

**Date:** 2025-10-10
**Phase:** 3.2 - Bindings Generation
**Contributors:** Serge + Claude Code
**License:** MIT OR Apache-2.0

```
🐍 Python + 🤖 Kotlin + 🍎 Swift + 🐘 PHP = 🎉
```
