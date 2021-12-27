// cln_methods_review_list_mod.rs

//! functions to render the html for reviews list

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
// use crate::utils_mod::join_crate_version;
use crate::*;

// region: mutable static, because it is hard to pass variables around with on_click events
lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref REVIEW_LIST_DATA: Mutex<ReviewListData> = Mutex::new(ReviewListData::default());
}

/// store data in static Mutex because of events like on_click
fn store_static_review_list_data(srv_response: RpcResponse) {
    *REVIEW_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
}
// endregion: mutable static, because it is hard to pass variables around with on_click events

impl tmplt::HtmlTemplatingDataTrait for ReviewListData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ReviewListData")
    }

    /// renders sub-template: "stmplt_" or "wtmplt_"
    fn process_sub_template(&self, template_name: &str, sub_templates: &Vec<tmplt::utils::SubTemplate>) -> Vec<tmplt::utils::Node> {
        log::info!("{}", template_name);
        match template_name {
            "wtmplt_ReviewItemData" => {
                let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for (row_number, review_item) in self.list_of_review.iter().enumerate() {
                    let vec_node = unwrap!(tmplt::utils::process_template_raw_to_nodes(
                        review_item,
                        &sub_template.template,
                        tmplt::utils::HtmlOrSvg::Html,
                        "",
                        row_number,
                    ));
                    nodes.extend_from_slice(&vec_node);
                }
                // return
                nodes
            }
            _ => tmplt::utils::match_else_for_process_sub_template(&self.data_model_name(), template_name),
        }
    }

    /// the use of complete string wt_xxx enables easy and exact text search around the source code
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // log::debug!(&placeholder);
        match placeholder {
            "wt_cargo_crev_reviews_version" => s!(env!("CARGO_PKG_VERSION")),
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }
}

// endregion: HtmlTemplatingDataTrait for data structs

// region: cln methods to render the page and data

pub fn routing_edit_or_new(param2: &str, param3: &str) {
    let request_data = ReviewFilterData {
        crate_name: param2.to_string(),
        crate_version: Some(param3.to_string()),
        old_crate_version: None,
    };
    srv_methods::srv_review_edit_or_new(request_data);
}
/// the code for processing the html srv_review_list
/// the data is already in static Mutex REVIEW_LIST_DATA
#[named]
pub fn cln_review_list(srv_response: RpcResponse) {
    log::info!("{}", function_name!());

    let html = extract_html(&srv_response);
    store_static_review_list_data(srv_response);

    // the mutex is locked inside a scope. When this structure falls out of scope, the lock will be unlocked.
    let html_after_process = {
        let data = REVIEW_LIST_DATA.lock().unwrap();
        tmplt::process_html(data.deref(), &html)
    };

    inject_into_html(&html_after_process);

    // on_click for every row of the list
    for (row_number, _item) in REVIEW_LIST_DATA.lock().unwrap().list_of_review.iter().enumerate() {
        row_on_click!("button_review_edit", row_number, request_review_edit_from_list);
        row_on_click!("button_review_new_version", row_number, request_review_new_version);
        row_on_click!("button_open_crev_dev", row_number, button_open_crev_dev_onclick);
        row_on_click!("button_open_crates_io", row_number, button_open_crates_io_onclick);
        row_on_click!("button_open_lib_rs", row_number, button_open_lib_rs_onclick);
        row_on_click!("button_open_source_code", row_number, button_open_source_code_onclick);
        row_on_click!("button_review_delete", row_number, modal_delete);
    }
}

#[named]
pub fn cln_review_publish_modal(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    let data: RpcMessageData = unwrap!(serde_json::from_value(srv_response.response_data));
    let html_after_process = tmplt::process_html(&data, &html);
    w::set_inner_html("div_for_modal", &html_after_process);
    use crate::cln_methods_mod::modal_close_on_click;
    on_click!("modal_close", modal_close_on_click);
}

// endregion: cln methods to render the page and data

// region: functions for event handlers (on_click)

#[named]
fn button_open_crates_io_onclick(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_number];
    let url = format!("https://crates.io/crates/{}/{}", item.crate_name, item.crate_version);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_crev_dev_onclick(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_number];
    let url = format!("https://web.crev.dev/rust-reviews/crate/{}/", item.crate_name);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_lib_rs_onclick(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_number];
    let url = format!("https://lib.rs/crates/{}", item.crate_name);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_source_code_onclick(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_number];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_open_source_code(request_data);
}

#[named]
pub fn request_update_registry_index(_element_id: &str) {
    log::info!("{}", function_name!());
    let html = r#"
    <div id="modal_message" class="w3_modal">
        <div class="w3_modal_content">
            <div>Updating local cargo registry index. Wait a minute...</div>
            <div>This is necessary for your own crates that are published to crates.io just a moment ago.</div>
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

#[named]
fn request_review_new_version(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_number];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_new_version(request_data);
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

#[named]
fn request_review_edit_from_list(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_number];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_edit(request_data);
}

#[named]
pub fn modal_delete(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    let html = format!(
        r#"
    <div id="modal_message" class="w3_modal">
        <div class="w3_modal_content">
            <div>Do you really want to delete?</div>        
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
    use crate::cln_methods_mod::modal_close_on_click;
    modal_close_on_click("");

    // from list get crate name and version
    let item = &REVIEW_LIST_DATA.lock().unwrap().list_of_review[row_number];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_delete(request_data);
}

// region: functions for event handlers (on_click)
