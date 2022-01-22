// cargo_registry_mod.rs

//! functions around the cargo registry  d
//! ~\.cargo\registry\index\github.com-1ecc6299db9ec823\an\yh\anyhow  
//! ~\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.37\  

#![allow(dead_code)]

use anyhow::Context;
use unwrap::unwrap;

use crate::utils_mod::*;

/// cargo registry src directory for a crate
pub fn cargo_registry_src_dir() -> anyhow::Result<std::path::PathBuf> {
    let home_dir = home::home_dir().with_context(|| "home::home_dir() is None.")?;
    let crate_root = home_dir.join(".cargo/registry/src/github.com-1ecc6299db9ec823");

    // return
    Ok(crate_root)
}

/// cargo registry src directory for a crate
pub fn cargo_registry_src_dir_for_crate(crate_name: &str, crate_version: &str) -> anyhow::Result<std::path::PathBuf> {
    let home_dir = home::home_dir().with_context(|| "home::home_dir() is None.")?;
    let crate_root = home_dir
        .join(".cargo/registry/src/github.com-1ecc6299db9ec823")
        .join(&crate_version_for_src_folder(crate_name, crate_version));

    // return
    Ok(crate_root)
}

/// cargo registry cache file `.crate` for a crate
pub fn cargo_registry_cache_file_for_crate(crate_name: &str, crate_version: &str) -> anyhow::Result<std::path::PathBuf> {
    let home_dir = home::home_dir().with_context(|| "home::home_dir() is None.")?;
    let crate_root = home_dir
        .join(".cargo/registry/cache/github.com-1ecc6299db9ec823")
        .join(format!("{}.crate", &crate_version_for_src_folder(crate_name, crate_version)));

    // return
    Ok(crate_root)
}

fn open_index() -> anyhow::Result<crates_index::Index> {
    let index = crates_index::Index::new_cargo_default()?;
    Ok(index)
}

/// yanked
pub fn yanked_for_version(crate_name: &str, crate_version: &str) -> anyhow::Result<bool> {
    let crate_info = unwrap!(open_index())
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
    let crate_info = unwrap!(open_index())
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
    let crate_info = unwrap!(open_index())
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
    let crate_info = unwrap!(open_index())
        .crate_(crate_name)
        .context(format!("Cannot find crate name in registry: {}", crate_name))?;
    // max version by semver
    let crate_version = crate_info.highest_version();
    Ok(crate_version.version().to_string())
}

/// fetch/pull the registry index from github
pub fn update_registry_index() -> anyhow::Result<()> {
    unwrap!(open_index()).update()?;
    Ok(())
}

#[derive(Debug)]
struct FileMetadata {
    file_path: String,
    size: u64,
    mtime: Option<u64>,
}

/// when we are writing reviews, the source code of the crate must be locally in the `src` folder.
/// In that time it is a dependency of our project, so `cargo build` has downloaded the crates.
/// But later this source code can be removed because all our projects will use newer versions.
/// If we want to check that the reviews have the correct digest, we need the old source code.
/// I don't want to use cargo as a library (for now). And there is not a raw CLI command to download and unpack a crate.
/// First I will check if the crate_version is in the `src` folder. If not try to unpack it.
/// If it does not exist neither in the `cache` folder, then download it like
/// `https://crates.io/api/v1/crates/block-cipher-trait/0.6.0/download`
/// This works also for yanked crates.
pub fn fix_missing_src_folder_for_crates_that_have_review() -> anyhow::Result<String> {
    let mut crates_downloaded = 0;
    let mut crates_unpacked = 0;
    // my reviews are already copied in a sled database for speed
    let vec = crate::db_sled_mod::db_review_mod::list_all_crate_name_version()?;
    let review_count = vec.len();

    for crate_name_version in vec.iter() {
        let (crate_name, crate_version) = crate::utils_mod::crate_version_split(&crate_name_version);
        let folder = cargo_registry_src_dir_for_crate(&crate_name, &crate_version)?;
        if !folder.exists() {
            // check if the crate_version is in `cache` folder
            let crate_file = cargo_registry_cache_file_for_crate(&crate_name, &crate_version)?;
            if !crate_file.exists() {
                download_crate_from_crate_io(&crate_name, &crate_version)?;
                crates_downloaded += 1;
            }
            unpack_from_cache_to_src(&crate_name, &crate_version)?;
            crates_unpacked += 1;
        }
    }
    return Ok(format!(
        "All reviews: {}
Downloaded (missing): {}
Unpacked (missing): {}",
        review_count, crates_downloaded, crates_unpacked
    ));
}

