// html_mod.rs

use dev_bestia_string_utils::*;

// extract only the html inside the <body> </body>
pub fn extract_body_inner(html: &str) -> String {
    let (html_fragment, _new_pos_cursor) = get_delimited_text(html, 0, "<body>", "</body>").unwrap();
    html_fragment
}

pub fn process_include(response_html: &str) -> String {
    let footer_html = crate::html_mod::extract_body_inner(crate::files_mod::footer_html());
    // brute force replace
    let response_html = response_html.replace("<!--s_include footer.html-->", &footer_html);
    // return
    response_html
}
