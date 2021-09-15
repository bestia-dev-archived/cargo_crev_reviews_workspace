// cargo_crev_reviews/src/lib.rs

//! This module contains the boilerplate to parse and match URI and POST rpc.
//! The real code for methods is in methods_mod.rs
//! The real content of "static files" is in the module files_mod.rs
mod cargo_mod;
mod crev_mod;
mod files_mod;
mod response_get_mod;
mod response_post_mod;
mod rpc_methods_mod;
mod stdio_input_password_mod;

pub use crev_mod::unlock_crev_id_interactively;

use lazy_static::lazy_static;
use simple_server::{Method, Server};
use std::sync::Mutex;
// use unwrap::unwrap;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with async closures
    static ref CREV_UNLOCKED: Mutex<Option<crev_data::id::UnlockedId>>=Mutex::new(None);
    static ref LOCAL: Mutex<Option<crev_lib::Local>>=Mutex::new(None);
}

lazy_static! {
    /// ansi color
    pub static ref GREEN: String = termion::color::Fg(termion::color::Green).to_string();
    /// ansi color
    pub static ref YELLOW: String = termion::color::Fg(termion::color::Yellow).to_string();
    /// ansi color
    pub static ref RED: String = termion::color::Fg(termion::color::Red).to_string();
    /// ansi reset color
    pub static ref RESET: String = termion::color::Fg(termion::color::Reset).to_string();
    /// ansi clear line
    pub static ref CLEAR_LINE: String = termion::clear::CurrentLine.to_string();
    /// ansi clear all
    pub static ref CLEAR_ALL: String = termion::clear::All.to_string();
    /// ansi unhide cursor
    pub static ref UNHIDE_CURSOR: String = termion::cursor::Show.to_string();
}

// region: server - parse, match

pub fn start_web_server(host: &str, port: &str) {
    println!("cargo_crev_reviews server started");
    let server = Server::new(|request, response_builder| {
        let path = request.uri().to_string();
        // println!("Request received. {} {}", request.method(), request.uri());
        if !request.uri().to_string().starts_with("/cargo_crev_reviews") {
            return Ok(response_get_mod::response_404_not_found(response_builder, &path));
        }
        match request.method() {
            &Method::GET => {
                // GET is used only to request files
                let response = response_get_mod::parse_get_uri_and_response_file(&path, response_builder);
                Ok(response)
            }
            &Method::POST => {
                let request_body: &Vec<u8> = request.body();
                let response_body = response_post_mod::parse_post_data_and_match_method(request_body);
                match response_body {
                    Ok(response_body) => Ok(response_builder.body(response_body.into_bytes())?),
                    Err(err) => {
                        let response_body = response_post_mod::response_err_message(&err);
                        Ok(response_builder.body(response_body.into_bytes())?)
                    }
                }
            }
            _ => {
                let response_body = response_post_mod::response_modal_message("Unknown request method!");
                Ok(response_builder.body(response_body.into_bytes())?)
            }
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
