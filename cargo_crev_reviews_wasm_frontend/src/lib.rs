// cargo_crev_reviews_wasm_frontend lib.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod page_main_mod;
mod utils_mod;
mod web_sys_mod;

use wasm_bindgen_futures::spawn_local;

use crate::web_sys_mod as w;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this function
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    w::debug_write(&format!("cargo_crev_reviews v{}", env!("CARGO_PKG_VERSION")));

    //async block
    spawn_local(async {
        crate::page_main_mod::page_main().await;
    });

    // return
    Ok(())
}
