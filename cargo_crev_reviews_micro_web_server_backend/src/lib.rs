// cargo_crev_reviews_micro_web_server_backend/src/lib.rs

//! This module contains the boilerplate to parse and match
//! The real code is in functions_mod
mod functions_mod;

use cargo_crev_reviews_common::*;
use functions_mod::*;
use simple_server::{Method, Server, StatusCode};
use unwrap::unwrap;

pub fn start_web_server(host: &str, port: &str) {
    println!("cargo_crev_reviews_micro_web_server_backend started");
    let server = Server::new(|request, response| {
        //println!("Request received. {} {}", request.method(), request.uri());

        match request.method() {
            &Method::GET => {
                // I will use only POST, never GET
                // let body = format!("The path you requested was '{}'", request.uri().path());
                // Ok(response.body(body.into_bytes())?)
                Ok(response.body(b"<h1>404</h1><p>Not found!<p>".to_vec())?)
            }
            &Method::POST => {
                let data = String::from_utf8_lossy(request.body()).into_owned();
                let body = parse_post_data_and_match_method(&data);
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

/// <https://www.jsonrpc.org/specification>
fn parse_post_data_and_match_method(data: &str) -> String {
    //println!("data: {}", data);
    let p: RpcMethod = unwrap!(serde_json::from_str(data));
    //println!("deserialized = {:?}", &p);
    if p.jsonrpc != "2.0" {
        format!("error: jsonrpc != 2.0")
    } else {
        match p.method.as_str() {
            // here add methods and functions that this server recognizes
            "subtract" => subtract_json(p.params, p.id),
            _ => format!("unknown method = {}", &p.method),
        }
    }
}
// region: server - parse, match

fn subtract_json(params: serde_json::Value, id: u32) -> String {
    //println!("SubtractParams: {}", &params);
    let p: SubtractParams = unwrap!(serde_json::from_value(params));
    println!("SubtractParams = {:?}", &p);

    let subtraction = subtract(p.subtrahend, p.minuend);

    let r = SubtractResult {
        jsonrpc: "2.0".to_string(),
        result: subtraction,
        id,
    };
    let body = unwrap!(serde_json::to_string(&r));
    // return
    body
}
