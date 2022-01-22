// srv_review_mod.rs

//! rpc methods prepare the data to respond the POST rpc requests

use crate::auto_generated_mod::cln_methods;
use crate::common_structs_mod::*;
use crate::crev_mod::*;

use anyhow::Context;
use function_name::named;
use std::str::FromStr;
use std::time::Duration;
use unwrap::unwrap;

/// maybe add filter for one crate_name
#[named]
pub fn srv_reviews_list(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let mut vec_review: Vec<ReviewItemData> = vec![];
    let vec_proof = unwrap!(crev_list_my_reviews(&None));
    // reverse, newest on top
    for p in vec_proof.iter().rev() {
        vec_review.push(crate::utils_mod::from_crev_to_item(p));
    }

    let response_data = ReviewListData {
        filter: "".to_string(),
        list_of_review: vec_review,
    };
    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("cargo_crev_reviews/review_list.html"));

    cln_methods::cln_review_list(response_data, &response_html)
}

#[named]
pub fn srv_review_new(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));

    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/review_new.html"));
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
owners (in crates.io) reputation: unknown / Rust team / active in Rust community
used in projects:  
alternative crates explored:
        "#
        .to_string(),
    };
    cln_methods::cln_review_new(response_data, &response_html)
}

#[named]
pub fn srv_review_save(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let p: ReviewItemData = unwrap!(serde_json::from_value(request_data));

    match crev_save_review(
        &p.crate_name,
        &p.crate_version,
        crev_data::Level::from_str(&p.thoroughness)?,
        crev_data::Level::from_str(&p.understanding)?,
        rating_parse(&p.rating)?,
        &p.comment_md,
    ) {
        Err(err) => Err(err),
        Ok(()) => crate::response_post_mod::response_modal_message("Review saved."),
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
    log::info!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));
    // find the item from the list
    let p = crev_edit_review(filter)?;
    let response_data = crate::utils_mod::from_crev_to_item(&p);
    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/review_edit.html"));

    cln_methods::cln_review_edit(response_data, &response_html)
}

/// edit the review or copy the last review to create a new review
#[named]
pub fn srv_review_edit_or_new(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data.clone()));

    match crev_edit_or_new_review(filter) {
        Err(_err) => srv_review_new(request_data),
        Ok(p) => {
            let response_data = crate::utils_mod::from_crev_to_item(&p);
            let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/review_edit.html"));
            cln_methods::cln_review_edit(response_data, &response_html)
        }
    }
}

#[named]
pub fn srv_review_new_version(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));
    // find the item from the list
    let p = crev_new_version(filter)?;
    let response_data = crate::utils_mod::from_crev_to_item(&p);
    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/review_edit.html"));
    cln_methods::cln_review_edit(response_data, &response_html)
}

#[named]
pub fn srv_review_publish(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    match crev_publish() {
        Ok(ret_val) => crate::response_post_mod::response_modal_message(&ret_val),
        Err(err) => Err(err),
    }
}

/// The source code of the dependency crate is in the `cargo registry src` folder
/// But it must not be opened with a code editor, because the intellisense server will alter the files: add the target folder and Cargo.lock file.
/// This is why I copy this folder into a temp directory first.
/// To avoid copying some unclean crate, I will unpack the `.crate` file from the `cargo registry cache` folder
#[named]
pub fn srv_review_open_source_code(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data));
    let version = filter.crate_version.context("Parameter version in None.")?;

    let cache_crate_file = crate::cargo_registry_mod::cargo_registry_cache_file_for_crate(&filter.crate_name, &version)?;
    if !cache_crate_file.exists() {
        anyhow::bail!("Crate version {} {} is not cached on your system.", &filter.crate_name, &version);
    }
    log::info!("Unpack and open source code from {:#?}", &cache_crate_file);

    // unpack the .crate into temp directory
    // VSCode cannot open a folder in the tmp dir. I will use the existing:
    // ~\.config\crev\cargo_crev_reviews_data\tmp_src
    let home_dir = home::home_dir().with_context(|| "home::home_dir() is None.")?;
    let tempdir = home_dir.join(".config/crev/cargo_crev_reviews_data/tmp_src");
    if !tempdir.exists() {
        std::fs::create_dir(&tempdir)?;
    }
    let folder_name = crate::utils_mod::crate_version_for_src_folder(&filter.crate_name, &version);
    let temp_path_dir = tempdir.join(&folder_name);
    if temp_path_dir.exists() {
        std::fs::remove_dir_all(&temp_path_dir)?;
    }
    crate::cargo_registry_mod::unpack_from_cache_to_folder(&filter.crate_name, &version, &tempdir)?;
    log::info!("Unpacked into {:#?}", &temp_path_dir);

    let config = unwrap!(crate::get_config());
    // test if the `/usr/bin/code` exists.
    if !std::path::Path::new(&config.code_editor_path).exists() {
        log::error!(
            "The editor `{}` does not exist. Change it in the config menu. Or open manually the directory `{}`.",
            &config.code_editor_path,
            unwrap!(temp_path_dir.to_str())
        );
    } else {
        let mut child = std::process::Command::new(&config.code_editor_path).arg(&temp_path_dir).spawn()?;
        std::thread::sleep(Duration::new(1, 0));
        child.kill()?;
    }
    // return nothing
    crate::response_post_mod::response_no_action()
}

#[named]
pub fn srv_review_delete(filter_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let filter: ReviewFilterData = unwrap!(serde_json::from_value(filter_data));
    let version = filter.crate_version.context("Parameter version in None.")?;
    crate::crev_mod::delete_review_proofs(filter.crate_name.as_str(), &version)?;

    request_review_list()
}
