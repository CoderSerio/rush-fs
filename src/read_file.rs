use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use std::fs;
use std::path::Path;

fn decode_data(data: Vec<u8>, encoding: Option<&str>) -> Result<Either<String, Buffer>> {
  match encoding {
    Some("utf8" | "utf-8") => {
      let s =
        String::from_utf8(data).map_err(|e| Error::from_reason(format!("Invalid UTF-8: {}", e)))?;
      Ok(Either::A(s))
    }
    Some("ascii") => {
      let s: String = data.iter().map(|&b| (b & 0x7f) as char).collect();
      Ok(Either::A(s))
    }
    Some("latin1" | "binary") => {
      let s: String = data.iter().map(|&b| b as char).collect();
      Ok(Either::A(s))
    }
    Some("base64") => Ok(Either::A(base64_encode(&data, false))),
    Some("base64url") => Ok(Either::A(base64_encode(&data, true))),
    Some("hex") => {
      let s: String = data.iter().map(|b| format!("{:02x}", b)).collect();
      Ok(Either::A(s))
    }
    Some(enc) => Err(Error::from_reason(format!("Unknown encoding: {}", enc))),
    None => Ok(Either::B(Buffer::from(data))),
  }
}

fn base64_encode(data: &[u8], url_safe: bool) -> String {
  const STD: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
  const URL: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
  let table = if url_safe { URL } else { STD };

  let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
  let chunks = data.chunks(3);
  for chunk in chunks {
    let b0 = chunk[0] as u32;
    let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
    let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
    let triple = (b0 << 16) | (b1 << 8) | b2;

    result.push(table[((triple >> 18) & 0x3F) as usize] as char);
    result.push(table[((triple >> 12) & 0x3F) as usize] as char);
    if chunk.len() > 1 {
      result.push(table[((triple >> 6) & 0x3F) as usize] as char);
    } else if !url_safe {
      result.push('=');
    }
    if chunk.len() > 2 {
      result.push(table[(triple & 0x3F) as usize] as char);
    } else if !url_safe {
      result.push('=');
    }
  }
  result
}

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

  let flag = opts.flag.as_deref().unwrap_or("r");

  let mut open_opts = fs::OpenOptions::new();
  match flag {
    "r" => {
      open_opts.read(true);
    }
    "rs" | "sr" => {
      open_opts.read(true);
    }
    "r+" => {
      open_opts.read(true).write(true);
    }
    "rs+" | "sr+" => {
      open_opts.read(true).write(true);
    }
    "a+" => {
      open_opts.read(true).append(true).create(true);
    }
    "ax+" | "xa+" => {
      open_opts.read(true).append(true).create_new(true);
    }
    "w+" => {
      open_opts.read(true).write(true).create(true).truncate(true);
    }
    "wx+" | "xw+" => {
      open_opts.read(true).write(true).create_new(true);
    }
    _ => {
      open_opts.read(true);
    }
  }

  let mut file = open_opts.open(path).map_err(|e| {
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
    } else if e.kind() == std::io::ErrorKind::AlreadyExists {
      Error::from_reason(format!(
        "EEXIST: file already exists, open '{}'",
        path.to_string_lossy()
      ))
    } else {
      Error::from_reason(e.to_string())
    }
  })?;

  use std::io::Read;
  let mut data = Vec::new();
  file
    .read_to_end(&mut data)
    .map_err(|e| Error::from_reason(e.to_string()))?;

  decode_data(data, opts.encoding.as_deref())
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
