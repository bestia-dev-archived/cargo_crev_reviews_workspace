// srv_methods_publisher_mod.rs

//! rpc methods prepare the data to respond the POST rpc requests

use crate::auto_generated_mod::cln_methods;
use crate::common_structs_mod::*;

use function_name::named;

#[named]
pub fn srv_publisher_list(_filter_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let vec_publisher = crate::db_sled_mod::db_publisher_mod::list()?;
    let response_data = PublisherListData {
        list_of_publisher: vec_publisher,
    };
    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/publisher_list.html"));

    cln_methods::cln_publisher_list(response_data, &response_html)
}

#[named]
pub fn srv_publisher_new(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let response_data = RpcEmptyData {};
    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/publisher_new.html"));

    cln_methods::cln_publisher_new_modal(response_data, &response_html)
}

#[named]
pub fn srv_publisher_save(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let p: PublisherItemData = serde_json::from_value(request_data)?;
    crate::db_sled_mod::db_publisher_mod::insert(&p.url, &p)?;

    crate::response_post_mod::response_modal_close()
}
