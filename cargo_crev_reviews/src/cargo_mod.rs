use std::str::FromStr;

use anyhow::Context;

// cargo_mod.rs

/// data from the cargo world
/// \\wsl$\Debian\home\luciano\.cargo\registry\index\github.com-1ecc6299db9ec823\an\yh\anyhow
/// \\wsl$\Debian\home\luciano\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.37\

// use unwrap::unwrap;

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

pub fn update_registry_index() -> anyhow::Result<()> {
    let index = crates_index::Index::new_cargo_default();
    if !index.exists() {
        anyhow::bail!("Local Registry IndexCould does not exist");
    }
    index.update()?;
    Ok(())
}

/*
pub fn cargo_registry_index_file_for_crate(crate_name: &str) -> anyhow::Result<std::path::PathBuf> {
    // TODO: parse the file and get newest version, for example.
    let mut crate_index_path = format!("{}/registry/index/github.com-1ecc6299db9ec823/", home::cargo_home()?.to_str().unwrap());
    let mut chars = crate_name.chars();
    let sub_string = (0..)
        .map(|_| chars.by_ref().take(2).collect::<String>())
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>();

    println!("Safe: {:?}", sub_string);
    for sub in sub_string {
        crate_index_path.push_str(&sub);
        crate_index_path.push_str("/");
    }
    println!("crate_index_path: {}", &crate_index_path);
    let crate_file = std::path::PathBuf::from_str(&crate_index_path)?;
    // return
    Ok(crate_file)
}

/// compare 2 semver,
/// return enum std::cmp::Ordering {Less,Equal,Greater,}
pub fn semver_compare(semver_1: &str, semver_2: &str) -> std::cmp::Ordering {
    if semver_1 == semver_2 {
        return std::cmp::Ordering::Equal;
    } else {
        let sem_1: Vec<&str> = semver_1.split(".").collect();
        let sem_2: Vec<&str> = semver_2.split(".").collect();
        if unwrap!(sem_1[0].parse::<u32>()) > unwrap!(sem_2[0].parse::<u32>()) {
            return std::cmp::Ordering::Greater;
        } else if unwrap!(sem_1[0].parse::<u32>()) < unwrap!(sem_2[0].parse::<u32>()) {
            return std::cmp::Ordering::Less;
        } else {
            // the first part is equal
            if unwrap!(sem_1[1].parse::<u32>()) > unwrap!(sem_2[1].parse::<u32>()) {
                return std::cmp::Ordering::Greater;
            } else if unwrap!(sem_1[1].parse::<u32>()) < unwrap!(sem_2[1].parse::<u32>()) {
                return std::cmp::Ordering::Less;
            } else {
                // the first and second part is equal
                if unwrap!(sem_1[2].parse::<u32>()) > unwrap!(sem_2[2].parse::<u32>()) {
                    return std::cmp::Ordering::Greater;
                } else if unwrap!(sem_1[2].parse::<u32>()) < unwrap!(sem_2[2].parse::<u32>()) {
                    return std::cmp::Ordering::Less;
                } else {
                    // the 3 parts are equal. I will look no further
                    return std::cmp::Ordering::Equal;
                }
            }
        }
    }
}

 */
