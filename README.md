<div align="center">
  
# Hyper-FS
  
  <p align="center">
    <img src="https://img.shields.io/badge/Written%20in-Rust-orange?style=flat-square" alt="Written in Rust">
    <img src="https://img.shields.io/npm/v/hyper-fs?style=flat-square" alt="NPM Version">
    <img src="https://img.shields.io/npm/l/hyper-fs?style=flat-square" alt="License">
  </p>
  
  <p align="center">
    A high-performance, drop-in replacement for Node.js <code>fs</code> module, powered by Rust.
  </p>
</div>

## Installation (⚠️ Not Ready Yet)

```bash
npm install hyper-fs
# or
pnpm add hyper-fs
```

## How does it works

For the original Node.js, it works serially and cost lots of memory to parse os object and string into JS style: 

```mermaid
graph TD
    A["JS: readdir"] -->|Call| B("Node.js C++ Binding")
    B -->|Submit Task| C{"Libuv Thread Pool"}
    
    subgraph "Native Layer (Serial)"
    C -->|"Syscall: getdents"| D[OS Kernel]
    D -->|"Return File List"| C
    C -->|"Process Paths"| C
    end
    
    C -->|"Results Ready"| E("V8 Main Thread")
    
    subgraph "V8 Interaction (Heavy)"
    E -->|"Create JS String 1"| F[V8 Heap]
    E -->|"String 2"| F
    E -->|"String N..."| F
    F -->|"GC Pressure Rising"| F
    end
    
    E -->|"Return Array"| G["JS Callback/Promise"]
```

But, it's saved with Rust now:

```mermaid
graph TD
    A["JS: readdir"] -->|"N-API Call"| B("Rust Wrapper")
    B -->|"Spawn Thread/Task"| C{"Rust Thread Pool"}
    
    subgraph "Rust 'Black Box'"
    C -->|"Rayon: Parallel work"| D[OS Kernel]
    D -->|"Syscall: getdents"| C
    C -->|"Store as Rust Vec<String>"| H[Rust Heap]
    H -->|"No V8 Interaction yet"| H
    end
    
    C -->|"All Done"| I("Convert to JS")
    
    subgraph "N-API Bridge"
    I -->|"Batch Create JS Array"| J[V8 Heap]
    end
    
    J -->|Return| K["JS Result"]
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
    concurrency?: number; // ✨
  };
  ```
- **Return Type**:
  ```ts
    string[]
    | {
      name: string, // ✅
      parentPath: string, // ✅
      isDir: boolean // ✅
    }[]
  ```

### `readFile`

- **Status**: ❌

### `writeFile`

- **Status**: ❌

### `copyFile`

- **Status**: ❌

### `mkdir`

- **Status**: ❌

### `rm`

- **Node.js Arguments**:
  ```ts
  path: string; // ✅
  options?: {
    force?: boolean; // ✅
    maxRetries?: number; // ❌
    recursive?: boolean; // ✅
    retryDelay?: number; // ❌
    concurrency?: number; // ✨
  };
  ```
- **Return Type**:
  ```ts
    void
  ```

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
