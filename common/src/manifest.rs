use crate::package::PackageAssets;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct AssetManifest {
    pub(crate) assets: Vec<PackageAssets>,
}

impl AssetManifest {
    pub fn new(assets: Vec<PackageAssets>) -> Self {
        Self { assets }
    }

    pub fn assets(&self) -> &Vec<PackageAssets> {
        &self.assets
    }
}
