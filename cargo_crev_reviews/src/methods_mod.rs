// methods_mod

use crate::*;
use cargo_crev_reviews_common::*;
use unwrap::unwrap;

// region: boilerplate to convert json to call methods

pub fn review_save_json(params: serde_json::Value, id: u32) -> String {
    println!("review_save_json()");
    let p: ReviewSaveParams = unwrap!(serde_json::from_value(params));

    let result = ReviewShowParams {
        page_html: crate::files_mod::pages_review_show_html().to_string(),
        crate_name: p.crate_name,
        crate_version: p.crate_version,
        thoroughness: p.thoroughness,
        understanding: p.understanding,
        rating: p.rating,
        comment_md: p.comment_md,
    };
    let page_method = "page_review_show";

    return_json_rpc_result(result, page_method, id)
}

pub fn review_edit_json(params: serde_json::Value, id: u32) -> String {
    println!("review_edit_json()");
    let p: ReviewSaveParams = unwrap!(serde_json::from_value(params));

    let result = ReviewShowParams {
        page_html: crate::files_mod::pages_review_edit_html().to_string(),
        crate_name: p.crate_name,
        crate_version: p.crate_version,
        thoroughness: p.thoroughness,
        understanding: p.understanding,
        rating: p.rating,
        comment_md: p.comment_md,
    };
    let page_method = "page_review_edit";

    return_json_rpc_result(result, page_method, id)
}

// endregion: boilerplate to convert json to call methods
