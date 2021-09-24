// page_review_mod.rs

//! functions to render the pages for reviews

use function_name::named;
use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::common_mod::*;

use crate::on_click;
use crate::pages_mod::*;
use crate::*;

// this structs are in the common module: ReviewItemData, ReviewListData

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref REVIEW_ITEM_DATA: Mutex<ReviewItemData> = Mutex::new(ReviewItemData::default());
    static ref REVIEW_LIST_DATA: Mutex<ReviewListData> = Mutex::new(ReviewListData::default());
}

impl PageProcessor for RpcMessageData {
    /// process template and push as many &str is needed
    fn process_repetitive_items(&self, name_of_repeat_segment: &str, _html_repetitive_template: &str, html_new: &mut String) {
        match name_of_repeat_segment {
            _ => {
                let msg = format!("unrecognized name_of_repeat_segment {}", name_of_repeat_segment);
                w::debug_write(&msg);
                html_new.push_str(&msg);
            }
        }
    }

    /// the use of complete string wt_xxx enables easy and exact text search around the source code
    fn match_wt(&self, wt_name: &str) -> String {
        match wt_name {
            "wt_cargo_crev_reviews_version" => env!("CARGO_PKG_VERSION").to_string(),
            "wt_message" => self.message.to_string(),
            _ => {
                let html_error = format!("Unrecognized replace_wt method {}", wt_name);
                w::debug_write(&html_error);
                html_error
            }
        }
    }
    /// the use of complete string wb_xxx enables easy and exact text search around the source code
    fn match_wb(&self, wb_name: &str) -> bool {
        match wb_name {
            _ => {
                let html_error = format!("Unrecognized wb_exist_next_attribute method {}", wb_name);
                w::debug_write(&html_error);
                false
            }
        }
    }
}

impl PageProcessor for ReviewListData {
    /// process template and push as many &str is needed
    fn process_repetitive_items(&self, name_of_repeat_segment: &str, html_repetitive_template: &str, html_new: &mut String) {
        match name_of_repeat_segment {
            "review" => {
                w::debug_write(&format!("process_repetitive_items {}", name_of_repeat_segment));
                for (row_num, data) in self.list_of_review.iter().enumerate() {
                    let list_item_html = data.process_html_with_item(html_repetitive_template, Some(row_num));
                    html_new.push_str(&list_item_html);
                }
            }
            _ => {
                let msg = format!("unrecognized name_of_repeat_segment {}", name_of_repeat_segment);
                w::debug_write(&msg);
                html_new.push_str(&msg);
            }
        }
    }

    /// the use of complete string wt_xxx enables easy and exact text search around the source code
    fn match_wt(&self, wt_name: &str) -> String {
        match wt_name {
            "wt_cargo_crev_reviews_version" => env!("CARGO_PKG_VERSION").to_string(),
            _ => {
                let html_error = format!("Unrecognized replace_wt method {}", wt_name);
                w::debug_write(&html_error);
                html_error
            }
        }
    }
    /// the use of complete string wb_xxx enables easy and exact text search around the source code
    fn match_wb(&self, wb_name: &str) -> bool {
        match wb_name {
            _ => {
                let html_error = format!("Unrecognized wb_exist_next_attribute method {}", wb_name);
                w::debug_write(&html_error);
                false
            }
        }
    }
}

impl PageProcessor for ReviewItemData {
    /// process template and push as many &str is needed
    fn process_repetitive_items(&self, name_of_repeat_segment: &str, _html_repetitive_template: &str, html_new: &mut String) {
        match name_of_repeat_segment {
            _ => {
                let msg = format!("unrecognized name_of_repeat_segment {}", name_of_repeat_segment);
                w::debug_write(&msg);
                html_new.push_str(&msg);
            }
        }
    }

