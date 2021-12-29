// cln_version_mod.rs

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
use crate::utils_mod::join_crate_version;
use crate::{html_mod::*, on_click, row_on_click};

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref VERSION_ITEM_DATA: Mutex<VersionItemData> = Mutex::new(VersionItemData::default());
    static ref VERSION_LIST_DATA: Mutex<VersionListData> = Mutex::new(VersionListData::default());
}

impl tmplt::HtmlTemplatingDataTrait for VersionListData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("VersionListData")
    }

    /// renders sub-template: "stmplt_" or "wtmplt_"
    fn process_sub_template(&self, template_name: &str, sub_templates: &Vec<tmplt::utils::SubTemplate>) -> Vec<tmplt::utils::Node> {
        log::info!("{}", template_name);
        match template_name {
            "wtmplt_VersionItemData" => {
                let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for (row_number, item) in self.list_of_version.iter().enumerate() {
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

impl tmplt::HtmlTemplatingDataTrait for VersionItemData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("VersionItemData")
    }
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // log::debug!(&placeholder);
        match placeholder {
            "wt_crate_name" => self.crate_name.clone(),
            "wt_crate_version" => self.crate_version.clone(),
            "wt_crate_name_version" => join_crate_version(&self.crate_name, &self.crate_version),
            "wt_crate_published_by_url" => {
                crate::cln_methods_mod::cln_publisher_item_mod::published_by_url_shorten(self.published_by_url.as_deref().unwrap_or("")).to_string()
            }
            "wt_edit_or_new" => {
                if self.yanked {
                    "".to_string()
                } else if self.is_src_cached.unwrap_or(false) {
                    if self.my_review.is_some() {
                        "Edit".to_string()
                    } else {
                        "New".to_string()
                    }
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
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }

    /// boolean : is the next node rendered or not: "wb_" or "sb_"
    fn exists_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // log::debug!( &placeholder);
        match placeholder {
            "wb_has_review" => self.my_review.is_some(),
            _ => tmplt::utils::match_else_for_exists_next_node_or_attribute(&self.data_model_name(), placeholder),
        }
    }
}

pub fn routing_version_list(param2: &str) {
    let request_data = ReviewFilterData {
        crate_name: param2.to_string(),
        crate_version: None,
        old_crate_version: None,
    };
    srv_methods::srv_version_list(request_data);
}

#[named]
pub fn cln_version_list(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    *VERSION_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
    // the mutex is locked inside a scope. When this structure falls out of scope, the lock will be unlocked.
    let html_after_process = {
        let data = VERSION_LIST_DATA.lock().unwrap();
        tmplt::process_html(data.deref(), &html)
    };

    inject_into_html(&html_after_process);

    // on_click for every row of the list
    for (row_number, item) in VERSION_LIST_DATA.lock().unwrap().list_of_version.iter().enumerate() {
        // button New or Edit
        if item.my_review.is_some() {
            row_on_click!("button_review_edit_or_new", row_number, request_review_edit_or_new);
        } else if item.is_src_cached == Some(true) {
            row_on_click!("button_review_edit_or_new", row_number, request_review_edit_or_new);
        }
        // sub-menu
        if item.my_review.is_some() {
            row_on_click!("button_open_crev_dev", row_number, button_open_crev_dev_onclick);
            row_on_click!("button_open_crates_io", row_number, button_open_crates_io_onclick);
            row_on_click!("button_open_lib_rs", row_number, button_open_lib_rs_onclick);
            row_on_click!("button_open_source_code", row_number, button_open_source_code_onclick);
            row_on_click!("button_review_delete", row_number, modal_delete);
        }
    }
}

#[named]
fn request_review_edit_or_new(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &VERSION_LIST_DATA.lock().unwrap().list_of_version[row_number];
    let url = format!("index.html#edit_or_new/{}/{}", item.crate_name, item.crate_version);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_crev_dev_onclick(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &VERSION_LIST_DATA.lock().unwrap().list_of_version[row_number];
    let url = format!("https://web.crev.dev/rust-reviews/crate/{}/", item.crate_name);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_crates_io_onclick(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &VERSION_LIST_DATA.lock().unwrap().list_of_version[row_number];
    let url = format!("https://crates.io/crates/{}/{}", item.crate_name, item.crate_version);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_lib_rs_onclick(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &VERSION_LIST_DATA.lock().unwrap().list_of_version[row_number];
    let url = format!("https://lib.rs/crates/{}", item.crate_name);
    unwrap!(w::window().open_with_url(&url));
}

#[named]
fn button_open_source_code_onclick(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &VERSION_LIST_DATA.lock().unwrap().list_of_version[row_number];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_open_source_code(request_data);
}

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
    use crate::cln_methods_mod::cln_utils_mod::modal_close_on_click;
    on_click!("modal_close", modal_close_on_click);
    // I had to add modal_yes_delete(0), because row_on_click works that way.
    row_on_click!("modal_yes_delete", row_number, request_review_delete);
}

#[named]
fn request_review_delete(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    use crate::cln_methods_mod::cln_utils_mod::modal_close_on_click;
    modal_close_on_click("");

    // from list get crate name and version
    let item = &VERSION_LIST_DATA.lock().unwrap().list_of_version[row_number];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_delete(request_data);
}
