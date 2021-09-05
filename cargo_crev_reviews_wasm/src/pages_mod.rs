// pages_mod.rs

use cargo_crev_reviews_common::*;
use reader_for_microxml::ReaderForMicroXml;
use reader_for_microxml::Token;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;

use crate::web_sys_mod as w;

pub async fn post_request(json_request: JsValue) -> cargo_crev_reviews_common::RpcResponse {
    let json = Some(&json_request);
    let resp_body_text = w::fetch_post_response("submit", json).await;
    let rpc_response: cargo_crev_reviews_common::RpcResponse = unwrap!(serde_json::from_str(&resp_body_text));
    rpc_response
}

/// inside the html_fragment there can be repetitive segments
/// in the fragment they are marked with <!--wtemplate_review start--><!--wtemplate_review end-->
/// sometimes we need some html in the fragment, that will not be processed and must be removed.
/// it is marked with: <!--wtemplate_not_for_render start--> <!--wtemplate_not_for_render end-->
pub fn process_list_html_templates(
    html_fragment: &str,
    params_as_list: &ReviewListParams,
    replace_next_attribute: &dyn Fn(&str, &str, &mut Option<(&str, String)>, &ReviewItemParams) -> String,
    replace_next_text_node: &dyn Fn(&str, &mut Option<String>, &ReviewItemParams) -> String,
    exist_next_attribute: &dyn Fn(&str, &str, &mut Option<bool>, &ReviewItemParams) -> String,
) -> String {
    // region: first remove the marked segments
    let mut html_after_remove_segments = String::with_capacity(html_fragment.len());
    let mut cursor = 0;
    let mut old_end = 0;
    while let Some(range_to_remove) = cargo_crev_reviews_common::find_range_including_delimiters(
        &html_fragment,
        &mut cursor,
        "<!--wtemplate_not_for_render start-->",
        "<!--wtemplate_not_for_render end-->",
    ) {
        w::debug_write(&format!("range_to_remove: {:?}", &range_to_remove));
        html_after_remove_segments.push_str(&html_fragment[old_end..range_to_remove.start.clone()]);
        old_end = range_to_remove.end.clone();
    }
    html_after_remove_segments.push_str(&html_fragment[old_end..]);
    // endregion: first remove the marked segments
    w::debug_write(&format!("html_after_remove_segments: {}", &html_after_remove_segments));

    // region: find templates, process html and push str
    let mut html = String::with_capacity(html_after_remove_segments.len());
    let mut cursor = 0;
    let mut old_end = 0;
    while let Some(range_including_delimiters) = cargo_crev_reviews_common::find_range_including_delimiters(
        &html_after_remove_segments,
        &mut cursor,
        "<!--wtemplate_review start-->",
        "<!--wtemplate_review end-->",
    ) {
        w::debug_write(&format!("range_including_delimiters: {:?}", &range_including_delimiters));
        let template_with_delimiters = &html_after_remove_segments[range_including_delimiters.clone()];
        w::debug_write(&format!("template_with_delimiters: {}", template_with_delimiters));
        // add part before delimiter
        html.push_str(&html_after_remove_segments[old_end..range_including_delimiters.start.clone()]);
        old_end = range_including_delimiters.end.clone();

        // exclude delimiters
        if let Some(range_excluding_delimiters) = cargo_crev_reviews_common::find_range_between_delimiters(
            &template_with_delimiters,
            &mut 0,
            "<!--wtemplate_review start-->",
            "<!--wtemplate_review end-->",
        ) {
            w::debug_write(&format!("range_excluding_delimiters: {:?}", &range_excluding_delimiters));
            let html_repetitive_template = &template_with_delimiters[range_excluding_delimiters];
            w::debug_write(&format!("html_repetitive_template: {}", html_repetitive_template));
            // process template and push as many &str is needed
            for params in params_as_list.list_of_review.iter() {
                let list_item_html = crate::pages_mod::process_html(
                    html_repetitive_template,
                    params,
                    &replace_next_attribute,
                    &replace_next_text_node,
                    &exist_next_attribute,
                );
                html.push_str(&list_item_html);
            }
        }
    }
    // finally push the rest of html_after_remove_segments
    html.push_str(&html_after_remove_segments[old_end..]);
    // endregion: find templates, process html and push str
    // return
    html
}

