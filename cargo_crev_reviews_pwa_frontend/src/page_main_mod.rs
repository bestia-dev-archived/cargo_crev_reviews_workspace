// page_main_mod.rs

// use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::on_click;
use crate::utils_mod as ut;
use crate::web_sys_mod as w;

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn page_main() {
    // fetch page_main.html and inject it
    let resp_body_text = w::fetch_response("pages/page_main.html").await;
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) =
        ut::get_delimited_text(&resp_body_text, 0, "<body>", "</body>").unwrap();
    w::set_inner_html("div_for_wasm_html_injecting", &html_fragment);

    // region: binding - read from config

    //w::set_text("div_input_unit", input_unit);

    // endregion: binding - read from config

    // region: event handlers
    // on_click!("div_1", div_cell_on_click);

    // endregion: event handlers
}
