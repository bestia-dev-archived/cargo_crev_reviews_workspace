// cln_methods_verify_mod.rs

use function_name::named;
use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::auto_generated_mod::common_structs_mod::*;
use crate::auto_generated_mod::srv_methods;

// use crate::on_click;
use crate::html_mod::*;
use crate::utils_mod::join_crate_version;
use crate::*;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref VERIFY_ITEM_DATA: Mutex<VerifyItemData> = Mutex::new(VerifyItemData::default());
    static ref VERIFY_LIST_DATA: Mutex<VerifyListData> = Mutex::new(VerifyListData::default());
}

impl HtmlProcessor for VerifyListData {
    /// process template and push as many &str is needed
    fn process_repetitive_items(&self, name_of_repeat_segment: &str, html_repetitive_template: &str, html_new: &mut String) {
        match name_of_repeat_segment {
            "VerifyItemData" => {
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

impl HtmlProcessor for VerifyItemData {
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
            "wt_my_review" => self.my_review.clone(),
            "wt_crate_name" => self.crate_name.clone(),
            "wt_crate_version" => self.crate_version.clone(),
            "wt_crate_name_version" => join_crate_version(&self.crate_name, &self.crate_version),
            "wt_published_by" => self.published_by.clone(),
            "wt_cargo_crev_reviews_version" => env!("CARGO_PKG_VERSION").to_string(),
            "wt_status_class" => format!("review_header0_cell left c_{}", &self.status),
            "wt_published_by_class" => format!("review_header0_cell left c_{}", &self.trusted_publisher),
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
    srv_methods::srv_verify_project(request_data);
}

#[named]
pub fn cln_verify_list(srv_response: RpcResponse) {
    w::debug_write(function_name!());
    let html = extract_html(&srv_response);
    *VERIFY_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
    // modal dialog box with error, don't change the html and data
    let html_after_process = VERIFY_LIST_DATA.lock().unwrap().process_html(&html);

    inject_into_html(&html_after_process);
    navigation_on_click();

    // on_click for every row of the list
    for (row_num, _item) in VERIFY_LIST_DATA.lock().unwrap().list_of_verify.iter().enumerate() {
        row_on_click!("crate_name_version", row_num, open_all_links);
    }
}

#[named]
pub fn request_review_edit_or_new(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    // from list get crate name and version
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_num];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_edit_or_new(request_data);
}

#[named]
fn open_all_links(_element_id: &str, row_num: usize) {
    w::debug_write(function_name!());
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_num];
    /*
       let url = format!("https://web.crev.dev/rust-reviews/crate/{}/", item.crate_name);
       unwrap!(w::window().open_with_url(&url));

       let url = format!("https://lib.rs/crates/{}", item.crate_name);
       unwrap!(w::window().open_with_url(&url));

       let url = format!("https://crates.io/crates/{}/{}", item.crate_name, item.crate_version);
       unwrap!(w::window().open_with_url(&url));

       let request_data = ReviewFilterData {
           crate_name: item.crate_name.clone(),
           crate_version: Some(item.crate_version.clone()),
           old_crate_version: None,
       };
       srv_methods::srv_review_open_source_code(request_data);
    */
    // list versions for this crate
    let url = format!(
        "http://{}:{}/{}/index.html#version_list/{}",
        SERVER_HOST.as_str(),
        SERVER_PORT.as_str(),
        SERVER_FIRST_SUBDIRECTORY.as_str(),
        item.crate_name,
    );
    unwrap!(w::window().open_with_url(&url));
    /*
    // edit_or_new in a new tab
    let url = format!(
        "http://{}:{}/{}/index.html#edit_or_new/{}/{}",
        SERVER_HOST.as_str(),
        SERVER_PORT.as_str(),
        SERVER_FIRST_SUBDIRECTORY.as_str(),
        item.crate_name,
        item.crate_version,
    );
    unwrap!(w::window().open_with_url(&url));
     */
}
