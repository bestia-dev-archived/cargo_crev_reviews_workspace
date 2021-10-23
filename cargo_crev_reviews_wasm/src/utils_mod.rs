// utils_mod.rs

//! small utility functions
#![allow(dead_code)]

// use unwrap::unwrap;

pub fn split_crate_version(crate_name_version: &str) -> (String, String) {
    let s: Vec<&str> = crate_name_version.split_whitespace().collect();
    let crate_name = s[1].to_string();
    let crate_version = s[2].to_string();
    (crate_name, crate_version)
}

pub fn join_crate_version(crate_name: &str, crate_version: &str) -> String {
    format!("{} {}", crate_name, crate_version)
}
