// utils_mod.rs

#![allow(dead_code)]

use chrono::prelude::*;
use unwrap::unwrap;

use crate::*;

// region: crate_version can be in different formats
// separate crate_name and crate_version
// inside Cargo.toml: dev_bestia_string_utils="0.1.19"  (dash is replaced with underscore)
// folder name in src: wasm-bindgen-test-macro-0.3.28
// in sled database: wasm-bindgen-test-macro 0.3.28

pub fn crate_version_split(crate_name_version: &str) -> (String, String) {
    let s: Vec<&str> = crate_name_version.split_whitespace().collect();
    let crate_name = s[0].to_string();
    let crate_version = s[1].to_string();
    (crate_name, crate_version)
}

pub fn crate_version_join(crate_name: &str, crate_version: &str) -> String {
    format!("{} {}", crate_name, crate_version)
}

pub fn crate_version_for_src_folder(crate_name: &str, crate_version: &str) -> String {
    format!("{}-{}", crate_name, crate_version)
}

pub fn crate_version_for_cargo_toml(crate_name: &str, crate_version: &str) -> String {
    format!(r#"{}="{}""#, crate_name, crate_version)
}
// endregion: crate_version can be in different formats

/// returns the now in nanoseconds
pub fn ns_start(text: &str) -> i64 {
    let now = Utc::now();
    if !text.is_empty() {
        log::info!("{}{}: {}{}", *GREEN, &Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), text, *RESET);
    }
    now.timestamp_nanos()
}

/// returns the elapsed nanoseconds
pub fn ns_elapsed(ns_start: i64) -> i64 {
    let now_ns = Utc::now().timestamp_nanos();
    let duration_ns = now_ns - ns_start;
    // return
    duration_ns
}

/// print elapsed time in milliseconds and returns the new now in nanoseconds
pub fn ns_print_ms(name: &str, ns_start: i64) -> i64 {
    // milliseconds
    let duration_ns = ns_elapsed(ns_start) / 1_000_000;
    if !name.is_empty() {
        use num_format::{Locale, WriteFormatted};
        let mut string_duration_ns = String::new();
        unwrap!(string_duration_ns.write_formatted(&duration_ns, &Locale::en));

        log::info!("{}{:>15} ms: {}{}", *GREEN, string_duration_ns, name, *RESET);
    }
    // return new now_ns
    Utc::now().timestamp_nanos()
}

/// print elapsed time in nanoseconds and returns the new now in nanoseconds
pub fn ns_print_ns(name: &str, ns_start: i64) -> i64 {
    // milliseconds
    let duration_ns = ns_elapsed(ns_start);
    if !name.is_empty() {
        use num_format::{Locale, WriteFormatted};
        let mut string_duration_ns = String::new();
        unwrap!(string_duration_ns.write_formatted(&duration_ns, &Locale::en));

        log::info!("{}{:>15} ns: {}{}", *GREEN, string_duration_ns, name, *RESET);
    }
    // return new now_ns
    Utc::now().timestamp_nanos()
}

/// convert ProofCrevForReview into ReviewItemData
pub fn from_crev_to_item(p: &crev_mod::ProofCrevForReview) -> common_structs_mod::ReviewItemData {
    // log::debug!("from_crev_to_item {:?}", p);
    // it is possible that `review` is not defined in the proof. Then use some defaults.
    let thoroughness = match p.review.as_ref() {
        None => "None".to_string(),
        Some(review) => review.thoroughness.to_string(),
    };
    let understanding = match p.review.as_ref() {
        None => "None".to_string(),
        Some(review) => review.understanding.to_string(),
    };
    let rating = match p.review.as_ref() {
        None => "Neutral".to_string(),
        Some(review) => crev_mod::rating_to_string(&review.rating),
    };
    common_structs_mod::ReviewItemData {
        crate_name: p.package.name.clone(),
        crate_version: p.package.version.clone(),
        date: p.date.clone(),
        thoroughness,
        understanding,
        rating,
        comment_md: p.comment.as_ref().unwrap_or(&"".to_string()).clone(),
    }
}
