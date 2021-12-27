// cln_methods_publisher_mod.rs

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

// use crate::on_click;
use crate::{html_mod::*, on_click, row_on_click};

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref PUBLISHER_ITEM_DATA: Mutex<PublisherItemData> = Mutex::new(PublisherItemData::default());
    static ref PUBLISHER_LIST_DATA: Mutex<PublisherListData> = Mutex::new(PublisherListData::default());
}

impl tmplt::HtmlTemplatingDataTrait for PublisherListData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("PublisherListData")
    }

    /// renders sub-template: "stmplt_" or "wtmplt_"
    fn process_sub_template(&self, template_name: &str, sub_templates: &Vec<tmplt::utils::SubTemplate>) -> Vec<tmplt::utils::Node> {
        log::info!("{}", template_name);
        match template_name {
            "wtmplt_PublisherItemData" => {
                let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for (row_number, item) in self.list_of_publisher.iter().enumerate() {
                    let vec_node = unwrap!(tmplt::utils::process_template_raw_to_nodes(
                        item,
                        &sub_template.template,
                        tmplt::utils::HtmlOrSvg::Html,
                        "",
                        row_number,
                    ));
                    // log::debug!("vec_node {:?}", &vec_node);
                    nodes.extend_from_slice(&vec_node);
                }

                // return
                nodes
            }
            _ => tmplt::utils::match_else_for_process_sub_template(&self.data_model_name(), template_name),
        }
    }

    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // log::debug!(&placeholder);
        match placeholder {
            "wt_cargo_crev_reviews_version" => s!(env!("CARGO_PKG_VERSION")),
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }
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
            "wt_publisher_url" => self.url.clone(),
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

// open_publisher_list > routing_publisher_list > srv_publisher_list > cln_publisher_list

#[named]
pub fn open_publisher_list(_element_id: &str) {
    log::info!("{}", function_name!());
    let url = format!("index.html#publisher_list");
    unwrap!(w::window().open_with_url(&url));
}

pub fn routing_publisher_list() {
    let request_data = RpcEmptyData {};
    srv_methods::srv_publisher_list(request_data);
}

#[named]
pub fn cln_publisher_list(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    *PUBLISHER_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
    // the mutex is locked inside a scope. When this structure falls out of scope, the lock will be unlocked.
    let html_after_process = {
        let data = PUBLISHER_LIST_DATA.lock().unwrap();
        tmplt::process_html(data.deref(), &html)
    };

    inject_into_html(&html_after_process);

    on_click!("button_new_publisher", button_new_publisher_on_click);

    // on_click for every row of the list
    for (row_number, _item) in PUBLISHER_LIST_DATA.lock().unwrap().list_of_publisher.iter().enumerate() {
        row_on_click!("publisher_url", row_number, button_open_publisher_url_onclick);
    }
}

#[named]
fn button_open_publisher_url_onclick(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and publisher
    let item = &PUBLISHER_LIST_DATA.lock().unwrap().list_of_publisher[row_number];
    let url = format!("{}", item.url);
    unwrap!(w::window().open_with_url(&url));
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
    w::set_inner_html("div_for_modal", &html);

    use crate::cln_methods_mod::modal_close_on_click;

    on_click!("modal_close", modal_close_on_click);
    on_click!("publisher_save", request_publisher_save);
}

/// send rpc requests
#[named]
fn request_publisher_save(_element_id: &str) {
    log::info!("{}", function_name!());
    // values from form
    let request_data = PublisherItemData {
        url: w::get_input_element_value_string_by_id("modal_publisher_url"),
        note: w::get_text_area_element_value_string_by_id("modal_note"),
    };
    srv_methods::srv_publisher_save(request_data);
    // srv returns response_modal_close
}

pub fn published_by_url_shorten(url: &str) -> &str {
    if url.starts_with("https://github.com/") {
        return &url[19..];
    } else if url.starts_with("https://") {
        return &url[8..];
    } else {
        return url;
    }
}

/*

#[named]
pub fn modal_delete(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    let html = format!(
        r#"
    <div id="modal_message" class="w3_modal">
        <div class="w3_modal_content">
            <div>Do you really want to delete your review?</div>
            <button id="modal_yes_delete({})">Yes</button>
            <button id="modal_close">No</button>
        </div>
    </div>"#,
        row_number
    );
    w::set_inner_html("div_for_modal", &html);
use crate::cln_methods_mod::modal_close_on_click;
    on_click!("modal_close", modal_close_on_click);
    // I had to add modal_yes_delete(0), because row_on_click works that way.
    row_on_click!("modal_yes_delete", row_number, request_review_delete);
}


#[named]
fn request_review_delete(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    modal_close_on_click("");

    // from list get crate name and version
    let item = &PUBLISHER_LIST_DATA.lock().unwrap().list_of_version[row_number];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_delete(request_data);
}
 */
