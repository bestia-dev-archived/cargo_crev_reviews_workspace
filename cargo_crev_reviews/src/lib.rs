// cargo_crev_reviews/src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_crev_reviews
//!
//! **Write cargo-crev reviews in a Graphical User Interface with a cross-platform app written in rust**  
//! ***[repository](https://github.com/lucianobestia/cargo_crev_reviews_workspace); version: 2021.925.1120  date: 2021-09-25 authors: Luciano Bestia***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-17784-green.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-266-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-137-purple.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//!
//! ## Try it
//!
//! This is a GUI wrapper around [cargo](https://doc.rust-lang.org/cargo/) and [cargo-crev](https://lib.rs/crates/cargo-crev). First, install and configure cargo and cargo-crev. Then:  
//!
//! ```bash
//! cargo install cargo_crev_reviews
//! cd ~/rustprojects/your-project-name
//! cargo_crev_reviews
//! ```
//!
//! ![screen_3](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/raw/main/images/screen_3.png "screen_3")  
//! Input your passphrase for cargo-crev proof signing.  
//! ![screen_4](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/raw/main/images/screen_4.png "screen_4") ![screen_2](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/raw/main/images/screen_2.png "screen_2")  
//!
//! ## Motivation
//!
//! I think [cargo-crev](https://lib.rs/crates/cargo-crev) is a great tool to express trustworthiness in the open-source community, especially for the [rust programming language](https://www.rust-lang.org/).  I fear so much of [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack) using dependencies from [crates.io](https://crates.io/). For the smallest project you can get 100 dependencies easily. How to trust them all? To review them all manually? It is just crazy.  
//! But if enough people write reviews, it will be so much easier to trust the code. It is the same principle as [booking.com](https://www.booking.com/) or [air-bnb](https://www.airbnb.com/). Guests of a hotel write a review about their actual experience in the hotel. And you can read a hopefully truthful review and can understand if the hotel is good or bad. Sometimes can happen to find a fake review, but if there is enough people, most of them will be sincere.  
//! Sadly, writing reviews in `cargo-crev` is hard. Let's make a [GUI](https://en.wikipedia.org/wiki/Graphical_user_interface) wrapper around `cargo-crev` to make write reviews easier.  
//! We will see walking the path what obstacles we must overcome.  
//!
//! ## Technical decisions
//!
//! Rust does not have a true GUI story. It is mostly for [CLI](https://en.wikipedia.org/wiki/Command-line_interface) and libraries. Because GUI is mostly non cross-platform. But rust is the best language for [Wasm/Webassembly](https://webassembly.org/). So let's combine this.  
//! I will make a rust [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) (multi-projects):
//!
//! 1. CLI for a web server (local micro-server)
//! 2. Wasm for the browser (chrome and similar)
//!
//! The web server CLI will access files, commands, libraries and the network. This will work only in [Linux](https://en.wikipedia.org/wiki/Linux), but today Win10 has integrated Linux with [WSL2](https://en.wikipedia.org/wiki/Windows_Subsystem_for_Linux). It will work just fine on all the operating systems sensible for rust development.  
//! Wasm in browser will just access the local micro web server. This is gonna be the GUI and because the browser works in every [OS](https://en.wikipedia.org/wiki/Operating_system), this is [cross-platform](https://en.wikipedia.org/wiki/Cross-platform_software) development.  
//!
//! I want the simplest [web server](https://en.wikipedia.org/wiki/Web_server) ever. It will be used exclusively locally from one super simple [web-application](https://en.wikipedia.org/wiki/Web_application), so  don't need to care much about security. I choose [simple server](https://crates.io/crates/simple-server) from the [rust book](https://doc.rust-lang.org/1.30.0/book/second-edition/ch20-01-single-threaded.html). I don't care about [multi-threading](https://en.wikipedia.org/wiki/Multithreading_(computer_architecture)) or [async](https://en.wikipedia.org/wiki/Asynchrony_(computer_programming)) , because it will be used by only one browser. The example from the book evolved into the github repository of the author of the book [github.com/steveklabnik](https://github.com/steveklabnik/simple-server). I cloned it and update the dependencies and consequently fixed some broken code. I published it as dev_bestia_simple_server on crates.io.  
//!
//! For the browser I will create a simple web app. All the code will be in rust, I will avoid javascript. The GUI will be in [HTML5](https://en.wikipedia.org/wiki/HTML5) and [CSS3](https://en.wikipedia.org/wiki/CSS#CSS_3). This is all supported by all [modern browsers](https://www.bopdesign.com/bop-blog/2012/01/why-use-a-modern-web-browser/).  
//!
//! ## Development
//!
//! I will use [cargo-auto](https://crates.io/crates/cargo-auto) to automate the tasks needed to build the project.  
//! The sub-directory `automation_tasks_rs` is the rust project for [cargo-auto](https://crates.io/crates/cargo-auto).  
//!
//! The rust workspace is made of members:
//!
//! - backend CLI (this will be the main and only project to be published on crates.io)
//! - GUI frontend
//!
//! The sub-directory `web_server_folder` contains all the files and folder structure for a working development web_server.  
//! But this files are not used directly. Because of the way the publish to crates.io works, I will embed them inside the rust code as strings (base64 encoded if needed). I will make an automation task for that.  
//!
//! There is a file `auto_generated_mod.rs` where automation tasks writes boilerplate code.
//!
//! ## backend - cargo_crev_reviews
//!
//! This is a micro-web-server intended for local use with only one local browser connected to it.  
//! It is the backend of the application `cargo_crev_reviews`. I had to use the same name here.  
//! Together the backend and the frontend form a complete application that is cross-platform.  
//! They share some structs for communication that are defined in the `common_structs_mod` module. One automation task copies the content from backend to frontend projects to keep them in sync.  
//! The only URL the server operates is: <http://127.0.0.1:8182/cargo_crev_reviews>
//!
//! If I want to publish this on crates.io it must all be inside one binary executable file.  
//! It means that all the static files: css, html, icons, images, ... must be inside the rust code.  
//! For developing it is practical to have all this files as files.  
//! But before release an automation task converts this files to strings and put them into the rust code.  
//!
//! The micro-server will accept mostly [POST](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST) with json similar to  [json-rpc](https://www.jsonrpc.org/specification). But sure I had to modify it to something more adequate for my use-case. I think in the future I will change that even more to something even more adequate.  
//!
//! ```bash
//! Syntax:
//!
//! --> data sent to Server - request
//! <-- data sent to Client - response
//!
//! rpc call with named parameters:
//!
//! --> {
//! "request_method": "subtract_calculate",
//! "request_data": {
//!     "subtrahend": 23,
//!     "minuend": 42,
//!     }
//! }
//!
//! <-- {
//! "response_method": "subtract_show",
//! "response_data": {
//!     "subtracted": 19,     
//!     },
//! "response_html":"..."    
//! }
//! ```
//!
//! An example how to test a POST request with curl:
//!
//! ```bash
//! curl -d '{"request_method": "subtract", "request_data": {"subtrahend": 23, "minuend": 42}}' -H 'Content-Type: application/json' http://127.0.0.1:8182/cargo_crev_reviews
//! ```
//!
//! There are also a small number of GET requests for static files mostly to start the communication between the browser and the server.  
//!
//! ## GUI frontend - cargo_crev_reviews_wasm
//!
//! This simple web app is the GUI frontend of the application `cargo_crev_reviews`.  
//! It is strictly designed for use on desktops as it is a tool for programers. No need to do a mobile version. The response from the rpc server contains:
//!
//! 1. response method name
//! 2. html template
//! 3. data
//!
//! The method name is used to match and call the appropriate function.  
//! The html template must be microXml compatible. The wasm code reads element by element and when finds a marker, inserts the data. I wanted the html template to be complete with some sample texts. So the markers are added in front of the element or attribute they are meant to replace.  
//!
//! ## common structs - common_structs_mod.rs
//!
//! Common structures between backend and frontend. It is kind of a contract for communication.  
//! All in 100% rust language. One automation task keeps in sync the backend and frontend module.  
//! TODO: Using structs is not generic enough. Most of the time I need then name of a field. Because with the name I can bind in different scenarios. Using structs I don't have the name of the field at runtime. I think I will ditch most of the structs to have just a plain old flat text with QVS21. Inside that text every field will have a slice and a name. And I can then use that in runtime for bindings.  
//!
//! ## cargo-crev integration
//!
//! The [cargo-crev](https://github.com/crev-dev/cargo-crev) project contains many crates. The crates `crev-lib` and `crev-data` are libraries for integration. All the code working with crev is encapsulated in the crev_mod.rs module.  
//!
//! ## cargo registry
//!
//! The cargo application is essential for work with the rust language. It maintains a local `cargo registry cache`. The registry index is git fetched from github. Path to an index file:  
//! `~\.cargo\registry\index\github.com-1ecc6299db9ec823\cache\re\ad\reader_for_microxml`  
//! The content of this file is roughly:  
//!
//! ```yaml
//! bc688d353fc7c7a2f3f1f5fed9a27fc1773fc710
//! 1.0.0 {
//!     "name": "reader_for_microxml",
//!     "vers": "1.0.0",
//!     "deps": [],
//!     "cksum": "623616f68a6441e2f61aa01c9bbcf76f4c9989328e0e10ab747e936718791912",
//!     "features": {},
//!     "yanked": true,
//!     "links": null
//! }
//! 1.1.11 {
//!     "name": "reader_for_microxml",
//!     "vers": "1.1.11",
//!     "deps": [],
//!     "cksum": "fd50abb1f0d11a59ebe6d3f31446e4af8d0f8a7df668034b6c9b94453fa30c42",
//!     "features": {},
//!     "yanked": true,
//!     "links": null
//! }
//! ```
//!
//! Cargo also downloads from crates.io the complete source code for every dependency in the path `~/.cargo/registry/src/github.com-1ecc6299db9ec823/`. `Crates.io` guarantees the source code for a crate+version cannot be altered or deleted. We know it will never change, so we can review exactly this local code with confidence.  
//!
//! ## crates.io API
//!
//! Some data are not available locally and need to be obtained from <https://crates.io//api/v1/crates/{}/{}/>. Then I store them in `~/.config/crev/cargo_crev_reviews_versions.json`
//!
//! ## trusted publishers
//!
//! There is a confusion on crates.io who is the owner, author or group that is responsible for a crate version. Lately they added a `published_by` field for a crate_version. That sounds more accurate. In the file `~/.config/crev/cargo_crev_reviews_data/trusted_publishers.json` is the list of your trusted publishers. You can edit this file manually.  
//!
//! ## cargo crev reviews and advisory
//!
//! We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! You can also read reviews quickly on the web:  
//! <https://web.crev.dev/rust-reviews/crates/>  
//!
//! ## open-source free and free as a beer
//!
//! My open-source projects are free and free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful,  
//! please buy me a beer donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod auto_generated_mod;
mod cargo_mod;
mod common_structs_mod;
mod crates_io_mod;
mod crev_mod;
mod db_version_mod;
mod files_mod;
mod response_get_mod;
mod response_post_mod;
mod srv_methods_mod;
mod stdio_input_password_mod;
mod utils_mod;

