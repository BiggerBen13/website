#![warn(clippy::all, clippy::pedantic)]
use page::handle_request;
use tiny_http::Server;

mod home;
mod page;
mod util;

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    for p in std::fs::read_dir(std::path::Path::new(".")).unwrap() {
        println!("{}", p.unwrap().file_name().into_string().unwrap());
    }

    for request in server.incoming_requests() {
        match handle_request(request) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e.to_string()),
        };
    }
}
