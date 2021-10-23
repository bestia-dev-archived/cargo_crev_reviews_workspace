// cargo_registry_mod.rs

//! functions around the cargo registry  
//! ~\.cargo\registry\index\github.com-1ecc6299db9ec823\an\yh\anyhow  
//! ~\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.37\  

#![allow(dead_code)]

use anyhow::Context;
use lazy_static::lazy_static;
use std::str::FromStr;
use unwrap::unwrap;

lazy_static! {
    /// open only one time
    static ref INDEX: crates_index::Index=unwrap!(open_index());
}

/// cargo registry src directory for a crate
pub fn cargo_registry_src_dir_for_crate(crate_name: &str, crate_version: &str) -> anyhow::Result<std::path::PathBuf> {
    let crate_dir_name = format!("{}-{}", crate_name, crate_version);
    let crate_src_path = format!(
        "{}/registry/src/github.com-1ecc6299db9ec823/{}",
        home::cargo_home()?.to_str().unwrap(),
        crate_dir_name
    );
    let crate_root = std::path::PathBuf::from_str(&crate_src_path)?;
    // return
    Ok(crate_root)
}

fn open_index() -> anyhow::Result<crates_index::Index> {
    let index = crates_index::Index::new_cargo_default();
    if !index.exists() {
        anyhow::bail!("Local Cargo Registry Src does not exist");
    }
    Ok(index)
}

/// yanked
pub fn yanked_for_version(crate_name: &str, crate_version: &str) -> anyhow::Result<bool> {
    let crate_info = INDEX
        .crate_(crate_name)
        .context(format!("Cannot find crate name in registry: {}", crate_name))?;
    for x in crate_info.versions().iter() {
        if x.version() == crate_version {
            if x.is_yanked() {
                return Ok(true);
            }
            break;
        }
    }
    Ok(false)
}

/// list only yanked versions for one crate
pub fn yanked_for_one_crate(crate_name: &str) -> anyhow::Result<Vec<String>> {
    let crate_info = INDEX
        .crate_(crate_name)
        .context(format!("Cannot find crate name in registry: {}", crate_name))?;
    let mut vec = vec![];
    for x in crate_info.versions().iter() {
        if x.is_yanked() {
            vec.push(x.version().to_string())
        }
    }
    Ok(vec)
}

/// info of all versions for one crate
pub fn info_for_one_crate(crate_name: &str) -> anyhow::Result<Vec<(String, bool)>> {
    let crate_info = INDEX
        .crate_(crate_name)
        .context(format!("Cannot find crate name in registry: {}", crate_name))?;
    let mut vec = vec![];
    for x in crate_info.versions().iter() {
        vec.push((x.version().to_string(), x.is_yanked()));
    }
    Ok(vec)
}

/// max version from registry index cache
pub fn max_version_from_registry_index(crate_name: &str) -> anyhow::Result<String> {
    let crate_info = INDEX
        .crate_(crate_name)
        .context(format!("Cannot find crate name in registry: {}", crate_name))?;
    // max version by semver
    let crate_version = crate_info.highest_version();
    Ok(crate_version.version().to_string())
}

/// fetch/pull the registry index from github
pub fn update_registry_index() -> anyhow::Result<()> {
    INDEX.update()?;
    Ok(())
}
