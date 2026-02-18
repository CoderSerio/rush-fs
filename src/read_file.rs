use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use std::fs;
use std::path::Path;

#[napi(object)]
#[derive(Clone)]
pub struct ReadFileOptions {
  pub encoding: Option<String>,
  pub flag: Option<String>,
}

fn read_file_impl(
  path_str: String,
  options: Option<ReadFileOptions>,
) -> Result<Either<String, Buffer>> {
  let path = Path::new(&path_str);
  let opts = options.unwrap_or(ReadFileOptions {
    encoding: None,
    flag: None,
  });

  let data = fs::read(path).map_err(|e| {
    if e.kind() == std::io::ErrorKind::NotFound {
      Error::from_reason(format!(
        "ENOENT: no such file or directory, open '{}'",
        path.to_string_lossy()
      ))
    } else if e.kind() == std::io::ErrorKind::PermissionDenied {
      Error::from_reason(format!(
        "EACCES: permission denied, open '{}'",
        path.to_string_lossy()
      ))
    } else {
      Error::from_reason(e.to_string())
    }
  })?;

  match opts.encoding.as_deref() {
    Some("utf8" | "utf-8") => {
      let s = String::from_utf8(data)
        .map_err(|e| Error::from_reason(format!("Invalid UTF-8: {}", e)))?;
      Ok(Either::A(s))
    }
    Some(enc) => Err(Error::from_reason(format!(
      "Unknown encoding: {}",
      enc
    ))),
    None => Ok(Either::B(Buffer::from(data))),
  }
}

#[napi(js_name = "readFileSync")]
pub fn read_file_sync(
  path: String,
  options: Option<ReadFileOptions>,
) -> Result<Either<String, Buffer>> {
  read_file_impl(path, options)
}

// ========= async version =========

pub struct ReadFileTask {
  pub path: String,
  pub options: Option<ReadFileOptions>,
}

impl Task for ReadFileTask {
  type Output = Either<String, Buffer>;
  type JsValue = Either<String, Buffer>;

  fn compute(&mut self) -> Result<Self::Output> {
    read_file_impl(self.path.clone(), self.options.clone())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "readFile")]
pub fn read_file(path: String, options: Option<ReadFileOptions>) -> AsyncTask<ReadFileTask> {
  AsyncTask::new(ReadFileTask { path, options })
}
