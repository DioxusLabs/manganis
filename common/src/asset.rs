use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
};

use anyhow::Context;
use base64::Engine;
use cargo_config2::Config;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{cache::manifest_dir, FileOptions};
// use crate::{cache::manifest_dir, Config, FileOptions};

/// The maximum length of a path segment
const MAX_PATH_LENGTH: usize = 128;
/// The length of the hash in the output path
const HASH_SIZE: usize = 16;

/// The type of asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum AssetType {
    /// A file asset
    File(FileAsset),
    /// A folder asset
    Folder(FolderAsset),
    /// A tailwind class asset
    Tailwind(TailwindAsset),
    /// A metadata asset
    Metadata(MetadataAsset),
}

/// The source of a file asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash, Eq)]
pub enum AssetSource {
    /// A local file
    Local(PathBuf),
    /// A remote file
    Remote(Url),
}

impl Display for AssetSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = match self {
            Self::Local(path) => path.display().to_string(),
            Self::Remote(url) => url.as_str().to_string(),
        };
        if as_string.len() > 25 {
            write!(f, "{}...", &as_string[..25])
        } else {
            write!(f, "{}", as_string)
        }
    }
}

impl AssetSource {
    /// Try to convert the asset source to a path
    pub fn as_path(&self) -> Option<&PathBuf> {
        match self {
            Self::Local(path) => Some(path),
            Self::Remote(_) => None,
        }
    }

    /// Try to convert the asset source to a url
    pub fn as_url(&self) -> Option<&Url> {
        match self {
            Self::Local(_) => None,
            Self::Remote(url) => Some(url),
        }
    }

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

    /// Reads the file to a string
    pub fn read_to_string(&self) -> anyhow::Result<String> {
        match &self {
            AssetSource::Local(path) => Ok(std::fs::read_to_string(path).with_context(|| {
                format!("Failed to read file from location: {}", path.display())
            })?),
            AssetSource::Remote(url) => {
                let response = reqwest::blocking::get(url.as_str())
                    .with_context(|| format!("Failed to asset from url: {}", url.as_str()))?;
                Ok(response.text().with_context(|| {
                    format!("Failed to read text for asset from url: {}", url.as_str())
                })?)
            }
        }
    }

    /// Reads the file to bytes
    pub fn read_to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        match &self {
            AssetSource::Local(path) => Ok(std::fs::read(path).with_context(|| {
                format!("Failed to read file from location: {}", path.display())
            })?),
            AssetSource::Remote(url) => {
                let response = reqwest::blocking::get(url.as_str())
                    .with_context(|| format!("Failed to asset from url: {}", url.as_str()))?;
                Ok(response.bytes().map(|b| b.to_vec()).with_context(|| {
                    format!("Failed to read text for asset from url: {}", url.as_str())
                })?)
            }
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
    get_mime_from_ext(trimmed.extension().and_then(|e| e.to_str()))
}

/// Get the mime type from a URI using its extension
pub fn get_mime_from_ext(extension: Option<&str>) -> &'static str {
    match extension {
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
        Some("png") => "image/png",
        Some("jpg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("avif") => "image/avif",
        Some("txt") => "text/plain",
        // Assume HTML when a TLD is found for eg. `dioxus:://dioxuslabs.app` | `dioxus://hello.com`
        Some(_) => "text/html",
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
        // using octet stream according to this:
        None => "application/octet-stream",
    }
}

/// The location of an asset before and after it is collected
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash, Eq)]
pub struct AssetLocation {
    unique_name: String,
    source: AssetSource,
}

impl AssetLocation {
    /// Returns the unique name of the file that the asset will be served from
    pub fn unique_name(&self) -> &str {
        &self.unique_name
    }

    /// Returns the source of the file that the asset will be collected from
    pub fn source(&self) -> &AssetSource {
        &self.source
    }
}

/// Error while checking an asset exists
#[derive(Debug)]
pub enum AssetError {
    /// The relative path does not exist
    NotFoundRelative(PathBuf, String),
    /// The path exist but is not a file
    NotFile(PathBuf),
    /// The path exist but is not a folder
    NotFolder(PathBuf),
    /// Unknown IO error
    IO(PathBuf, std::io::Error),
}

