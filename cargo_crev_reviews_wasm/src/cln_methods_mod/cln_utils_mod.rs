// cln_utils_mod.rs

//! helper functions and utils

use crate::auto_generated_mod::common_structs_mod::{RpcMessageData, RpcResponse};
use crate::on_click;
use crate::*;
use dev_bestia_html_templating as tmplt;
use function_name::named;
use unwrap::unwrap;
use wasm_bindgen::JsCast;

#[named]
pub fn cln_modal_error(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = html_mod::extract_html(&srv_response);
    let data: RpcMessageData = unwrap!(serde_json::from_value(srv_response.response_data));
    let html_after_process = tmplt::process_html(&data, &html);
    html_mod::show_modal_html(&html_after_process);
    on_click!("modal_close", modal_close_on_click);
}

#[named]
pub fn cln_no_action(_srv_response: RpcResponse) {
    log::info!("{}", function_name!());
}

pub fn cln_modal_close(_srv_response: RpcResponse) {
    html_mod::close_modal();
}

pub fn modal_close_on_click(_element_id: &str) {
    html_mod::close_modal();
}
