// response_get_mod.rs

//! response for the GET method. Mainly the static files, that are embedded in Rust code.

use derivative::*;

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
/// Files are stored in functions in the auto_generated_files_mod.rs module
/// there is an automation task to copy files from web_server_folder to the module
pub fn parse_get_uri_and_response_file(path: &str) -> ResponseWithBytes {
    log::info!("path: {}", path);
    let (mime_type, text_or_else_base64, cache) = if path.ends_with(".html") {
        ("text/html", true, Cache::NoStore)
    } else if path.ends_with(".css") {
        ("text/css", true, Cache::Ok)
    } else if path.ends_with(".js") {
        ("application/javascript", true, Cache::Ok)
    } else if path.ends_with(".json") {
        ("application/json", true, Cache::NoStore)
    } else if path.ends_with(".png") {
        ("image/png", false, Cache::Ok)
    } else if path.ends_with(".woff2") {
        ("font/woff2", false, Cache::Ok)
    } else if path.ends_with(".wasm") {
        ("application/wasm", false, Cache::Ok)
    } else {
        ("text/html", true, Cache::NoStore)
    };
    let file_as_body = crate::auto_generated_files_mod::get_file_bytes(path);

    if file_as_body.is_empty() {
        return response_404_not_found(path);
    }
    let cache_control = match cache {
        Cache::NoStore => Some("no-store, max-age=0".to_string()),
        Cache::Ok => None,
    };
    if text_or_else_base64 {
        return response_file_text(file_as_body, cache_control, mime_type);
    } else {
        return response_file_base64(file_as_body, cache_control, mime_type);
    }
}

pub fn response_404_not_found(path: &str) -> ResponseWithBytes {
    log::warn!("404 not found: {}", path);
    let file_as_body = r#"<h1>404</h1><p>Not found! URI must start with `/cargo_crev_reviews`<p>"#.as_bytes().to_vec();
    let mut response_with_text = response_file_text(file_as_body, None, "text/html");
    response_with_text.status_code = Status::NotFound;
    response_with_text
}

fn response_file_text(file_as_body: Vec<u8>, cache_control: Option<String>, mime_type: &str) -> ResponseWithBytes {
    let mime_type = mime_type.to_string();

    ResponseWithBytes {
        mime_type,
        cache_control,
        body: file_as_body,
        ..Default::default()
    }
}

fn response_file_base64(file_as_body: Vec<u8>, cache_control: Option<String>, mime_type: &str) -> ResponseWithBytes {
    let mime_type = mime_type.to_string();

    ResponseWithBytes {
        mime_type,
        cache_control,
        body: file_as_body,
        ..Default::default()
    }
}
