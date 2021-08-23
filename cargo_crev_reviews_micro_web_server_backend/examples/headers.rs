use env_logger;
use log::*;

use http::header;
use cargo_crev_reviews_micro_web_server_backend::Server;

fn main() {
    env_logger::init();

    let host = "127.0.0.1";
    let port = "8182";

    let server = Server::new(|request, response| {
        info!("Request received. {} {}", request.method(), request.uri());
        let response = response.header(header::CONTENT_TYPE, "text/plain".as_bytes());
        Ok(response.body("Hello Rust! (headers.rs)".as_bytes().to_vec())?)
    });

    server.listen(host, port);
}