impl Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetError::NotFoundRelative(manifest_dir, path) =>
                write!(f,"cannot find file `{}` in `{}`, please make sure it exists.\nAny relative paths are resolved relative to the manifest directory.",
                       path,
                       manifest_dir.display()
                ),
            AssetError::NotFile(absolute_path) =>
                write!(f, "`{}` is not a file, please choose a valid asset.\nAny relative paths are resolved relative to the manifest directory.", absolute_path.display()),
            AssetError::NotFolder(absolute_path) =>
                write!(f, "`{}` is not a folder, please choose a valid asset.\nAny relative paths are resolved relative to the manifest directory.", absolute_path.display()),
            AssetError::IO(absolute_path, err) =>
                write!(f, "unknown error when accessing `{}`: \n{}", absolute_path.display(), err)
        }
    }
}

impl AssetSource {
    /// Parse a string as a file source
    pub fn parse_file(path: &str) -> Result<Self, AssetError> {
        let myself = Self::parse_any(path)?;
        if let Self::Local(path) = &myself {
            if !path.is_file() {
                return Err(AssetError::NotFile(path.to_path_buf()));
            }
        }
        Ok(myself)
    }

    /// Parse a string as a folder source
    pub fn parse_folder(path: &str) -> Result<Self, AssetError> {
        let myself = Self::parse_any(path)?;
        if let Self::Local(path) = &myself {
            if !path.is_dir() {
                return Err(AssetError::NotFolder(path.to_path_buf()));
            }
        }
        Ok(myself)
    }

    /// Parse a string as a file or folder source
    pub fn parse_any(src: &str) -> Result<Self, AssetError> {
        match Url::parse(src) {
            Ok(url) => Ok(Self::Remote(url)),
            Err(_) => {
                let manifest_dir = manifest_dir();
                let path = PathBuf::from(src);
                // Paths are always relative to the manifest directory.
                // If the path is absolute, we need to make it relative to the manifest directory.
                let path = path
                    .strip_prefix(std::path::MAIN_SEPARATOR_STR)
                    .unwrap_or(&path);
                let path = manifest_dir.join(path);

                match path.canonicalize() {
                    Ok(x) => Ok(Self::Local(x)),
                    // relative path does not exist
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        Err(AssetError::NotFoundRelative(manifest_dir, src.into()))
                    }
                    // other error
                    Err(e) => Err(AssetError::IO(path, e)),
                }
            }
        }
    }
}

/// A folder asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FolderAsset {
    location: AssetLocation,
}