    /// the use of complete string wt_xxx enables easy and exact text search around the source code
    fn match_wt(&self, wt_name: &str) -> String {
        match wt_name {
            "wt_comment_md" => self.comment_md.clone(),
            "wt_crate_name" => self.crate_name.clone(),
            "wt_crate_version" => self.crate_version.clone(),
            "wt_crate_name_version" => format!("{} {}", self.crate_name, self.crate_version),
            "wt_thoroughness" => self.thoroughness.clone(),
            "wt_understanding" => self.understanding.clone(),
            "wt_crate_thoroughness_understanding" => format!("{} {}", self.thoroughness, self.understanding),
            "wt_rating" => self.rating.clone(),
            "wt_review_date" => self.date[..10].to_string(),
            "wt_rating_class_color" => format!("review_header0_cell c_{} bold", self.rating),
            "wt_cargo_crev_reviews_version" => env!("CARGO_PKG_VERSION").to_string(),
            _ => {
                let html_error = format!("Unrecognized replace_wt method {}", wt_name);
                w::debug_write(&html_error);
                html_error
            }
        }
    }
    /// the use of complete string wb_xxx enables easy and exact text search around the source code
    fn match_wb(&self, wb_name: &str) -> bool {
        match wb_name {
            "wb_checked_th_none" => self.thoroughness == "none",
            "wb_checked_th_low" => self.thoroughness == "low",
            "wb_checked_th_medium" => self.thoroughness == "medium",
            "wb_checked_th_high" => self.thoroughness == "high",

            "wb_checked_un_none" => self.understanding == "none",
            "wb_checked_un_low" => self.understanding == "low",
            "wb_checked_un_medium" => self.understanding == "medium",
            "wb_checked_un_high" => self.understanding == "high",

            "wb_checked_ra_none" => self.rating == "none",
            "wb_checked_ra_negative" => self.rating == "negative",
            "wb_checked_ra_neutral" => self.rating == "neutral",
            "wb_checked_ra_positive" => self.rating == "positive",
            "wb_checked_ra_strong" => self.rating == "strong",
            _ => {
                let html_error = format!("Unrecognized wb_exist_next_attribute method {}", wb_name);
                w::debug_write(&html_error);
                false
            }
        }
    }
}

/// store data in static Mutex because of events like on_click
fn store_to_review_item_data(rpc_response: RpcResponse) {
    *REVIEW_ITEM_DATA.lock().unwrap() = unwrap!(serde_json::from_value(rpc_response.response_data));
}

/// store data in static Mutex because of events like on_click
fn store_static_review_list_data(rpc_response: RpcResponse) {
    *REVIEW_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(rpc_response.response_data));
}

pub fn request_review_list(_element_id: &str) {
    post_request_await_run_response_method(RequestMethod::RpcReviewList, RpcEmptyData {});
}

/// the code for processing the page rpc_review_list
/// the data is already in static Mutex REVIEW_LIST_DATA
#[named]
pub fn page_review_list(rpc_response: RpcResponse) {
    w::debug_write(function_name!());

    let page_html = page_html(&rpc_response);
    store_static_review_list_data(rpc_response);

    // call process with functions as parameters, to use for replace attributes and text nodes
    let html_after_process = REVIEW_LIST_DATA.lock().unwrap().process_html(&page_html);

    inject_into_html(&html_after_process);

    navigation_on_click();

    // on_click for every row of the list
    for (row_num, _item) in REVIEW_LIST_DATA.lock().unwrap().list_of_review.iter().enumerate() {
        row_on_click!("button_review_edit", row_num, request_review_edit_from_list);
        row_on_click!("button_review_new_version", row_num, request_review_new_version);
        row_on_click!("button_open_crev_dev", row_num, button_open_crev_dev_onclick);
        row_on_click!("button_open_crates_io", row_num, button_open_crates_io_onclick);
        row_on_click!("button_open_lib_rs", row_num, button_open_lib_rs_onclick);
        row_on_click!("button_open_source_code", row_num, button_open_source_code_onclick);
        row_on_click!("button_review_delete", row_num, modal_delete);
    }
}

#[named]
fn button_open_crates_io_onclick(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_num];
    let url = format!("https://crates.io/crates/{}/{}", item.crate_name, item.crate_version);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_crev_dev_onclick(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_num];
    let url = format!("https://web.crev.dev/rust-reviews/crate/{}/", item.crate_name);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_lib_rs_onclick(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_num];
    let url = format!("https://lib.rs/crates/{}", item.crate_name);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_source_code_onclick(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_num];
    let review_filter_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    post_request_await_run_response_method(RequestMethod::RpcReviewOpenSourceCode, review_filter_data);
}

#[named]
pub fn request_review_publish(_element_id: &str) {
    w::debug_write(function_name!());
    let page_html = r#"
<div id="modal_message" class="w3_modal">
    <div class="w3_modal_content">
        <code>$ cargo crev publish</code>
        <div>publishing to remote repository. Wait a minute...</div>        
    </div>
</div>"#;
    w::set_inner_html("div_for_modal", page_html);
    post_request_await_run_response_method(RequestMethod::RpcReviewPublish, RpcEmptyData {});
}

