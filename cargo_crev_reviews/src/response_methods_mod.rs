// server_methods_mod

use crate::crev_mod::*;
use crate::response_post_method_mod::return_rpc_response;
use cargo_crev_reviews_common::*;
use std::str::FromStr;
use unwrap::unwrap;

// region: review

/// maybe add filter for one crate_name
pub fn reviews_list_rpc(_params: serde_json::Value) -> anyhow::Result<String> {
    println!("list_my_reviews_json()");
    let mut vec_review: Vec<ReviewItemParams> = vec![];
    let vec_proof = unwrap!(crev_list_my_reviews());
    for p in vec_proof.iter() {
        vec_review.push(ReviewItemParams {
            crate_name: p.package.name.clone(),
            crate_version: p.package.version.clone(),
            date: p.date.clone(),
            thoroughness: p.review.as_ref().unwrap().thoroughness.to_string(),
            understanding: p.review.as_ref().unwrap().understanding.to_string(),
            rating: rating_to_string(&(p.review.as_ref().unwrap().rating)),
            comment_md: p.comment.as_ref().unwrap_or(&"".to_string()).clone(),
        })
    }
    let client_method = "page_review_list";
    let client_params = ReviewListParams { list_of_review: vec_review };
    let page_html = crate::files_mod::review_list_html();
    Ok(return_rpc_response(client_method, client_params, page_html))
}

pub fn review_new_rpc(_params: serde_json::Value) -> anyhow::Result<String> {
    let client_method = "page_review_new";
    let page_html = crate::files_mod::review_new_html();
    let client_params = RpcEmptyParams {};
    Ok(return_rpc_response(client_method, client_params, page_html))
}

pub fn review_save_rpc(params: serde_json::Value) -> anyhow::Result<String> {
    println!("review_save_json()");

    let p: ReviewItemParams = unwrap!(serde_json::from_value(params));

    match crev_new_review(
        &p.crate_name,
        &p.crate_version,
        crev_data::Level::from_str(&p.thoroughness)?,
        crev_data::Level::from_str(&p.understanding)?,
        rating_parse(&p.rating)?,
        &p.comment_md,
    ) {
        Err(err) => {
            let client_method = "page_review_error";
            let client_params = RpcMessageParams { message: err.to_string() };
            let page_html = "";
            Ok(return_rpc_response(client_method, client_params, page_html))
        }
        Ok(()) => {
            let client_method = "page_review_show";
            let client_params = ReviewItemParams {
                crate_name: p.crate_name,
                crate_version: p.crate_version,
                date: p.date,
                thoroughness: p.thoroughness,
                understanding: p.understanding,
                rating: p.rating,
                comment_md: p.comment_md,
            };
            let page_html = crate::files_mod::review_show_html();
            Ok(return_rpc_response(client_method, client_params, page_html))
        }
    }
}

pub fn review_edit_rpc(params: serde_json::Value) -> anyhow::Result<String> {
    println!("review_edit_json()");
    let p: ReviewItemParams = unwrap!(serde_json::from_value(params));
    let client_method = "page_review_edit";
    let client_params = ReviewItemParams {
        crate_name: p.crate_name,
        crate_version: p.crate_version,
        date: p.date,
        thoroughness: p.thoroughness,
        understanding: p.understanding,
        rating: p.rating,
        comment_md: p.comment_md,
    };
    let page_html = crate::files_mod::review_edit_html();

    Ok(return_rpc_response(client_method, client_params, page_html))
}

// endregion: review
