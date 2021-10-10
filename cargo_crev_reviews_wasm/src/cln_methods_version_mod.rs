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
                log::info!("process_repetitive_items {}", name_of_repeat_segment);
                for (row_number, data) in self.list_of_version.iter().enumerate() {
                    let list_item_html = data.process_html_with_item(html_repetitive_template, Some(row_number));
                    html_new.push_str(&list_item_html);
                }
            }
            _ => {
                let msg = format!("unrecognized name_of_repeat_segment {}", name_of_repeat_segment);
                log::info!("{}", &msg);
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
                log::error!("{}", &html_error);
                html_error
            }
        }
    }
    /// the use of complete string wb_xxx enables easy and exact text search around the source code
    fn match_wb(&self, wb_name: &str) -> bool {
        match wb_name {
            _ => {
                let html_error = format!("Unrecognized wb_exist_next_attribute method {}", wb_name);
                log::error!("{}", &html_error);
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
                log::info!("{}", &msg);
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
            "wt_crate_yanked_or_cached" => {
                if self.yanked {
                    "yanked".to_string()
                } else if self.is_src_cached.unwrap_or(false) {
                    "cached".to_string()
                } else {
                    "".to_string()
                }
            }
            "wt_crate_yanked_or_cached_class" => {
                if self.yanked {
                    "review_header0_cell c_yanked".to_string()
                } else if self.is_src_cached.unwrap_or(false) {
                    "review_header0_cell c_cached".to_string()
                } else {
                    "review_header0_cell".to_string()
                }
            }
            "wt_crate_published_date" => self.published_date[..10].to_string(),
            "wt_cargo_crev_reviews_version" => env!("CARGO_PKG_VERSION").to_string(),
            // region: Option of my_review
            "wt_rating" => match &self.my_review {
                Some(my_review) => my_review.rating.clone(),
                None => "".to_string(),
            },
            "wt_rating_class_color" => format!(
                "review_header0_cell c_{} bold",
                match &self.my_review {
                    Some(my_review) => my_review.rating.clone(),
                    None => "".to_string(),
                }
            ),
            "wt_review_date" => match &self.my_review {
                Some(my_review) => my_review.date[..10].to_string(),
                None => "".to_string(),
            },
            "wt_crate_thoroughness_understanding" => match &self.my_review {
                Some(my_review) => format!("{} {}", my_review.thoroughness, my_review.understanding),
                None => "".to_string(),
            },
            "wt_comment_md" => match &self.my_review {
                Some(my_review) => my_review.comment_md.clone(),
                None => "".to_string(),
            },
            _ => {
                let html_error = format!("Unrecognized replace_wt method {}", wt_name);
                log::error!("{}", &html_error);
                html_error
            }
        }
    }
    /// the use of complete string wb_xxx enables easy and exact text search around the source code
    /// is the next node rendered or deleted
    fn match_wb(&self, wb_name: &str) -> bool {
        match wb_name {
            "wb_has_review" => self.my_review.is_some(),
            _ => {
                let html_error = format!("Unrecognized wb_exist_next_attribute method {}", wb_name);
                log::error!("{}", &html_error);
                false
            }
        }
    }
}

#[named]
pub fn cln_version_list(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    *VERSION_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
    // modal dialog box with error, don't change the html and data
    let html_after_process = VERSION_LIST_DATA.lock().unwrap().process_html(&html);

    inject_into_html(&html_after_process);
    navigation_on_click();
}
