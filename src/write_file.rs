use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

#[napi(object)]
#[derive(Clone)]
pub struct WriteFileOptions {
  pub encoding: Option<String>,
  pub mode: Option<u32>,
  pub flag: Option<String>,
}

fn write_file_impl(path_str: String, data: Either<String, Buffer>, options: Option<WriteFileOptions>) -> Result<()> {
  let path = Path::new(&path_str);
  let opts = options.unwrap_or(WriteFileOptions {
    encoding: None,
    mode: None,
    flag: None,
  });

  let flag = opts.flag.as_deref().unwrap_or("w");
  let bytes: Vec<u8> = match &data {
    Either::A(s) => s.as_bytes().to_vec(),
    Either::B(b) => b.to_vec(),
  };

  let mut open_opts = OpenOptions::new();
  match flag {
    "w" => { open_opts.write(true).create(true).truncate(true); }
    "wx" | "xw" => { open_opts.write(true).create_new(true); }
    "a" => { open_opts.append(true).create(true); }
    "ax" | "xa" => { open_opts.append(true).create_new(true); }
    _ => { open_opts.write(true).create(true).truncate(true); }
  }

  let mut file = open_opts.open(path).map_err(|e| {
    if e.kind() == std::io::ErrorKind::NotFound {
      Error::from_reason(format!(
        "ENOENT: no such file or directory, open '{}'",
        path.to_string_lossy()
      ))
    } else if e.kind() == std::io::ErrorKind::AlreadyExists {
      Error::from_reason(format!(
        "EEXIST: file already exists, open '{}'",
        path.to_string_lossy()
      ))
    } else {
      Error::from_reason(e.to_string())
    }
  })?;

  file.write_all(&bytes).map_err(|e| Error::from_reason(e.to_string()))?;

  #[cfg(unix)]
  if let Some(mode) = opts.mode {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(mode));
  }

  Ok(())
}

#[napi(js_name = "writeFileSync")]
pub fn write_file_sync(
  path: String,
  data: Either<String, Buffer>,
  options: Option<WriteFileOptions>,
) -> Result<()> {
  write_file_impl(path, data, options)
}

// ========= async version =========

pub struct WriteFileTask {
  pub path: String,
  pub data: Vec<u8>,
  pub options: Option<WriteFileOptions>,
}

impl Task for WriteFileTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let data_clone = self.data.clone();
    write_file_impl(
      self.path.clone(),
      Either::B(Buffer::from(data_clone)),
      self.options.clone(),
    )
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi(js_name = "writeFile")]
pub fn write_file(
  path: String,
  data: Either<String, Buffer>,
  options: Option<WriteFileOptions>,
) -> AsyncTask<WriteFileTask> {
  let bytes = match &data {
    Either::A(s) => s.as_bytes().to_vec(),
    Either::B(b) => b.to_vec(),
  };
  AsyncTask::new(WriteFileTask {
    path,
    data: bytes,
    options,
  })
}

// appendFile is writeFile with flag='a'

fn append_file_impl(path_str: String, data: Either<String, Buffer>, options: Option<WriteFileOptions>) -> Result<()> {
  let opts = options.unwrap_or(WriteFileOptions {
    encoding: None,
    mode: None,
    flag: None,
  });
  let merged = WriteFileOptions {
    encoding: opts.encoding,
    mode: opts.mode,
    flag: Some(opts.flag.unwrap_or_else(|| "a".to_string())),
  };
  write_file_impl(path_str, data, Some(merged))
}

#[napi(js_name = "appendFileSync")]
pub fn append_file_sync(
  path: String,
  data: Either<String, Buffer>,
  options: Option<WriteFileOptions>,
) -> Result<()> {
  append_file_impl(path, data, options)
}

pub struct AppendFileTask {
  pub path: String,
  pub data: Vec<u8>,
  pub options: Option<WriteFileOptions>,
}

impl Task for AppendFileTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let data_clone = self.data.clone();
    append_file_impl(
      self.path.clone(),
      Either::B(Buffer::from(data_clone)),
      self.options.clone(),
    )
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi(js_name = "appendFile")]
pub fn append_file(
  path: String,
  data: Either<String, Buffer>,
  options: Option<WriteFileOptions>,
) -> AsyncTask<AppendFileTask> {
  let bytes = match &data {
    Either::A(s) => s.as_bytes().to_vec(),
    Either::B(b) => b.to_vec(),
  };
  AsyncTask::new(AppendFileTask {
    path,
    data: bytes,
    options,
  })
}
