use jwalk::{Parallelism, WalkDir};
use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
// use std::fs;
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
  pub recursive: Option<bool>,
}

// TODO: Make it similar to node::fs.Dirent
#[napi(object)]
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
    recursive: Some(false),
  });
  let skip_hidden = opts.skip_hidden.unwrap_or(false);
  // let recursive = opts.recursive.unwrap_or(false);

  let root_path = if path.is_empty() {
    Path::new(".")
  } else {
    Path::new(&path)
  };

  // OPTIMIZATION: Use std::fs::read_dir for non-recursive calls
  // This avoids the overhead of jwalk's thread pool for simple flat listings
  // if !recursive {
  //   let entries = fs::read_dir(root_path).map_err(|e| Error::from_reason(e.to_string()))?;

  //   let mut result = Vec::new();
  //   // Pre-allocate based on an estimate? fs::read_dir doesn't give size hint easily

  //   for entry in entries {
  //     let entry = entry.map_err(|e| Error::from_reason(e.to_string()))?;
  //     let file_name = entry.file_name();
  //     let file_name_str = file_name.to_string_lossy();

  //     if skip_hidden && file_name_str.starts_with('.') {
  //       continue;
  //     }

  //     let file_type = entry
  //       .file_type()
  //       .map_err(|e| Error::from_reason(e.to_string()))?;

  //     result.push(Dirent {
  //       name: file_name_str.to_string(),
  //       path: entry.path().to_string_lossy().to_string(),
  //       is_dir: file_type.is_dir(),
  //     });
  //   }

  //   return Ok(result);
  // }

  // Recursive path using jwalk
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
      // jwalk's max_depth(1) includes the root directory itself, which we need to filter out
      // to match the behavior of fs.readdir, keeping only the children.
      if e.depth() == 0 {
        return false;
      }
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
