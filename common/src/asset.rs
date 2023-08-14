use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{cache::manifest_dir, FileOptions};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum AssetType {
    File(FileAsset),
    Tailwind(TailwindAsset),
    Metadata(MetadataAsset),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FileAsset {
    unique_name: String,
    path: PathBuf,
    options: FileOptions,
}

impl FileAsset {
    pub fn new(path: PathBuf, options: FileOptions) -> std::io::Result<Self> {
        let manifest_dir = manifest_dir();
        let path = manifest_dir.join(path);
        let uuid = uuid::Uuid::new_v4();
        let file_name = path.file_stem().unwrap().to_string_lossy();
        let extension = path
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();
        let uuid_hex = uuid.simple().to_string();
        let unique_name = format!("{file_name}{uuid_hex}{extension}");

        Ok(Self {
            unique_name,
            path: path.canonicalize()?,
            options,
        })
    }

    pub fn unique_name(&self) -> &str {
        &self.unique_name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct MetadataAsset {
    key: String,
    value: String,
}

impl MetadataAsset {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct TailwindAsset {
    classes: String,
}

impl TailwindAsset {
    pub fn new(classes: &str) -> Self {
        Self {
            classes: classes.to_string(),
        }
    }

    pub fn classes(&self) -> &str {
        &self.classes
    }
}
