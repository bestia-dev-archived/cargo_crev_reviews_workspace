// page_main_mod.rs

// use unwrap::unwrap;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// use cargo_crev_reviews_common::*;
use wasm_bindgen_futures::spawn_local;
//use web_sys::Request;
//use web_sys::RequestInit;

use crate::on_click;
use crate::utils_mod as ut;
use crate::web_sys_mod as w;

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn page_main() {
    // fetch page_main.html and inject it
    let resp_body_text = w::fetch_response("pages/new_review.html").await;
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = ut::get_delimited_text(&resp_body_text, 0, "<body>", "</body>").unwrap();
    w::set_inner_html("div_for_wasm_html_injecting", &html_fragment);

    on_click!("button_save_review", button_save_review_on_click);
}

/// requests json_rpc
/// TODO: how to read the value of radio box
fn button_save_review_on_click(_element_id: &str) {
    let crate_name = w::get_input_element_value_string_by_id("crate_name");
    let crate_version = w::get_input_element_value_string_by_id("crate_version");
    let thoroughness = w::get_value_of_radio_group_by_name("thoroughness");
    let understanding = w::get_value_of_radio_group_by_name("understanding");
    let rating = w::get_value_of_radio_group_by_name("rating");
    let comment_md = w::get_text_area_element_value_string_by_id("comment_md");

    let params = cargo_crev_reviews_common::SaveReviewParams {
        crate_name,
        crate_version,
        thoroughness,
        understanding,
        rating,
        comment_md,
    };
    let params = unwrap!(serde_json::to_value(params));
    let rpc = cargo_crev_reviews_common::RpcMethod {
        jsonrpc: String::from("2.0"),
        method: "save_review".to_string(),
        id: 1,
        params,
    };
    let json_string = unwrap!(serde_json::to_string(&rpc));
    let json_js_value = JsValue::from_str(&json_string);
    spawn_local(async move {
        let json = Some(&json_js_value);
        let resp_body_text = w::fetch_post_response("submit", json).await;
        w::set_inner_html("div_for_wasm_html_injecting", &resp_body_text);
    });
}
