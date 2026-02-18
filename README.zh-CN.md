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
    encoding?: string; // ✅ (utf8, ascii, latin1, base64, base64url, hex)
    flag?: string; // ✅ (r, r+, w+, a+ 等)
  };
  ```
- **返回类型**：`string | Buffer`

### `writeFile`

- **Node.js 参数**：
  ```ts
  path: string; // ✅
  data: string | Buffer; // ✅
  options?: {
    encoding?: string; // ✅ (utf8, ascii, latin1, base64, base64url, hex)
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
    encoding?: string; // ✅ (utf8, ascii, latin1, base64, base64url, hex)
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

### `cp`

- **Node.js 参数**（Node 16.7+）：
  ```ts
  src: string; // ✅
  dest: string; // ✅
  options?: {
    recursive?: boolean; // ✅
    force?: boolean; // ✅（默认 true）
    errorOnExist?: boolean; // ✅
    preserveTimestamps?: boolean; // ✅
    dereference?: boolean; // ✅
    verbatimSymlinks?: boolean; // ✅
    concurrency?: number; // ✨
  };
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
    maxRetries?: number; // ✅
    retryDelay?: number; // ✅（默认 100ms）
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

### `symlink`

- **Node.js 参数**：
  ```ts
  target: string // ✅
  path: string // ✅
  ```

### `link`

- **Node.js 参数**：
  ```ts
  existingPath: string // ✅
  newPath: string // ✅
  ```

### `mkdtemp`

- **Node.js 参数**：
  ```ts
  prefix: string // ✅
  ```
- **返回类型**：`string`

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

## 性能基准

> 测试环境：Apple Silicon (arm64)，Node.js 24.0.2，release 构建（开启 LTO）。
> 运行 `pnpm build && pnpm bench` 可复现。

### Hyper-FS 显著更快的场景

这些场景中 Rust 的并行遍历和零拷贝 I/O 发挥了真正优势：

| 场景                                        | Node.js   | Hyper-FS | 加速比    |
| ------------------------------------------- | --------- | -------- | --------- |
| `readdir` 递归（node_modules，约 3 万条目） | 281 ms    | 23 ms    | **12x**   |
| `glob` 递归（`**/*.rs`）                    | 25 ms     | 1.46 ms  | **17x**   |
| `glob` 递归 vs fast-glob                    | 102 ms    | 1.46 ms  | **70x**   |
| `copyFile` 4 MB                             | 4.67 ms   | 0.09 ms  | **50x**   |
| `readFile` 4 MB utf8                        | 1.86 ms   | 0.92 ms  | **2x**    |
| `readFile` 64 KB utf8                       | 42 µs     | 18 µs    | **2.4x**  |
| `rm` 2000 个文件（4 线程）                  | 92 ms     | 53 ms    | **1.75x** |
| `access` R_OK（目录）                       | 4.18 µs   | 1.55 µs  | **2.7x**  |
| `cp` 500 文件平铺目录（4 线程）             | 86.45 ms  | 32.88 ms | **2.6x**  |
| `cp` 树形目录 ~363 节点（4 线程）           | 108.73 ms | 46.88 ms | **2.3x**  |

### 与 Node.js 持平的场景

单文件操作有约 0.3 µs 的 napi 桥接开销，整体表现基本一致：

| 场景                         | Node.js | Hyper-FS | 比率 |
| ---------------------------- | ------- | -------- | ---- |
| `stat`（单文件）             | 1.45 µs | 1.77 µs  | 1.2x |
| `readFile` 小文件（Buffer）  | 8.86 µs | 9.46 µs  | 1.1x |
| `writeFile` 小文件（string） | 74 µs   | 66 µs    | 0.9x |
| `writeFile` 小文件（Buffer） | 115 µs  | 103 µs   | 0.9x |
| `appendFile`                 | 30 µs   | 27 µs    | 0.9x |

### Node.js 更快的场景

极轻量级的内置调用，napi 开销占比较大：

| 场景                       | Node.js | Hyper-FS | 说明                     |
| -------------------------- | ------- | -------- | ------------------------ |
| `existsSync`（已存在文件） | 444 ns  | 1.34 µs  | Node.js 内部有 fast path |
| `accessSync` F_OK          | 456 ns  | 1.46 µs  | 同上——napi 开销占主导    |
| `writeFile` 4 MB string    | 2.93 ms | 5.69 ms  | 大字符串跨 napi 桥传输   |

### 并行支持

Hyper-FS 在文件系统遍历类操作中使用多线程并行：

| API               | 并行库                                                                    | `concurrency` 选项 | 默认值 |
| ----------------- | ------------------------------------------------------------------------- | ------------------ | ------ |
| `readdir`（递归） | [jwalk](https://github.com/Byron/jwalk)                                   | ✅                 | auto   |
| `glob`            | [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) | ✅                 | 4      |
| `rm`（递归）      | [rayon](https://github.com/rayon-rs/rayon)                                | ✅                 | 1      |
| `cp`（递归）      | [rayon](https://github.com/rayon-rs/rayon)                                | ✅                 | 1      |

单文件操作（`stat`、`readFile`、`writeFile`、`chmod` 等）是原子系统调用，不适用并行化。

### 核心结论

**Hyper-FS 在递归/批量文件系统操作上表现卓越**（readdir、glob、rm、cp），Rust 的并行遍历器带来 2–70 倍加速。单文件操作与 Node.js 基本持平。napi 桥接带来固定约 0.3 µs 的每次调用开销，仅在亚微秒级操作（如 `existsSync`）中有感知。

**`cp` 基准详情**（Apple Silicon，release 构建）：

| 场景                                  | Node.js   | Hyper-FS 1 线程 | Hyper-FS 4 线程 | Hyper-FS 8 线程 |
| ------------------------------------- | --------- | --------------- | --------------- | --------------- |
| 平铺目录（500 文件）                  | 86.45 ms  | 61.56 ms        | 32.88 ms        | 36.67 ms        |
| 树形目录（宽度=4，深度=3，~84 节点）  | 23.80 ms  | 16.94 ms        | 10.62 ms        | 9.76 ms         |
| 树形目录（宽度=3，深度=5，~363 节点） | 108.73 ms | 75.39 ms        | 46.88 ms        | 46.18 ms        |

`cp` 的最优并发数在 Apple Silicon 上为 **4 线程**——超过后受 I/O 带宽限制，收益趋于平稳。

## 贡献

参阅 [CONTRIBUTING.md](./CONTRIBUTING.md) — 完整的开发指南，涵盖环境搭建、参考 Node.js 源码、编写 Rust 实现、测试与性能基准。

## 许可证

MIT
