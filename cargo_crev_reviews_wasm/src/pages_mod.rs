// pages_mod.rs

use cargo_crev_reviews_common::ReviewItemParams;
use reader_for_microxml::ReaderForMicroXml;
use reader_for_microxml::Token;
use std::sync::MutexGuard;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;

use crate::web_sys_mod as w;

pub async fn post_request(json_request: JsValue) -> cargo_crev_reviews_common::RpcResponse {
    let json = Some(&json_request);
    let resp_body_text = w::fetch_post_response("submit", json).await;
    let rpc_response: cargo_crev_reviews_common::RpcResponse = unwrap!(serde_json::from_str(&resp_body_text));
    rpc_response
}

/// With reader_for_microXml parse the xml.
/// If found the magic word `wt_` then run some code and push the result instead of the next element or attribute
/// If normal element or attribute push it to the new String builder
/// It is just strings so, that should be super fast.
pub fn process_html(
    html_fragment: String,
    params: MutexGuard<ReviewItemParams>,
    replace_next_attribute: &dyn Fn(&str, &str, &mut Option<(&str, String)>, &MutexGuard<ReviewItemParams>) -> String,
    replace_next_text_node: &dyn Fn(&str, &mut Option<String>, &MutexGuard<ReviewItemParams>) -> String,
    exist_next_attribute: &dyn Fn(&str, &str, &mut Option<bool>, &MutexGuard<ReviewItemParams>) -> String,
) -> String {
    let reader_iterator = ReaderForMicroXml::new(&html_fragment);
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
