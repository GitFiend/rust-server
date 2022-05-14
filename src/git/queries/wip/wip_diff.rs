use crate::git::git_types::{Commit, HunkLine, PatchType, WipPatch, WipPatchType};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ReqWipHunksOptions {
  pub repo_path: String,
  pub patch: WipPatch,
  pub head_commit: Option<Commit>,
}

pub fn load_wip_hunks(options: &ReqWipHunksOptions) {
  let _lines = load_wip_hunk_lines(options);
}

pub fn load_wip_hunk_lines(options: &ReqWipHunksOptions) -> Option<Vec<HunkLine>> {
  let WipPatch {
    new_file,
    is_image,
    patch_type,
    ..
  } = &options.patch;

  if *is_image {
    return None;
  }

  let new_file_info = load_file(&options.repo_path, new_file);

  if *patch_type == WipPatchType::A || options.head_commit.is_none() {
    //
  }

  None
}

struct FileInfo {
  text: String,
  line_ending: String,
}

fn load_file(repo_path: &String, file_path: &String) -> Option<FileInfo> {
  match read_to_string(Path::new(repo_path).join(file_path)) {
    Ok(text) => {
      let line_ending = detect_new_line(&text);

      return Some(FileInfo { text, line_ending });
    }
    Err(e) => {
      println!("{}", e)
    }
  }

  None
}

fn detect_new_line(text: &String) -> String {
  let re = Regex::new(r"\r?\n").unwrap();

  let mut crlf = 0;
  let mut lf = 0;

  for nl in re.find_iter(text) {
    if nl.as_str() == "\r\n" {
      crlf += 1;
    } else {
      lf += 1;
    }
  }

  String::from(if crlf > lf { "\r\n" } else { "\n" })
}
