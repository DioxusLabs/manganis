use crate::AssetType;

/// A manifest of all assets collected from dependencies
#[derive(Debug, PartialEq, Default, Clone)]
pub struct AssetManifest {
    pub(crate) assets: Vec<AssetType>,
}

impl AssetManifest {
    /// Creates a new asset manifest
    pub fn new(assets: Vec<AssetType>) -> Self {
        Self { assets }
    }

    /// Returns all assets collected from dependencies
    pub fn assets(&self) -> &Vec<AssetType> {
        &self.assets
    }

    //#[cfg(feature = "html")]
    /// Returns the HTML that should be injected into the head of the page
    pub fn head(&self) -> String {
        let mut head = String::new();
        for asset in &self.assets {
            if let crate::AssetType::File(file) = asset {
                match file.options() {
                    crate::FileOptions::Css(_) => {
                        let asset_path = file.served_location();
                        head.push_str(&format!(
                            "<link rel=\"stylesheet\" href=\"{asset_path}\">\n"
                        ))
                    }
                    crate::FileOptions::Image(image_options) => {
                        if image_options.preload() {
                            let asset_path = file.served_location();
                            head.push_str(&format!(
                                "<link rel=\"preload\" as=\"image\" href=\"{asset_path}\">\n"
                            ))
                        }
                    }
                    crate::FileOptions::Js(_) => {
                        let asset_path = file.served_location();
                        head.push_str(&format!("<script src=\"{asset_path}\"></script>\n"))
                    }
                    _ => {}
                }
            }
        }
        head
    }
}
