// cln_methods_mod.rs

//! generic functions on the client

use crate::auto_generated_mod::common_structs_mod::RpcResponse;
use crate::web_sys_mod as w;
use function_name::named;

#[named]
pub fn cln_no_action(_srv_response: RpcResponse) {
    log::info!("{}", function_name!());
}

#[named]
pub fn cln_modal_close(_srv_response: RpcResponse) {
    w::set_inner_html("div_for_modal", "");
}

pub fn modal_close_on_click(_element_id: &str) {
    w::set_inner_html("div_for_modal", "");
}
