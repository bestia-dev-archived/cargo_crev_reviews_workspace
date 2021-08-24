// cargo_crev_reviews_micro_web_server_backend/src/lib.rs

//! This module contains the boilerplate to parse and match
//! The real code is in methods_mod
mod methods_mod;
mod template_mod;

use cargo_crev_reviews_common::*;
use methods_mod::*;
use template_mod::*;

use simple_server::{Method, Response, Server, StatusCode};
use unwrap::unwrap;

pub fn start_web_server(host: &str, port: &str) {
    println!("cargo_crev_reviews_micro_web_server_backend started");
    let server = Server::new(|request, response| {
        //println!("Request received. {} {}", request.method(), request.uri());

        match request.method() {
            &Method::GET => {
                // GET is used only to request files
                let response = parse_get_data_and_match_method(request.uri().to_string().as_str(), response);
                Ok(response)
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

/// GET is used only to request files
fn parse_get_data_and_match_method(path: &str, response: simple_server::Builder) -> Response<Vec<u8>> {
    println!("path: {}", path);
    if path == "/cargo_crev_reviews/index.html" {
        let response = response.header(http::header::CONTENT_TYPE, "text/html".as_bytes());
        let response = response.header(http::header::CACHE_CONTROL, "no-store, max-age=0".as_bytes());
        let body = template_mod::template_html().to_string();
        unwrap!(response.body(body.into_bytes()))
    } else if path == "/cargo_crev_reviews/css/cargo_crev_reviews.css" {        
        let response = response.header(http::header::CONTENT_TYPE, "text/css".as_bytes());
        let body = template_mod::css().to_string();
        unwrap!(response.body(body.into_bytes()))
    } else if path == "/cargo_crev_reviews/favicon.png" {
        let response = response.header(http::header::CONTENT_TYPE, "image/png".as_bytes());
        let body = template_mod::icon_original_png().replace("\n","");
        unwrap!(response.body(unwrap!(base64::decode(body))))
    } else {
        let body = "<h1>404</h1><p>Not found!<p>".to_string();
        unwrap!(response.body(body.into_bytes()))
    }
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
            // here add methods that this server recognizes
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

fn index_html_json(params: serde_json::Value, id: u32) -> String {
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