/// download url from crates.io into `cargo registry cache` folder
/// like: https://crates.io/api/v1/crates/block-cipher-trait/0.6.0/download
/// it works also for yanked crates. This downloads are immutable.
pub fn download_crate_from_crate_io(crate_name: &str, crate_version: &str) -> anyhow::Result<()> {
    let url = format!(
        "https://crates.io/api/v1/crates/{}/{}/download",
        percent_encoding::utf8_percent_encode(crate_name, percent_encoding::NON_ALPHANUMERIC),
        percent_encoding::utf8_percent_encode(crate_version, percent_encoding::NON_ALPHANUMERIC)
    );
    log::info!("download_crate_from_crate_io: {}", &url);
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?;
    let crate_file = cargo_registry_cache_file_for_crate(&crate_name, &crate_version)?;

    let mut file = std::fs::File::create(crate_file)?;
    let mut content = std::io::Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    // TODO: maybe use the `cksum` from registry index to check integrity
    Ok(())
}

/// unpack the .crate file from the `cache` to the `src` folder
pub fn unpack_from_cache_to_src(crate_name: &str, crate_version: &str) -> anyhow::Result<()> {
    let src_folder = cargo_registry_src_dir()?;
    unpack_from_cache_to_folder(crate_name, crate_version, &src_folder)?;
    // create `.cargo-ok`. It is a signal for async use, that the unpacking is completed.
    let file_path = cargo_registry_src_dir_for_crate(crate_name, crate_version)?.join(".cargo-ok");
    std::fs::write(file_path, "ok")?;
    Ok(())
}

/// unpack the .crate file from the `cache` to a folder
pub fn unpack_from_cache_to_folder(crate_name: &str, crate_version: &str, folder: &std::path::Path) -> anyhow::Result<()> {
    log::info!("unpack_from_cache_to_folder: {:#?}", &folder);
    let crate_file = cargo_registry_cache_file_for_crate(&crate_name, &crate_version)?;
    let tar_gz = std::fs::File::open(crate_file)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(folder)?;

    Ok(())
}

/// compare the files in `src` and `cache` folders of `cargo registry`
/// return a list of folders to delete
/// and on the end the user will call `cargo check` to re-create a fresh copy of these folders
pub fn list_unclean_crates() -> anyhow::Result<String> {
    let mut ret_list = String::new();
    let home_dir = home::home_dir().with_context(|| "home::home_dir() is None.")?;
    let src_folder = home_dir.join(".cargo/registry/src/github.com-1ecc6299db9ec823");
    let cache_folder = home_dir.join(".cargo/registry/cache/github.com-1ecc6299db9ec823");
    // region: fill vectors to not repeat the listings of files
    let mut src_crates = vec![];
    for entry in walkdir::WalkDir::new(src_folder).min_depth(1).max_depth(1).sort_by_file_name() {
        let entry = entry?;
        src_crates.push(entry.path().to_owned());
    }
    let mut cache_crates = vec![];
    for entry in walkdir::WalkDir::new(cache_folder).min_depth(1).max_depth(1).sort_by_file_name() {
        let entry = entry?;
        cache_crates.push(entry.path().to_owned());
    }
    // from mut to not mut (does it allocate? Probably not, because is move.)
    let src_crates = src_crates;
    let cache_crates = cache_crates;
    // endregion: fill vectors to not repeat the listings of files

    // a folder in src mut also be in cache. But a folder in cache does not need to be in src.
    for src_path in src_crates.iter() {
        let mut contains = false;
        for cache_path in cache_crates.iter() {
            let cache_crate = cache_path
                .file_name()
                .with_context(|| "cache_path.file_name() is None.")?
                .to_string_lossy()
                .trim_end_matches(".crate")
                .to_string();
            let src_crate = src_path
                .file_name()
                .with_context(|| "src_path.file_name() is None.")?
                .to_string_lossy()
                .to_string();
            if cache_crate == src_crate {
                log::info!("check if unclean: {}", src_crate);
                // check if the content is equal
                compare_src_and_cache_content(src_path, cache_path, &mut ret_list)?;
                contains = true;
                break;
            }
        }
        if contains == false {
            // "crate exists in src, but not in cache: {}",
            ret_list.push_str(&format!("rm -r {}\n", src_path.to_string_lossy()));
        }
    }
    // return
    Ok(ret_list)
}

