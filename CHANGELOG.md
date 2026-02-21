# Changelog

All notable changes to Rush-FS are documented here. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- (Add new changes here before each release.)

## [0.0.4] - (release date TBD)

### Fixed

- **Publish:** `optionalDependencies` are now injected **after** `napi prepublish` in the Release workflow, so the published npm package correctly lists platform packages. Installing `rush-fs` will again auto-install the native binding for your OS/arch (e.g. `@rush-fs/rush-fs-darwin-arm64`). If you are on an older version and see "Cannot find native binding", see [README#Installation](./README.md#installation) for a manual fix.

### Added

- **Docs:** Nextra-based documentation site under `docs/` with i18n (EN / 中文), guide, API reference, and benchmarks.
- **README:** Installation troubleshooting: how to reinstall or manually add the platform package when the native binding is missing.

## [0.0.3] - (historical)

- Earlier releases; see [GitHub Releases](https://github.com/CoderSerio/rush-fs/releases) for tags and assets.

---

[Unreleased]: https://github.com/CoderSerio/rush-fs/compare/v0.0.4...HEAD
[0.0.4]: https://github.com/CoderSerio/rush-fs/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/CoderSerio/rush-fs/releases/tag/v0.0.3
