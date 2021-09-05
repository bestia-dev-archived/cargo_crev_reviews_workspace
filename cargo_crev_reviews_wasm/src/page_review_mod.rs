// page_review_mod.rs

use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use cargo_crev_reviews_common::*;

use crate::on_click;
// use crate::web_sys_mod as w;
use crate::*;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref REVIEW_ITEM_DATA: Mutex<ReviewItemParams> = Mutex::new(ReviewItemParams::default());
    static ref REVIEW_LIST_DATA: Mutex<ReviewListParams> = Mutex::new(ReviewListParams::default());
}

// this structs are in the common project:
// ReviewItemParams

pub async fn request_review_list() {
    // dummy message, just to satisfy the request struct
    let params = cargo_crev_reviews_common::RpcMessageParams { message: "".to_string() };
    let rpc_request = rpc_request_value(params, "review_list");
    spawn_local(async move {
        let rpc_response = crate::pages_mod::post_request(rpc_request).await;
        match rpc_response.response_method.as_str() {
            "page_review_list" => {
                // prepare the static Mutex for data and call the function
                *REVIEW_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(rpc_response.response_params));
                page_review_list(&rpc_response.page_html);
            }
            _ => w::debug_write(&format!("Error: Unrecognized client_method {}", &rpc_response.response_method)),
        }
    });
}

/// the code for processing the page review_list
/// the data is already in static Mutex REVIEW_LIST_DATA
pub fn page_review_list(page_html: &str) {
    w::debug_write("page_review_list()");
    // lock static variable with page data
    let list_data = REVIEW_LIST_DATA.lock().unwrap();
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = get_delimited_text(page_html, 0, "<body>", "</body>").unwrap();

    // call process with functions as parameters, to use for replace attributes and text nodes
    let html_after_process = crate::pages_mod::process_list_html_templates(
        &html_fragment,
        &list_data,
        &review_replace_next_attribute,
        &review_replace_next_text_node,
        &review_exist_next_attribute,
    );

    w::set_inner_html("div_for_wasm_html_injecting", &html_after_process);

    //on_click!("button_review_edit", button_review_edit_on_click);
    // on_click!("button_review_delete", button_review_delete_on_click);
    // on_click!("button_review_publish", button_review_publish_on_click);
}

// region: new

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn page_review_new() {
    w::debug_write("page_review_new()");
    // fetch page_main.html and inject it
    let resp_body_text = w::fetch_response("pages/review_new.html").await;
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = get_delimited_text(&resp_body_text, 0, "<body>", "</body>").unwrap();
    w::set_inner_html("div_for_wasm_html_injecting", &html_fragment);

    on_click!("button_review_save", button_review_save_on_click);
}

/// send rpc requests
fn button_review_save_on_click(_element_id: &str) {
    w::debug_write("button_review_save_on_click()");
    // values from page and form
    let params = cargo_crev_reviews_common::ReviewItemParams {
        crate_name: w::get_input_element_value_string_by_id("crate_name"),
        crate_version: w::get_input_element_value_string_by_id("crate_version"),
        date: "".to_string(),
        thoroughness: w::get_value_of_radio_group_by_name("thoroughness"),
        understanding: w::get_value_of_radio_group_by_name("understanding"),
        rating: w::get_value_of_radio_group_by_name("rating"),
        comment_md: w::get_text_area_element_value_string_by_id("comment_md"),
    };
    let rpc_request = rpc_request_value(params, "review_save");
    spawn_local(async move {
        let rpc_response = crate::pages_mod::post_request(rpc_request).await;
        match rpc_response.response_method.as_str() {
            "page_review_show" => {
                // prepare the static Mutex for data and call the function
                *REVIEW_ITEM_DATA.lock().unwrap() = unwrap!(serde_json::from_value(rpc_response.response_params));
                page_review_show(&rpc_response.page_html);
            }
            "page_review_error" => {
                // show dialog box with error, don't change the html and params
                let err: cargo_crev_reviews_common::RpcMessageParams = unwrap!(serde_json::from_value(rpc_response.response_params));
                unwrap!(w::window().alert_with_message(err.message.as_str()));
            }
            _ => w::debug_write(&format!("Error: Unrecognized client_method {}", &rpc_response.response_method)),
        }
    });
}

