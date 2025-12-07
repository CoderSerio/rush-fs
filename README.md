# Hyper-FS

<p align="center">
  <img src="https://img.shields.io/badge/Written%20in-Rust-orange?style=flat-square" alt="Written in Rust">
  <img src="https://img.shields.io/npm/v/hyper-fs?style=flat-square" alt="NPM Version">
  <img src="https://img.shields.io/npm/l/hyper-fs?style=flat-square" alt="License">
</p>

<p align="center">
  A high-performance, drop-in replacement for Node.js <code>fs</code> module, powered by Rust.
</p>

## Installation (⚠️ Not Ready Yet)

```bash
npm install hyper-fs
# or
pnpm add hyper-fs
```

## Status & Roadmap

We are rewriting `fs` APIs one by one.

> **Legend**
>
> - ✅: Fully Supported
> - 🚧: Partially Supported / WIP
> - ✨：New feature from hyper-fs
> - ❌: Not Supported Yet

### `readdir`

- **Node.js Arguments**:
  ```ts
  path: string; // ✅
  options?: {
    encoding?: string; // ❌
    withFileTypes?: boolean; // ✅
    recursive?: boolean; // ✅
  };
  ```
- **Return Type Diff**: `Buffer` return not supported yet.
- **Performance**: TBD
- **Supported Version**: TBD
- **Notes**:
  - ✨ Supports `options.concurrency` to control parallelism.

### `readFile`

- **Status**: ❌

### `writeFile`

- **Status**: ❌

### `copyFile`

- **Status**: ❌

### `mkdir`

- **Status**: ❌

### `rm`

- **Status**: ❌

### `rmdir`

- **Status**: ❌

### `stat`

- **Status**: ❌

### `lstat`

- **Status**: ❌

### `fstat`

- **Status**: ❌

### `access`

- **Status**: ❌

### `open`

- **Status**: ❌

### `opendir`

- **Status**: ❌

### `close`

- **Status**: ❌

### `unlink`

- **Status**: ❌

### `rename`

- **Status**: ❌

### `readlink`

- **Status**: ❌

### `realpath`

- **Status**: ❌

### `chmod`

- **Status**: ❌

### `chown`

- **Status**: ❌

### `utimes`

- **Status**: ❌

### `truncate`

- **Status**: ❌

### `appendFile`

- **Status**: ❌

### `watch`

- **Status**: ❌

## Usage

```ts
import { readdir } from 'hyper-fs'

// Basic usage
const files = await readdir('./src')

// Recursive with file types
const entries = await readdir('./src', {
  recursive: true,
  withFileTypes: true,
})
```

## License

MIT
