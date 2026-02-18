#![allow(dead_code)]

use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct Dirent {
  #[napi(readonly)]
  pub name: String,
  #[napi(readonly, js_name = "parentPath")]
  pub parent_path: String,
  // 1: file, 2: dir, 3: symlink, 4: block, 5: char, 6: fifo, 7: socket, 0: unknown
  pub(crate) file_type: u8,
}

#[napi]
impl Dirent {
  #[napi(js_name = "isFile")]
  pub fn is_file(&self) -> bool {
    self.file_type == 1
  }

  #[napi(js_name = "isDirectory")]
  pub fn is_directory(&self) -> bool {
    self.file_type == 2
  }

  #[napi(js_name = "isSymbolicLink")]
  pub fn is_symbolic_link(&self) -> bool {
    self.file_type == 3
  }

  #[napi(js_name = "isBlockDevice")]
  pub fn is_block_device(&self) -> bool {
    self.file_type == 4
  }

  #[napi(js_name = "isCharacterDevice")]
  pub fn is_character_device(&self) -> bool {
    self.file_type == 5
  }

  #[napi(js_name = "isFIFO")]
  pub fn is_fifo(&self) -> bool {
    self.file_type == 6
  }

  #[napi(js_name = "isSocket")]
  pub fn is_socket(&self) -> bool {
    self.file_type == 7
  }

  // Deprecated alias
  #[napi(getter)]
  pub fn path(&self) -> String {
    self.parent_path.clone()
  }
}

// S_IFMT mask and mode constants (matching Node.js / POSIX)
const S_IFMT: u32 = 0o170000;
const S_IFREG: u32 = 0o100000;
const S_IFDIR: u32 = 0o040000;
const S_IFLNK: u32 = 0o120000;
const S_IFBLK: u32 = 0o060000;
const S_IFCHR: u32 = 0o020000;
const S_IFIFO: u32 = 0o010000;
const S_IFSOCK: u32 = 0o140000;

#[napi]
pub struct Stats {
  #[napi(readonly)]
  pub dev: f64,
  #[napi(readonly)]
  pub mode: u32,
  #[napi(readonly)]
  pub nlink: f64,
  #[napi(readonly)]
  pub uid: u32,
  #[napi(readonly)]
  pub gid: u32,
  #[napi(readonly)]
  pub rdev: f64,
  #[napi(readonly)]
  pub blksize: f64,
  #[napi(readonly)]
  pub ino: f64,
  #[napi(readonly)]
  pub size: f64,
  #[napi(readonly)]
  pub blocks: f64,
  #[napi(readonly, js_name = "atimeMs")]
  pub atime_ms: f64,
  #[napi(readonly, js_name = "mtimeMs")]
  pub mtime_ms: f64,
  #[napi(readonly, js_name = "ctimeMs")]
  pub ctime_ms: f64,
  #[napi(readonly, js_name = "birthtimeMs")]
  pub birthtime_ms: f64,
}

#[napi]
impl Stats {
  #[napi(js_name = "isFile")]
  pub fn is_file(&self) -> bool {
    (self.mode & S_IFMT) == S_IFREG
  }

  #[napi(js_name = "isDirectory")]
  pub fn is_directory(&self) -> bool {
    (self.mode & S_IFMT) == S_IFDIR
  }

  #[napi(js_name = "isSymbolicLink")]
  pub fn is_symbolic_link(&self) -> bool {
    (self.mode & S_IFMT) == S_IFLNK
  }

  #[napi(js_name = "isBlockDevice")]
  pub fn is_block_device(&self) -> bool {
    (self.mode & S_IFMT) == S_IFBLK
  }

  #[napi(js_name = "isCharacterDevice")]
  pub fn is_character_device(&self) -> bool {
    (self.mode & S_IFMT) == S_IFCHR
  }

  #[napi(js_name = "isFIFO")]
  pub fn is_fifo(&self) -> bool {
    (self.mode & S_IFMT) == S_IFIFO
  }

  #[napi(js_name = "isSocket")]
  pub fn is_socket(&self) -> bool {
    (self.mode & S_IFMT) == S_IFSOCK
  }

  #[napi(getter)]
  pub fn atime(&self) -> f64 {
    self.atime_ms
  }

  #[napi(getter)]
  pub fn mtime(&self) -> f64 {
    self.mtime_ms
  }

  #[napi(getter)]
  pub fn ctime(&self) -> f64 {
    self.ctime_ms
  }

  #[napi(getter)]
  pub fn birthtime(&self) -> f64 {
    self.birthtime_ms
  }
}
