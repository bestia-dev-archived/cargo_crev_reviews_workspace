// cln_config_mod.rs

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
use crate::{html_mod::*, on_click};

lazy_static! {
    /// mutable static, because it is hard to pass variables around with on_click events
    static ref CONFIG_DATA: Mutex<ConfigData> = Mutex::new(ConfigData::default());
}

/// store data in static Mutex because of events like on_click
fn store_to_config_data(srv_response: RpcResponse) {
    *CONFIG_DATA.lock().unwrap() = unwrap!(serde_json::from_value(srv_response.response_data));
}

impl tmplt::HtmlTemplatingDataTrait for ConfigData {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("ConfigData")
    }
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // log::debug!(&placeholder);
        match placeholder {
            "wt_code_editor_path" => self.code_editor_path.clone(),
            "wt_browser_path" => self.browser_path.clone(),
            _ => tmplt::utils::match_else_for_replace_with_string(&self.data_model_name(), placeholder),
        }
    }
}

#[named]
pub fn button_open_config_edit_on_click(_element_id: &str) {
    log::info!("{}", function_name!());
    let url = format!("index.html#config_edit");
    unwrap!(w::window().open_with_url(&url));
}

pub fn routing_config_edit() {
    let request_data = RpcEmptyData {};
    srv_methods::srv_config_edit(request_data);
}

#[named]
pub fn cln_config_edit(srv_response: RpcResponse) {
    log::info!("{}", function_name!());
    // button_config_edit_on_click > srv_config_edit > cln_config_edit
    let html = extract_html(&srv_response);
    store_to_config_data(srv_response);
    // process with ConfigData
    let html_after_process = {
        let data = CONFIG_DATA.lock().unwrap();
        tmplt::process_html(data.deref(), &html)
    };
    w::set_inner_html("div_for_wasm_html_injecting", &html_after_process);

    on_click!("button_config_save", request_config_save);
    on_click!("button_close", close_on_click);
}

/// send rpc requests
#[named]
fn request_config_save(_element_id: &str) {
    log::info!("{}", function_name!());
    // values from form
    let request_data = ConfigData {
        code_editor_path: w::get_input_element_value_string_by_id("code_editor_path"),
        browser_path: w::get_input_element_value_string_by_id("browser_path"),
    };
    srv_methods::srv_config_save(request_data);
}

fn close_on_click(_element_id: &str) {
    w::close_tab();
}
