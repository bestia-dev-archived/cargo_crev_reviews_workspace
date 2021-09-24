// cargo_mod.rs

//! functions around the cargo registry  
//! ~\.cargo\registry\index\github.com-1ecc6299db9ec823\an\yh\anyhow  
//! ~\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.37\  

use anyhow::Context;
use std::str::FromStr;
// use unwrap::unwrap;

/// cargo registry src directory for a crate
pub fn cargo_registry_src_dir_for_crate(crate_name: &str, crate_version_str: &str) -> anyhow::Result<std::path::PathBuf> {
    let crate_dir_name = format!("{}-{}", crate_name, crate_version_str);
    let crate_src_path = format!(
        "{}/registry/src/github.com-1ecc6299db9ec823/{}",
        home::cargo_home()?.to_str().unwrap(),
        crate_dir_name
    );
    let crate_root = std::path::PathBuf::from_str(&crate_src_path)?;
    // return
    Ok(crate_root)
}

/// max version from registry index cache
pub fn max_version_from_registry_index(crate_name: &str) -> anyhow::Result<String> {
    let index = crates_index::Index::new_cargo_default();
    if !index.exists() {
        anyhow::bail!("Local Registry IndexCould does not exist");
    }
    index.update()?;
    let crate_releases = index.crate_(crate_name).context("Cannot find crate name in registry.")?;
    // max version by semver
    let crate_version = crate_releases.highest_version();
    Ok(crate_version.version().to_string())
}

/// fetch/pull the registry index from github
pub fn update_registry_index() -> anyhow::Result<()> {
    let index = crates_index::Index::new_cargo_default();
    if !index.exists() {
        anyhow::bail!("Local Registry IndexCould does not exist");
    }
    index.update()?;
    Ok(())
}
