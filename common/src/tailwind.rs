pub use railwind::warning::Warning as TailwindWarning;

use crate::{AssetManifest, AssetType};

impl AssetManifest {
    pub fn tailwind_css(
        &self,
        include_preflight: bool,
        warnings: &mut Vec<TailwindWarning>,
    ) -> String {
        let mut all_classes = String::new();

        for package in &self.assets {
            for asset in package.assets() {
                if let AssetType::Tailwind(classes) = asset {
                    all_classes.push_str(classes.classes());
                    all_classes.push(' ');
                }
            }
        }

        let source = railwind::Source::String(all_classes, railwind::CollectionOptions::String);

        railwind::parse_to_string(source, include_preflight, warnings)
    }
}
