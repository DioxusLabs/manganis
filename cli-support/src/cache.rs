use std::path::PathBuf;

pub(crate) fn current_cargo_toml() -> PathBuf {
    cargo_metadata::MetadataCommand::new()
        .exec()
        .unwrap()
        .root_package()
        .unwrap()
        .manifest_path
        .clone()
        .into()
}

pub(crate) fn root_dir() -> PathBuf {
    cargo_metadata::MetadataCommand::new()
        .exec()
        .unwrap()
        .workspace_root.into()
}

pub(crate) fn lock_path() -> PathBuf {
    root_dir().join("Cargo.lock")
}
