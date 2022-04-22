use crate::git::queries::commits::{load_commits_and_stashes, load_head_commit};
use crate::git::queries::config::load_full_config;
use parser::input::Input;
use tiny_http::{Response, Server};

mod git;
mod parser;
mod server;

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
// const PORT: u16 = 0;
const PORT: u16 = 29997;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

fn main() {
  start_server();
}

fn start_server() {
  let server = Server::http(ADDRESS()).unwrap();

  let port = server.server_addr().port();

  println!("Address: {}:{}", server.server_addr().ip(), port);

  for mut request in server.incoming_requests() {
    println!("received url: {:?}", request.url());

    match request.url() {
      "/load-commits" => handle_request!(request, load_commits_and_stashes),
      "/load-config" => handle_request!(request, load_full_config),
      "/head-commit" => handle_request!(request, load_head_commit),
      unknown_request => {
        let response = Response::from_string(format!("Unknown request: '{}'", unknown_request));
        let send_result = request.respond(response);

        println!("{:?}", send_result);
      }
    }
  }
}