impl Display for FolderAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/**", self.location.source(),)
    }
}

impl FolderAsset {
    /// Creates a new folder asset
    pub fn new(source: AssetSource) -> Self {
        let AssetSource::Local(source) = source else {
            panic!("Folder asset must be a local path");
        };
        assert!(source.is_dir());

        let mut myself = Self {
            location: AssetLocation {
                unique_name: Default::default(),
                source: AssetSource::Local(source),
            },
        };

        myself.regenerate_unique_name();

        myself
    }

    /// Returns the location where the folder asset will be served from or None if the asset cannot be served
    pub fn served_location(&self) -> Result<String, ManganisSupportError> {
        resolve_asset_location(&self.location)
    }

    /// Returns the unique name of the folder asset
    pub fn unique_name(&self) -> &str {
        &self.location.unique_name
    }

    /// Returns the location of the folder asset
    pub fn location(&self) -> &AssetLocation {
        &self.location
    }

    /// Create a unique hash for the source folder by recursively hashing the files
    fn hash(&self) -> u64 {
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        let folder = self
            .location
            .source
            .as_path()
            .expect("Folder asset must be a local path");
        let mut folders_queued = vec![folder.clone()];
        while let Some(folder) = folders_queued.pop() {
            // Add the folder to the hash
            for segment in folder.iter() {
                segment.hash(&mut hash);
            }

            let files = std::fs::read_dir(folder).into_iter().flatten().flatten();
            for file in files {
                let path = file.path();
                let metadata = path.metadata().unwrap();
                // If the file is a folder, add it to the queue otherwise add it to the hash
                if metadata.is_dir() {
                    folders_queued.push(path);
                } else {
                    hash_file(&AssetSource::Local(path), &mut hash);
                }
            }
        }

        // Add the manganis version to the hash
        hash_version(&mut hash);

        hash.finish()
    }

    /// Regenerate the unique name of the folder asset
    fn regenerate_unique_name(&mut self) {
        let uuid = self.hash();
        let file_name = normalized_file_name(&self.location.source, None);
        self.location.unique_name = format!("{file_name}{uuid:x}");
        assert!(self.location.unique_name.len() <= MAX_PATH_LENGTH);
    }
}

/// A file asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FileAsset {
    location: AssetLocation,
    options: FileOptions,
    url_encoded: bool,
}

impl Display for FileAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url_encoded = if self.url_encoded {
            " [url encoded]"
        } else {
            ""
        };
        write!(
            f,
            "{} [{}]{}",
            self.location.source(),
            self.options,
            url_encoded
        )
    }
}

impl FileAsset {
    /// Creates a new file asset
    pub fn new(source: AssetSource) -> Self {
        if let Some(path) = source.as_path() {
            assert!(!path.is_dir());
        }

        let options = FileOptions::default_for_extension(source.extension().as_deref());

        let mut myself = Self {
            location: AssetLocation {
                unique_name: Default::default(),
                source,
            },
            options,
            url_encoded: false,
        };

        myself.regenerate_unique_name();

        myself
    }

    /// Set the file options
    pub fn with_options(self, options: FileOptions) -> Self {
        let mut myself = Self {
            location: self.location,
            options,
            url_encoded: false,
        };

        myself.regenerate_unique_name();

        myself
    }

    /// Set whether the file asset should be url encoded
    pub fn set_url_encoded(&mut self, url_encoded: bool) {
        self.url_encoded = url_encoded;
    }

    /// Returns whether the file asset should be url encoded
    pub fn url_encoded(&self) -> bool {
        self.url_encoded
    }

    /// Returns the location where the file asset will be served from or None if the asset cannot be served
    pub fn served_location(&self) -> Result<String, ManganisSupportError> {
        if self.url_encoded {
            let data = self.location.source.read_to_bytes().unwrap();
            let data = base64::engine::general_purpose::STANDARD_NO_PAD.encode(data);
            let mime = self.location.source.mime_type().unwrap();
            Ok(format!("data:{mime};base64,{data}"))
        } else {
            resolve_asset_location(&self.location)
        }
    }

    /// Returns the location of the file asset
    pub fn location(&self) -> &AssetLocation {
        &self.location
    }

    /// Returns the options for the file asset
    pub fn options(&self) -> &FileOptions {
        &self.options
    }

    /// Returns the options for the file asset mutably
    pub fn with_options_mut(&mut self, f: impl FnOnce(&mut FileOptions)) {
        f(&mut self.options);
        self.regenerate_unique_name();
    }

    /// Hash the file asset source and options
    fn hash(&self) -> u64 {
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        hash_file(&self.location.source, &mut hash);
        self.options.hash(&mut hash);
        hash_version(&mut hash);
        hash.finish()
    }

    /// Regenerates the unique name of the file asset
    fn regenerate_unique_name(&mut self) {
        // Generate an unique name for the file based on the options, source, and the current version of manganis
        let uuid = self.hash();
        let extension = self.options.extension();
        let file_name = normalized_file_name(&self.location.source, extension);
        let extension = extension.map(|e| format!(".{e}")).unwrap_or_default();
        self.location.unique_name = format!("{file_name}{uuid:x}{extension}");
        assert!(self.location.unique_name.len() <= MAX_PATH_LENGTH);
    }
}

/// Create a normalized file name from the source
fn normalized_file_name(location: &AssetSource, extension: Option<&str>) -> String {
    let last_segment = location.last_segment();
    let mut file_name = to_alphanumeric_string_lossy(last_segment);

    let extension_len = extension.map(|e| e.len() + 1).unwrap_or_default();
    let extension_and_hash_size = extension_len + HASH_SIZE;
    // If the file name is too long, we need to truncate it
    if file_name.len() + extension_and_hash_size > MAX_PATH_LENGTH {
        file_name = file_name[..MAX_PATH_LENGTH - extension_and_hash_size].to_string();
    }
    file_name
}

/// Normalize a string to only contain alphanumeric characters
fn to_alphanumeric_string_lossy(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

fn hash_file(location: &AssetSource, hash: &mut DefaultHasher) {
    // Hash the last time the file was updated and the file source. If either of these change, we need to regenerate the unique name
    let updated = location.last_updated();
    updated.hash(hash);
    location.hash(hash);
}

fn hash_version(hash: &mut DefaultHasher) {
    // Hash the current version of manganis. If this changes, we need to regenerate the unique name
    crate::built::PKG_VERSION.hash(hash);
    crate::built::GIT_COMMIT_HASH.hash(hash);
}

fn resolve_asset_location(location: &AssetLocation) -> Result<String, ManganisSupportError> {
    // If manganis is being used without CLI support, we will fallback to providing a local path.
    let manganis_support = std::env::var("MANGANIS_SUPPORT");
    if manganis_support.is_err() {
        match location.source() {
            AssetSource::Remote(url) => Ok(url.as_str().to_string()),
            AssetSource::Local(path) => {
                // If this is not the main package, we can't include assets from it without CLI support
                let primary_package = std::env::var("CARGO_PRIMARY_PACKAGE").is_ok();
                if !primary_package {
                    return Err(ManganisSupportError::ExternalPackageCollection);
                }

                // Tauri doesn't allow absolute paths(??) so we convert to relative.
                let Ok(cwd) = std::env::var("CARGO_MANIFEST_DIR") else {
                    return Err(ManganisSupportError::FailedToFindCargoManifest);
                };

                // Windows adds `\\?\` to longer path names. We'll try to remove it.
                #[cfg(windows)]
                let path = {
                    let path_as_string = path.display().to_string();
                    let path_as_string = path_as_string
                        .strip_prefix("\\\\?\\")
                        .unwrap_or(&path_as_string);
                    PathBuf::from(path_as_string)
                };

                let rel_path = path
                    .strip_prefix(cwd)
                    .map_err(|_| ManganisSupportError::FailedToFindCargoManifest)?;
                let path = PathBuf::from(".").join(rel_path);
                Ok(path.display().to_string())
            }
        }
    } else {
        let config = Config::current();
        let root = config.assets_serve_location();
        let unique_name = location.unique_name();
        Ok(format!("{root}{unique_name}"))
    }
}

/// An error that can occur while collecting assets without CLI support
#[derive(Debug)]
pub enum ManganisSupportError {
    /// An error that can occur while collecting assets from other packages without CLI support
    ExternalPackageCollection,
    /// Manganis failed to find the current package's manifest
    FailedToFindCargoManifest,
}

impl Display for ManganisSupportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExternalPackageCollection => write!(f, "Attempted to collect assets from other packages without a CLI that supports Manganis. Please recompile with a CLI that supports Manganis like the `dioxus-cli`."),
            Self::FailedToFindCargoManifest => write!(f, "Manganis failed to find the current package's manifest. Please recompile with a CLI that supports Manganis like the `dioxus-cli`."),
        }
    }
}

impl std::error::Error for ManganisSupportError {}

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

// // If manganis is being used without CLI support, we will fallback to providing a local path.
// else if manganis_support.is_err() {
//     match self.location.source() {
//         FileSource::Remote(url) => Ok(url.as_str().to_string()),
//         FileSource::Local(path) => {
//             // If this is not the main package, we can't include assets from it without CLI support
//             let primary_package = std::env::var("CARGO_PRIMARY_PACKAGE").is_ok();
//             if !primary_package {
//                 return Err(ManganisSupportError::ExternalPackageCollection);
//             }

//             // Tauri doesn't allow absolute paths(??) so we convert to relative.
//             let Ok(cwd) = std::env::var("CARGO_MANIFEST_DIR") else {
//                 return Err(ManganisSupportError::FailedToFindCargoManifest);
//             };

//             // Windows adds `\\?\` to longer path names. We'll try to remove it.
//             #[cfg(windows)]
//             let path = {
//                 let path_as_string = path.display().to_string();
//                 let path_as_string = path_as_string
//                     .strip_prefix("\\\\?\\")
//                     .unwrap_or(&path_as_string);
//                 PathBuf::from(path_as_string)
//             };

//             let rel_path = path.strip_prefix(cwd).unwrap();
//             let path = PathBuf::from(".").join(rel_path);
//             Ok(path.display().to_string())
//         }
//     }
// } else {
// let config = Config::current();
// let root = config.assets_serve_location();
// }
