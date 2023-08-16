use std::path::PathBuf;

use project_root::get_project_root;

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
