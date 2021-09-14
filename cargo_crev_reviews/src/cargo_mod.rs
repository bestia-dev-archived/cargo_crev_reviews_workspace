use std::str::FromStr;

// cargo_mod.rs

/// data from the cargo world
/// \\wsl$\Debian\home\luciano\.cargo\registry\index\github.com-1ecc6299db9ec823\an\yh\anyhow
/// \\wsl$\Debian\home\luciano\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.37\

// use unwrap::unwrap;

pub fn crate_registry_src_dir(crate_name:&str,crate_version_str:&str )-> anyhow::Result<std::path::PathBuf>{
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

pub fn crate_registry_index_file(crate_name:&str){
    // TODO: parse the file and get newest version, for example.

}

