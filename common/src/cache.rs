//! Utilities for the cache that is used to collect assets

use std::{path::PathBuf, fmt::{Write, Display}, hash::{Hash, Hasher}};

use home::cargo_home;

/// The location where assets are cached
pub fn asset_cache_dir() -> PathBuf {
    let mut dir = cargo_home().unwrap();
    dir.push("assets");
    dir
}

pub(crate) fn config_path() -> PathBuf {
    asset_cache_dir().join("config.toml")
}

pub(crate) fn current_package_identifier() -> String {
    package_identifier(
        &std::env::var("CARGO_PKG_NAME").unwrap(),
        &current_package_version(),
    )
}

/// The identifier for a package used to cache assets
pub fn package_identifier(package: &str, version: &str) -> String {
    package.to_string() + "-" + version
}

/// Like `package_identifier`, but appends the identifier to the given path
pub fn push_package_cache_dir(package: &str, version: impl Hash, dir: &mut PathBuf) {
    let as_string = dir.as_mut_os_string();
    as_string.write_char(std::path::MAIN_SEPARATOR).unwrap();
    as_string.write_str(&package).unwrap();
    as_string.write_char('-').unwrap();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    version.hash(&mut hasher);
    let version = hasher.finish();
    as_string.write_fmt(format_args!("{:x}", version)).unwrap();
}

pub(crate) fn current_package_version() -> String {
    std::env::var("CARGO_PKG_VERSION").unwrap()
}

pub(crate) fn manifest_dir() -> PathBuf {
    std::env::var("CARGO_MANIFEST_DIR").unwrap().into()
}

pub(crate) fn current_package_cache_dir() -> PathBuf {
    let mut dir = asset_cache_dir();
    dir.push(current_package_identifier());
    dir
}
