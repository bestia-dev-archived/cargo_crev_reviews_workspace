// cln_methods_verify_mod.rs

use function_name::named;
use lazy_static::lazy_static;
use std::sync::Mutex;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::auto_generated_mod::common_structs_mod::*;
use crate::auto_generated_mod::srv_methods;

// use crate::on_click;
use crate::html_mod::*;
use crate::html_template_mod::*;
use crate::*;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref VERIFY_ITEM_DATA: Mutex<VerifyItemData> = Mutex::new(VerifyItemData::default());
    static ref VERIFY_LIST_DATA: Mutex<VerifyListData> = Mutex::new(VerifyListData::default());
}

impl HtmlServerTemplateRender for VerifyItemData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("VerifyItemData")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html(&self, html: &str) -> String {
        // find node <html >, jump over <!DOCTYPE html> because it is not microXml compatible
        // I will add <!DOCTYPE html> when the rendering ends, before returning the html.
        if let Some(pos_html) = html.find("<html") {
            let template_raw = &html[pos_html..];
            let html = self.render(template_raw, ServerOrClient::WebBrowserClient);
            return html;
        } else {
            html.to_string()
        }
    }
    /// boolean : is the next node rendered or not: "wb_" or "sb_"
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!( &placeholder);
        match placeholder {
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "wt_row_number" => format!("{}.", pos_cursor + 1),
            "wt_status" => self.status.clone(),
            "wt_my_review" => self.my_review.clone(),
            "wt_crate_name" => self.crate_name.clone(),
            "wt_crate_version" => self.crate_version.clone(),
            "wt_crate_name_version" => utils_mod::join_crate_version(&self.crate_name, &self.crate_version),
            "wt_published_by" => self.published_by.clone(),
            "wt_cargo_crev_reviews_version" => env!("CARGO_PKG_VERSION").to_string(),
            "wt_status_class" => format!("review_header0_cell left c_{}", &self.status),
            "wt_my_review_class" => {
                if vec!["strong", "positive", "neutral", "negative"].contains(&self.my_review.as_str()) {
                    format!("review_header0_cell left c_{}", &self.my_review)
                } else {
                    "review_header0_cell left".to_string()
                }
            }
            "wt_published_by_class" => format!("review_header0_cell left c_{}", &self.trusted_publisher),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src: "wu_" or "su"
    fn replace_with_url(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            _ => replace_with_url_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a vector of Nodes to replace the next Node: "wn_" or "sn"
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // dbg!(&placeholder);
        match placeholder {
            _ => replace_with_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// renders sub-template: "stmplt_" or "wtmplt_"
    fn render_sub_template(&self, template_name: &str, _sub_templates: &Vec<SubTemplate>, _prefixes: &PrefixForTemplateVariables) -> Vec<Node> {
        // dbg!(&placeholder);
        match template_name {
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}

impl HtmlServerTemplateRender for VerifyListData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("VerifyListData")
    }
    /// renders the complete html file. Not a sub-template/fragment.
    fn render_html(&self, html: &str) -> String {
        let html = self.render(html, ServerOrClient::WebBrowserClient);
        // return
        html
    }
    /// boolean : is the next node rendered or not: "wb_" or "sb_"
    fn retain_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!( &placeholder);
        match placeholder {
            _ => retain_next_node_or_attribute_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "wt_cargo_crev_reviews_version" => s!(env!("CARGO_PKG_VERSION")),
            "wt_project_dir" => s!(self.project_dir),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// exclusive url encoded for href and src: "wu_" or "su"
    fn replace_with_url(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            // the href for css is good for static data. For dynamic route it must be different.
            "su_css_route" => url_u!("/rust-reviews/css/rust-reviews.css"),
            "su_favicon_route" => url_u!("/rust-reviews/favicon.png"),
            "su_img_src_logo" => url_u!("/rust-reviews/images/Logo_02.png"),
            _ => replace_with_url_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// returns a vector of Nodes to replace the next Node: "wn_" or "sn"
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<Node> {
        // dbg!(&placeholder);
        match placeholder {
            _ => replace_with_nodes_match_else(&self.data_model_name(), placeholder),
        }
    }
    /// renders sub-template: "stmplt_" or "wtmplt_"
    fn render_sub_template(&self, template_name: &str, sub_templates: &Vec<SubTemplate>, prefixes: &PrefixForTemplateVariables) -> Vec<Node> {
        log::info!("{}", template_name);
        match template_name {
            "wtmplt_verify_item_data" => {
                let sub_template = unwrap!(sub_templates.iter().find(|&template| template.name == template_name));
                let mut nodes = vec![];
                for (row_number, verify_item) in self.list_of_verify.iter().enumerate() {
                    let vec_node = unwrap!(verify_item.render_template_raw_to_nodes(&sub_template.template, HtmlOrSvg::Html, "", row_number, prefixes));
                    nodes.extend_from_slice(&vec_node);
                }
                // return
                nodes
            }
            _ => render_sub_template_match_else(&self.data_model_name(), template_name),
        }
    }
}

#[named]
pub fn request_verify_list(_element_id: &str) {
    log::info!("{}", function_name!());
    let request_data = RpcEmptyData {};
    srv_methods::srv_verify_project(request_data);
}

#[named]
pub fn cln_verify_list(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    let html = extract_html(&srv_response);
    *VERIFY_LIST_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
    // modal dialog box with error, don't change the html and data
    let html_after_process = VERIFY_LIST_DATA.lock().unwrap().render_html(&html);

    inject_into_html(&html_after_process);
    navigation_on_click();

    // on_click for every row of the list
    /*   for (row_number, _item) in VERIFY_LIST_DATA.lock().unwrap().list_of_verify.iter().enumerate() {
        row_on_click!("crate_name_version", row_number, open_all_links);
    } */
}

#[named]
pub fn request_review_edit_or_new(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    // from list get crate name and version
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_number];
    let request_data = ReviewFilterData {
        crate_name: item.crate_name.clone(),
        crate_version: Some(item.crate_version.clone()),
        old_crate_version: None,
    };
    srv_methods::srv_review_edit_or_new(request_data);
}

#[named]
fn open_all_links(_element_id: &str, row_number: usize) {
    log::info!("{}", function_name!());
    let item = &VERIFY_LIST_DATA.lock().unwrap().list_of_verify[row_number];
    /*
       let url = format!("https://web.crev.dev/rust-reviews/crate/{}/", item.crate_name);
       unwrap!(w::window().open_with_url(&url));

       let url = format!("https://lib.rs/crates/{}", item.crate_name);
       unwrap!(w::window().open_with_url(&url));

       let url = format!("https://crates.io/crates/{}/{}", item.crate_name, item.crate_version);
       unwrap!(w::window().open_with_url(&url));

       let request_data = ReviewFilterData {
           crate_name: item.crate_name.clone(),
           crate_version: Some(item.crate_version.clone()),
           old_crate_version: None,
       };
       srv_methods::srv_review_open_source_code(request_data);
    */
    // list versions for this crate
    let url = format!(
        "http://{}:{}/{}/index.html#version_list/{}",
        SERVER_HOST.as_str(),
        SERVER_PORT.as_str(),
        SERVER_FIRST_SUBDIRECTORY.as_str(),
        item.crate_name,
    );
    unwrap!(w::window().open_with_url(&url));
    /*
    // edit_or_new in a new tab
    let url = format!(
        "http://{}:{}/{}/index.html#edit_or_new/{}/{}",
        SERVER_HOST.as_str(),
        SERVER_PORT.as_str(),
        SERVER_FIRST_SUBDIRECTORY.as_str(),
        item.crate_name,
        item.crate_version,
    );
    unwrap!(w::window().open_with_url(&url));
     */
}
