use std::path::PathBuf;

pub fn asset_cache_dir() -> PathBuf {
    let dir = std::env::var("CARGO_HOME").unwrap();
    let mut dir = PathBuf::from(dir);
    dir.push("assets");
    dir
}

pub(crate) fn config_path() -> PathBuf {
    asset_cache_dir().join("config.toml")
}

pub fn current_package_identifier() -> String {
    package_identifier(
        &std::env::var("CARGO_PKG_NAME").unwrap(),
        &current_package_version(),
    )
}

pub fn package_identifier(package: &str, version: &str) -> String {
    package.to_string() + "-" + version
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
