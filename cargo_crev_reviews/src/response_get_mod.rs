// response_get_mod.rs

//! response for the GET method. Mainly the static files, that are embedded in Rust code.

use crate::files_mod::*;
use derivative::*;
use unwrap::unwrap;

#[derive(Derivative)]
#[derivative(Default)]
pub struct ResponseWithBytes {
    #[derivative(Default(value = "Status::Ok"))]
    pub status_code: Status,
    #[derivative(Default(value = "default_mime()"))]
    pub mime_type: String,
    #[derivative(Default(value = "None"))]
    pub cache_control: Option<String>,
    pub body: Vec<u8>,
}

pub enum Status {
    NotFound,
    Ok,
}

enum Cache {
    NoStore,
    Ok,
}

fn default_mime() -> String {
    String::from("text/html")
}

/// GET is used only to request files
/// Files are stored in functions in the files_mod.rs module
/// there is an automation task to copy files from web_server_folder to the module
pub fn parse_get_uri_and_response_file(path: &str) -> ResponseWithBytes {
    println!("path: {}", path);
    match path {
        "/cargo_crev_reviews/index.html" => response_file_text(index_html, path, Cache::NoStore),
        "/cargo_crev_reviews/css/cargo_crev_reviews.css" => response_file_text(css_cargo_crev_reviews_css, path, Cache::Ok),
        "/cargo_crev_reviews/css/fontawesome.css" => response_file_text(css_fontawesome_css, path, Cache::Ok),
        "/cargo_crev_reviews/css/normalize.css" => response_file_text(css_normalize_css, path, Cache::Ok),
        "/cargo_crev_reviews/css/Roboto-Medium.woff2" => response_file_base64(css_roboto_medium_woff2, path),
        "/cargo_crev_reviews/css/fa-solid-900.woff2" => response_file_base64(css_fa_solid_900_woff2, path),
        "/cargo_crev_reviews/icons/icon-032.png" => response_file_base64(icons_icon_032_png, path),
        "/cargo_crev_reviews/icons/icon-128.png" => response_file_base64(icons_icon_128_png, path),
        "/cargo_crev_reviews/icons/icon-192.png" => response_file_base64(icons_icon_192_png, path),
        "/cargo_crev_reviews/images/Logo_02.png" => response_file_base64(images_logo_02_png, path),
        "/cargo_crev_reviews/js/dropdown.js" => response_file_text(js_dropdown_js, path, Cache::Ok),
        "/cargo_crev_reviews/pkg/cargo_crev_reviews_wasm.js" => response_file_text(pkg_cargo_crev_reviews_wasm_js, path, Cache::NoStore),
        "/cargo_crev_reviews/pkg/cargo_crev_reviews_wasm_bg.wasm" => response_file_base64(pkg_cargo_crev_reviews_wasm_bg_wasm, path),
        _ => return response_404_not_found(path),
    }
}

pub fn response_404_not_found(path: &str) -> ResponseWithBytes {
    println!("404 not found: {}", path);
    let mut response_with_text = response_file_text(file_not_found_404, ".html", Cache::Ok);
    response_with_text.status_code = Status::NotFound;
    response_with_text
}

fn file_not_found_404() -> &'static str {
    r#"<h1>404</h1><p>Not found! URI must start with `/cargo_crev_reviews`<p>"#
}

fn response_file_text(f: fn() -> &'static str, path: &str, cache: Cache) -> ResponseWithBytes {
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
    let mime_type = mime_type.to_string();

    let cache_control = match cache {
        Cache::NoStore => Some("no-store, max-age=0".to_string()),
        Cache::Ok => None,
    };
    let body = f().as_bytes().to_vec();

    ResponseWithBytes {
        mime_type,
        cache_control,
        body,
        ..Default::default()
    }
}

fn response_file_base64(f: fn() -> &'static str, path: &str) -> ResponseWithBytes {
    let mime_type = if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else {
        "image/png"
    };
    let mime_type = mime_type.to_string();
    // I artificially added \n to base64 to make it more text editor friendly
    let body = f().replace("\n", "");
    // builder.body() returns Response
    let body = unwrap!(base64::decode(body));

    ResponseWithBytes {
        mime_type,
        body,
        ..Default::default()
    }
}
