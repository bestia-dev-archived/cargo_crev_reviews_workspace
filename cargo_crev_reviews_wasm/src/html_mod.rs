// cln_methods_mod.rs

//! generic code to process html

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::auto_generated_mod::common_structs_mod::*;
use crate::on_click;
use crate::utils_mod::*;
use crate::web_sys_mod as w;

pub fn post_request_await_run_response_method<T>(request_method: &str, request_data: T)
where
    T: serde::Serialize,
{
    let data = unwrap!(serde_json::to_value(request_data));
    let rpc = RpcRequest {
        request_method: request_method.to_string(),
        request_data: data,
    };
    let json_string = unwrap!(serde_json::to_string(&rpc));
    let rpc_request = JsValue::from_str(&json_string);

    spawn_local(async move {
        let rpc_request = Some(&rpc_request);
        let resp_body_text = w::fetch_post_response("submit", rpc_request).await;
        let srv_response: RpcResponse = unwrap!(serde_json::from_str(&resp_body_text));
        crate::auto_generated_mod::match_response_method_and_call_function(srv_response).await;
    });
}

// extract only the html inside the <body> </body>
pub fn extract_html(response: &RpcResponse) -> String {
    let response_html = &response.response_html;
    let (html_fragment, _new_pos_cursor) = get_delimited_text(response_html, 0, "<body>", "</body>").unwrap();
    html_fragment
}

pub fn inject_into_html(html_after_process: &str) {
    w::set_inner_html("div_for_wasm_html_injecting", html_after_process);
}

pub fn navigation_on_click() {
    use crate::cln_methods_review_mod::*;
    use crate::cln_methods_verify_mod::*;
    use wasm_bindgen::JsCast;
    on_click!("button_review_new", request_review_new);
    on_click!("button_review_publish", request_review_publish);
    on_click!("button_update_registry_index", request_update_registry_index);
    on_click!("button_verify_project", request_verify_list);
}
