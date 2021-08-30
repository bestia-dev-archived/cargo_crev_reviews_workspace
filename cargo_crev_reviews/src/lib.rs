// cargo_crev_reviews/src/lib.rs

//! This module contains the boilerplate to parse and match URI and POST json-rpc.
//! The real code for methods is in methods_mod.rs
//! The real content of "static files" is in the module files_mod.rs
mod files_mod;
mod methods_mod;

use cargo_crev_reviews_common::*;
use files_mod::*;
// use methods_mod::*;

use simple_server::{Method, Response, Server, StatusCode};
use unwrap::unwrap;
enum Cache {
    NoStore,
    Ok,
}

// region: server - parse, match

pub fn start_web_server(host: &str, port: &str) {
    println!("cargo_crev_reviews started");
    let server = Server::new(|request, response| {
        let path = request.uri().to_string();
        // println!("Request received. {} {}", request.method(), request.uri());
        if !request.uri().to_string().starts_with("/cargo_crev_reviews") {
            return Ok(response_404_not_found(response, &path));
        }
        match request.method() {
            &Method::GET => {
                // GET is used only to request files
                let response = parse_get_uri_and_response_file(&path, response);
                Ok(response)
            }
            &Method::POST => {
                let request_body: &Vec<u8> = request.body();
                let response_body = parse_post_data_and_match_method(request_body);
                Ok(response.body(response_body.into_bytes())?)
            }
            _ => Ok(response_404_not_found(response, &path)),
        }
    });
    let x = std::process::Command::new("xdg-open")
        .arg("http://127.0.0.1:8182/cargo_crev_reviews/index.html")
        .spawn()
        .unwrap();
    drop(x);
    server.listen(host, port);
}

/// GET is used only to request files
/// Files are stored in functions in the files_mod.rs module
/// there is an automation task to copy files from web_server_folder to the module
fn parse_get_uri_and_response_file(path: &str, response: simple_server::Builder) -> Response<Vec<u8>> {
    println!("path: {}", path);
    match path {
        "/cargo_crev_reviews/index.html" => response_file_text(response, index_html, path, Cache::NoStore),
        "/cargo_crev_reviews/pages/new_review.html" => response_file_text(response, pages_new_review_html, path, Cache::NoStore),
        "/cargo_crev_reviews/css/cargo_crev_reviews.css" => response_file_text(response, files_mod::css_cargo_crev_reviews_css, path, Cache::Ok),
        "/cargo_crev_reviews/css/fontawesome.css" => response_file_text(response, css_fontawesome_css, path, Cache::Ok),
        "/cargo_crev_reviews/css/normalize.css" => response_file_text(response, css_normalize_css, path, Cache::Ok),
        "/cargo_crev_reviews/css/Roboto-Medium.woff2" => response_file_base64(response, css_roboto_medium_woff2, path),
        "/cargo_crev_reviews/icons/fa-solid-900.woff2" => response_file_base64(response, css_fa_solid_900_woff2, path),
        "/cargo_crev_reviews/icons/icon-032.png" => response_file_base64(response, icons_icon_032_png, path),
        "/cargo_crev_reviews/icons/icon-128.png" => response_file_base64(response, icons_icon_128_png, path),
        "/cargo_crev_reviews/icons/icon-192.png" => response_file_base64(response, icons_icon_192_png, path),

        "/cargo_crev_reviews/pkg/cargo_crev_reviews_wasm.js" => response_file_text(response, pkg_cargo_crev_reviews_wasm_js, path, Cache::NoStore),
        "/cargo_crev_reviews/pkg/cargo_crev_reviews_wasm_bg.wasm" => response_file_base64(response, pkg_cargo_crev_reviews_wasm_bg_wasm, path),

        _ => response_404_not_found(response, path),
    }
}

fn response_404_not_found(response: simple_server::Builder, path: &str) -> Response<Vec<u8>> {
    println!("404 not found: {}", path);
    let response = response.status(StatusCode::NOT_FOUND);
    let response = response_file_text(response, file_not_found_404, ".html", Cache::Ok);
    response
}

fn file_not_found_404() -> &'static str {
    r#"<h1>404</h1><p>Not found! URI must start with `/cargo_crev_reviews`<p>"#
}

fn response_file_text(response: simple_server::Builder, f: fn() -> &'static str, path: &str, cache: Cache) -> Response<Vec<u8>> {
    let mime_type = if path.ends_with(".html") {
        "text/html"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".json") {
        "application/json"
    } else {
        "text/html"
    };
    let response = response.header(http::header::CONTENT_TYPE, mime_type.as_bytes());
    let response = match cache {
        Cache::NoStore => response.header(http::header::CACHE_CONTROL, "no-store, max-age=0".as_bytes()),
        Cache::Ok => response,
    };
    let body = f().to_string();
    unwrap!(response.body(body.into_bytes()))
}

fn response_file_base64(response: simple_server::Builder, f: fn() -> &'static str, path: &str) -> Response<Vec<u8>> {
    let mime_type = if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else {
        "image/png"
    };
    let response = response.header(http::header::CONTENT_TYPE, mime_type.as_bytes());
    let body = f().replace("\n", "");
    unwrap!(response.body(unwrap!(base64::decode(body))))
}

/// <https://www.jsonrpc.org/specification>
fn parse_post_data_and_match_method(body: &Vec<u8>) -> String {
    let p: RpcMethod = unwrap!(serde_json::from_slice(body));
    //println!("deserialized = {:?}", &p);
    if p.jsonrpc != "2.0" {
        format!("error: jsonrpc != 2.0")
    } else {
        match p.method.as_str() {
            // here add methods that this server recognizes
            "save_review" => save_review_json(p.params, p.id),
            _ => format!("unknown method = {}", &p.method),
        }
    }
}

// endregion: server - parse, match

// region: boilerplate to convert json to call methods

fn save_review_json(params: serde_json::Value, id: u32) -> String {
    println!("save_review_json");
    let p: SaveReviewParams = unwrap!(serde_json::from_value(params));
    println!("SaveReviewParams = {:?}", &p);

    let r = StringResult {
        jsonrpc: "2.0".to_string(),
        result: "request received".to_string(),
        id,
    };
    let body = unwrap!(serde_json::to_string(&r));
    // return
    body
}

// endregion: boilerplate to convert json to call methods
