use crate::git::git_types::Commit;
use crate::git::queries::commits_parsers::{PRETTY_FORMATTED, P_COMMITS, P_COMMIT_ROW, P_ID_LIST};
use crate::git::queries::stashes::load_stashes;
use crate::git::queries::store::{load_commits_from_store, store_commits};
use crate::git::{run_git, RunGitOptions};
use crate::parser::parse_all;
use crate::server::git_request::{ReqCommitsOptions, ReqOptions};
use cached::proc_macro::cached;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::thread;
use std::time::Instant;
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TopCommitOptions {
  pub repo_path: String,
  pub branch_name: String,
}

pub fn load_top_commit_for_branch(options: &TopCommitOptions) -> Option<Commit> {
  let now = Instant::now();

  let out = run_git(RunGitOptions {
    args: [
      "log",
      &options.branch_name,
      "--decorate=full",
      PRETTY_FORMATTED,
      "-n1",
      "--date=raw",
    ],
    repo_path: &options.repo_path,
  });

  println!(
    "Took {}ms to request top commit from Git",
    now.elapsed().as_millis(),
  );

  parse_all(P_COMMIT_ROW, out?.as_str())
}

pub fn load_head_commit(options: &ReqOptions) -> Option<Commit> {
  let out = run_git(RunGitOptions {
    args: [
      "log",
      "--decorate=full",
      PRETTY_FORMATTED,
      "-n1",
      "--date=raw",
    ],
    repo_path: &options.repo_path,
  });

  parse_all(P_COMMIT_ROW, out?.as_str())
}

pub fn load_commits_and_stashes(options: &ReqCommitsOptions) -> Option<Vec<Commit>> {
  let ReqCommitsOptions {
    repo_path,
    num_commits,
  } = options;

  let now = Instant::now();

  let p1 = repo_path.clone();
  let p2 = repo_path.clone();
  let num = num_commits.clone();

  let stashes_thread = thread::spawn(move || load_stashes(&p1));
  let commits_thread = thread::spawn(move || load_commits(&p2, num));

  let stashes = stashes_thread.join().unwrap();
  let mut commits = commits_thread.join().unwrap()?;

  println!(
    "Took {}ms to request stashes and commits from Git",
    now.elapsed().as_millis(),
  );

  if stashes.is_some() {
    commits.append(&mut stashes.unwrap());
  }

  commits.sort_by(|a, b| {
    if b.stash_id.is_some() || a.stash_id.is_some() {
      b.date.ms.partial_cmp(&a.date.ms).unwrap_or(Ordering::Equal)
    } else {
      Ordering::Equal
    }
  });

  for i in 0..commits.len() {
    let mut c = &mut commits[i];
    c.index = i;
  }

  store_commits(&repo_path, &commits);

  Some(commits)
}

pub fn load_commits(repo_path: &String, num: u32) -> Option<Vec<Commit>> {
  let now = Instant::now();

  let out = run_git(RunGitOptions {
    args: [
      "log",
      "--branches",
      "--tags",
      "--remotes",
      "--decorate=full",
      PRETTY_FORMATTED,
      format!("-n{}", num).as_str(),
      "--date=raw",
    ],
    repo_path,
  })?;

  println!(
    "Took {}ms to request {} commits from Git",
    now.elapsed().as_millis(),
    num
  );

  let now = Instant::now();
  let result = parse_all(P_COMMITS, &out);

  println!(
    "Took {}ms to parse {} commits. Length {}",
    now.elapsed().as_millis(),
    num,
    out.len()
  );

  result
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommitDiffOpts {
  pub repo_path: String,
  pub commit_id1: String,
  pub commit_id2: String,
}

pub fn commit_ids_between_commits(options: &CommitDiffOpts) -> Option<Vec<String>> {
  let CommitDiffOpts {
    repo_path,
    commit_id1,
    commit_id2,
  } = options;

  commit_ids_between_commits_inner(repo_path.clone(), commit_id1.clone(), commit_id2.clone())
}

#[cached(option = true, time = 1000)]
fn commit_ids_between_commits_inner(
  repo_path: String,
  commit_id1: String,
  commit_id2: String,
) -> Option<Vec<String>> {
  let now = Instant::now();

  let out = run_git(RunGitOptions {
    args: [
      "log",
      &format!("{}..{}", commit_id1, commit_id2),
      "--pretty=format:%H",
    ],
    repo_path: &repo_path,
  })?;

  println!("Took {}ms to request ids", now.elapsed().as_millis());

  parse_all(P_ID_LIST, &out)
}

// Use this as a fallback when calculation fails.
pub fn get_un_pushed_commits(options: &ReqOptions) -> Vec<String> {
  get_un_pushed_commits_computed(&options);

  if let Some(out) = run_git(RunGitOptions {
    repo_path: &options.repo_path,
    args: ["log", "HEAD", "--not", "--remotes", "--pretty=format:%H"],
  }) {
    if let Some(ids) = parse_all(P_ID_LIST, &out) {
      return ids;
    }
  }

  Vec::new()
}

fn get_un_pushed_commits_computed(options: &ReqOptions) -> Option<Vec<String>> {
  let commits = load_commits_from_store(&options.repo_path)?;

  let commit = commits.iter().find(|c| c.refs.iter().any(|r| r.head));

  println!("{:?}", commit.unwrap());

  None
}
