mod git;
mod parser;
mod server;

use crate::git::queries::commits::load_commits;
use crate::git::queries::config::req_config;
use crate::server::git_request::req_commits;
use parser::input::Input;
use tiny_http::{Response, Server};

#[cfg(debug_assertions)]
const PORT: u16 = 29997;
#[cfg(not(debug_assertions))]
const PORT: u16 = 0;

const ADDRESS: fn() -> String = || format!("127.0.0.1:{}", PORT);

fn main() {
  // let now = Instant::now();
  // load_commits(5000);
  // println!("load commits took {}ms", now.elapsed().as_millis());

  start_server();
}

fn start_server() {
  let server = Server::http(ADDRESS()).unwrap();

  let port = server.server_addr().port();

  println!("Address: {}:{}", server.server_addr().ip(), port);

  let mut n = 0;

  for request in server.incoming_requests() {
    println!(
      "received request! method: {:?}, url: {:?}, headers: {:?}",
      request.method(),
      request.url(),
      request.headers()
    );

    println!("Req num {n}");

    match request.url() {
      "/load-commits" => req_commits(request),
      "/load-config" => req_config(request),
      _ => {
        let response = Response::from_string(format!("hello world {n}"));
        request.respond(response).expect("Result to be sent");
      }
    }

    n += 1;
  }
}
