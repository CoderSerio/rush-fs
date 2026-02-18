use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use std::fs;
use std::path::Path;

fn generate_random_suffix() -> String {
  use std::time::{SystemTime, UNIX_EPOCH};
  let stack_var: u8 = 0;
  let addr = &stack_var as *const u8 as u128;
  let seed = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_nanos()
    ^ addr;
  let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let mut result = String::with_capacity(6);
  let mut val = seed;
  for _ in 0..6 {
    result.push(chars[(val % 62) as usize] as char);
    val /= 62;
  }
  result
}

fn mkdtemp_impl(prefix: String) -> Result<String> {
  if let Some(parent) = Path::new(&prefix).parent() {
    if !parent.as_os_str().is_empty() && !parent.exists() {
      return Err(Error::from_reason(format!(
        "ENOENT: no such file or directory, mkdtemp '{}'",
        prefix
      )));
    }
  }

  for _ in 0..3 {
    let suffix = generate_random_suffix();
    let dir_path = format!("{}{}", prefix, suffix);
    match fs::create_dir(&dir_path) {
      Ok(()) => return Ok(dir_path),
      Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
      Err(e) => {
        return Err(Error::from_reason(format!(
          "{}, mkdtemp '{}'",
          e, prefix
        )));
      }
    }
  }

  Err(Error::from_reason(format!(
    "EEXIST: could not create unique temporary directory, mkdtemp '{}'",
    prefix
  )))
}

#[napi(js_name = "mkdtempSync")]
pub fn mkdtemp_sync(prefix: String) -> Result<String> {
  mkdtemp_impl(prefix)
}

// ========= async version =========

pub struct MkdtempTask {
  pub prefix: String,
}

impl Task for MkdtempTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> Result<Self::Output> {
    mkdtemp_impl(self.prefix.clone())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "mkdtemp")]
pub fn mkdtemp(prefix: String) -> AsyncTask<MkdtempTask> {
  AsyncTask::new(MkdtempTask { prefix })
}
