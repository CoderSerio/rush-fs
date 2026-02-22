# Changelog

All notable changes to Rush-FS are documented here. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- (Add new changes here before each release.)

## [0.1.0] - (release date TBD)

**This release is in beta.** API and behavior may change before 0.1.0 stable. Feedback and issues are welcome.

### Changed

- **Package name:** The main npm package is now **`@rush-fs/core`** (scoped). Install with `pnpm add @rush-fs/core` or `npm i @rush-fs/core`, and import with `import { readdir, readFile, ... } from '@rush-fs/core'`.
- **Migration from `rush-fs`:** If you were using the old unscoped package `rush-fs`, replace it with `@rush-fs/core` in `package.json` and in all imports. The API is unchanged; only the package name and version differ. The old `rush-fs` package may be deprecated on npm in a separate step; prefer `@rush-fs/core` for new installs.
- **glob:** `gitIgnore` option now defaults to **`false`** to align with Node.js `fs.globSync` (no .gitignore filtering by default).

### Fixed

- **glob:** Patterns with a path prefix (e.g. `.dir/**/*.txt` or `src/**/*.ts`) now work when used without an explicit `cwd`; the prefix is used as the search root, matching Node.js behavior.
- **glob:** Recursive pattern `**/*.ext` with `cwd` now correctly recurses into subdirectories.

## [0.0.5]

- Re-publish with `optionalDependencies` correctly injected after `napi prepublish`, so `pnpm i rush-fs` / `npm i rush-fs` auto-installs the platform native binding. No API or behavior changes from 0.0.4.

## [0.0.4]

### Fixed

- **Publish:** `optionalDependencies` are now injected **after** `napi prepublish` in the Release workflow, so the published npm package correctly lists platform packages. Installing `rush-fs` will again auto-install the native binding for your OS/arch (e.g. `@rush-fs/rush-fs-darwin-arm64`). If you are on an older version and see "Cannot find native binding", see [README#Installation](./README.md#installation) for a manual fix.

### Added

- **Docs:** Nextra-based documentation site under `docs/` with i18n (EN / 中文), guide, API reference, and benchmarks.
- **README:** Installation troubleshooting: how to reinstall or manually add the platform package when the native binding is missing.

## [0.0.3] - (historical)

- Earlier releases; see [GitHub Releases](https://github.com/CoderSerio/rush-fs/releases) for tags and assets.

---

[Unreleased]: https://github.com/CoderSerio/rush-fs/compare/v0.1.0-beta.1...HEAD
[0.1.0]: https://github.com/CoderSerio/rush-fs/compare/v0.0.5...v0.1.0-beta.1
[0.0.5]: https://github.com/CoderSerio/rush-fs/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/CoderSerio/rush-fs/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/CoderSerio/rush-fs/releases/tag/v0.0.3
