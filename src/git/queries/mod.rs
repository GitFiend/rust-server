use crate::git::git_types::RefInfo;
use crate::git::queries::refs::P_OPTIONAL_REFS;

pub(crate) mod commits;
mod commits_parsers;
mod commits_test;

pub(crate) mod commit_calcs;
pub(crate) mod config;
pub(crate) mod git_types_impl;
pub(crate) mod hunks;
pub(crate) mod patches;
mod refs;
pub(crate) mod scan_workspace;
pub(crate) mod stashes;
mod stashes_test;
pub(crate) mod wip;

pub const COMMIT_0_ID: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
