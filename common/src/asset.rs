use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum AssetType {
    File(FileAsset),
    Tailwind(TailwindAsset),
    Metadata(MetadataAsset),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FileAsset {
    name: String,
    path: PathBuf,
}

impl FileAsset {
    pub fn new(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            path: path.canonicalize().unwrap(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
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
