// cargo_crev_reviews/src/bin/main.rs

use std::env;
use unwrap::unwrap;

use cargo_crev_reviews::*;

/// start of CLI with this main() function
/// check if cargo_crev is installed,
/// if the program is run in the directory where Cargo.toml is,
/// if the host+port TcpListener is free to bind. That means that this is the only one instance of the program running.
/// User input the passphrase for crev signing.
/// open browser and start server
fn main() -> anyhow::Result<()> {
    // I don't need to check for `cargo` or Rust, because cargo_crev_reviews is installed with `cargo install`.
    // It means that cargo and Rust are already installed.
    // priority order: error!, warn!, info!, debug! and trace!
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Info).init();

    if let Some(host_port_already_busy) = host_port_is_busy() {
        one_instance_of_the_program_already_running(&host_port_already_busy);
    } else if !home::cargo_home()?.join("bin").join("cargo-crev").exists() {
        // check if cargo-crev is installed
        cargo_crev_not_installed();
    } else if !env::current_dir()?.join("Cargo.toml").exists() {
        not_started_inside_rust_project()?;
    } else {
        welcome_print();
        println!(
            r#"
The crev reviews are cryptographically signed, so you must first enter you crev passphrase to enable the signing of your crev reviews.
Then this CLI opens the default browser. This is the frontend graphical (GUI) part of the app.
If the default browser does not open from WSL2, you can see my project `https://github.com/LucianoBestia/wsl_open_browser`.
"#
        );
        unlock_crev_id_interactively()?;
        open_browser();
        // this must be the last command, because the server lasts
        start_web_server();
    }
    Ok(())
}

/// open browser with xdg-open
pub fn open_browser() {
    // open default browser in Linux
    // for WSL2 in Win10 I used my project https://crates.io/crates/wsl_open_browser
    let x = std::process::Command::new("xdg-open")
        .arg(&format!(
            "http://{}:{}/{}/index.html",
            SERVER_HOST.as_str(),
            SERVER_PORT.as_str(),
            SERVER_FIRST_SUBDIRECTORY.as_str()
        ))
        .spawn()
        .expect("Failed to open browser using `xdg-open`. Probably it is not installed on your system. Try to instal it with `xdg-utils`.");
    drop(x);
}

/// start the simple web server and match the GET or POST method
/// this is the only place where the web server is explicitly stated.
/// Changing this simple function, you can use any other web server library of your choice easily.
pub fn start_web_server() {
    use dev_bestia_simple_server::*;
    log::info!("cargo_crev_reviews server started");
    /// nested_function: convert response structs
    fn convert_response(response_with_bytes: ResponseWithBytes, builder: Builder) -> Response<Vec<u8>> {
        let builder = match response_with_bytes.status_code {
            Status::NotFound => builder.status(StatusCode::NOT_FOUND),
            Status::Ok => builder.status(StatusCode::OK),
        };
        let builder = builder.header(http::header::CONTENT_TYPE, response_with_bytes.mime_type.as_bytes());
        let builder = match response_with_bytes.cache_control {
            Some(cache_control) => builder.header(http::header::CACHE_CONTROL, cache_control.as_bytes()),
            None => builder,
        };
        let response = unwrap!(builder.body(response_with_bytes.body));
        // return
        response
    }

    let server = Server::new(|request, response_builder| {
        let path = request.uri().to_string();
        // log::info!("Request received. {} {}", request.method(), request.uri());
        if !request.uri().to_string().starts_with(&format!("/{}", SERVER_FIRST_SUBDIRECTORY.as_str())) {
            let response = convert_response(response_404_not_found(&path), response_builder);
            return Ok(response);
        }
        match request.method() {
            &Method::GET => {
                // GET is used only to request files
                let response_with_bytes = parse_get_uri_and_response_file(&path);
                let response = convert_response(response_with_bytes, response_builder);
                Ok(response)
            }
            &Method::POST => {
                let request_body: &Vec<u8> = request.body();
                let response_body = parse_post_data_and_match_method(request_body);
                match response_body {
                    Ok(response_body) => Ok(response_builder.body(response_body.into_bytes())?),
                    Err(err) => {
                        let response_body = unwrap!(response_err_message(&err));
                        Ok(response_builder.body(response_body.into_bytes())?)
                    }
                }
            }
            _ => {
                let response_body = unwrap!(response_modal_message("Unknown request method!"));
                Ok(response_builder.body(response_body.into_bytes())?)
            }
        }
    });
    server.listen(SERVER_HOST.as_str(), SERVER_PORT.as_str());
}
