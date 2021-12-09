// cargo_tree_mod.rs

// use crate::auto_generated_mod::cln_methods;
use crate::common_structs_mod::*;
// use crate::crev_mod::*;

// use anyhow::Context;
// use function_name::named;
use std::env;
// use std::str::FromStr;
// use std::time::Duration;
//use unwrap::unwrap;

/// cargo_tree
pub fn cargo_tree_project() -> anyhow::Result<CargoTreeListData> {
    let ns_started = crate::utils_mod::ns_start("cargo_tree_project");

    let output = std::process::Command::new("cargo").arg("tree").output().unwrap();
    let output = format!("{} {}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));

    let mut list_of_cargo_tree = vec![];
    for line in output.lines() {
        if !line.trim().starts_with("warning:") && !line.starts_with("package:") && !line.starts_with("workspace:") {
            // TODO: add my reviews, publisher, crev status (from cache)
            list_of_cargo_tree.push(CargoTreeItemData {
                cargo_tree_line: line.to_string(),
            })
        }
    }

    crate::utils_mod::ns_print_ms("cargo_tree_project", ns_started);

    Ok(CargoTreeListData {
        project_dir: env::current_dir()?.to_string_lossy().to_string(),
        list_of_cargo_tree,
    })
}
