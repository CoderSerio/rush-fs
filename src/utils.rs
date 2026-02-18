#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

pub fn get_file_type_id(ft: &std::fs::FileType) -> u8 {
  if ft.is_file() {
    1
  } else if ft.is_dir() {
    2
  } else if ft.is_symlink() {
    3
  } else {
    #[cfg(unix)]
    {
      if ft.is_block_device() {
        return 4;
      }
      if ft.is_char_device() {
        return 5;
      }
      if ft.is_fifo() {
        return 6;
      }
      if ft.is_socket() {
        return 7;
      }
    }
    0
  }
}
