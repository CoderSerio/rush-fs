#![allow(dead_code)]

use napi_derive::napi;

#[napi]
#[derive(Clone)]
pub struct Dirent {
  #[napi(readonly)]
  pub name: String,
  #[napi(readonly, js_name = "parentPath")]
  pub parent_path: String,
  // We store type info internally
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
