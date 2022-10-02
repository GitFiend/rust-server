use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use tiny_http::{Header, Request, Response};

use crate::dprintln;

pub fn handle_resource_request(request: Request) -> Option<()> {
  let dir = get_server_dir()?;

  // Remove any extra query part.
  let url = request.url().split('?').next()?;
  let file_path = dir.join(&url[3..]);

  dprintln!("file_path {:?}", file_path);

  let file = File::open(&file_path).ok()?;
  let mut response = Response::from_file(file);

  let content_type = get_content_type(&file_path.to_string_lossy())?;

  let header = Header::from_str(&content_type).ok()?;
  response.add_header(header);

  let _ = request.respond(response);

  Some(())
}

fn get_content_type(file_path: &str) -> Option<String> {
  let guess = mime_guess::from_path(&file_path);

  Some(format!("Content-Type: {}", guess.first()?))
}

fn get_server_dir() -> Option<PathBuf> {
  #[cfg(debug_assertions)]
  return Some(env::current_dir().ok()?.parent()?.join("git-fiend"));

  // TODO: Sort this out. May need to unpack all from asar.
  #[cfg(not(debug_assertions))]
  Some(env::current_exe().ok()?.parent()?.parent()?)
}
