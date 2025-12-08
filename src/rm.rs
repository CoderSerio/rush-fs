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

/// Recursively removes the file or directory at `path` according to `opts`.
///
/// If `path` is a directory and `opts.recursive` is `true`, the directory's
/// contents are removed first (optionally in parallel when `opts.concurrency`
/// is greater than 1) and then the directory itself is removed. If the path
/// is a directory and `opts.recursive` is `false`, the function attempts to
/// remove the directory and maps "directory not empty" conditions to an
/// `ENOTEMPTY`-style error message. If the path is not a directory, the file
/// is removed.
///
/// # Parameters
///
/// - `path`: filesystem path to remove.
/// - `opts`: removal options; `recursive` controls directory recursion and
///   `concurrency` (when > 1) enables parallel traversal.
///
/// # Returns
///
/// `Ok(())` on successful removal, or an `napi::Error` created from the
/// underlying I/O error on failure.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
///
/// let opts = RmOptions {
///     force: None,
///     max_retries: None,
///     recursive: Some(true),
///     retry_delay: None,
///     concurrency: None,
/// };
///
/// // Remove the current directory contents (for demonstration; be careful)
/// let _ = remove_recursive(Path::new("."), &opts).unwrap();
/// ```
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

/// Remove the filesystem entry at `path_str` using the provided rm-style options.
///
/// The empty string for `path_str` is treated as `"."`. When `options` is `None`,
/// defaults are used (force = false, recursive = false, other fields unset).
/// If `options.force` is true and the path does not exist, the call succeeds silently.
///
/// # Parameters
///
/// - `path_str` — Path to remove; `""` is interpreted as the current directory (`"."`).
/// - `options` — Optional `RmOptions` controlling behavior (e.g. `force`, `recursive`, `concurrency`).
///
/// # Returns
///
/// `Ok(())` on successful removal, or an `Err` containing a `napi::Error` describing the failure.
///
/// # Examples
///
/// ```
/// // remove a single file (non-recursive)
/// let _ = remove("tmp/file.txt".to_string(), None);
///
/// // remove or ignore missing path
/// let opts = RmOptions { force: Some(true), recursive: Some(false), max_retries: None, retry_delay: None, concurrency: None };
/// let _ = remove("tmp/missing".to_string(), Some(opts));
/// ```
fn remove(path_str: String, options: Option<RmOptions>) -> Result<()> {
  let search_path_str = if path_str.is_empty() { "." } else { &path_str };
  let path = Path::new(search_path_str);

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
      // If force is true, silently succeed when path doesn't exist
      return Ok(());
    }
    return Err(Error::from_reason(format!(
      "ENOENT: no such file or directory, rm '{}'",
      path.to_string_lossy()
    )));
  }

  remove_recursive(path, &opts)
}

// ========= async version =========

pub struct RmTask {
  pub path: String,
  pub options: Option<RmOptions>,
}

impl Task for RmTask {
  type Output = ();
  type JsValue = ();

  /// Performs the removal operation described by this task.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// let mut task = RmTask { path: ".".to_string(), options: None };
  /// let result = task.compute();
  /// assert!(result.is_ok());
  /// ```
  fn compute(&mut self) -> Result<Self::Output> {
    remove(self.path.clone(), self.options.clone())
  }

  /// Convert the completed task result into a JavaScript value for the N-API environment.
  ///
  /// This implementation produces no JavaScript value and signals successful resolution.
  ///
  /// # Returns
  ///
  /// `Ok(())` indicating the task resolved with no value to return to JavaScript.
  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

/// Creates an asynchronous remove task for the given filesystem path using the provided options.
///
/// The returned task, when scheduled by the N-API runtime, will perform the removal work off the
/// main thread and resolve with no value.
///
/// # Examples
///
/// ```
/// let task = rm("some/path".to_string(), None);
/// // `task` can be returned to JavaScript or scheduled with the napi runtime.
/// ```
#[napi(js_name = "rm")]
pub fn rm(path: String, options: Option<RmOptions>) -> AsyncTask<RmTask> {
  AsyncTask::new(RmTask { path, options })
}

/// Synchronously removes the filesystem entry at the given path using the provided options.
///
/// Returns `Ok(())` on success or an error describing the failure.
///
/// # Examples
///
/// ```
/// // Remove a file or directory at "./tmp" using default options.
/// rm_sync("./tmp".to_string(), None).unwrap();
/// ```
#[napi(js_name = "rmSync")]
pub fn rm_sync(path: String, options: Option<RmOptions>) -> Result<()> {
  remove(path, options)
}