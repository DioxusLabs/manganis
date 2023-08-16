use serde::{Deserialize, Serialize};

use crate::{
    asset::AssetType,
    cache::{current_package_cache_dir, current_package_identifier},
};

/// Clears all assets from the current package
pub fn clear_assets() {
    let dir = current_package_cache_dir();
    std::fs::remove_dir_all(dir).unwrap();
}

/// Adds an asset to the current package
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
                if file.location().source() == this_file.location().source()
                    && file.options() == this_file.options()
                {
                    this_file.set_unique_name(file.location().unique_name());
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

/// All assets collected from a specific package
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct PackageAssets {
    package: String,
    assets: Vec<AssetType>,
}

impl PackageAssets {
    /// Adds an asset to the package
    pub fn add(&mut self, asset: AssetType) {
        self.assets.push(asset);
    }

    /// Returns a reference to the package name
    pub fn package(&self) -> &str {
        &self.package
    }

    /// Returns a reference to the assets in this package
    pub fn assets(&self) -> &Vec<AssetType> {
        &self.assets
    }
}
