use std::{
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    str::FromStr,
};

use base64::Engine;
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

    /// Attempts to get the mime type of the file source
    pub fn mime_type(&self) -> Option<String> {
        match self {
            Self::Local(path) => get_mime_from_path(path).ok().map(|mime| mime.to_string()),
            Self::Remote(url) => reqwest::blocking::get(url.as_str())
                .ok()
                .and_then(|request| {
                    request
                        .headers()
                        .get("content-type")
                        .and_then(|content_type| Some(content_type.to_str().ok()?.to_string()))
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

/// Get the mime type from a path-like string
fn get_mime_from_path(trimmed: &Path) -> std::io::Result<&'static str> {
    if trimmed.extension().is_some_and(|ext| ext == "svg") {
        return Ok("image/svg+xml");
    }

    let res = match infer::get_from_path(trimmed)?.map(|f| f.mime_type()) {
        Some(f) => {
            if f == "text/plain" {
                get_mime_by_ext(trimmed)
            } else {
                f
            }
        }
        None => get_mime_by_ext(trimmed),
    };

    Ok(res)
}

/// Get the mime type from a URI using its extension
fn get_mime_by_ext(trimmed: &Path) -> &'static str {
    match trimmed.extension().and_then(|e| e.to_str()) {
        Some("bin") => "application/octet-stream",
        Some("css") => "text/css",
        Some("csv") => "text/csv",
        Some("html") => "text/html",
        Some("ico") => "image/vnd.microsoft.icon",
        Some("js") => "text/javascript",
        Some("json") => "application/json",
        Some("jsonld") => "application/ld+json",
        Some("mjs") => "text/javascript",
        Some("rtf") => "application/rtf",
        Some("svg") => "image/svg+xml",
        Some("mp4") => "video/mp4",
        // Assume HTML when a TLD is found for eg. `dioxus:://dioxuslabs.app` | `dioxus://hello.com`
        Some(_) => "text/html",
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
        // using octet stream according to this:
        None => "application/octet-stream",
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
    url_encoded: bool,
}

impl FileAsset {
    /// Creates a new file asset
    pub fn new(source: FileSource) -> Self {
        let options = FileOptions::default_for_extension(source.extension().as_deref());
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
        options.hash(&mut hash);
        source.hash(&mut hash);
        let uuid = hash.finish();
        let unique_name = format!("{file_name}{uuid}{extension}");

        Self {
            location: FileLocation {
                unique_name,
                source,
            },
            options,
            url_encoded: false,
        }
    }

    /// Set the file options
    pub fn with_options(self, options: FileOptions) -> Self {
        let manifest_dir = manifest_dir();
        let path = manifest_dir.join(self.location.source.last_segment());
        let updated = self.location.source.last_updated();
        let file_name = path.file_stem().unwrap().to_string_lossy();
        let extension = options
            .extension()
            .map(|e| format!(".{e}"))
            .unwrap_or_default();
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        updated.hash(&mut hash);
        options.hash(&mut hash);
        self.location.source.hash(&mut hash);
        let uuid = hash.finish();
        let unique_name = format!("{file_name}{uuid}{extension}");

        Self {
            location: FileLocation {
                unique_name,
                source: self.location.source,
            },
            options,
            url_encoded: false,
        }
    }

    /// Set whether the file asset should be url encoded
    pub fn set_url_encoded(&mut self, url_encoded: bool) {
        self.url_encoded = url_encoded;
    }

    /// Returns whether the file asset should be url encoded
    pub fn url_encoded(&self) -> bool {
        self.url_encoded
    }

    /// Returns the location where the file asset will be served from
    pub fn served_location(&self) -> String {
        if self.url_encoded {
            let data = self.location.read_to_bytes().unwrap();
            let data = base64::engine::general_purpose::STANDARD_NO_PAD.encode(data);
            let mime = self.location.source.mime_type().unwrap();
            format!("data:{mime};base64,{data}")
        } else {
            let config = Config::current();
            let root = config.assets_serve_location();
            let unique_name = self.location.unique_name();
            format!("{root}{unique_name}")
        }
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

    /// Returns the options for the file asset mutably
    pub fn with_options_mut(&mut self, with: impl FnOnce(&mut FileOptions)) -> &mut Self {
        with(&mut self.options);
        let manifest_dir = manifest_dir();
        let path = manifest_dir.join(self.location.source.last_segment());
        let updated = self.location.source.last_updated();
        let file_name = path.file_stem().unwrap().to_string_lossy();
        let extension = self
            .options
            .extension()
            .map(|e| format!(".{e}"))
            .unwrap_or_default();
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        updated.hash(&mut hash);
        self.options.hash(&mut hash);
        self.location.source.hash(&mut hash);
        let uuid = hash.finish();
        let unique_name = format!("{file_name}{uuid}{extension}");
        self.location.unique_name = unique_name;
        self
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
