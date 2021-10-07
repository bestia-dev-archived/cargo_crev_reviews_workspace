// cln_methods_version_mod.rs

use function_name::named;
use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;
//use wasm_bindgen::prelude::*;
//use wasm_bindgen::JsCast;

use crate::auto_generated_mod::common_structs_mod::*;
//use crate::auto_generated_mod::srv_methods;

// use crate::on_click;
use crate::html_mod::*;
use crate::utils_mod::join_crate_version;
use crate::*;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref VERSION_ITEM_DATA: Mutex<VersionItemData> = Mutex::new(VersionItemData::default());
    static ref VERSION_LIST_DATA: Mutex<VersionListData> = Mutex::new(VersionListData::default());
}

impl HtmlProcessor for VersionListData {
    /// process template and push as many &str is needed
    fn process_repetitive_items(&self, name_of_repeat_segment: &str, html_repetitive_template: &str, html_new: &mut String) {
        match name_of_repeat_segment {
            "wr_repeat_VersionItemData" => {
                w::debug_write(&format!("process_repetitive_items {}", name_of_repeat_segment));
                for (row_number, data) in self.list_of_version.iter().enumerate() {
                    let list_item_html = data.process_html_with_item(html_repetitive_template, Some(row_number));
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
    fn match_wt(&self, wt_name: &str, _row_num: Option<usize>) -> String {
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

impl HtmlProcessor for VersionItemData {
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
    fn match_wt(&self, wt_name: &str, _row_num: Option<usize>) -> String {
        match wt_name {
            "wt_crate_name" => self.crate_name.clone(),
            "wt_crate_version" => self.crate_version.clone(),
            "wt_crate_name_version" => join_crate_version(&self.crate_name, &self.crate_version),
            "wt_crate_published_by_login" => self.published_by_login.as_ref().unwrap_or(&"".to_string()).clone(),
            "wt_is_src_cached" => {
                if *self.is_src_cached.as_ref().unwrap_or(&false) {
                    "cached".to_string()
                } else {
                    "".to_string()
                }
            }
            "wt_crate_published_date" => self.published_date.clone(),
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
pub fn cln_version_list(srv_response: RpcResponse) {
    w::debug_write(function_name!());
    let html = extract_html(&srv_response);
    *VERSION_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
    // modal dialog box with error, don't change the html and data
    let html_after_process = VERSION_LIST_DATA.lock().unwrap().process_html(&html);

    inject_into_html(&html_after_process);
    navigation_on_click();
}
