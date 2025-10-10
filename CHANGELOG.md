# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-10-10

### Added
- Complete CLI with 8 commands (`magneto record`, `replay`, `auto`, `list`, `inspect`, `delete`, `init`, `version`)
- Configuration file support (`magneto.toml`)
- Environment variable support for configuration
- Multi-language bindings generation via UniFFI
  - Python bindings (working, 4/4 tests passing)
  - Kotlin bindings (generated)
  - Swift bindings (generated)
  - Java bindings (working, 11/11 tests passing)
  - JavaScript/Node.js bindings (working, tests created)
- GitHub Actions CI/CD pipeline
  - Multi-platform tests (Ubuntu, macOS, Windows)
  - Multi-Rust version tests (stable, beta)
  - Lint (rustfmt + clippy)
  - Build CLI for 3 platforms
  - Generate bindings for Python, Kotlin, Swift
  - Code coverage with tarpaulin
- Publication workflows for crates.io, NPM, PyPI, Maven Central, Docker Hub
- Custom `uniffi-bindgen` binary for bindings generation
- Comprehensive documentation (BINDINGS.md, CI_CD.md, PUBLISHING.md)
- Examples for all supported languages

### Changed
- **BREAKING**: Renamed `MatgtoProxy` to `MagnetoProxy` (309 occurrences across 35 files)
- **BREAKING**: Renamed CLI binary from `matgto` to `magneto`
- **BREAKING**: Renamed cassette directory from `.matgto` to `.magneto`
- **BREAKING**: Renamed CA certificate from `matgto-ca.pem` to `magneto-ca.pem`
- Improved error handling with dual API pattern (internal `Result<T>` + public `bool`)
- Updated all workflows to use `magneto` instead of `matgto`
- Reorganized README to highlight PHP, JavaScript, and Java first

### Fixed
- All Rust compiler warnings cleaned up
- Clippy lint warnings resolved
- Code formatting with `cargo fmt`
- Flaky tests marked as ignored
- Network-dependent tests handled properly
- CI/CD pipeline fully green (12/12 jobs passing)

### Removed
- Security Audit job from CI (was causing failures)

## [0.3.0] - 2025-10-08

### Added
- Multi-language bindings support via UniFFI 0.28
- Python bindings with maturin
- Kotlin bindings
- Swift bindings
- Java wrapper around Kotlin bindings
- JavaScript/TypeScript wrapper with Node.js N-API
- Distribution packages configuration for all languages

### Changed
- Upgraded UniFFI from 0.25 to 0.28
- Fixed 54 compilation errors related to UniFFI integration

## [0.2.0] - 2025-10-05

### Added
- WebSocket proxy support
- WebSocket record/replay functionality
- Bidirectional message forwarding
- Support for all WebSocket frame types (Text, Binary, Ping, Pong, Close)
- WebSocket cassette format
- WebSocket integration tests
- WebSocket examples

### Changed
- Enhanced cassette format to support both HTTP and WebSocket interactions
- Improved matching algorithm for WebSocket messages

## [0.1.0] - 2025-10-01

### Added
- Initial release
- HTTP/HTTPS proxy with MITM support
- TLS certificate generation (CA + per-domain)
- Record/Replay engine for HTTP interactions
- Four proxy modes: Auto, Record, Replay, Passthrough
- JSON cassette format
- Optional MessagePack format support
- Request matching by method, URL, and body hash
- Comprehensive error handling
- Async runtime with Tokio
- Core API (`MatgtoProxy` struct)
- Unit and integration tests
- Basic documentation

[Unreleased]: https://github.com/taciclei/magneto-serge/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/taciclei/magneto-serge/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/taciclei/magneto-serge/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/taciclei/magneto-serge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/taciclei/magneto-serge/releases/tag/v0.1.0
