#![warn(clippy::all, clippy::pedantic)]
use std::{sync::Arc, thread};

use page::handle_request;
use tiny_http::Server;

mod blog;
mod home;
mod page;
mod photography;
mod util;

fn main() {
    let server = Arc::new(Server::http("0.0.0.0:80").unwrap());

    let mut thread_handles = Vec::new();

    for _ in 0..std::thread::available_parallelism()
        .unwrap_or(std::num::NonZero::<usize>::new(1).expect("THIS SHOULD NEVER FAIL!"))
        .into()
    {
        let server = server.clone();
        thread_handles.push(thread::spawn(move || {
            for request in server.incoming_requests() {
                match handle_request(request) {
                    Ok(()) => (),
                    Err(e) => println!("Error: {e}"),
                };
            }
        }));
    }

    for handle in thread_handles {
        let _ = handle.join();
    }
}
