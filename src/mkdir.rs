use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

#[napi(object)]
#[derive(Clone)]
pub struct MkdirOptions {
  pub recursive: Option<bool>,
  pub mode: Option<u32>,
}

fn mkdir_impl(path_str: String, options: Option<MkdirOptions>) -> Result<Option<String>> {
  let path = Path::new(&path_str);
  let opts = options.unwrap_or(MkdirOptions {
    recursive: None,
    mode: None,
  });
  let recursive = opts.recursive.unwrap_or(false);

  #[cfg(unix)]
  let _mode = opts.mode.unwrap_or(0o777);

  if recursive {
    // Node.js returns the first directory path created, or undefined if it already existed
    if path.exists() {
      if path.is_dir() {
        return Ok(None);
      }
      return Err(Error::from_reason(format!(
        "EEXIST: file already exists, mkdir '{}'",
        path.to_string_lossy()
      )));
    }

    // Find the first ancestor that doesn't exist
    let mut ancestors = vec![];
    let mut current = path.to_path_buf();
    while !current.exists() {
      ancestors.push(current.clone());
      match current.parent() {
        Some(parent) => current = parent.to_path_buf(),
        None => break,
      }
    }

    if current.exists() && !current.is_dir() {
      return Err(Error::from_reason(format!(
        "ENOTDIR: not a directory, mkdir '{}'",
        path.to_string_lossy()
      )));
    }

    fs::create_dir_all(path).map_err(|e| mkdir_error(path, e))?;

    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;
      for ancestor in &ancestors {
        let _ = fs::set_permissions(ancestor, fs::Permissions::from_mode(_mode));
      }
    }

    let first_created = ancestors.last().map(|p| p.to_string_lossy().to_string());
    Ok(first_created)
  } else {
    fs::create_dir(path).map_err(|e| mkdir_error(path, e))?;

    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;
      let _ = fs::set_permissions(path, fs::Permissions::from_mode(_mode));
    }

    Ok(None)
  }
}

fn mkdir_error(path: &Path, err: std::io::Error) -> Error {
  let path_display = path.to_string_lossy();
  match err.kind() {
    ErrorKind::NotFound => Error::from_reason(format!(
      "ENOENT: no such file or directory, mkdir '{}'",
      path_display
    )),
    ErrorKind::AlreadyExists => Error::from_reason(format!(
      "EEXIST: file already exists, mkdir '{}'",
      path_display
    )),
    ErrorKind::PermissionDenied => Error::from_reason(format!(
      "EACCES: permission denied, mkdir '{}'",
      path_display
    )),
    ErrorKind::NotADirectory => Error::from_reason(format!(
      "ENOTDIR: not a directory, mkdir '{}'",
      path_display
    )),
    _ => Error::from_reason(format!("{}, mkdir '{}'", err, path_display)),
  }
}

#[napi(js_name = "mkdirSync")]
pub fn mkdir_sync(path: String, options: Option<MkdirOptions>) -> Result<Option<String>> {
  mkdir_impl(path, options)
}

// ========= async version =========

pub struct MkdirTask {
  pub path: String,
  pub options: Option<MkdirOptions>,
}

impl Task for MkdirTask {
  type Output = Option<String>;
  type JsValue = Option<String>;

  fn compute(&mut self) -> Result<Self::Output> {
    mkdir_impl(self.path.clone(), self.options.clone())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "mkdir")]
pub fn mkdir(path: String, options: Option<MkdirOptions>) -> AsyncTask<MkdirTask> {
  AsyncTask::new(MkdirTask { path, options })
}
