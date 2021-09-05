// cargo_crev_reviews/src/lib.rs

//! This module contains the boilerplate to parse and match URI and POST rpc.
//! The real code for methods is in methods_mod.rs
//! The real content of "static files" is in the module files_mod.rs
mod crev_mod;
mod files_mod;
mod response_get_files_mod;
mod response_post_method_mod;
mod server_methods_mod;
mod stdio_input_password_mod;

pub use crev_mod::unlock_crev_id_interactively;

use lazy_static::lazy_static;
use simple_server::{Method, Server};
use std::sync::Mutex;
use unwrap::unwrap;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with async closures
    static ref CREV_UNLOCKED: Mutex<Option<crev_data::id::UnlockedId>>=Mutex::new(None);
    static ref LOCAL: Mutex<Option<crev_lib::Local>>=Mutex::new(None);
}

// region: server - parse, match

pub fn start_web_server(host: &str, port: &str) {
    println!("cargo_crev_reviews server started");
    let server = Server::new(|request, response_builder| {
        let path = request.uri().to_string();
        // println!("Request received. {} {}", request.method(), request.uri());
        if !request.uri().to_string().starts_with("/cargo_crev_reviews") {
            return Ok(response_get_files_mod::response_404_not_found(response_builder, &path));
        }
        match request.method() {
            &Method::GET => {
                // GET is used only to request files
                let response = response_get_files_mod::parse_get_uri_and_response_file(&path, response_builder);
                Ok(response)
            }
            &Method::POST => {
                let request_body: &Vec<u8> = request.body();
                let response_body = unwrap!(response_post_method_mod::parse_post_data_and_match_method(request_body));
                Ok(response_builder.body(response_body.into_bytes())?)
            }
            _ => Ok(response_get_files_mod::response_404_not_found(response_builder, &path)),
        }
    });
    // open default browser in Linux
    // for WSL2 in Win10 I used my project https://crates.io/crates/wsl_open_browser
    let x = std::process::Command::new("xdg-open")
        .arg("http://127.0.0.1:8182/cargo_crev_reviews/index.html")
        .spawn()
        .unwrap();
    drop(x);
    server.listen(host, port);
}

// endregion: server - parse, match