/// With reader_for_microXml parse the xml.
/// If found the magic word `wt_` then run some code and push the result instead of the next element or attribute
/// If normal element or attribute push it to the new String builder
/// It is just strings so, that should be super fast.
pub fn process_html(
    html_fragment: &str,
    params: &ReviewItemParams,
    replace_next_attribute: &dyn Fn(&str, &str, &mut Option<(&str, String)>, &ReviewItemParams) -> String,
    replace_next_text_node: &dyn Fn(&str, &mut Option<String>, &ReviewItemParams) -> String,
    exist_next_attribute: &dyn Fn(&str, &str, &mut Option<bool>, &ReviewItemParams) -> String,
) -> String {
    let reader_iterator = ReaderForMicroXml::new(html_fragment);
    let mut html_after_process = String::with_capacity(html_fragment.len());
    let mut next_attribute_replace: Option<(&str, String)> = None;
    let mut next_attribute_exist: Option<bool> = None;
    let mut next_text_node_replace: Option<String> = None;
    let mut last_token = "";
    let mut breadcrumb: Vec<&str> = vec![];
    for result_token in reader_iterator {
        match result_token {
            Ok(token) => match token {
                Token::StartElement(name) => {
                    if last_token == "element" || last_token == "attribute" {
                        html_after_process.push_str(">");
                    }
                    if last_token != "text"
                        && (name == "div" || name == "textarea" || name == "h1" || name == "h2" || name == "h3" || name == "h4" || name == "pre")
                    {
                        html_after_process.push_str("\n");
                        html_after_process.push_str(&"    ".repeat(breadcrumb.len() + 1));
                    }
                    html_after_process.push_str(&format!("<{} ", name));
                    breadcrumb.push(name.clone());
                    last_token = "element";
                }
                Token::Attribute(name, value) => {
                    let exist_attribute = match next_attribute_exist {
                        None => true,
                        Some(is_exist) => {
                            next_attribute_exist = None;
                            is_exist
                        }
                    };
                    if exist_attribute {
                        let html = match next_attribute_replace {
                            Some(tuple_next_attribute_replace) => {
                                let return_value = format!(r#"{}="{}" "#, tuple_next_attribute_replace.0, tuple_next_attribute_replace.1,);
                                next_attribute_replace = None;
                                return_value
                            }
                            None => {
                                if name.starts_with("data-wt_") {
                                    // returns nothing, it writes into `next_attribute_replace`
                                    replace_next_attribute(name, value, &mut next_attribute_replace, &params);
                                    String::new()
                                } else if name.starts_with("data-wb_") {
                                    // returns nothing, it writes into `next_attribute_exist`
                                    exist_next_attribute(name, value, &mut next_attribute_exist, &params);
                                    String::new()
                                } else {
                                    format!(r#"{}="{}" "#, name, value,)
                                }
                            }
                        };
                        html_after_process.push_str(&html);
                        last_token = "attribute";
                    }
                }
                // the txt is still encoded for xml text node
                Token::TextNode(txt) => {
                    if last_token == "element" || last_token == "attribute" {
                        html_after_process.push_str(">");
                    }
                    match next_text_node_replace {
                        None => html_after_process.push_str(&format!("{}", txt)),
                        Some(ref new_txt) => {
                            html_after_process.push_str(&format!("{}", new_txt));
                            next_text_node_replace = None;
                        }
                    }
                    last_token = "text";
                }
                Token::Comment(txt) => {
                    if last_token == "element" || last_token == "attribute" {
                        html_after_process.push_str(">");
                    }
                    let html = if txt.starts_with("wt_") {
                        replace_next_text_node(txt, &mut next_text_node_replace, &params)
                    } else {
                        format!(r#"<!--{}-->"#, txt)
                    };

                    html_after_process.push_str(&html);
                    last_token = "comment"
                }
                Token::EndElement(end_element_name) => {
                    breadcrumb.pop();
                    if end_element_name.is_empty() {
                        html_after_process.push_str("/>");
                    } else {
                        if last_token == "element" || last_token == "attribute" {
                            html_after_process.push_str(">");
                        }
                        html_after_process.push_str(&format!("</{}>", end_element_name));
                    }
                    last_token = "end";
                }
            },
            Err(err_msg) => {
                w::debug_write(&format!("MicroXml incorrect : {}", err_msg));
            }
        }
    }
    html_after_process
}
