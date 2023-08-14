use serde::{Deserialize, Serialize};

use crate::{
    asset::AssetType,
    cache::{current_package_cache_dir, current_package_identifier},
};

pub fn clear_assets() {
    let dir = current_package_cache_dir();
    std::fs::remove_dir_all(dir).unwrap();
}

pub fn add_asset(asset: AssetType) {
    let mut dir = current_package_cache_dir();
    dir.push("assets.toml");
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

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
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
