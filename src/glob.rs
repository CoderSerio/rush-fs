use crate::types::Dirent;
use crate::utils::get_file_type_id;
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[napi(object)]
#[derive(Clone)]
pub struct GlobOptions {
  pub cwd: Option<String>,
  pub with_file_types: Option<bool>,
  pub exclude: Option<Vec<String>>,
  pub concurrency: Option<u32>,
  pub git_ignore: Option<bool>,
}

#[napi(js_name = "globSync")]
pub fn glob_sync(
  pattern: String,
  options: Option<GlobOptions>,
) -> Result<Either<Vec<String>, Vec<Dirent>>> {
  let opts = options.unwrap_or(GlobOptions {
    cwd: None,
    with_file_types: None,
    exclude: None,
    concurrency: None,
    git_ignore: None,
  });

  let cwd = opts.cwd.unwrap_or_else(|| ".".to_string());
  let with_file_types = opts.with_file_types.unwrap_or(false);
  let concurrency = opts.concurrency.unwrap_or(4) as usize;

  // Build match rules (Matcher)
  // ignore crate handles glob patterns via override
  let mut override_builder = OverrideBuilder::new(&cwd);
  override_builder
    .add(&pattern)
    .map_err(|e| Error::from_reason(e.to_string()))?;

  if let Some(excludes) = opts.exclude {
    for ex in excludes {
      // ignore crate exclusions usually start with !, or use builder.add_ignore
      // For simplicity here, we assume exclude is also a glob pattern, prepend !
      override_builder
        .add(&format!("!{}", ex))
        .map_err(|e| Error::from_reason(e.to_string()))?;
    }
  }

  let overrides = override_builder
    .build()
    .map_err(|e| Error::from_reason(e.to_string()))?;

  let mut builder = WalkBuilder::new(&cwd);
  builder
    .overrides(overrides) // Apply glob patterns
    .standard_filters(opts.git_ignore.unwrap_or(true)) // Automatically handle .gitignore, .ignore etc
    .threads(concurrency); // Core: Enable multithreading with one line!

  // We use two vectors to avoid enum overhead in the lock if possible, but Mutex<Vec<T>> is easier
  let result_strings = Arc::new(Mutex::new(Vec::new()));
  let result_dirents = Arc::new(Mutex::new(Vec::new()));

  let result_strings_clone = result_strings.clone();
  let result_dirents_clone = result_dirents.clone();

  let root_path = Path::new(&cwd).to_path_buf();

  builder.build_parallel().run(move || {
    let result_strings = result_strings_clone.clone();
    let result_dirents = result_dirents_clone.clone();
    let root = root_path.clone();

    Box::new(move |entry| {
      match entry {
        Ok(entry) => {
          // WalkBuilder's overrides already help us include or exclude
          // However, ignore crate returns directories too if they match.
          // Usually globs like "**/*.js" only match files.
          // But "src/*" matches both.
          // Let's keep logic: if it matches, we keep it.
          // But typically glob returns files.
          // If the user wants directories, pattern usually ends with /.
          // Standard glob behavior varies.
          // For now, let's include everything that matches the pattern overrides.

          if entry.depth() == 0 {
            return ignore::WalkState::Continue;
          }

          if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            return ignore::WalkState::Continue;
          }

          let path = entry.path();
          // Make path relative to cwd if possible, similar to node-glob
          let relative_path = path.strip_prefix(&root).unwrap_or(path);
          let relative_path_str = relative_path.to_string_lossy().to_string();

          if with_file_types {
            let mut lock = result_dirents.lock().unwrap();

            // Convert to Dirent
            let parent_path = relative_path
              .parent()
              .unwrap_or(Path::new(""))
              .to_string_lossy()
              .to_string();
            let name = relative_path
              .file_name()
              .unwrap_or_default()
              .to_string_lossy()
              .to_string();

            let file_type = if let Some(ft) = entry.file_type() {
              get_file_type_id(&ft)
            } else {
              0 // Unknown
            };

            lock.push(Dirent {
              name,
              parent_path,
              file_type,
            });
          } else {
            let mut lock = result_strings.lock().unwrap();
            lock.push(relative_path_str);
          }
        }
        Err(_) => {
          // Handle errors or ignore permission errors
        }
      }
      ignore::WalkState::Continue
    })
  });

  if with_file_types {
    let final_results = Arc::try_unwrap(result_dirents)
      .map_err(|_| Error::from_reason("Lock error"))?
      .into_inner()
      .map_err(|_| Error::from_reason("Mutex error"))?;
    Ok(Either::B(final_results))
  } else {
    let final_results = Arc::try_unwrap(result_strings)
      .map_err(|_| Error::from_reason("Lock error"))?
      .into_inner()
      .map_err(|_| Error::from_reason("Mutex error"))?;
    Ok(Either::A(final_results))
  }
}

// ===== Async version =====
pub struct GlobTask {
  pub pattern: String,
  pub options: Option<GlobOptions>,
}

impl Task for GlobTask {
  type Output = Either<Vec<String>, Vec<Dirent>>;
  type JsValue = Either<Vec<String>, Vec<Dirent>>;

  fn compute(&mut self) -> Result<Self::Output> {
    glob_sync(self.pattern.clone(), self.options.clone())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "glob")]
pub fn glob(pattern: String, options: Option<GlobOptions>) -> AsyncTask<GlobTask> {
  AsyncTask::new(GlobTask { pattern, options })
}
