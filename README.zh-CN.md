# Hyper-FS

[English](./README.md) | 中文

<p align="center">
  <img src="https://img.shields.io/badge/Written%20in-Rust-orange?style=flat-square" alt="Written in Rust">
  <img src="https://img.shields.io/npm/v/hyper-fs?style=flat-square" alt="NPM Version">
  <img src="https://img.shields.io/npm/l/hyper-fs?style=flat-square" alt="License">
</p>

<p align="center">
  由 Rust 驱动的高性能 Node.js <code>fs</code> 模块「即插即用」替代品。
</p>

## 安装（⚠️ 暂未就绪）

```bash
npm install hyper-fs
# or
pnpm add hyper-fs
```

## 状态与路线图

我们正在逐个重写 `fs` 的 API。

> **图例**
>
> - ✅：完全支持
> - 🚧：部分支持 / 开发中
> - ✨：hyper-fs 的新增能力
> - ❌：暂未支持

### `readdir`

- **Node.js 参数**：
  ```ts
  path: string; // ✅
  options?: {
    encoding?: string; // ❌
    withFileTypes?: boolean; // ✅
    recursive?: boolean; // ✅
    concurrency?: number; // ✨
  };
  ```
- **返回类型**：
  ```ts
    string[]
    | {
      name: string, // ✅
      parentPath: string, // ✅
      isDir: boolean // ✅
    }[]
  ```

### `readFile`

- **Node.js 参数**：
  ```ts
  path: string; // ✅
  options?: {
    encoding?: string; // ✅ (utf8)
    flag?: string; // ❌
  };
  ```
- **返回类型**：`string | Buffer`

### `writeFile`

- **Node.js 参数**：
  ```ts
  path: string; // ✅
  data: string | Buffer; // ✅
  options?: {
    encoding?: string; // ❌
    mode?: number; // ✅
    flag?: string; // ✅ (w, wx, a, ax)
  };
  ```

### `appendFile`

- **Node.js 参数**：
  ```ts
  path: string; // ✅
  data: string | Buffer; // ✅
  options?: {
    encoding?: string; // ❌
    mode?: number; // ✅
    flag?: string; // ✅
  };
  ```

### `copyFile`

- **Node.js 参数**：
  ```ts
  src: string; // ✅
  dest: string; // ✅
  mode?: number; // ✅ (COPYFILE_EXCL)
  ```

### `mkdir`

- **Node.js 参数**：
  ```ts
  path: string; // ✅
  options?: {
    recursive?: boolean; // ✅
    mode?: number; // ✅
  };
  ```
- **返回类型**：`string | undefined`（recursive 模式下返回首个创建的路径）

### `rm`

- **Node.js 参数**：
  ```ts
  path: string; // ✅
  options?: {
    force?: boolean; // ✅
    maxRetries?: number; // ❌
    retryDelay?: number; // ❌
    recursive?: boolean; // ✅
    concurrency?: number; // ✨
  };
  ```

### `rmdir`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  ```

### `stat`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  ```
- **返回类型**：`Stats`（dev, mode, nlink, uid, gid, rdev, blksize, ino, size, blocks, atimeMs, mtimeMs, ctimeMs, birthtimeMs + isFile/isDirectory/isSymbolicLink/...）

### `lstat`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  ```
- **返回类型**：`Stats`

### `fstat`

- **状态**：❌

### `access`

- **Node.js 参数**：
  ```ts
  path: string; // ✅
  mode?: number; // ✅ (F_OK, R_OK, W_OK, X_OK)
  ```

### `exists`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  ```
- **返回类型**：`boolean`

### `open`

- **状态**：❌

### `opendir`

- **状态**：❌

### `close`

- **状态**：❌

### `unlink`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  ```

### `rename`

- **Node.js 参数**：
  ```ts
  oldPath: string // ✅
  newPath: string // ✅
  ```

### `readlink`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  ```
- **返回类型**：`string`

### `realpath`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  ```
- **返回类型**：`string`

### `chmod`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  mode: number // ✅
  ```

### `chown`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  uid: number // ✅
  gid: number // ✅
  ```

### `utimes`

- **Node.js 参数**：
  ```ts
  path: string // ✅
  atime: number // ✅
  mtime: number // ✅
  ```

### `truncate`

- **Node.js 参数**：
  ```ts
  path: string; // ✅
  len?: number; // ✅
  ```

### `glob`

- **Node.js 参数**：
  ```ts
  pattern: string; // ✅
  options?: {
    cwd?: string; // ✅
    withFileTypes?: boolean; // ✅
    exclude?: string[]; // ✅
    concurrency?: number; // ✨
    gitIgnore?: boolean; // ✨
  };
  ```

### `watch`

- **状态**：❌

## 用法

```ts
import { readdir, stat, readFile, writeFile, mkdir, rm } from 'hyper-fs'

// 读取目录
const files = await readdir('./src')

// 递归 + 返回文件类型
const entries = await readdir('./src', {
  recursive: true,
  withFileTypes: true,
})

// 读写文件
const content = await readFile('./package.json', { encoding: 'utf8' })
await writeFile('./output.txt', 'hello world')

// 文件信息
const s = await stat('./package.json')
console.log(s.size, s.isFile())

// 创建目录
await mkdir('./new-dir', { recursive: true })

// 删除
await rm('./temp', { recursive: true, force: true })
```

## 贡献

参阅 [CONTRIBUTING.md](./CONTRIBUTING.md) — 完整的开发指南，涵盖环境搭建、参考 Node.js 源码、编写 Rust 实现、测试与性能基准。

## 许可证

MIT
