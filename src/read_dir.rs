use jwalk::{Parallelism, WalkDir};
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use std::path::Path;

// basic usage
// ls('./node_modules')

// advanced usage
// readdirSync('./src', {
//   recursive: true,
//   concurrency: 8,
//   ignore: ['.git'],
//   returnType: 'Tree'
// });

#[napi(object)]
#[derive(Clone)]
pub struct ReaddirOptions {
  pub skip_hidden: Option<bool>,
  pub concurrency: Option<u32>,
}

#[napi(object)] // Similar to fs.Dirent
#[derive(Clone)]
pub struct Dirent {
  pub name: String,
  pub path: String,
  pub is_dir: bool,
}

// #[napi] // marco: expose the function to Node
fn ls(path: String, options: Option<ReaddirOptions>) -> Result<Vec<Dirent>> {
  if !Path::new(&path).exists() {
    return Err(Error::from_reason(format!(
      "ENOENT: no such file or directory, scandir '{}'",
      path
    )));
  }

  let opts = options.unwrap_or(ReaddirOptions {
    skip_hidden: Some(false),
    concurrency: None,
  });
  let skip_hidden = opts.skip_hidden.unwrap_or(false);
  let root_path = if path.is_empty() {
    Path::new(".")
  } else {
    Path::new(&path)
  };
  let walk_dir = WalkDir::new(root_path)
    .skip_hidden(skip_hidden)
    .parallelism(match opts.concurrency {
      Some(n) => Parallelism::RayonNewPool(n as usize),
      None => Parallelism::RayonNewPool(0),
    });

  // TODO: maybe we'd better limit the max number of threads?

  let result = walk_dir
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| {
      if skip_hidden {
        !e.file_name().to_string_lossy().starts_with('.')
      } else {
        true
      }
    })
    .map(|e| Dirent {
      name: e.file_name().to_string_lossy().to_string(),
      path: e.path().to_string_lossy().to_string(),
      is_dir: e.file_type().is_dir(),
    })
    .collect();

  Ok(result)
}

#[napi(js_name = "readdirSync")]
pub fn readdir_sync(path: String, options: Option<ReaddirOptions>) -> Result<Vec<Dirent>> {
  ls(path, options)
}

// ========= async version =========

pub struct ReaddirTask {
  pub path: String,
  pub options: Option<ReaddirOptions>,
}

impl Task for ReaddirTask {
  type Output = Vec<Dirent>;
  type JsValue = Vec<Dirent>;

  fn compute(&mut self) -> Result<Self::Output> {
    ls(self.path.clone(), self.options.clone())
  }
  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "readdir")]
pub fn readdir(path: String, options: Option<ReaddirOptions>) -> AsyncTask<ReaddirTask> {
  AsyncTask::new(ReaddirTask { path, options })
}