#[named]
pub fn request_update_registry_index(_element_id: &str) {
    w::debug_write(function_name!());
    let page_html = r#"
    <div id="modal_message" class="w3_modal">
        <div class="w3_modal_content">
            <div>Updating registry index. Wait a minute...</div>        
        </div>
    </div>"#;
    w::set_inner_html("div_for_modal", page_html);
    post_request_await_run_response_method(RequestMethod::RpcUpdateRegistryIndex, RpcEmptyData {});
}

#[named]
pub fn request_review_new(_element_id: &str) {
    w::debug_write(function_name!());
    post_request_await_run_response_method(RequestMethod::RpcReviewNew, RpcEmptyData {});
}

#[named]
pub fn page_review_new(rpc_response: RpcResponse) {
    w::debug_write(function_name!());
    let page_html = page_html(&rpc_response);
    store_to_review_item_data(rpc_response);
    // call process with functions as parameters, to use for replace attributes and text nodes
    let data = &REVIEW_ITEM_DATA.lock().unwrap();
    let html_after_process = data.process_html(&page_html);
    inject_into_html(&html_after_process);

    on_click!("button_review_save", request_review_save);
    on_click!("button_review_list", request_review_list);
}

#[named]
fn request_review_new_version(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_num];
    let review_filter_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };

    post_request_await_run_response_method(RequestMethod::RpcReviewNewVersion, review_filter_data);
}

/// send rpc requests
#[named]
fn request_review_save(_element_id: &str) {
    w::debug_write(function_name!());
    // values from page and form
    let request_data = ReviewItemData {
        crate_name: w::get_input_element_value_string_by_id("crate_name"),
        crate_version: w::get_input_element_value_string_by_id("crate_version"),
        date: "".to_string(),
        thoroughness: w::get_value_of_radio_group_by_name("thoroughness"),
        understanding: w::get_value_of_radio_group_by_name("understanding"),
        rating: w::get_value_of_radio_group_by_name("rating"),
        comment_md: w::get_text_area_element_value_string_by_id("comment_md"),
    };
    post_request_await_run_response_method(RequestMethod::RpcReviewSave, request_data);
}

#[named]
fn request_review_edit_from_list(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_num];
    let review_filter_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };

    post_request_await_run_response_method(RequestMethod::RpcReviewEdit, review_filter_data);
}

/// the code for processing the page_review_edit
/// the data and html are already in static Mutex REVIEW_ITEM_DATA
#[named]
pub fn page_review_edit(rpc_response: RpcResponse) {
    w::debug_write(function_name!());
    let page_html = page_html(&rpc_response);
    store_to_review_item_data(rpc_response);

    // call process with functions as parameters, to use for replace attributes and text nodes
    let data = &REVIEW_ITEM_DATA.lock().unwrap();
    let html_after_process = data.process_html(&page_html);

    inject_into_html(&html_after_process);

    on_click!("button_review_save", request_review_save);
    on_click!("button_review_list", request_review_list);
}

#[named]
pub fn page_review_error(rpc_response: RpcResponse) {
    w::debug_write(function_name!());
    let page_html = page_html(&rpc_response);

    let data: RpcMessageData = unwrap!(serde_json::from_value(rpc_response.response_data));
    let html_after_process = data.process_html(&page_html);

    w::set_inner_html("div_for_modal", &html_after_process);
    on_click!("modal_close", modal_close_on_click);
}

fn modal_close_on_click(_element_id: &str) {
    w::set_inner_html("div_for_modal", "");
}

#[named]
pub fn page_review_publish_modal(rpc_response: RpcResponse) {
    w::debug_write(function_name!());
    let page_html = page_html(&rpc_response);

    // modal dialog box with error, don't change the html and data
    let data: RpcMessageData = unwrap!(serde_json::from_value(rpc_response.response_data));
    let html_after_process = data.process_html(&page_html);

    w::set_inner_html("div_for_modal", &html_after_process);
    on_click!("modal_close", modal_close_on_click);
}

#[named]
pub fn modal_delete(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    let page_html = format!(
        r#"
    <div id="modal_message" class="w3_modal">
        <div class="w3_modal_content">
            <div>Do you really want to delete?</div>        
            <button id="modal_yes_delete({})">Yes</button>
            <button id="modal_close">No</button>
        </div>
    </div>"#,
        row_num
    );
    w::set_inner_html("div_for_modal", &page_html);

    on_click!("modal_close", modal_close_on_click);
    // I had to add modal_yes_delete(0), because row_on_click works that way.
    row_on_click!("modal_yes_delete", row_num, request_review_delete);
}

#[named]
fn request_review_delete(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    modal_close_on_click("");

    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_num];
    let review_filter_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };

    post_request_await_run_response_method(RequestMethod::RpcReviewDelete, review_filter_data);
}
