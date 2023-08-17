use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{cache::manifest_dir, Config, FileOptions};

/// The type of asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum AssetType {
    /// A file asset
    File(FileAsset),
    /// A tailwind class asset
    Tailwind(TailwindAsset),
    /// A metadata asset
    Metadata(MetadataAsset),
}

/// The source of a file asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash)]
pub enum FileSource {
    /// A local file
    Local(PathBuf),
    /// A remote file
    Remote(Url),
}

impl FileSource {
    /// Returns the last segment of the file source used to generate a unique name
    pub fn last_segment(&self) -> &str {
        match self {
            Self::Local(path) => path.file_name().unwrap().to_str().unwrap(),
            Self::Remote(url) => url.path_segments().unwrap().last().unwrap(),
        }
    }

    /// Returns the extension of the file source
    pub fn extension(&self) -> Option<String> {
        match self {
            Self::Local(path) => path.extension().map(|e| e.to_str().unwrap().to_string()),
            Self::Remote(url) => reqwest::blocking::get(url.as_str())
                .ok()
                .and_then(|request| {
                    request
                        .headers()
                        .get("content-type")
                        .and_then(|content_type| {
                            content_type
                                .to_str()
                                .ok()
                                .map(|ty| ext_of_mime(ty).to_string())
                        })
                }),
        }
    }

    /// Find when the asset was last updated
    pub fn last_updated(&self) -> Option<String> {
        match self {
            Self::Local(path) => path.metadata().ok().and_then(|metadata| {
                metadata
                    .modified()
                    .ok()
                    .map(|modified| format!("{:?}", modified))
                    .or_else(|| {
                        metadata
                            .created()
                            .ok()
                            .map(|created| format!("{:?}", created))
                    })
            }),
            Self::Remote(url) => reqwest::blocking::get(url.as_str())
                .ok()
                .and_then(|request| {
                    request
                        .headers()
                        .get("last-modified")
                        .and_then(|last_modified| {
                            last_modified
                                .to_str()
                                .ok()
                                .map(|last_modified| last_modified.to_string())
                        })
                }),
        }
    }
}

/// Get the mime type from a URI using its extension
fn ext_of_mime(mime: &str) -> &str {
    let mime = mime.split(';').next().unwrap_or_default();
    match mime.trim() {
        "application/octet-stream" => "bin",
        "text/css" => "css",
        "text/csv" => "csv",
        "text/html" => "html",
        "image/vnd.microsoft.icon" => "ico",
        "text/javascript" => "js",
        "application/json" => "json",
        "application/ld+json" => "jsonld",
        "application/rtf" => "rtf",
        "image/svg+xml" => "svg",
        "video/mp4" => "mp4",
        "text/plain" => "txt",
        "application/xml" => "xml",
        "application/zip" => "zip",
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/avif" => "avif",
        "font/ttf" => "ttf",
        "font/woff" => "woff",
        "font/woff2" => "woff2",
        other => other.split('/').last().unwrap_or_default(),
    }
}

/// The location of an asset before and after it is collected
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FileLocation {
    unique_name: String,
    source: FileSource,
}

impl FileLocation {
    /// Returns the unique name of the file that the asset will be served from
    pub fn unique_name(&self) -> &str {
        &self.unique_name
    }

    /// Returns the source of the file that the asset will be collected from
    pub fn source(&self) -> &FileSource {
        &self.source
    }

    /// Reads the file to a string
    pub fn read_to_string(&self) -> anyhow::Result<String> {
        match &self.source {
            FileSource::Local(path) => Ok(std::fs::read_to_string(path)?),
            FileSource::Remote(url) => {
                let response = reqwest::blocking::get(url.as_str())?;
                Ok(response.text()?)
            }
        }
    }

    /// Reads the file to bytes
    pub fn read_to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        match &self.source {
            FileSource::Local(path) => Ok(std::fs::read(path)?),
            FileSource::Remote(url) => {
                let response = reqwest::blocking::get(url.as_str())?;
                Ok(response.bytes().map(|b| b.to_vec())?)
            }
        }
    }
}

impl FromStr for FileSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Url::parse(s) {
            Ok(url) => Ok(Self::Remote(url)),
            Err(_) => {
                let manifest_dir = manifest_dir();
                let path = manifest_dir.join(PathBuf::from(s));
                Ok(Self::Local(path.canonicalize()?))
            }
        }
    }
}

/// A file asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FileAsset {
    location: FileLocation,
    options: FileOptions,
}

impl FileAsset {
    /// Creates a new file asset
    pub fn new(source: FileSource) -> std::io::Result<Self> {
        let options = FileOptions::default_for_extension(source.extension().as_deref());
        Self::new_with_options(source, options)
    }

    /// Creates a new file asset with options
    pub fn new_with_options(source: FileSource, options: FileOptions) -> std::io::Result<Self> {
        let manifest_dir = manifest_dir();
        let path = manifest_dir.join(source.last_segment());
        let updated = source.last_updated();
        let file_name = path.file_stem().unwrap().to_string_lossy();
        let extension = options
            .extension()
            .map(|e| format!(".{e}"))
            .unwrap_or_default();
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        updated.hash(&mut hash);
        source.hash(&mut hash);
        let uuid = hash.finish();
        let unique_name = format!("{file_name}{uuid}{extension}");

        Ok(Self {
            location: FileLocation {
                unique_name,
                source,
            },
            options,
        })
    }

    /// Returns the location where the file asset will be served from
    pub fn served_location(&self) -> String {
        let config = Config::current();
        let root = config.assets_serve_location();
        let unique_name = self.location.unique_name();
        format!("{root}{unique_name}")
    }

    /// Returns the location of the file asset
    pub fn location(&self) -> &FileLocation {
        &self.location
    }

    /// Returns the location of the file asset
    pub fn set_unique_name(&mut self, unique_name: &str) {
        self.location.unique_name = unique_name.to_string();
    }

    /// Returns the options for the file asset
    pub fn options(&self) -> &FileOptions {
        &self.options
    }
}

/// A metadata asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct MetadataAsset {
    key: String,
    value: String,
}

impl MetadataAsset {
    /// Creates a new metadata asset
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    /// Returns the key of the metadata asset
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the value of the metadata asset
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// A tailwind class asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct TailwindAsset {
    classes: String,
}

impl TailwindAsset {
    /// Creates a new tailwind class asset
    pub fn new(classes: &str) -> Self {
        Self {
            classes: classes.to_string(),
        }
    }

    /// Returns the classes of the tailwind class asset
    pub fn classes(&self) -> &str {
        &self.classes
    }
}
