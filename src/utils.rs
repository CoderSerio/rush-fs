#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

pub fn get_file_type_id(ft: &std::fs::FileType) -> u8 {
  if ft.is_file() {
    1
  } else if ft.is_dir() {
    2
  } else if ft.is_symlink() {
    3
  } else if cfg!(unix) && ft.is_block_device() {
    4
  } else if cfg!(unix) && ft.is_char_device() {
    5
  } else if cfg!(unix) && ft.is_fifo() {
    6
  } else if cfg!(unix) && ft.is_socket() {
    7
  } else {
    0
  }
}
