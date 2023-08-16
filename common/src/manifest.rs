use crate::package::PackageAssets;

/// A manifest of all assets collected from dependencies
#[derive(Debug, PartialEq, Default, Clone)]
pub struct AssetManifest {
    pub(crate) assets: Vec<PackageAssets>,
}

impl AssetManifest {
    /// Creates a new asset manifest
    pub fn new(assets: Vec<PackageAssets>) -> Self {
        Self { assets }
    }

    /// Returns all assets collected from dependencies
    pub fn assets(&self) -> &Vec<PackageAssets> {
        &self.assets
    }
}
