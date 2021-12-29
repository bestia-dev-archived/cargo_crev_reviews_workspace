// cln_methods_cargo_tree_mod.rs

use function_name::named;
use lazy_static::__Deref;
use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::auto_generated_mod::common_structs_mod::*;
use crate::auto_generated_mod::srv_methods;

use dev_bestia_html_templating as tmplt;
use dev_bestia_string_utils::*;

use crate::html_mod::*;
use crate::row_on_click;
use crate::*;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref CARGO_TREE_ITEM_DATA: Mutex<CargoTreeItemData> = Mutex::new(CargoTreeItemData::default());
    static ref CARGO_TREE_LIST_DATA: Mutex<CargoTreeListData> = Mutex::new(CargoTreeListData::default());
}

impl tmplt::HtmlTemplatingDataTrait for CargoTreeItemData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("CargoTreeItemData")
    }

    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // log::debug!(&placeholder);
        match placeholder {
            "wt_tree_line" => self.cargo_tree_line.replace("──", "─"),
            "wt_tree_line_class" => format!("review_header0_cell left codetree pointer c_{}", self.my_rating.as_deref().unwrap_or("")),
            "wt_my_rating" => self.my_rating.as_deref().unwrap_or("").to_string(),
            "wt_crate_description" => self.crate_description.as_deref().unwrap_or("").to_string(),
            "wt_published_by_url" => {
                cln_methods_mod::cln_methods_publisher_mod::published_by_url_shorten(self.published_by_url.as_deref().unwrap_or("")).to_string()
            }
            "wt_published_by_class" => format!(
                "review_header0_cell left codetree pointer c_{}",
                self.trusted_publisher.as_deref().unwrap_or("")
            ),
            "wt_status" => self.status.as_deref().unwrap_or("").to_string(),
            "wt_status_class" => format!("review_header0_cell left codetree c_{}", self.status.as_deref().unwrap_or("")),
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }
}

impl tmplt::HtmlTemplatingDataTrait for CargoTreeListData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("CargoTreeListData")
    }
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // log::debug!(&placeholder);
        match placeholder {
            // for footer
            "wt_cargo_crev_reviews_version" => s!(env!("CARGO_PKG_VERSION")),
            "wt_project_dir" => s!(self.project_dir),
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }

    /// renders sub-template: "stmplt_" or "wtmplt_"
    fn process_sub_template(&self, template_name: &str, sub_templates: &Vec<tmplt::utils::SubTemplate>) -> Vec<tmplt::utils::Node> {
        log::info!("{}", template_name);
        match template_name {
            "wtmplt_TreeData" => {
                let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for (row_number, cargo_tree_item) in self.list_of_cargo_tree.iter().enumerate() {
                    let vec_node = unwrap!(tmplt::utils::process_template_raw_to_nodes(
                        cargo_tree_item,
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
}

#[named]
pub fn request_cargo_tree_list(_element_id: &str) {
    log::info!("{}", function_name!());
    let request_data = RpcEmptyData {};
    srv_methods::srv_cargo_tree_project(request_data);
}

#[named]
pub fn cln_cargo_tree_list(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    dbg!(&html);
    *CARGO_TREE_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
    // the mutex is locked inside a scope. When this structure falls out of scope, the lock will be unlocked.
    let html_after_process = {
        let data = CARGO_TREE_LIST_DATA.lock().unwrap();
        tmplt::process_html(data.deref(), &html)
    };

    inject_into_html(&html_after_process);
    // navigation menu bar
    use cln_methods_mod::cln_methods_publisher_mod::*;
    use cln_methods_mod::cln_methods_review_item_mod::*;
    use cln_methods_mod::cln_methods_review_list_mod::*;
    on_click!("button_open_publisher_list", open_publisher_list);
    on_click!("button_update_registry_index", request_update_registry_index);
    on_click!("button_review_publish", request_review_publish);

    // on_click for every row of the list
    for (row_number, _item) in CARGO_TREE_LIST_DATA.lock().unwrap().list_of_cargo_tree.iter().enumerate() {
        row_on_click!("crate_name_version", row_number, open_all_links);
        row_on_click!("published_by_url", row_number, open_published_by_url);
    }
}

#[named]
pub fn request_review_edit_or_new(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &CARGO_TREE_LIST_DATA.lock().unwrap().list_of_cargo_tree[row_number];
    match &item.crate_name_version {
        None => {}
        Some(crate_name_version) => {
            let mut spl = crate_name_version.split_whitespace();
            let crate_name = spl.next().unwrap();
            let crate_version = spl.next().unwrap();

            let request_data = ReviewFilterData {
                crate_name: crate_name.to_string(),
                crate_version: Some(crate_version.to_string()),
                old_crate_version: None,
            };
            srv_methods::srv_review_edit_or_new(request_data);
        }
    }
}

#[named]
fn open_all_links(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    let item = &CARGO_TREE_LIST_DATA.lock().unwrap().list_of_cargo_tree[row_number];

    match &item.crate_name_version {
        None => {}
        Some(crate_name_version) => {
            let mut spl = crate_name_version.split_whitespace();
            let crate_name = spl.next().unwrap();
            let crate_version = spl.next().unwrap();

            let url = format!("https://web.crev.dev/rust-reviews/crate/{}/", crate_name);
            unwrap!(w::window().open_with_url(&url));

            let url = format!("https://lib.rs/crates/{}", crate_name);
            unwrap!(w::window().open_with_url(&url));

            let url = format!("https://crates.io/crates/{}/{}", crate_name, crate_version);
            unwrap!(w::window().open_with_url(&url));

            let request_data = ReviewFilterData {
                crate_name: crate_name.to_string(),
                crate_version: Some(crate_version.to_string()),
                old_crate_version: None,
            };
            srv_methods::srv_review_open_source_code(request_data);

            // list versions for this crate
            let url = format!("index.html#version_list/{}", crate_name);
            unwrap!(w::window().open_with_url(&url));
        }
    }
}

#[named]
fn open_published_by_url(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    let item = &CARGO_TREE_LIST_DATA.lock().unwrap().list_of_cargo_tree[row_number];

    match &item.published_by_url {
        None => {}
        Some(url) => {
            unwrap!(unwrap!(w::window().open_with_url(&url)));
        }
    }
}
