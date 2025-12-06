use napi_derive::napi;
use walkdir::WalkDir;

#[napi] // marco: expose the function to Node
pub fn ls(path: String) -> Vec<String> {
  let mut res = Vec::new();

  // create a recursive iterator to get dirs and files
  for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
    if entry.file_type().is_file() {
      res.push(entry.path().to_string_lossy().to_string());
    }
  }

  res
}
