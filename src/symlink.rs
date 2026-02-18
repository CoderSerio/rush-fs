use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use std::path::Path;

fn symlink_impl(target: String, path_str: String) -> Result<()> {
  let path = Path::new(&path_str);
  let target_path = Path::new(&target);

  if path.exists() || path.symlink_metadata().is_ok() {
    return Err(Error::from_reason(format!(
      "EEXIST: file already exists, symlink '{}' -> '{}'",
      target, path_str
    )));
  }

  #[cfg(unix)]
  {
    std::os::unix::fs::symlink(target_path, path).map_err(|e| {
      if e.kind() == std::io::ErrorKind::NotFound {
        Error::from_reason(format!(
          "ENOENT: no such file or directory, symlink '{}' -> '{}'",
          target, path_str
        ))
      } else {
        Error::from_reason(format!(
          "{}, symlink '{}' -> '{}'",
          e, target, path_str
        ))
      }
    })?;
  }

  #[cfg(not(unix))]
  {
    if target_path.is_dir() {
      std::os::windows::fs::symlink_dir(target_path, path)
    } else {
      std::os::windows::fs::symlink_file(target_path, path)
    }
    .map_err(|e| {
      if e.kind() == std::io::ErrorKind::NotFound {
        Error::from_reason(format!(
          "ENOENT: no such file or directory, symlink '{}' -> '{}'",
          target, path_str
        ))
      } else {
        Error::from_reason(format!(
          "{}, symlink '{}' -> '{}'",
          e, target, path_str
        ))
      }
    })?;
  }

  Ok(())
}

#[napi(js_name = "symlinkSync")]
pub fn symlink_sync(target: String, path: String) -> Result<()> {
  symlink_impl(target, path)
}

// ========= async version =========

pub struct SymlinkTask {
  pub target: String,
  pub path: String,
}

impl Task for SymlinkTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    symlink_impl(self.target.clone(), self.path.clone())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi(js_name = "symlink")]
pub fn symlink(target: String, path: String) -> AsyncTask<SymlinkTask> {
  AsyncTask::new(SymlinkTask { target, path })
}
