use env_logger;
use log::*;

use dev_bestia_simple_server::{Method, Server, StatusCode};

fn main() {
    env_logger::init();

    let host = "127.0.0.1";
    let port = "8182";

    let server = Server::new(|request, response| {
        info!("Request received. {} {}", request.method(), request.uri());

        match request.method() {
            &Method::GET => {
                let body = format!("The path you requested was '{}'", request.uri().path());
                Ok(response.body(body.into_bytes())?)
            }
            &Method::POST => {
                let data = String::from_utf8_lossy(request.body()).into_owned();
                let body = format!("The data you posted was '{}'", data);
                Ok(response.body(body.into_bytes())?)
            }
            _ => {
                let response = response.status(StatusCode::NOT_FOUND);
                Ok(response.body(b"<h1>404</h1><p>Not found!<p>".to_vec())?)
            }
        }
    });

    server.listen(host, port);
}
