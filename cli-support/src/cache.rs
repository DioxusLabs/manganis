use std::path::PathBuf;

use project_root::get_project_root;

pub(crate) fn current_cargo_toml() -> PathBuf {
    cargo_metadata::MetadataCommand::new().exec().unwrap().root_package().unwrap().manifest_path.clone().into()
}

pub(crate) fn root_dir() -> PathBuf {
    get_project_root().unwrap()
}

pub(crate) fn lock_path() -> PathBuf {
    root_dir().join("Cargo.lock")
}
