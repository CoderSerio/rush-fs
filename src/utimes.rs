use napi::bindgen_prelude::*;
use napi::Task;
use napi_derive::napi;
use std::path::Path;

#[cfg(not(unix))]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(not(unix))]
fn to_system_time(time_secs: f64) -> SystemTime {
  if time_secs >= 0.0 {
    UNIX_EPOCH + Duration::from_secs_f64(time_secs)
  } else {
    UNIX_EPOCH - Duration::from_secs_f64(-time_secs)
  }
}

fn utimes_impl(path_str: String, atime: f64, mtime: f64) -> Result<()> {
  let path = Path::new(&path_str);

  if !path.exists() {
    return Err(Error::from_reason(format!(
      "ENOENT: no such file or directory, utimes '{}'",
      path.to_string_lossy()
    )));
  }

  #[cfg(unix)]
  {
    use std::ffi::CString;

    let c_path = CString::new(path.to_string_lossy().as_bytes())
      .map_err(|_| Error::from_reason("Invalid path"))?;

    let atime_sec = atime as i64;
    let atime_nsec = ((atime - atime as i64 as f64) * 1_000_000_000.0) as i64;
    let mtime_sec = mtime as i64;
    let mtime_nsec = ((mtime - mtime as i64 as f64) * 1_000_000_000.0) as i64;

    let times = [
      libc::timespec {
        tv_sec: atime_sec,
        tv_nsec: atime_nsec,
      },
      libc::timespec {
        tv_sec: mtime_sec,
        tv_nsec: mtime_nsec,
      },
    ];

    let ret = unsafe { libc::utimensat(libc::AT_FDCWD, c_path.as_ptr(), times.as_ptr(), 0) };
    if ret != 0 {
      let e = std::io::Error::last_os_error();
      return Err(Error::from_reason(format!(
        "{}, utimes '{}'",
        e,
        path.to_string_lossy()
      )));
    }
  }

  #[cfg(not(unix))]
  {
    use std::fs;
    let atime_sys = to_system_time(atime);
    let mtime_sys = to_system_time(mtime);
    let file = fs::OpenOptions::new()
      .write(true)
      .open(path)
      .map_err(|e| Error::from_reason(e.to_string()))?;
    file
      .set_modified(mtime_sys)
      .map_err(|e| Error::from_reason(e.to_string()))?;
    let _ = atime_sys; // Windows doesn't easily support setting atime via std
  }

  Ok(())
}

#[napi(js_name = "utimesSync")]
pub fn utimes_sync(path: String, atime: f64, mtime: f64) -> Result<()> {
  utimes_impl(path, atime, mtime)
}

// ========= async version =========

pub struct UtimesTask {
  pub path: String,
  pub atime: f64,
  pub mtime: f64,
}

impl Task for UtimesTask {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    utimes_impl(self.path.clone(), self.atime, self.mtime)
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }
}

#[napi(js_name = "utimes")]
pub fn utimes(path: String, atime: f64, mtime: f64) -> AsyncTask<UtimesTask> {
  AsyncTask::new(UtimesTask { path, atime, mtime })
}
