pub use railwind::warning::Warning as TailwindWarning;
use std::path::{Path, PathBuf};

use manganis_common::{
    AssetManifest, AssetType
};

use crate::file::process_file;

use object::{File, Object, ObjectSection};
use std::fs;
use serde_json;

// get the text containing all the asset descriptions
// in the "link section" of the binary
fn get_string_manganis(file: &File) -> Option<String> {
    for section in file.sections() {
        match section.name() {
            Ok(n) if n == "manganis" => {
                let bytes = section.uncompressed_data().ok()?;
                return Some(std::str::from_utf8(&bytes).ok()?.to_string());
            }
            _ => {}
        }
    }
    None
}


/// An extension trait CLI support for the asset manifest
pub trait AssetManifestExt {
    /// Load a manifest from the assets in the executable at the given path
    /// The asset descriptions are stored inside the binary, in the link-section
    fn load(executable: &Path) -> Self;
    /// Optimize and copy all assets in the manifest to a folder
    fn copy_static_assets_to(&self, location: impl Into<PathBuf>) -> anyhow::Result<()>;
    /// Collect all tailwind classes and generate string with the output css
    fn collect_tailwind_css(
        &self,
        include_preflight: bool,
        warnings: &mut Vec<TailwindWarning>
    ) -> String;
}

impl AssetManifestExt for AssetManifest {
    fn load(executable: &Path) -> Self {
        let binary_data = fs::read(executable).unwrap();
        let file = object::File::parse(&*binary_data).unwrap();

        let manganis_data = get_string_manganis(&file).unwrap();

        tracing::info!("I found manganis data:\n```\n{manganis_data}\n```");

        let deserializer = serde_json::Deserializer::from_str(&manganis_data);
        let assets = deserializer
            .into_iter::<AssetType>()
            .map(|x| x.unwrap())
            .collect();

        Self::new(assets)
    }

    fn copy_static_assets_to(&self, location: impl Into<PathBuf>) -> anyhow::Result<()> {
        let location = location.into();
        match std::fs::create_dir_all(&location) {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("Failed to create directory for static assets: {}", err);
                return Err(err.into());
            }
        }

        self.assets().iter().try_for_each(|asset| {
            if let AssetType::File(file_asset) = asset {
                tracing::info!("Optimizing and bundling {}", file_asset);
                tracing::trace!("Copying asset from {:?} to {:?}", file_asset, location);
                match process_file(file_asset, &location) {
                    Ok(_) => {}
                    Err(err) => {
                        tracing::error!("Failed to copy static asset: {}", err);
                        return Err(err);
                    }
                }
            }
            Ok::<(), anyhow::Error>(())
        })
    }

    fn collect_tailwind_css(
        self: &AssetManifest,
        include_preflight: bool,
        warnings: &mut Vec<TailwindWarning>,
    ) -> String {
        let mut all_classes = String::new();

        for asset in self.assets() {
            if let AssetType::Tailwind(classes) = asset {
                all_classes.push_str(classes.classes());
                all_classes.push(' ');
            }
        }

        let source = railwind::Source::String(all_classes, railwind::CollectionOptions::String);

        let css = railwind::parse_to_string(source, include_preflight, warnings);

        crate::file::minify_css(&css)
    }
}
