// cargo_crev_reviews_wasm lib.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod page_review_mod;
mod pages_mod;
mod utils_mod;
mod web_sys_mod;

use crate::web_sys_mod as w;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this function
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    w::debug_write(&format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));

    crate::page_review_mod::request_review_list("");

    // return
    Ok(())
}
