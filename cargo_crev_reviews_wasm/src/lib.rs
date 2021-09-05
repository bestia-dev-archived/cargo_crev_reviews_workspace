// cargo_crev_reviews_wasm lib.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod page_review_mod;
mod pages_mod;
mod utils_mod;
mod web_sys_mod;

use crate::web_sys_mod as w;
use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this function
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    w::debug_write(&format!("cargo_crev_reviews v{}", env!("CARGO_PKG_VERSION")));

    //async block
    spawn_local(async {
        crate::page_review_mod::request_review_list().await;
    });

    // return
    Ok(())
}

pub fn rpc_request_value<T>(params: T, server_method: &str) -> wasm_bindgen::JsValue
where
    T: serde::Serialize,
{
    let params = unwrap!(serde_json::to_value(params));
    let rpc = cargo_crev_reviews_common::RpcRequest {
        request_method: server_method.to_string(),
        request_params: params,
    };
    let json_string = unwrap!(serde_json::to_string(&rpc));
    let json_js_value = JsValue::from_str(&json_string);
    json_js_value
}
