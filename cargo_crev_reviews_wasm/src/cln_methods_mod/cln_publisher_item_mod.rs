// cln_publisher_item_mod.rs

use crate::web_sys_mod as w;
use function_name::named;
use lazy_static::{__Deref, lazy_static};
use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use dev_bestia_html_templating as tmplt;
use dev_bestia_string_utils::*;

use crate::auto_generated_mod::{common_structs_mod::*, srv_methods};
//use crate::auto_generated_mod::srv_methods;
use crate::{html_mod::*, on_click};

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref PUBLISHER_ITEM_DATA: Mutex<PublisherItemData> = Mutex::new(PublisherItemData::default());
}

/// store data in static Mutex because of events like on_click
fn store_to_publisher_item_data(srv_response: RpcResponse) {
    *PUBLISHER_ITEM_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
}

impl tmplt::HtmlTemplatingDataTrait for PublisherItemData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("PublisherItemData")
    }
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // log::debug!(&placeholder);
        match placeholder {
            "wt_publisher_url" => self.publisher_url.clone(),
            "wt_note" => self.note.clone(),
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }

    /// boolean : is the next node rendered or not: "wb_" or "sb_"
    fn exists_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // log::debug!( &placeholder);
        match placeholder {
            _ => tmplt::utils::match_else_for_exists_next_node_or_attribute(&self.data_model_name(), placeholder),
        }
    }
}

#[named]
pub fn button_new_publisher_on_click(_element_id: &str) {
    log::info!("{}", function_name!());
    let request_data = RpcEmptyData {};
    srv_methods::srv_publisher_new(request_data);
    // button_new_publisher_on_click > srv_publisher_new > cln_publisher_new_modal
}

#[named]
pub fn cln_publisher_new_modal(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    // button_new_publisher_on_click > srv_publisher_new > cln_publisher_new_modal
    let html = extract_html(&srv_response);
    show_modal_html(&html);

    use crate::cln_methods_mod::cln_utils_mod::modal_close_on_click;

    on_click!("modal_close", modal_close_on_click);
    on_click!("publisher_save", request_publisher_save);
}

#[named]
pub fn cln_publisher_edit_modal(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    // button_edit_publisher_on_click > srv_publisher_edit > cln_publisher_edit_modal
    let html = extract_html(&srv_response);
    store_to_publisher_item_data(srv_response);
    // process with PublisherItemData
    let html_after_process = {
        let data = PUBLISHER_ITEM_DATA.lock().unwrap();
        tmplt::process_html(data.deref(), &html)
    };
    show_modal_html(&html_after_process);

    use crate::cln_methods_mod::cln_utils_mod::modal_close_on_click;
    on_click!("modal_close", modal_close_on_click);
    on_click!("publisher_save", request_publisher_save);
    on_click!("publisher_delete", request_publisher_delete);
}

/// send rpc requests
#[named]
fn request_publisher_save(_element_id: &str) {
    log::info!("{}", function_name!());
    // values from form
    let request_data = PublisherItemData {
        publisher_url: w::get_input_element_value_string_by_id("modal_publisher_url"),
        note: w::get_text_area_element_value_string_by_id("modal_note"),
    };
    srv_methods::srv_publisher_save(request_data);
    // srv returns response_modal_close
}

pub fn published_by_url_shorten(publisher_url: &str) -> &str {
    if publisher_url.starts_with("https://github.com/") {
        return &publisher_url[19..];
    } else if publisher_url.starts_with("https://") {
        return &publisher_url[8..];
    } else {
        return publisher_url;
    }
}

#[named]
fn request_publisher_delete(_element_id: &str) {
    log::info!("{}", function_name!());

    let request_data = PublisherFilterData {
        publisher_url: w::get_input_element_value_string_by_id("modal_publisher_url"),
    };
    srv_methods::srv_publisher_delete(request_data);
}
