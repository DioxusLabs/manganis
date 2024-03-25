//! Utilities for the cache that is used to collect assets

use std::{
    fmt::{Display, Write},
    path::PathBuf,
};

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
        std::env::var("CARGO_BIN_NAME").ok().as_deref(),
        current_package_version(),
    )
}

/// The identifier for a package used to cache assets
pub fn package_identifier(package: &str, bin: Option<&str>, version: impl Display) -> String {
    let mut id = String::new();
    push_package_identifier(package, bin, version, &mut id);
    id
}

/// Like `package_identifier`, but appends the identifier to the given writer
pub fn push_package_identifier(
    package: &str,
    bin: Option<&str>,
    version: impl Display,
    to: &mut impl Write,
) {
    to.write_str(package).unwrap();
    if let Some(bin) = bin {
        to.write_char('-').unwrap();
        to.write_str(bin).unwrap();
    }
    to.write_char('-').unwrap();
    to.write_fmt(format_args!("{}", version)).unwrap();
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

/// The location where logs are stored while expanding macros
pub fn macro_log_directory() -> PathBuf {
    let mut dir = asset_cache_dir();
    dir.push("logs");
    dir
}

/// The current log file for the macro expansion
pub fn macro_log_file() -> PathBuf {
    let mut dir = macro_log_directory();
    dir.push(current_package_identifier());
    dir
}
