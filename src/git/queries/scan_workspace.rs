use serde::Deserialize;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use ts_rs::TS;

const MAX_SCAN_DEPTH: u8 = 5;
const MAX_DIR_SIZE: usize = 50;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ScanOptions {
  pub repo_path: String,
  pub workspaces_enabled: bool,
}

pub fn scan_workspace(options: &ScanOptions) -> Vec<PathBuf> {
  let dir = PathBuf::from(&options.repo_path);
  let mut repo_paths: Vec<PathBuf> = Vec::new();
  let mut git_modules_path: Option<PathBuf> = None;

  scan_workspace_inner(
    dir,
    options.workspaces_enabled,
    &mut repo_paths,
    &mut git_modules_path,
    0,
  );

  repo_paths
}

fn scan_workspace_inner(
  dir: PathBuf,
  workspaces_enabled: bool,
  repo_paths: &mut Vec<PathBuf>,
  git_modules_path: &mut Option<PathBuf>,
  depth: u8,
) {
  if !workspaces_enabled {
    if is_git_repo(&dir) {
      repo_paths.push(dir);
    }
  } else {
    if is_git_repo(&dir) {
      repo_paths.push(dir.clone());
    }

    if depth < MAX_SCAN_DEPTH {
      let entries = get_dir_entries(&dir);

      if entries.len() < MAX_DIR_SIZE {
        for e in entries {
          if e.is_dir() && !is_hidden(&e) {
            scan_workspace_inner(
              e,
              workspaces_enabled,
              repo_paths,
              git_modules_path,
              depth + 1,
            );
          } else if git_modules_path.is_none()
            && depth == 0
            && e.iter().any(|c| c == ".gitmodules")
          {
            // TODO: Is there any point in continuing to scan after finding the first .gitmodules file?
            *git_modules_path = Some(e.clone());
          }
        }
      }
    }
  }
}

fn get_dir_entries(dir: &PathBuf) -> Vec<PathBuf> {
  if let Ok(entries) = read_dir(dir) {
    let paths: Vec<PathBuf> = entries
      .filter(|e| e.is_ok())
      .map(|e| e.unwrap().path())
      .collect();

    return paths;
  }

  vec![]
}

fn is_git_repo(dir: &Path) -> bool {
  if dir.is_dir() {
    let git_file_path = dir.join(".git");

    return git_file_path.exists();
  }

  false
}

fn is_hidden(entry: &Path) -> bool {
  if let Some(last) = entry.components().last() {
    return last.as_os_str().to_str().unwrap_or("").starts_with('.');
  }
  false
}
