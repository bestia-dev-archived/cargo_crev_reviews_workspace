// srv_methods_mod.rs

//! rpc methods prepare the data to respond the POST rpc requests

use crate::auto_generated_mod::cln_methods;
use crate::common_structs_mod::*;
use crate::crev_mod::*;

use anyhow::Context;
use function_name::named;
use std::str::FromStr;
use std::time::Duration;
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
pub fn srv_reviews_list(_request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let mut vec_review: Vec<ReviewItemData> = vec![];
    let vec_proof = unwrap!(crev_list_my_reviews(&None));
    // reverse, newest on top
    for p in vec_proof.iter().rev() {
        vec_review.push(from_crev_to_item(p));
    }

    let response_data = ReviewListData {
        filter: "".to_string(),
        list_of_review: vec_review,
    };
    let response_html = crate::files_mod::review_list_html();

    cln_methods::cln_review_list(response_data, response_html)
}

#[named]
pub fn srv_review_new(request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));

    let response_html = crate::files_mod::review_new_html();
    let response_data = ReviewItemData {
        crate_name: filter.crate_name.to_string(),
        crate_version: filter.crate_version.context("none version")?.to_string(),
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
    cln_methods::cln_review_new(response_data, response_html)
}

#[named]
pub fn srv_review_save(request_data: serde_json::Value) -> anyhow::Result<String> {
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
        Err(err) => crate::response_post_mod::response_err_message(&err),
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
    srv_reviews_list(request_data)
}

#[named]
pub fn srv_review_edit(request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));
    // find the item from the list
    let p = crev_edit_review(filter)?;
    let response_data = from_crev_to_item(&p);
    let response_html = crate::files_mod::review_edit_html();

    cln_methods::cln_review_edit(response_data, response_html)
}

/// edit the review or copy the last review to create a new review
#[named]
pub fn srv_review_edit_or_new(request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data.clone()));

    match crev_edit_or_new_review(filter) {
        Err(_err) => srv_review_new(request_data),
        Ok(p) => {
            let response_data = from_crev_to_item(&p);
            let response_html = crate::files_mod::review_edit_html();
            cln_methods::cln_review_edit(response_data, response_html)
        }
    }
}

/// list of all versions for one crate: from registry index with data from src cached and my_reviews
#[named]
pub fn srv_version_list(request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data.clone()));

    let mut vec = crev_crate_versions(&filter.crate_name)?;
    // descending order
    vec.sort_by(|a, b| {
        let a = semver::Version::parse(&a.num).unwrap();
        let b = semver::Version::parse(&b.num).unwrap();
        b.cmp(&a)
    });

    let mut response_data = VersionListData { list_of_version: vec![] };
    for x in vec.iter() {
        let v = VersionItemData {
            crate_name: x.crate_name.clone(),
            crate_version: x.num.clone(),
            yanked: x.yanked,
            published_by_login: None,
            published_date: x.published_date.clone(),
            is_src_cached: x.is_src_cached,
            my_review: x.my_review.clone(),
        };
        response_data.list_of_version.push(v);
    }

    let response_html = crate::files_mod::version_list_html();

    cln_methods::cln_version_list(response_data, response_html)
}

#[named]
pub fn srv_review_new_version(request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));
    // find the item from the list
    let p = crev_new_version(filter)?;
    let response_data = from_crev_to_item(&p);
    let response_html = crate::files_mod::review_edit_html();
    cln_methods::cln_review_edit(response_data, response_html)
}

#[named]
pub fn srv_review_publish(_request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    match crev_publish() {
        Ok(ret_val) => crate::response_post_mod::response_modal_message(&ret_val),
        Err(err) => crate::response_post_mod::response_err_message(&err),
    }
}

#[named]
pub fn srv_update_registry_index(_request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    match crate::cargo_registry_mod::update_registry_index() {
        Ok(_ret_val) => crate::response_post_mod::response_modal_message("Registry index updated."),
        Err(err) => crate::response_post_mod::response_err_message(&err),
    }
}

#[named]
pub fn srv_review_open_source_code(request_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));
    let version = filter.crate_version.context("Parameter version in None.")?;
    let path_dir = crate::cargo_registry_mod::cargo_registry_src_dir_for_crate(&filter.crate_name, &version)?;
    if !path_dir.exists() {
        anyhow::bail!("Src for version {} is not cached on your system.", &version);
    }
    let mut child = std::process::Command::new("code").arg(path_dir).spawn()?;
    std::thread::sleep(Duration::new(1, 0));
    child.kill()?;

    crate::response_post_mod::response_modal_message("VSCode started.")
}

#[named]
pub fn srv_review_delete(filter_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());

    let filter: ReviewFilterData = unwrap!(serde_json::from_value(filter_data));
    let version = filter.crate_version.context("Parameter version in None.")?;
    crate::crev_mod::delete_review_proofs(filter.crate_name.as_str(), &version)?;

    request_review_list()
}

#[named]
pub fn srv_verify_project(_filter_data: serde_json::Value) -> anyhow::Result<String> {
    println!(function_name!());

    let response_data = crate::crev_mod::verify_project()?;
    let response_html = crate::files_mod::verify_list_html();
    cln_methods::cln_verify_list(response_data, response_html)
}
// endregion: review