fn compare_src_and_cache_content(src_folder: &std::path::Path, cache_folder: &std::path::Path, ret_list: &mut String) -> anyhow::Result<()> {
    // region: fill vectors to not repeat the listings of files
    let mut src_files = vec![];
    for entry in walkdir::WalkDir::new(src_folder).min_depth(1).sort_by_file_name() {
        let entry = entry?;
        // only files, not directories
        if entry.path().is_file() {
            let file_path = entry
                .path()
                .to_string_lossy()
                .trim_start_matches(&src_folder.to_string_lossy().to_string())
                .to_string();

            // exception /.cargo-ok is added later to src and it is ok
            if file_path != "/.cargo-ok" {
                let mtime = match entry.metadata() {
                    Ok(metadata) => Some(std::os::unix::prelude::MetadataExt::mtime(&metadata) as u64),
                    Err(_err) => None,
                };

                src_files.push(FileMetadata {
                    file_path,
                    size: std::os::unix::prelude::MetadataExt::size(&entry.metadata()?),
                    mtime,
                });
            }
        }
    }

    let archive_folder_name = src_folder.file_name().with_context(|| "src_folder.file_name() is None.")?.to_string_lossy();
    let tar_gz = std::fs::File::open(cache_folder)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    let mut cache_files = vec![];
    for cache_file in archive.entries()? {
        let cache_file = cache_file?;

        let mtime = match cache_file.header().mtime() {
            Ok(mtime) => {
                // when uncompressed mtime 0 becomes magically 1
                if mtime == 0 {
                    Some(1)
                } else {
                    Some(mtime)
                }
            }
            Err(_err) => None,
        };
        // warning: cache_file.path() has long names, cache_file.header().path() does not have long names
        let file_path = cache_file.path()?.display().to_string();

        cache_files.push(FileMetadata {
            file_path: file_path.trim_start_matches(&archive_folder_name.to_string()).to_string(),
            size: cache_file.header().size()?,
            mtime,
        });
    }
    // sort
    cache_files.sort_by(|a, b| a.file_path.partial_cmp(&b.file_path).unwrap());
    // from mut to not mut (does it allocate? Probably not, because is move.)
    let src_files = src_files;
    let cache_files = cache_files;
    // endregion: fill vectors to not repeat the listings of files

    if src_files.len() != cache_files.len() {
        // "files count differ {} != {} {}",src_files.len(),cache_files.len(),
        ret_list.push_str(&format!("rm -r {}\n", src_folder.to_string_lossy()));
        return Ok(());
    }

    // 1. are the same files different in src than in cache?
    for src_file in src_files.iter() {
        for cache_file in cache_files.iter() {
            if cache_file.file_path == src_file.file_path {
                if src_file.size != cache_file.size {
                    // "Same file {} differ size: {} {}",src_file.file_path, src_file.size, cache_file.size
                    ret_list.push_str(&format!("rm -r {}\n", src_folder.to_string_lossy()));
                    return Ok(());
                }
                // if the cache mtime is None, then the src mtime is today. Don't need to check.
                if cache_file.mtime.is_some() {
                    if src_file.mtime != cache_file.mtime {
                        // "same file {} differ mtime: {:?} {:?}",src_file.file_path, src_file.mtime, cache_file.mtime
                        ret_list.push_str(&format!("rm -r {}\n", src_folder.to_string_lossy()));
                        return Ok(());
                    }
                }
                break;
            }
        }
    }

    // 2. are there more files in src than in cache?
    for src_file in src_files.iter() {
        let mut does_file_exist = false;
        for cache_file in cache_files.iter() {
            if cache_file.file_path == src_file.file_path {
                does_file_exist = true;
                break;
            }
        }
        if does_file_exist == false {
            // "src file not exist in cache {}", src_file.file_path);
            ret_list.push_str(&format!("rm -r {}\n", src_folder.to_string_lossy()));
            return Ok(());
        }
    }

    // 3. are there more files in cache than src?
    for cache_file in cache_files.iter() {
        let mut does_file_exist = false;
        for src_file in src_files.iter() {
            if cache_file.file_path == src_file.file_path {
                does_file_exist = true;
                break;
            }
        }
        if does_file_exist == false {
            // "cache file not exist in src {}", cache_file.file_path);
            ret_list.push_str(&format!("rm -r {}\n", src_folder.to_string_lossy()));
            return Ok(());
        }
    }

    //return
    Ok(())
}