// endregion: new

// region: show

/// the code for processing the page review_show
/// the data and html are already in static Mutex REVIEW_ITEM_DATA
pub fn page_review_show(page_html: &str) {
    w::debug_write("page_review_show()");
    // lock static variable with page data
    let params = REVIEW_ITEM_DATA.lock().unwrap();
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = get_delimited_text(page_html, 0, "<body>", "</body>").unwrap();

    // call process with functions as parameters, to use for replace attributes and text nodes
    let html_after_process = crate::pages_mod::process_html(
        &html_fragment,
        &params,
        &review_replace_next_attribute,
        &review_replace_next_text_node,
        &review_exist_next_attribute,
    );

    w::debug_write(&html_after_process);
    w::set_inner_html("div_for_wasm_html_injecting", &html_after_process);

    on_click!("button_review_edit", button_review_edit_on_click);
    // on_click!("button_review_delete", button_review_delete_on_click);
    // on_click!("button_review_publish", button_review_publish_on_click);
}

/// if the `next_attribute_replace` is not None then replace attribute with `next_attribute_replace`
/// if attribute starts with data-wt_ it is a replace command. Like: data-wt_width="width" width="90"
/// the attribute value is the name of the next attribute, just for security
/// Execute the client_method and save the result in `next_attribute_replace`, don't push attribute to string
fn review_replace_next_attribute(name: &str, _value: &str, next_attribute_replace: &mut Option<(&str, String)>, params: &ReviewItemParams) -> String {
    w::debug_write("replace_next_attribute");
    // returns mostly empty string because it is all written in next_attribute_replace
    // only in case of error it writes something in the html, to find where the error occurred
    let mut html_error = String::new();
    match name {
        "data-wt_crate_name" => *next_attribute_replace = Some(("value", params.crate_name.clone())),
        "data-wt_crate_version" => *next_attribute_replace = Some(("value", params.crate_version.clone())),
        "data-wt_thoroughness" => *next_attribute_replace = Some(("value", params.thoroughness.clone())),
        "data-wt_understanding" => *next_attribute_replace = Some(("value", params.understanding.clone())),
        "data-wt_rating" => *next_attribute_replace = Some(("value", params.rating.clone())),
        _ => {
            html_error = format!("Unrecognized replace_next_attribute method {}", name);
            w::debug_write(&html_error);
        }
    }
    // return
    html_error
}

/// if the comment is like <!--wt_method_name-->, starts with `wt_` (web browser text)
/// Execute the replace_method and save the result in `next_text_node_replace`.
/// On the next text node it will use this value.
fn review_replace_next_text_node(txt: &str, next_text_node_replace: &mut Option<String>, params: &ReviewItemParams) -> String {
    w::debug_write("review_replace_next_text_node");
    // returns mostly empty string because it is all written in next_attribute_replace
    // only in case of error it writes something in the html, to find where the error occurred
    let mut html_error = String::new();
    match txt {
        "wt_comment_md" => *next_text_node_replace = Some(params.comment_md.clone()),
        "wt_crate_name_version" => *next_text_node_replace = Some(format!("{} {}", params.crate_name, params.crate_version)),
        "wt_crate_thoroughness_understanding" => *next_text_node_replace = Some(format!("{} {}", params.thoroughness, params.understanding)),
        "wt_rating" => *next_text_node_replace = Some(params.rating.clone()),
        "wt_review_date" => *next_text_node_replace = Some(params.date[..10].to_string()),
        _ => {
            html_error = format!("Unrecognized replace_next_text_node method {}", txt);
            w::debug_write(&html_error);
        }
    }
    // return
    html_error
}

