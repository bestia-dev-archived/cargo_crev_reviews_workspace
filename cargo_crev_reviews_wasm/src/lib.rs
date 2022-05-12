// cargo_crev_reviews_wasm lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_crev_reviews_wasm
//!
//! **Wasm web app that is the frontend of the application cargo_crev_reviews**  
//! ***version: 2022.512.1631 date: 2022-05-12 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/cargo_crev_reviews_workspace)***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1906-green.svg)](https://github.com/bestia-dev/cargo_crev_reviews_workspace/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-129-blue.svg)](https://github.com/bestia-dev/cargo_crev_reviews_workspace/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-157-purple.svg)](https://github.com/bestia-dev/cargo_crev_reviews_workspace/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/cargo_crev_reviews_workspace/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/cargo_crev_reviews_workspace/)
//!
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_crev_reviews_workspace/blob/main/LICENSE)
//!
//! This is a member of the workspace.
//! It is the wasm frontend for the GUI application.  
//! The main member project is `cargo_crev_reviews`.  
//! Please, continue reading `README.md` here:  
//!
//! ## [cargo_crev_reviews/README.md](https://github.com/bestia-dev/cargo_crev_reviews_workspace/tree/main/cargo_crev_reviews)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod auto_generated_mod;
mod cln_methods_mod;
mod html_mod;
mod routing_local_hash_mod;
mod utils_mod;
mod web_sys_mod;

use lazy_static::lazy_static;
use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub use dev_bestia_url_utf8::*;

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

    routing_local_hash_mod::router_boilerplate();

    // return
    Ok(())
}
