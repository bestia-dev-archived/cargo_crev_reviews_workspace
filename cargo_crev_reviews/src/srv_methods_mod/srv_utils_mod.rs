// srv_utils_mod.rs

//! rpc methods prepare the data to respond the POST rpc requests

use crate::auto_generated_mod::cln_methods;
use crate::common_structs_mod::*;
use crate::crev_mod::*;

use function_name::named;
use unwrap::unwrap;

#[named]
pub fn srv_cargo_tree_project(_filter_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let response_data = crate::cargo_tree_mod::cargo_tree_project()?;
    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/cargo_tree.html"));

    cln_methods::cln_cargo_tree_list(response_data, &response_html)
}

/// list of all versions for one crate: from registry index with data from src cached and my_reviews
#[named]
pub fn srv_version_list(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data.clone()));

    let mut vec = crev_crate_versions(&filter.crate_name)?;
    // descending order
    vec.sort_by(|a, b| {
        let a = semver::Version::parse(&a.crate_version).unwrap();
        let b = semver::Version::parse(&b.crate_version).unwrap();
        b.cmp(&a)
    });

    let response_data = VersionListData { list_of_version: vec };
    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/version_list.html"));

    cln_methods::cln_version_list(response_data, &response_html)
}

#[named]
pub fn srv_update_registry_index(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    match crate::cargo_registry_mod::update_registry_index() {
        Ok(_ret_val) => crate::response_post_mod::response_modal_message("Registry index updated."),
        Err(err) => crate::response_post_mod::response_err_message(&err),
    }
}

#[named]
pub fn srv_config_edit(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/config_edit.html"));
    let response_data = crate::db_sled_mod::db_metadata_mod::get_config()?;

    cln_methods::cln_config_edit(response_data, &response_html)
}

#[named]
pub fn srv_config_save(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let config: ConfigData = serde_json::from_value(request_data)?;
    crate::db_sled_mod::db_metadata_mod::set_config(config);

    crate::response_post_mod::response_modal_message("Config saved.")
}

/// list crates to delete manually. The next `cargo check` or `cargo build` will unzip them from the cache folder.
#[named]
pub fn srv_list_unclean_crates(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let mut list = crate::cargo_registry_mod::list_unclean_crates()?;
    log::info!("srv_list_unclean_crates() finished.");
    if list.is_empty() {
        list = "Everything is clean!".to_string();
    } else {
        list = format!("Run these commands to clean dependency crates src: \n{}", list);
    }
    crate::response_post_mod::response_modal_message(&list)
}

/// fix the scr folder for crates that have our review
#[named]
pub fn srv_fix_missing_src_folder(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let output = crate::cargo_registry_mod::fix_missing_src_folder_for_crates_that_have_review()?;
    log::info!("srv_fix_missing_src_folder() finished.");
    crate::response_post_mod::response_modal_message(&output)
}

/// correct digest to all reviews where needed
#[named]
pub fn srv_correct_digest(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let mut ret_string = String::new();
    let mut num_of_corrected = 0;

    // for all reviews
    let mut vec_proof = unwrap!(crev_list_my_reviews(&None));
    vec_proof.sort_by(|a, b| a.package.version_for_sorting.cmp(&b.package.version_for_sorting));
    let num_of_all = vec_proof.len();
    for p in vec_proof.iter() {
        log::info!("check digest: {} {}", &p.package.name, &p.package.version);
        match calculate_crate_digest(&p.package.name, &p.package.version) {
            Err(err) => ret_string.push_str(&format!("{}\n", &err)),
            Ok(digest) => {
                if p.package.digest != digest.to_string() {
                    match &p.review {
                        None => {
                            let thoroughness = crev_data::Level::None;
                            let understanding = crev_data::Level::None;
                            let rating = crev_data::Rating::Neutral;
                            // crev save review proof
                            unwrap!(crev_save_review(
                                &p.package.name,
                                &p.package.version,
                                thoroughness,
                                understanding,
                                rating,
                                &p.comment.as_deref().unwrap_or("")
                            ));
                        }
                        Some(review) => {
                            let thoroughness = crev_data::Level::from(review.thoroughness);
                            let understanding = crev_data::Level::from(review.understanding);
                            let rating = crev_data::Rating::from(review.rating);
                            // crev save review proof
                            unwrap!(crev_save_review(
                                &p.package.name,
                                &p.package.version,
                                thoroughness,
                                understanding,
                                rating,
                                &p.comment.as_deref().unwrap_or("")
                            ));
                        }
                    }
                    num_of_corrected += 1;
                }
            }
        }
    }
    log::info!("srv_correct_digest() finished.");
    let corr_string = if num_of_corrected == 0 && ret_string.is_empty() {
        format!("All digest are correct.")
    } else {
        format!("Digest corrected on {} reviews of {} reviews.", num_of_corrected, num_of_all)
    };
    if ret_string.is_empty() {
        ret_string = corr_string;
    } else {
        ret_string = format!("{}\nErrors:\n{}", corr_string, ret_string);
    }
    crate::response_post_mod::response_modal_message(&ret_string)
}
