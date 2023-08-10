use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn asset_cache_dir() -> PathBuf {
    let dir = std::env::var("CARGO_HOME").unwrap();
    let mut dir = PathBuf::from(dir);
    dir.push("assets");
    dir
}

fn current_package_identifier() -> String {
    std::env::var("CARGO_PKG_NAME").unwrap() + "-" + &current_package_version()
}

fn current_package_version() -> String {
    std::env::var("CARGO_PKG_VERSION").unwrap()
}

#[test]
fn env_variables() {
    assert_eq!(current_package_identifier(), "assets-0.1.0");
    asset_cache_dir().to_str().unwrap();
}

fn current_package_cache_dir() -> PathBuf {
    let mut dir = asset_cache_dir();
    dir.push(current_package_identifier());
    dir
}

pub fn add_asset(asset: AssetType) {
    let mut dir = current_package_cache_dir();
    dir.push("assets.toml");
    println!("Adding asset to {:?}", dir);
    // TODO: Clear this at some point
    let mut package_assets: PackageAssets = if dir.exists() {
        let contents = std::fs::read_to_string(&dir).unwrap();
        toml::from_str(&contents).unwrap()
    } else {
        std::fs::create_dir_all(&dir.parent().unwrap()).unwrap();
        PackageAssets {
            package: current_package_identifier(),
            assets: vec![],
        }
    };
    package_assets.assets.push(asset);
    let contents = toml::to_string(&package_assets).unwrap();
    std::fs::write(&dir, contents).unwrap();
}

// TODO: Filter this by dependencies found in your cargo.lock
pub fn all_assets() -> Vec<PackageAssets> {
    let dir = asset_cache_dir();
    let mut all_assets = Vec::new();
    for dir in std::fs::read_dir(&dir).unwrap() {
        let dir = dir.unwrap();
        let mut dir_path = dir.path();
        dir_path.push("assets.toml");
        if dir_path.exists() {
            let contents = std::fs::read_to_string(&dir_path).unwrap();
            let package_assets: PackageAssets = toml::from_str(&contents).unwrap();
            println!("Found assets for package {}", package_assets.package);
            all_assets.push(package_assets);
        }
    }
    all_assets
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageAssets {
    package: String,
    assets: Vec<AssetType>,
}

impl PackageAssets {
    pub fn package(&self) -> &str {
        &self.package
    }

    pub fn assets(&self) -> &Vec<AssetType> {
        &self.assets
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AssetType {
    File(FileAsset),
    Tailwind(TailwindAsset),
    Metadata(MetadataAsset),
}

#[derive(Serialize, Deserialize, Debug)]
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
}

#[derive(Serialize, Deserialize, Debug)]
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TailwindAsset {
    classes: String,
}

impl TailwindAsset {
    pub fn new(classes: &str) -> Self {
        Self {
            classes: classes.to_string(),
        }
    }
}
