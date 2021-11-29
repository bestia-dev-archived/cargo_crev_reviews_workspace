// cargo_crev_reviews_wasm lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_crev_reviews_wasm
//!
//! **Wasm web app that is the frontend of the application cargo_crev_reviews**  
//! ***[repository](https://github.com/LucianoBestia/cargo_crev_reviews_workspace); version: 2021.1129.1640  date: 2021-11-29 authors: Luciano Bestia***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1389-green.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-95-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-126-purple.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//!
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/blob/main/LICENSE)
//!
//! This is a member of the workspace.
//! It is the wasm frontend for the GUI application.  
//! The main member project is `cargo_crev_reviews`.  
//! Please, continue reading `README.md` here:  
//!
//! ## [cargo_crev_reviews/README.md](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/tree/main/cargo_crev_reviews)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod auto_generated_mod;
mod cln_methods_review_item_mod;
mod cln_methods_review_list_mod;
mod cln_methods_verify_mod;
mod cln_methods_version_mod;
mod html_mod;
mod utils_mod;
mod web_sys_mod;

use anyhow::Context;
use lazy_static::lazy_static;
use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use dev_bestia_url_utf8::*;

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
    // priority order: error!, warn!, info!, debug! and trace!
    wasm_logger::init(wasm_logger::Config::new(Level::Trace));
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
