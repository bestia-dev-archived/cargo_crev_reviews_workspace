// cln_methods_version_mod.rs

use function_name::named;
use lazy_static::{__Deref, lazy_static};
use std::sync::Mutex;
use unwrap::unwrap;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;

use dev_bestia_html_templating as tmplt;
use tmplt::s;

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
        // dbg!(&placeholder);
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
        // dbg!(&placeholder);
        match placeholder {
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
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }

    /// boolean : is the next node rendered or not: "wb_" or "sb_"
    fn exists_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!( &placeholder);
        match placeholder {
            "wb_has_review" => self.my_review.is_some(),
            _ => tmplt::utils::match_else_for_exists_next_node_or_attribute(&self.data_model_name(), placeholder),
        }
    }
}

#[named]
pub fn cln_version_list(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    *VERSION_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
    // the mutex is locked inside a scope. When this structure falls out of scope, the lock will be unlocked.
    let html_after_process = {
        let ver = VERSION_LIST_DATA.lock().unwrap();
        tmplt::process_html(ver.deref(), &html)
    };

    inject_into_html(&html_after_process);
    navigation_on_click();
}
