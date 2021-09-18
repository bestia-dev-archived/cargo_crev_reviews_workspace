// response_get_files_mod.rs

use crate::files_mod::*;
use dev_bestia_simple_server::{Response, StatusCode};
use unwrap::unwrap;

enum Cache {
    NoStore,
    Ok,
}

/// GET is used only to request files
/// Files are stored in functions in the files_mod.rs module
/// there is an automation task to copy files from web_server_folder to the module
pub fn parse_get_uri_and_response_file(path: &str, response: dev_bestia_simple_server::Builder) -> Response<Vec<u8>> {
    println!("path: {}", path);
    match path {
        "/cargo_crev_reviews/index.html" => response_file_text(response, index_html, path, Cache::NoStore),
        "/cargo_crev_reviews/css/cargo_crev_reviews.css" => response_file_text(response, css_cargo_crev_reviews_css, path, Cache::Ok),
        "/cargo_crev_reviews/css/fontawesome.css" => response_file_text(response, css_fontawesome_css, path, Cache::Ok),
        "/cargo_crev_reviews/css/normalize.css" => response_file_text(response, css_normalize_css, path, Cache::Ok),
        "/cargo_crev_reviews/css/Roboto-Medium.woff2" => response_file_base64(response, css_roboto_medium_woff2, path),
        "/cargo_crev_reviews/css/fa-solid-900.woff2" => response_file_base64(response, css_fa_solid_900_woff2, path),
        "/cargo_crev_reviews/icons/icon-032.png" => response_file_base64(response, icons_icon_032_png, path),
        "/cargo_crev_reviews/icons/icon-128.png" => response_file_base64(response, icons_icon_128_png, path),
        "/cargo_crev_reviews/icons/icon-192.png" => response_file_base64(response, icons_icon_192_png, path),
        "/cargo_crev_reviews/images/Logo_02.png" => response_file_base64(response, images_Logo_02_png, path),
        "/cargo_crev_reviews/js/dropdown.js" => response_file_text(response, js_dropdown_js, path, Cache::Ok),
        "/cargo_crev_reviews/pkg/cargo_crev_reviews_wasm.js" => response_file_text(response, pkg_cargo_crev_reviews_wasm_js, path, Cache::NoStore),
        "/cargo_crev_reviews/pkg/cargo_crev_reviews_wasm_bg.wasm" => response_file_base64(response, pkg_cargo_crev_reviews_wasm_bg_wasm, path),
        _ => response_404_not_found(response, path),
    }
}

pub fn response_404_not_found(response: dev_bestia_simple_server::Builder, path: &str) -> Response<Vec<u8>> {
    println!("404 not found: {}", path);
    let response = response.status(StatusCode::NOT_FOUND);
    let response = response_file_text(response, file_not_found_404, ".html", Cache::Ok);
    response
}

fn file_not_found_404() -> &'static str {
    r#"<h1>404</h1><p>Not found! URI must start with `/cargo_crev_reviews`<p>"#
}

fn response_file_text(response_builder: dev_bestia_simple_server::Builder, f: fn() -> &'static str, path: &str, cache: Cache) -> Response<Vec<u8>> {
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
    let response_builder = response_builder.header(http::header::CONTENT_TYPE, mime_type.as_bytes());
    let response_builder = match cache {
        Cache::NoStore => response_builder.header(http::header::CACHE_CONTROL, "no-store, max-age=0".as_bytes()),
        Cache::Ok => response_builder,
    };
    let body = f().to_string();
    // builder.body() returns Response
    unwrap!(response_builder.body(body.into_bytes()))
}

fn response_file_base64(response_builder: dev_bestia_simple_server::Builder, f: fn() -> &'static str, path: &str) -> Response<Vec<u8>> {
    let mime_type = if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else {
        "image/png"
    };
    let response_builder = response_builder.header(http::header::CONTENT_TYPE, mime_type.as_bytes());
    // I artificially added \n to base64 to make it more text editor friendly
    let body = f().replace("\n", "");
    // builder.body() returns Response
    unwrap!(response_builder.body(unwrap!(base64::decode(body))))
}
