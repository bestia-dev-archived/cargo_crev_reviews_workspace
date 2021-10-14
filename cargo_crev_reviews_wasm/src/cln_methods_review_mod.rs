// cln_methods_review_mod.rs

//! functions to render the html for reviews

use function_name::named;
use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use dev_bestia_html_templating::html_template_mod::*;

use crate::auto_generated_mod::common_structs_mod::*;
use crate::auto_generated_mod::srv_methods;

use crate::html_mod::*;
use crate::on_click;
use crate::utils_mod::join_crate_version;
use crate::*;

// region: mutable static, because it is hard to pass variables around with on_click events
lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref REVIEW_ITEM_DATA: Mutex<ReviewItemData> = Mutex::new(ReviewItemData::default());
    static ref REVIEW_LIST_DATA: Mutex<ReviewListData> = Mutex::new(ReviewListData::default());
}

/// store data in static Mutex because of events like on_click
fn store_to_review_item_data(srv_response: RpcResponse) {
    *REVIEW_ITEM_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
}

/// store data in static Mutex because of events like on_click
fn store_static_review_list_data(srv_response: RpcResponse) {
    *REVIEW_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
}
// endregion: mutable static, because it is hard to pass variables around with on_click events

// region: HtmlServerTemplateRender for data structs
impl HtmlServerTemplateRender for RpcMessageData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("RpcMessageData")
    }

    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "wt_cargo_crev_reviews_version" => s!(env!("CARGO_PKG_VERSION")),
            "wt_message" => s!(self.message),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
}

impl HtmlServerTemplateRender for ReviewListData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ReviewListData")
    }

    /// renders sub-template: "stmplt_" or "wtmplt_"
    fn render_sub_template(&self, template_name: &str, sub_templates: &Vec<SubTemplate>, prefixes: &PrefixForTemplateVariables) -> Vec<Node> {
        log::info!("{}", template_name);
        match template_name {
            "wtmplt_ReviewItemData" => {
                let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for (row_number, review_item) in self.list_of_review.iter().enumerate() {
                    let vec_node = unwrap!(render_template_raw_to_nodes(
                        review_item,
                        &sub_template.template,
                        HtmlOrSvg::Html,
                        "",
                        row_number,
                        prefixes
                    ));
                    nodes.extend_from_slice(&vec_node);
                }
                // return
                nodes
            }
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }

    /// the use of complete string wt_xxx enables easy and exact text search around the source code
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "wt_cargo_crev_reviews_version" => s!(env!("CARGO_PKG_VERSION")),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
}

impl HtmlServerTemplateRender for ReviewItemData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ReviewItemData")
    }

    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
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

            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// boolean : is the next node rendered or not: "wb_" or "sb_"
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!( &placeholder);
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
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }
}

// endregion: HtmlServerTemplateRender for data structs

// region: cln methods to render the page and data

/// the code for processing the html srv_review_list
/// the data is already in static Mutex REVIEW_LIST_DATA
#[named]
pub fn cln_review_list(srv_response: RpcResponse) {
    log::info!("{}", function_name!());

    let html = extract_html(&srv_response);
    store_static_review_list_data(srv_response);

    // call process with functions as parameters, to use for replace attributes and text nodes
    let html_after_process = REVIEW_LIST_DATA.lock().unwrap().render_html(&html);

    inject_into_html(&html_after_process);

    navigation_on_click();

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
pub fn cln_review_new(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    store_to_review_item_data(srv_response);
    // call process with functions as parameters, to use for replace attributes and text nodes
    let data = &REVIEW_ITEM_DATA.lock().unwrap();
    let html_after_process = data.render_html(&html);
    inject_into_html(&html_after_process);

    on_click!("button_review_save", request_review_save);
    on_click!("button_review_list", request_review_list);
}

/// the code for processing the cln_review_edit
/// the data and html are already in static Mutex REVIEW_ITEM_DATA
#[named]
pub fn cln_review_edit(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    store_to_review_item_data(srv_response);

    // call process with functions as parameters, to use for replace attributes and text nodes
    let data = &REVIEW_ITEM_DATA.lock().unwrap();
    let html_after_process = data.render_html(&html);

    inject_into_html(&html_after_process);

    on_click!("button_review_save", request_review_save);
    on_click!("button_review_list", request_review_list);
}

#[named]
pub fn cln_review_publish_modal(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);

    // modal dialog box with error, don't change the html and data
    let data: RpcMessageData = unwrap!(serde_json::from_value(srv_response.response_data));
    let html_after_process = data.render_html(&html);

    w::set_inner_html("div_for_modal", &html_after_process);
    on_click!("modal_close", modal_close_on_click);
}

#[named]
pub fn cln_review_error(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);

    let data: RpcMessageData = unwrap!(serde_json::from_value(srv_response.response_data));
    let html_after_process = data.render_html(&html);

    w::set_inner_html("div_for_modal", &html_after_process);
    on_click!("modal_close", modal_close_on_click);
}

// endregion: cln methods to render the page and data

// region: functions for event handlers (on_click)

pub fn request_review_list(_element_id: &str) {
    let request_data = RpcEmptyData {};
    srv_methods::srv_reviews_list(request_data);
}

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

fn modal_close_on_click(_element_id: &str) {
    w::set_inner_html("div_for_modal", "");
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

    on_click!("modal_close", modal_close_on_click);
    // I had to add modal_yes_delete(0), because row_on_click works that way.
    row_on_click!("modal_yes_delete", row_number, request_review_delete);
}

#[named]
fn request_review_delete(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
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
