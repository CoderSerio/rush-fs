use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

// Node.js access mode constants
pub const F_OK: u32 = 0;
pub const R_OK: u32 = 4;
pub const W_OK: u32 = 2;
pub const X_OK: u32 = 1;

fn access_impl(path_str: String, mode: Option<u32>) -> Result<()> {
  let path = Path::new(&path_str);
  let mode = mode.unwrap_or(F_OK);

  let meta = std::fs::symlink_metadata(path).map_err(|_| {
    Error::from_reason(format!(
      "ENOENT: no such file or directory, access '{}'",
      path.to_string_lossy()
    ))
  })?;

  // F_OK: just check existence (already passed above)
  if mode == F_OK {
    return Ok(());
  }

  #[cfg(unix)]
  {
    let file_mode = meta.mode();
    let uid = unsafe { libc::getuid() };
    let gid = unsafe { libc::getgid() };
    let is_owner = uid == meta.uid();
    let is_group = gid == meta.gid();

    let check = |flag: u32, owner_bit: u32, group_bit: u32, other_bit: u32| -> bool {
      if mode & flag == 0 {
        return true;
      }
      if uid == 0 {
        // root can read/write anything; execute requires at least one execute bit
        if flag == X_OK {
          return file_mode & (owner_bit | group_bit | other_bit) != 0;
        }
        return true;
      }
      if is_owner && (file_mode & owner_bit != 0) {
        return true;
      }
      if is_group && (file_mode & group_bit != 0) {
        return true;
      }
      file_mode & other_bit != 0
    };

    let ok = check(R_OK, 0o400, 0o040, 0o004)
      && check(W_OK, 0o200, 0o020, 0o002)
      && check(X_OK, 0o100, 0o010, 0o001);

    if !ok {
      return Err(Error::from_reason(format!(
        "EACCES: permission denied, access '{}'",
        path.to_string_lossy()
      )));
    }
  }

  #[cfg(not(unix))]
  {
    if mode & W_OK != 0 && meta.permissions().readonly() {
      return Err(Error::from_reason(format!(
        "EACCES: permission denied, access '{}'",
        path.to_string_lossy()
      )));
    }
  }

  Ok(())
}

#[napi(js_name = "accessSync")]
pub fn access_sync(path: String, mode: Option<u32>) -> Result<()> {
  access_impl(path, mode)
}

// ========= async version =========

pub struct AccessTask {
  pub path: String,
  pub mode: Option<u32>,
}

impl Task for AccessTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    access_impl(self.path.clone(), self.mode)
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi(js_name = "access")]
pub fn access(path: String, mode: Option<u32>) -> AsyncTask<AccessTask> {
  AsyncTask::new(AccessTask { path, mode })
}
