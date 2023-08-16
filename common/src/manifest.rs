use crate::package::PackageAssets;

/// A manifest of all assets collected from dependencies
#[derive(Debug, PartialEq, Default, Clone)]
pub struct AssetManifest {
    pub(crate) packages: Vec<PackageAssets>,
}

impl AssetManifest {
    /// Creates a new asset manifest
    pub fn new(packages: Vec<PackageAssets>) -> Self {
        Self { packages }
    }

    /// Returns all assets collected from dependencies
    pub fn packages(&self) -> &Vec<PackageAssets> {
        &self.packages
    }
}
