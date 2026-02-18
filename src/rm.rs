use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use rayon::prelude::*;
use std::fs;
use std::path::Path;

// nodejs rm jsdoc:
/**
 * Asynchronously removes files and
 * directories (modeled on the standard POSIX `rm` utility).
 * @param {string | Buffer | URL} path
 * @param {{
 *   force?: boolean;
 *   maxRetries?: number;
 *   recursive?: boolean;
 *   retryDelay?: number;
 *   }} [options]
 * @param {(err?: Error) => any} callback
 * @returns {void}
 */

#[napi(object)]
#[derive(Clone)]
pub struct RmOptions {
  pub force: Option<bool>,
  pub max_retries: Option<u32>,
  pub recursive: Option<bool>,
  pub retry_delay: Option<u32>,
  pub concurrency: Option<u32>,
}

fn remove_recursive(path: &Path, opts: &RmOptions) -> Result<()> {
  let meta = fs::symlink_metadata(path).map_err(|e| Error::from_reason(e.to_string()))?;

  if meta.is_dir() {
    if opts.recursive.unwrap_or(false) {
      let entries_iter = fs::read_dir(path).map_err(|e| Error::from_reason(e.to_string()))?;

      let concurrency = opts.concurrency.unwrap_or(0);
      if concurrency > 1 {
        let entries: Vec<_> = entries_iter
          .collect::<std::io::Result<_>>()
          .map_err(|e| Error::from_reason(e.to_string()))?;

        entries
          .par_iter()
          .try_for_each(|entry| -> Result<()> { remove_recursive(&entry.path(), opts) })?;
      } else {
        for entry in entries_iter {
          let entry = entry.map_err(|e| Error::from_reason(e.to_string()))?;
          remove_recursive(&entry.path(), opts)?;
        }
      }

      fs::remove_dir(path).map_err(|e| Error::from_reason(e.to_string()))?;
    } else {
      fs::remove_dir(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::AlreadyExists || e.to_string().contains("not empty") {
          Error::from_reason(format!(
            "ENOTEMPTY: directory not empty, rm '{}'",
            path.to_string_lossy()
          ))
        } else {
          Error::from_reason(e.to_string())
        }
      })?;
    }
  } else {
    fs::remove_file(path).map_err(|e| Error::from_reason(e.to_string()))?;
  }
  Ok(())
}

fn remove_with_retry(path: &Path, opts: &RmOptions) -> Result<()> {
  let max_retries = opts.max_retries.unwrap_or(0) as usize;
  let retry_delay = opts.retry_delay.unwrap_or(100) as u64;

  let mut last_err = None;
  for attempt in 0..=max_retries {
    if attempt > 0 {
      std::thread::sleep(std::time::Duration::from_millis(retry_delay));
    }
    match remove_recursive(path, opts) {
      Ok(()) => return Ok(()),
      Err(e) => last_err = Some(e),
    }
  }
  Err(last_err.unwrap())
}

fn remove(path_str: String, options: Option<RmOptions>) -> Result<()> {
  let path = Path::new(&path_str);

  let opts = options.unwrap_or(RmOptions {
    force: Some(false),
    recursive: Some(false),
    max_retries: None,
    retry_delay: None,
    concurrency: None,
  });
  let force = opts.force.unwrap_or(false);

  if !path.exists() {
    if force {
      return Ok(());
    }
    return Err(Error::from_reason(format!(
      "ENOENT: no such file or directory, rm '{}'",
      path.to_string_lossy()
    )));
  }

  let max_retries = opts.max_retries.unwrap_or(0);
  if max_retries > 0 {
    remove_with_retry(path, &opts)
  } else {
    remove_recursive(path, &opts)
  }
}

// ========= async version =========

pub struct RmTask {
  pub path: String,
  pub options: Option<RmOptions>,
}

impl Task for RmTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    remove(self.path.clone(), self.options.clone())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi(js_name = "rm")]
pub fn rm(path: String, options: Option<RmOptions>) -> AsyncTask<RmTask> {
  AsyncTask::new(RmTask { path, options })
}

#[napi(js_name = "rmSync")]
pub fn rm_sync(path: String, options: Option<RmOptions>) -> Result<()> {
  remove(path, options)
}
