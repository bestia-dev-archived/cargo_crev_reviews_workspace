use env_logger;
use log::*;

use simple_server::{Method, Server, StatusCode};

fn main() {
    env_logger::init();

    let host = "127.0.0.1";
    let port = "8182";

    let server = Server::new(|request, response| {
        info!("Request received. {} {}", request.method(), request.uri());

        match (request.method(), request.uri().path()) {
            (&Method::GET, "/hello") => {
                Ok(response.body("<h1>Hi!</h1><p>Hello Rust!(routes.rs)</p>".as_bytes().to_vec())?)
            }
            (_, _) => {
                let response = response.status(StatusCode::NOT_FOUND);
                Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
            }
        }
    });

    server.listen(host, port);
}