pub use crev_mod::unlock_crev_id_interactively;

use dev_bestia_simple_server::{Method, Server};
use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;

lazy_static! {
    /// 127.0.0.1
    static ref SERVER_HOST: String=String::from("127.0.0.1");
    /// 8182
    static ref SERVER_PORT: String=String::from("8182");
    // first subdirectory /cargo_crev_reviews/
    static ref SERVER_FIRST_SUBDIRECTORY: String=String::from("cargo_crev_reviews");
}

lazy_static! {
    /// mutable static, because it is hard to pass variables around with async closures
    static ref CREV_UNLOCKED: Mutex<Option<crev_data::id::UnlockedId>>=Mutex::new(None);
    /// crev_liv Local struct
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

/// start the simple web server and match the GET or POST method
pub fn start_web_server() {
    println!("cargo_crev_reviews server started");

    let server = Server::new(|request, response_builder| {
        let path = request.uri().to_string();
        // println!("Request received. {} {}", request.method(), request.uri());
        if !request.uri().to_string().starts_with(&format!("/{}", SERVER_FIRST_SUBDIRECTORY.as_str())) {
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
                        let response_body = unwrap!(response_post_mod::response_err_message(&err));
                        Ok(response_builder.body(response_body.into_bytes())?)
                    }
                }
            }
            _ => {
                let response_body = unwrap!(response_post_mod::response_modal_message("Unknown request method!"));
                Ok(response_builder.body(response_body.into_bytes())?)
            }
        }
    });
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
        .unwrap();
    drop(x);
    server.listen(SERVER_HOST.as_str(), SERVER_PORT.as_str());
}

// endregion: server - parse, match

/// check that this is the only instance of this server
/// return None if the host+port is free
/// if the server host+port is not free, returns the String for the error message.
pub fn host_port_is_busy() -> Option<String> {
    let url=format!( "{}:{}", SERVER_HOST.as_str(), SERVER_PORT.as_str() );
    let listener = std::net::TcpListener::bind(&url);
    match listener {
        Ok(listener) => {
            drop(listener);
            None
        }
        Err(_err) => Some(url),
    }
}
