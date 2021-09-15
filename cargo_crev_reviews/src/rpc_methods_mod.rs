// rpc_methods_mod.rs

use crate::crev_mod::*;
use crate::response_post_mod::return_rpc_response;
use ::function_name::named;
use cargo_crev_reviews_common::*;
use std::str::FromStr;
use unwrap::unwrap;

// region: review

fn from_crev_to_item(p: &ProofCrevForReview) -> ReviewItemData {
    ReviewItemData {
        crate_name: p.package.name.clone(),
        crate_version: p.package.version.clone(),
        date: p.date.clone(),
        thoroughness: p.review.as_ref().unwrap().thoroughness.to_string(),
        understanding: p.review.as_ref().unwrap().understanding.to_string(),
        rating: rating_to_string(&(p.review.as_ref().unwrap().rating)),
        comment_md: p.comment.as_ref().unwrap_or(&"".to_string()).clone(),
    }
}

/// maybe add filter for one crate_name
#[named]
pub fn rpc_reviews_list(_request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let mut vec_review: Vec<ReviewItemData> = vec![];
    let vec_proof = unwrap!(crev_list_my_reviews(&None));
    // reverse, newest on top
    for p in vec_proof.iter().rev() {
        vec_review.push(from_crev_to_item(p));
    }
    let response_method = ResponseMethod::PageReviewList;
    let response_data = ReviewListData {
        filter: "".to_string(),
        list_of_review: vec_review,
    };
    let response_html = crate::files_mod::review_list_html();
    Ok(return_rpc_response(response_method, response_data, response_html))
}

#[named]
pub fn rpc_review_new(_request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let response_method = ResponseMethod::PageReviewNew;
    let response_html = crate::files_mod::review_new_html();
    let response_data = ReviewItemData {
        crate_name: "crate_name".to_string(),
        crate_version: "version".to_string(),
        date: "".to_string(),
        thoroughness: "none".to_string(),
        understanding: "none".to_string(),
        rating: "neutral".to_string(),
        comment_md: r#"Try to comment important aspects: 
malicious potential:99%
file-read/write: explanation
macro_rules: explanation
lines of code: 6 dependencies: 6 
NONE: unsafe, FFI, asm!, no_mangle, network-access, build.rs, proc_macro, process::command
owners (in crates.io) reputation: unknown / rust team / active in rust community
used in projects:  
alternative crates explored:
        "#
        .to_string(),
    };
    Ok(return_rpc_response(response_method, response_data, response_html))
}

#[named]
pub fn rpc_review_save(request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());

    let p: ReviewItemData = unwrap!(serde_json::from_value(request_data));

    match crev_save_review(
        &p.crate_name,
        &p.crate_version,
        crev_data::Level::from_str(&p.thoroughness)?,
        crev_data::Level::from_str(&p.understanding)?,
        rating_parse(&p.rating)?,
        &p.comment_md,
    ) {
        Err(err) => Ok(crate::response_post_mod::response_err_message(&err)),
        Ok(()) => request_review_list(),
    }
}

fn request_review_list() -> anyhow::Result<String> {
    let request_data = ReviewFilterData {
        crate_name: String::new(),
        crate_version: None,
        old_crate_version: None,
    };
    let request_data = unwrap!(serde_json::to_value(request_data));
    rpc_reviews_list(request_data)
}

#[named]
pub fn rpc_review_edit(request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));
    // find the item from the list
    let p = crev_edit_review(filter)?;
    let response_method = ResponseMethod::PageReviewEdit;
    let response_data = from_crev_to_item(&p);
    let response_html = crate::files_mod::review_edit_html();

    Ok(return_rpc_response(response_method, response_data, response_html))
}

#[named]
pub fn rpc_review_new_version(request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));
    // find the item from the list
    let p = crev_new_version(filter)?;
    let response_method = ResponseMethod::PageReviewEdit;
    let response_data = from_crev_to_item(&p);
    let response_html = crate::files_mod::review_edit_html();

    Ok(return_rpc_response(response_method, response_data, response_html))
}

#[named]
pub fn rpc_review_publish(_request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    match crev_publish() {
        Ok(ret_val) => Ok(crate::response_post_mod::response_modal_message(&ret_val)),
        Err(err) => Ok(crate::response_post_mod::response_err_message(&err)),
    }
}
// endregion: review
