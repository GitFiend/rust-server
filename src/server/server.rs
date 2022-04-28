use crate::git::queries::config::load_full_config;
use crate::git::queries::patches::patches::load_all_commit_patches;
use crate::server::http::{parse_http_request, HttpRequest};
use crate::{
  commit_ids_between_commits, load_commits_and_stashes, load_head_commit,
  load_top_commit_for_branch, requests,
};

use std::io::Read;
use std::net::{TcpListener, TcpStream};

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
// const PORT: u16 = 0;
const PORT: u16 = 29997;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

pub fn start_sync_server() {
  let listener = TcpListener::bind(ADDRESS()).unwrap();

  match listener.local_addr() {
    Ok(r) => {
      println!("Port: {}", r.port())
    }
    Err(_) => {}
  }

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(stream: TcpStream) {
  let result = get_request_from_stream(&stream);

  if result.is_some() {
    let request = result.unwrap();

    requests!(
      request,
      stream,
      load_commits_and_stashes,
      load_full_config,
      load_head_commit,
      load_top_commit_for_branch,
      commit_ids_between_commits,
      load_all_commit_patches
    );
  }
}

fn get_request_from_stream(mut stream: &TcpStream) -> Option<HttpRequest> {
  let mut buffer = [0; 2048];

  stream.read(&mut buffer).unwrap();

  let request_text = String::from_utf8_lossy(&buffer[..]).to_string();

  parse_http_request(request_text)
}

#[cfg(test)]
mod tests {
  use crate::server::server::start_sync_server;

  #[test]
  fn test_start_server() {
    start_sync_server();
  }
}