/// if the attribute is like `data-wb_checked_th_none="checked" checked="checked"`, starts with `wb_` (web browser bool)
/// Execute the exists_method and store in `next_attribute_exist`
/// The next attribute will exist or not because of this bool.
fn review_exist_next_attribute(name: &str, _value: &str, next_attribute_exist: &mut Option<bool>, params: &ReviewItemParams) -> String {
    w::debug_write("replace_next_text_node");
    let mut html_error = String::new();
    match name {
        "data-wb_checked_th_none" => *next_attribute_exist = Some(if params.thoroughness == "none" { true } else { false }),
        "data-wb_checked_th_low" => *next_attribute_exist = Some(if params.thoroughness == "low" { true } else { false }),
        "data-wb_checked_th_medium" => *next_attribute_exist = Some(if params.thoroughness == "medium" { true } else { false }),
        "data-wb_checked_th_high" => *next_attribute_exist = Some(if params.thoroughness == "high" { true } else { false }),

        "data-wb_checked_un_none" => *next_attribute_exist = Some(if params.understanding == "none" { true } else { false }),
        "data-wb_checked_un_low" => *next_attribute_exist = Some(if params.understanding == "low" { true } else { false }),
        "data-wb_checked_un_medium" => *next_attribute_exist = Some(if params.understanding == "medium" { true } else { false }),
        "data-wb_checked_un_high" => *next_attribute_exist = Some(if params.understanding == "high" { true } else { false }),

        "data-wb_checked_ra_none" => *next_attribute_exist = Some(if params.rating == "none" { true } else { false }),
        "data-wb_checked_ra_negative" => *next_attribute_exist = Some(if params.rating == "negative" { true } else { false }),
        "data-wb_checked_ra_neutral" => *next_attribute_exist = Some(if params.rating == "neutral" { true } else { false }),
        "data-wb_checked_ra_positive" => *next_attribute_exist = Some(if params.rating == "positive" { true } else { false }),
        "data-wb_checked_ra_strong" => *next_attribute_exist = Some(if params.rating == "strong" { true } else { false }),
        _ => {
            html_error = format!("Unrecognized review_exist_next_attribute method {}", name);
            w::debug_write(&html_error);
        }
    }
    // return
    html_error
}

/// send rpc requests
fn button_review_edit_on_click(_element_id: &str) {
    w::debug_write("button_review_edit_on_click()");
    // lock static variable with page data
    let params = REVIEW_ITEM_DATA.lock().unwrap();

    let params = cargo_crev_reviews_common::ReviewItemParams {
        crate_name: params.crate_name.to_owned(),
        crate_version: params.crate_version.to_owned(),
        date: "".to_string(),
        thoroughness: params.thoroughness.to_owned(),
        understanding: params.understanding.to_owned(),
        rating: params.rating.to_owned(),
        comment_md: params.comment_md.to_owned(),
    };
    let rpc_request = rpc_request_value(params, "review_edit");

    spawn_local(async move {
        let rpc_response = crate::pages_mod::post_request(rpc_request).await;

        if rpc_response.response_method == "page_review_edit" {
            // prepare the static Mutex and call the function
            *REVIEW_ITEM_DATA.lock().unwrap() = unwrap!(serde_json::from_value(rpc_response.response_params));
            page_review_edit(&rpc_response.page_html);
        }
    });
}
// endregion: show

// region: edit

/// the code for processing the page review_edit
/// the data and html are already in static Mutex REVIEW_ITEM_DATA
pub fn page_review_edit(page_html: &str) {
    w::debug_write("page_review_edit()");
    // lock static variable with page data
    let params = REVIEW_ITEM_DATA.lock().unwrap();
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = get_delimited_text(&page_html, 0, "<body>", "</body>").unwrap();

    // call process with functions as parameters, to use for replace attributes and text nodes
    let html_after_process = crate::pages_mod::process_html(
        &html_fragment,
        &params,
        &review_replace_next_attribute,
        &review_replace_next_text_node,
        &review_exist_next_attribute,
    );

    w::debug_write(&html_after_process);
    w::set_inner_html("div_for_wasm_html_injecting", &html_after_process);

    on_click!("button_review_save", button_review_save_on_click);
}

// endregion: edit
