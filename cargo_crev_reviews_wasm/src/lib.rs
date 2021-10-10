// cargo_crev_reviews_wasm lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_crev_reviews_wasm
//!
//! **Wasm web app that is the frontend of the application cargo_crev_reviews**  
//! ***[repository](https://github.com/LucianoBestia/cargo_crev_reviews_workspace); version: 2021.925.1120  date: 2021-09-25 authors: Luciano Bestia***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1281-green.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-147-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-104-purple.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//!
//! This is a member of the workspace.
//! It is the wasm frontend for the GUI application.  
//!
//! ## Code-flow
//!
//! Everything is compiled into one single executable binary for Linux: `cargo_crev_reviews`.  
//! First it opens the default browser with `xdg-open` on <http://127.0.0.1:8182/cargo_crev_reviews/index.html>.  
//! If your WSL2 does not have yet a default browser run this:  
//!
//! ``` bash
//! ln -s "/mnt/c/Program Files/Mozilla Firefox/firefox.exe" /usr/bin/browser_in_win
//! export BROWSER='/usr/bin/browser_in_win'
//! ```
//!
//! The command `ln -sf` is permanent and persistent. It makes a symbolic link file that stays there forever. But `export BROWSER=` is NOT persistent. You need to add this command to `~/.bashrc` that runs it on every start of terminal.  
//!
//! In the next millisecond the web server starts listening to 127.0.0.1 port 8182.  
//! The first set of requests are GET and response is "static" files embedded in files_mod.rs
//!
//! 1. browser request for `/cargo_crev_reviews/index.html` is GET, the response is html text file embedded in files_mod.rs in the function: `index_html()`  
//!     This html is just an empty shell that gets the css and wasm code. There is no real content inside. This concept is [Single-page application SPA](https://en.wikipedia.org/wiki/Single-page_application).  
//! 2. index.html requests: 3 css files, `pkg/cargo_crev_reviews.js`, `pkg/cargo_crev_reviews_bg.wasm`, "favicon" `icons/icon-032.png`. All these requests are GET and responses come from files_mod.rs functions, some are text files and others are base64 files.
//! 3. the browser imports the wasm module and starts the init function that requests `srv_review_list`. This responds with: response_method_name, response_html and response_data.
//! 4. wasm (inside the browser) is rust code. First it matches method_name and calls the appropriate function. It processes the html with the data and inserts it into index.html (the empty shell).
//! 5. the browser renders our first page. Hooray!
//! 6. the user click on some button.
//! 7. the macro `on_click!` or `row_on_click!` hides the ugly rust code behind the definition of an event handler in web_sys and calls a function
//! 8. wasm creates a rpc request and sends/POST to the server
//! 9. the request is POST, the server first matches the method_name and calls the appropriate function. The function processes the call and prepares some data. It loads the html template.
//! 10. The response contains the html to be rendered and data to be inserted in this html before rendering.
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod auto_generated_mod;
mod cln_methods_review_mod;
mod cln_methods_verify_mod;
mod cln_methods_version_mod;
mod html_mod;
mod html_template_mod;
mod macros_mod;
mod url_utf8_mod;
mod utils_mod;
mod web_sys_mod;

use anyhow::Context;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use crate::macros_mod::*;
pub use crate::url_utf8_mod::*;

use crate::auto_generated_mod::common_structs_mod::ReviewFilterData;
use crate::auto_generated_mod::common_structs_mod::RpcEmptyData;
use crate::auto_generated_mod::srv_methods;
use crate::web_sys_mod as w;

lazy_static! {
    /// 127.0.0.1
    static ref SERVER_HOST: String=String::from("127.0.0.1");
    /// 8182
    static ref SERVER_PORT: String=String::from("8182");
    // first subdirectory /cargo_crev_reviews/
    static ref SERVER_FIRST_SUBDIRECTORY: String=String::from("cargo_crev_reviews");
}

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this function
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // for global macros log::info!(), log::debug!(), log_error!(), log::warn!(), log::trace!()
    wasm_logger::init(wasm_logger::Config::default());
    // write the app version just for debug purposes
    log::info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    // read the url hash parameters for local routing
    let location = w::window().location();
    match location.hash() {
        // if there is no url hash, show the first page: verify_list
        Err(_err) => crate::cln_methods_verify_mod::request_verify_list(""),
        Ok(location_hash) => {
            if location_hash.is_empty() {
                crate::cln_methods_verify_mod::request_verify_list("");
            } else {
                match get_3_url_param_from_hash(&location_hash) {
                    Err(_err) => {
                        let request_data = RpcEmptyData {};
                        srv_methods::srv_verify_project(request_data);
                    }
                    Ok((method, crate_name, crate_version)) => match method {
                        "edit_or_new" => {
                            let request_data = ReviewFilterData {
                                crate_name: crate_name.to_string(),
                                crate_version: Some(crate_version.to_string()),
                                old_crate_version: None,
                            };
                            srv_methods::srv_review_edit_or_new(request_data);
                        }
                        "version_list" => {
                            let request_data = ReviewFilterData {
                                crate_name: crate_name.to_string(),
                                crate_version: None,
                                old_crate_version: None,
                            };
                            srv_methods::srv_version_list(request_data);
                        }
                        _ => log::info!("unrecognized hash method: {}", method),
                    },
                }
            }
        }
    }

    // return
    Ok(())
}

/// get 3 param from hash
/// example "#edit/crate_name/crate_version" -> ["edit","crate_name","crate_version"]
pub fn get_3_url_param_from_hash(location_hash: &str) -> anyhow::Result<(&str, &str, &str)> {
    let mut spl = location_hash.trim_start_matches("#").split('/');
    Ok((
        spl.next().context("first method")?,
        spl.next().context("second crate_name")?,
        spl.next().unwrap_or(""),
    ))
}
