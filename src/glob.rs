use crate::types::Dirent;
use crate::utils::get_file_type_id;
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::Path;
use std::sync::{Arc, Mutex};

// ignore crate 会对文件按 override 白名单过滤，但目录无论是否匹配都会被遍历（以便
// 递归进去找匹配的子条目）。因此目录需要单独用 dir_matcher 测试其路径是否符合模式，
// 只有匹配的目录才加入结果——这与 Node.js fs.globSync 的行为一致：
//   - "src/*"   → 返回 src/ 下的文件 AND 子目录
//   - "**/*.rs" → 只返回 .rs 文件（目录不含 .rs 扩展名，不会匹配）
//   - "**"      → 返回所有文件和目录（但不含 cwd 根节点自身）

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

  // 构建 override（白名单模式）：ignore crate 利用它来过滤文件；
  // 同时保留一份 dir_matcher 副本，用于判断目录自身是否匹配模式。
  let mut override_builder = OverrideBuilder::new(&cwd);
  override_builder
    .add(&pattern)
    .map_err(|e| Error::from_reason(e.to_string()))?;

  if let Some(ref excludes) = opts.exclude {
    for ex in excludes {
      override_builder
        .add(&format!("!{}", ex))
        .map_err(|e| Error::from_reason(e.to_string()))?;
    }
  }

  let overrides = override_builder
    .build()
    .map_err(|e| Error::from_reason(e.to_string()))?;

  // 复制一份给目录匹配用（walker 会消耗 overrides 所有权）
  let dir_matcher = Arc::new(overrides.clone());

  let mut builder = WalkBuilder::new(&cwd);
  builder
    .overrides(overrides)
    .standard_filters(opts.git_ignore.unwrap_or(true))
    .threads(concurrency);

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
    let dir_matcher = dir_matcher.clone();

    Box::new(move |entry| {
      let entry = match entry {
        Ok(e) => e,
        Err(_) => return ignore::WalkState::Continue,
      };

      // 跳过 cwd 根节点自身（depth 0）
      if entry.depth() == 0 {
        return ignore::WalkState::Continue;
      }

      let path = entry.path();
      let relative_path = path.strip_prefix(&root).unwrap_or(path);
      let relative_path_str = relative_path.to_string_lossy().to_string();

      let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

      if is_dir {
        // 目录：ignore crate 只为遍历而产出，需单独测试路径是否符合模式。
        // 与 Node.js 行为一致：模式 "src/*" 会同时返回 src/ 下的文件和子目录。
        let matched = dir_matcher.matched(relative_path, true);
        if !matched.is_whitelist() {
          // 目录本身不匹配模式，但仍继续遍历以便找到匹配的子条目
          return ignore::WalkState::Continue;
        }
        // 目录匹配模式，加入结果后继续遍历
      }
      // 非目录条目：ignore crate 的 override 白名单已确保它们匹配模式

      if with_file_types {
        let mut lock = result_dirents.lock().unwrap();
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
          0
        };
        lock.push(Dirent { name, parent_path, file_type });
      } else {
        let mut lock = result_strings.lock().unwrap();
        lock.push(relative_path_str);
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
