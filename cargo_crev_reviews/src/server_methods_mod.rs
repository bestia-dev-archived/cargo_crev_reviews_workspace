// server_methods_mod

use crate::*;
use cargo_crev_reviews_common::*;
use unwrap::unwrap;

// region: boilerplate to convert json to call methods

pub fn list_my_reviews_json() -> anyhow::Result<String> {
    println!("list_my_reviews_json()");

    let vec_proof = unwrap!(crev_mod::list_my_reviews());
    let client_method = "client_review_list";
    let client_params = RpcMessageParams { message: "".to_string() };
    let page_html = "";
    Ok(return_rpc_response(client_method, client_params, page_html))
}

pub fn review_save_json(params: serde_json::Value) -> anyhow::Result<String> {
    println!("review_save_json()");

    let p: ReviewItemParams = unwrap!(serde_json::from_value(params));

    match crev_mod::create_new_review_proof(
        &p.crate_name,
        &p.crate_version,
        crev_mod::thoroughness_parse(&p.thoroughness)?,
        crev_mod::understanding_parse(&p.understanding)?,
        crev_mod::rating_parse(&p.rating)?,
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
                thoroughness: p.thoroughness,
                understanding: p.understanding,
                rating: p.rating,
                comment_md: p.comment_md,
            };
            let page_html = crate::files_mod::pages_review_show_html();
            Ok(return_rpc_response(client_method, client_params, page_html))
        }
    }
}

pub fn review_edit_json(params: serde_json::Value) -> String {
    println!("review_edit_json()");
    let p: ReviewItemParams = unwrap!(serde_json::from_value(params));
    let client_method = "page_review_edit";
    let client_params = ReviewItemParams {
        crate_name: p.crate_name,
        crate_version: p.crate_version,
        thoroughness: p.thoroughness,
        understanding: p.understanding,
        rating: p.rating,
        comment_md: p.comment_md,
    };
    let page_html = crate::files_mod::pages_review_edit_html();

    return_rpc_response(client_method, client_params, page_html)
}

// endregion: boilerplate to convert json to call methods
