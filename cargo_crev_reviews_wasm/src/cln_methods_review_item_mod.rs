// cln_methods_review_item_mod.rs

//! functions to render the html for reviews

use function_name::named;
use lazy_static::__Deref;
use lazy_static::lazy_static;

use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use dev_bestia_html_templating as tmplt;
use dev_bestia_string_utils::*;

use crate::auto_generated_mod::common_structs_mod::*;
use crate::auto_generated_mod::srv_methods;

use crate::html_mod::*;
use crate::utils_mod::join_crate_version;
use crate::*;

// region: mutable static, because it is hard to pass variables around with on_click events
lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref REVIEW_ITEM_DATA: Mutex<ReviewItemData> = Mutex::new(ReviewItemData::default());
}

/// store data in static Mutex because of events like on_click
fn store_to_review_item_data(srv_response: RpcResponse) {
    *REVIEW_ITEM_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
}

// endregion: mutable static, because it is hard to pass variables around with on_click events

impl tmplt::HtmlTemplatingDataTrait for ReviewItemData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ReviewItemData")
    }

    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // log::debug!(&placeholder);
        match placeholder {
            "wt_comment_md" => self.comment_md.clone(),
            "wt_crate_name" => self.crate_name.clone(),
            "wt_crate_version" => self.crate_version.clone(),
            "wt_crate_name_version" => join_crate_version(&self.crate_name, &self.crate_version),
            "wt_thoroughness" => self.thoroughness.clone(),
            "wt_understanding" => self.understanding.clone(),
            "wt_crate_thoroughness_understanding" => format!("{} {}", self.thoroughness, self.understanding),
            "wt_rating" => self.rating.clone(),
            "wt_review_date" => self.date[..10].to_string(),
            "wt_rating_class_color" => format!("review_header0_cell c_{} bold", self.rating),
            "wt_cargo_crev_reviews_version" => env!("CARGO_PKG_VERSION").to_string(),

            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }
    /// boolean : is the next node rendered or not: "wb_" or "sb_"
    fn exists_next_node_or_attribute(&self, placeholder: &str) -> bool {
        log::debug!("exists_next_node_or_attribute: {}", &placeholder);
        match placeholder {
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
            _ => tmplt::utils::match_else_for_exists_next_node_or_attribute(&self.data_model_name(), placeholder),
        }
    }
}

// endregion: HtmlTemplatingDataTrait for data structs

// region: cln methods to render the page and data

#[named]
pub fn cln_review_new(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    store_to_review_item_data(srv_response);
    // the mutex is locked inside a scope. When this structure falls out of scope, the lock will be unlocked.
    let html_after_process = {
        let data = REVIEW_ITEM_DATA.lock().unwrap();
        tmplt::process_html(data.deref(), &html)
    };
    inject_into_html(&html_after_process);

    on_click!("button_review_save", request_review_save);
    on_click!("button_review_close", close_on_click);
}

/// the code for processing the cln_review_edit
/// the data and html are already in static Mutex REVIEW_ITEM_DATA
#[named]
pub fn cln_review_edit(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    store_to_review_item_data(srv_response);
    // the mutex is locked inside a scope. When this structure falls out of scope, the lock will be unlocked.
    let html_after_process = {
        let data = REVIEW_ITEM_DATA.lock().unwrap();
        tmplt::process_html(data.deref(), &html)
    };

    inject_into_html(&html_after_process);

    on_click!("button_review_save", request_review_save);
    on_click!("button_review_close", close_on_click);
}

#[named]
pub fn cln_review_error(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    let data: RpcMessageData = unwrap!(serde_json::from_value(srv_response.response_data));
    let html_after_process = tmplt::process_html(&data, &html);
    w::set_inner_html("div_for_modal", &html_after_process);
    on_click!("modal_close", modal_close_on_click);
}

#[named]
pub fn cln_no_action(_srv_response: RpcResponse) {
    log::info!("{}", function_name!());
}

// endregion: cln methods to render the page and data

// region: functions for event handlers (on_click)

#[named]
pub fn request_review_publish(_element_id: &str) {
    log::info!("{}", function_name!());
    let html = r#"
<div id="modal_message" class="w3_modal">
    <div class="w3_modal_content">
        <code>$ cargo crev publish</code>
        <div>publishing to remote repository. Wait a minute...</div>        
    </div>
</div>"#;
    w::set_inner_html("div_for_modal", html);
    let request_data = RpcEmptyData {};
    srv_methods::srv_review_publish(request_data);
}

#[named]
pub fn request_update_registry_index(_element_id: &str) {
    log::info!("{}", function_name!());
    let html = r#"
    <div id="modal_message" class="w3_modal">
        <div class="w3_modal_content">
            <div>Updating registry index. Wait a minute...</div>        
        </div>
    </div>"#;
    w::set_inner_html("div_for_modal", html);
    let request_data = RpcEmptyData {};
    srv_methods::srv_update_registry_index(request_data);
}

#[named]
pub fn request_review_new(_element_id: &str) {
    log::info!("{}", function_name!());
    let request_data = RpcEmptyData {};
    srv_methods::srv_review_new(request_data);
}

/// send rpc requests
#[named]
fn request_review_save(_element_id: &str) {
    log::info!("{}", function_name!());
    // values from form
    let request_data = ReviewItemData {
        crate_name: w::get_input_element_value_string_by_id("crate_name"),
        crate_version: w::get_input_element_value_string_by_id("crate_version"),
        date: "".to_string(),
        thoroughness: w::get_value_of_radio_group_by_name("thoroughness"),
        understanding: w::get_value_of_radio_group_by_name("understanding"),
        rating: w::get_value_of_radio_group_by_name("rating"),
        comment_md: w::get_text_area_element_value_string_by_id("comment_md"),
    };
    srv_methods::srv_review_save(request_data);
}

fn modal_close_on_click(_element_id: &str) {
    w::set_inner_html("div_for_modal", "");
}

fn close_on_click(_element_id: &str) {
    w::close_tab();
}

// region: functions for event handlers (on_click)
