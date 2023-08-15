use std::path::PathBuf;

use project_root::get_project_root;

pub(crate) fn asset_cache_dir() -> PathBuf {
    let dir = std::env::var("CARGO_HOME").unwrap();
    let mut dir = PathBuf::from(dir);
    dir.push("assets");
    dir
}

pub fn current_package_identifier() -> String {
    package_identifier(
        &std::env::var("CARGO_PKG_NAME").unwrap(),
        &current_package_version(),
    )
}

pub(crate) fn package_identifier(package: &str, version: &str) -> String {
    package.to_string() + "-" + version
}

pub(crate) fn current_package_version() -> String {
    std::env::var("CARGO_PKG_VERSION").unwrap()
}

pub(crate) fn manifest_dir() -> PathBuf {
    std::env::var("CARGO_MANIFEST_DIR").unwrap().into()
}

pub(crate) fn current_cargo_toml() -> PathBuf {
    manifest_dir().join("Cargo.toml")
}

pub(crate) fn root_dir() -> PathBuf {
    get_project_root().unwrap()
}

pub(crate) fn lock_path() -> PathBuf {
    root_dir().join("Cargo.lock")
}

#[test]
fn env_variables() {
    assert_eq!(current_package_identifier(), "assets-0.1.0");
    asset_cache_dir().to_str().unwrap();
}

pub(crate) fn current_package_cache_dir() -> PathBuf {
    let mut dir = asset_cache_dir();
    dir.push(current_package_identifier());
    dir
}
