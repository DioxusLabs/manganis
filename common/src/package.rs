use serde::{Deserialize, Serialize};

use crate::{
    asset::AssetType,
    cache::{current_package_cache_dir, current_package_identifier},
};

pub fn clear_assets() {
    let dir = current_package_cache_dir();
    std::fs::remove_dir_all(dir).unwrap();
}

pub fn add_asset(mut asset: AssetType) -> AssetType {
    let mut dir = current_package_cache_dir();
    dir.push("assets.toml");
    let mut package_assets: PackageAssets = if dir.exists() {
        let contents = std::fs::read_to_string(&dir).unwrap();
        toml::from_str(&contents).unwrap_or_else(|_| PackageAssets {
            package: current_package_identifier(),
            assets: vec![],
        })
    } else {
        std::fs::create_dir_all(&dir.parent().unwrap()).unwrap();
        PackageAssets {
            package: current_package_identifier(),
            assets: vec![],
        }
    };

    // Deduplicate any files
    let mut add_asset = true;
    if let AssetType::File(this_file) = &mut asset {
        for asset in package_assets.assets() {
            if let AssetType::File(file) = asset {
                // If there is another file in the same package with the same path, use that instead
                if file.path() == this_file.path() && file.options() == this_file.options() {
                    this_file.set_unique_name(file.unique_name());
                    add_asset = false;
                }
            }
        }
    }

    if add_asset {
        package_assets.add(asset.clone());
        let contents = toml::to_string(&package_assets).unwrap();
        std::fs::write(&dir, contents).unwrap();
    }

    asset
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct PackageAssets {
    package: String,
    assets: Vec<AssetType>,
}

impl PackageAssets {
    pub fn add(&mut self, asset: AssetType) {
        if !self.assets.contains(&asset) {
            self.assets.push(asset);
        }
    }

    pub fn package(&self) -> &str {
        &self.package
    }

    pub fn assets(&self) -> &Vec<AssetType> {
        &self.assets
    }
}
