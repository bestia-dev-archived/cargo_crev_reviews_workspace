// pages_mod.rs

use cargo_crev_reviews_common::*;

use reader_for_microxml::ReaderForMicroXml;
use reader_for_microxml::Token;
use std::str::FromStr;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::page_review_mod;
use crate::web_sys_mod as w;

pub trait PageProcessor {
    // region: mandatory functions to implement
    fn replace_next_text_node(&self, name: &str, next_text_node_replace: &mut Option<String>);
    fn replace_next_attribute(&self, name: &str, value: &str, next_attribute_replace: &mut Option<(String, String)>);
    fn match_wt(&self, wt_name: &str) -> String;
    fn exist_next_attribute(&self, name: &str, _value: &str, next_attribute_exist: &mut Option<bool>);
    // endregion: mandatory functions to implement

    // region: trait functions
    /// With reader_for_microXml parse the xml.
    /// If found the magic word `wt_` then run some code and push the result instead of the next element or attribute
    /// If normal element or attribute push it to the new String builder
    /// It is just strings so, that should be super fast.
    fn process_html_with_item(&self, html_fragment: &str, row_num: Option<usize>) -> String {
        let reader_iterator = ReaderForMicroXml::new(html_fragment);
        let mut html_after_process = String::with_capacity(html_fragment.len());
        let mut next_attribute_replace: Option<(String, String)> = None;
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
                                        self.replace_next_attribute(name, value, &mut next_attribute_replace);
                                        String::new()
                                    } else if name.starts_with("data-wb_") {
                                        // returns nothing, it writes into `next_attribute_exist`
                                        self.exist_next_attribute(name, value, &mut next_attribute_exist);
                                        String::new()
                                    } else {
                                        // non processed attribute,
                                        // but `id` is special, it could have row_num in parenthesis like id="item(0)"
                                        if name == "id" && row_num.is_some() {
                                            format!(r#"{}="{}({})" "#, name, value, row_num.unwrap())
                                        } else {
                                            format!(r#"{}="{}" "#, name, value)
                                        }
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
                        if txt.starts_with("wt_") {
                            self.replace_next_text_node(txt, &mut next_text_node_replace);
                        } else {
                            let html = format!(r#"<!--{}-->"#, txt);
                            html_after_process.push_str(&html);
                        }

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

    // endregion: trait functions

    /// process repetitive segments with a list of data
    fn process_html_with_list(html_old: &str, data_as_list: &ReviewListData) -> String {
        let html_after_process_1 = Self::delete_wd(html_old);
        let html_new = Self::process_repetitive_html(html_after_process_1, data_as_list);
        // return
        html_new
    }

    /// in the html there can be segments that are only to help for graphical design
    /// this segments must be deleted before rendering the html
    /// it is marked with: <!--wd_delete start--> <!--wd_delete end-->
    fn delete_wd(html_old: &str) -> String {
        let mut html_new = String::with_capacity(html_old.len());
        let mut cursor = 0;
        let mut old_end = 0;
        while let Some(range_to_remove) =
            cargo_crev_reviews_common::find_range_including_delimiters(&html_old, &mut cursor, "<!--wd_delete start-->", "<!--wd_delete end-->")
        {
            html_new.push_str(&html_old[old_end..range_to_remove.start.clone()]);
            old_end = range_to_remove.end.clone();
        }
        html_new.push_str(&html_old[old_end..]);
        html_new
    }

    /// inside the html there can be `wr_ web-browser repetitive segments`
    /// marked with <!--wr_review start--><!--wr_review end-->
    fn process_repetitive_html(html_old: String, data_as_list: &ReviewListData) -> String {
        let mut html_new = String::with_capacity(html_old.len());
        let mut cursor = 0;
        let mut old_end = 0;
        while let Some(range_including_delimiters) =
            cargo_crev_reviews_common::find_range_including_delimiters(&html_old, &mut cursor, "<!--wr_review start-->", "<!--wr_review end-->")
        {
            let template_with_delimiters = &html_old[range_including_delimiters.clone()];
            // add part before delimiter
            html_new.push_str(&html_old[old_end..range_including_delimiters.start.clone()]);
            old_end = range_including_delimiters.end.clone();

            // exclude delimiters
            if let Some(range_excluding_delimiters) =
                cargo_crev_reviews_common::find_range_between_delimiters(&template_with_delimiters, &mut 0, "<!--wr_review start-->", "<!--wr_review end-->")
            {
                let html_repetitive_template = &template_with_delimiters[range_excluding_delimiters];
                // process template and push as many &str is needed
                for (row_num, data) in data_as_list.list_of_review.iter().enumerate() {
                    let list_item_html = data.process_html_with_item(html_repetitive_template, Some(row_num));
                    html_new.push_str(&list_item_html);
                }
            }
        }
        html_new.push_str(&html_old[old_end..]);
        html_new
    }
}

pub fn post_request_await_run_response_method<T>(request_method: RequestMethod, request_data: T)
where
    T: serde::Serialize,
{
    let request_method: &'static str = request_method.into();
    let data = unwrap!(serde_json::to_value(request_data));
    let rpc = cargo_crev_reviews_common::RpcRequest {
        request_method: request_method.to_string(),
        request_data: data,
    };
    let json_string = unwrap!(serde_json::to_string(&rpc));
    let rpc_request = JsValue::from_str(&json_string);

    spawn_local(async move {
        let rpc_request = Some(&rpc_request);
        let resp_body_text = w::fetch_post_response("submit", rpc_request).await;
        let rpc_response: cargo_crev_reviews_common::RpcResponse = unwrap!(serde_json::from_str(&resp_body_text));
        match_response_method_and_call_function(rpc_response).await;
    });
}

pub async fn match_response_method_and_call_function(response: RpcResponse) {
    let response_enum = ResponseMethod::from_str(response.response_method.as_str());
    match response_enum {
        Ok(response_enum) => match response_enum {
            ResponseMethod::PageReviewList => page_review_mod::page_review_list(response),
            ResponseMethod::PageReviewNew => page_review_mod::page_review_new(response),
            ResponseMethod::PageReviewEdit => page_review_mod::page_review_edit(response),
            ResponseMethod::PageReviewError => page_review_mod::page_review_error(response),
        },
        Err(_err) => w::debug_write(&format!("Error: Unrecognized response_method {}", response.response_method)),
    }
}

pub fn page_html(response: &RpcResponse) -> String {
    let response_html = &response.response_html;
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = get_delimited_text(response_html, 0, "<body>", "</body>").unwrap();
    html_fragment
}

pub fn inject_into_html(html_after_process: &str) {
    w::set_inner_html("div_for_wasm_html_injecting", html_after_process);
}
