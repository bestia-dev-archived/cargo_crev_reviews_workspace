
use log::*;
use simple_server::{Method, Server, StatusCode};

pub fn start_web_server(host: &str, port: &str) {
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