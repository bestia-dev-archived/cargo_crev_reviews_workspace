// page_verify_mod.rs

use function_name::named;
use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::auto_generated_mod::common_structs_mod::*;
use crate::auto_generated_mod::rpc_server;

// use crate::on_click;
use crate::pages_mod::*;
use crate::*;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref VERIFY_ITEM_DATA: Mutex<VerifyItemData> = Mutex::new(VerifyItemData::default());
    static ref VERIFY_LIST_DATA: Mutex<VerifyListData> = Mutex::new(VerifyListData::default());
}

impl PageProcessor for VerifyListData {
    /// process template and push as many &str is needed
    fn process_repetitive_items(&self, name_of_repeat_segment: &str, html_repetitive_template: &str, html_new: &mut String) {
        match name_of_repeat_segment {
            "verify" => {
                w::debug_write(&format!("process_repetitive_items {}", name_of_repeat_segment));
                for (row_num, data) in self.list_of_verify.iter().enumerate() {
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
            "wt_project_dir" => self.project_dir.clone(),
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

impl PageProcessor for VerifyItemData {
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
            "wt_status" => self.status.clone(),
            "wt_crate_name" => self.crate_name.clone(),
            "wt_crate_version" => self.crate_version.clone(),
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

#[named]
pub fn request_verify_list(_element_id: &str) {
    w::debug_write(function_name!());
    let request_data = RpcEmptyData {};
    rpc_server::rpc_verify_project(request_data);
}

#[named]
pub fn page_verify_list(rpc_response: RpcResponse) {
    w::debug_write(function_name!());
    let page_html = page_html(&rpc_response);
    *VERIFY_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(rpc_response.response_data));
    // modal dialog box with error, don't change the html and data
    let html_after_process = VERIFY_LIST_DATA.lock().unwrap().process_html(&page_html);

    inject_into_html(&html_after_process);
    navigation_on_click();

    // on_click for every row of the list
    for (row_num, _item) in VERIFY_LIST_DATA.lock().unwrap().list_of_verify.iter().enumerate() {
        row_on_click!("button_review_edit_or_new", row_num, request_review_edit_or_new);
        row_on_click!("button_open_crev_dev", row_num, button_verify_open_crev_dev_onclick);
        row_on_click!("button_open_crates_io", row_num, button_verify_open_crates_io_onclick);
        row_on_click!("button_open_lib_rs", row_num, button_verify_open_lib_rs_onclick);
        row_on_click!("button_open_source_code", row_num, button_verify_open_source_code_onclick);
    }
}

#[named]
fn request_review_edit_or_new(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_num];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    rpc_server::rpc_review_edit_or_new(request_data);
}

#[named]
fn button_verify_open_crev_dev_onclick(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_num];
    let url = format!("https://web.crev.dev/rust-reviews/crate/{}/", item.crate_name);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_verify_open_lib_rs_onclick(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_num];
    let url = format!("https://lib.rs/crates/{}", item.crate_name);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_verify_open_crates_io_onclick(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_num];
    let url = format!("https://crates.io/crates/{}/{}", item.crate_name, item.crate_version);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_verify_open_source_code_onclick(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_num];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    rpc_server::rpc_review_open_source_code(request_data);
}
