pub use railwind::warning::Warning as TailwindWarning;
use std::path::{Path, PathBuf};

use manganis_common::{linker, AssetManifest, AssetType};

use crate::file::process_file;

use object::{File, Object, ObjectSection};
use std::fs;

// get the text containing all the asset descriptions
// in the "link section" of the binary
fn get_string_manganis(file: &File) -> Option<String> {
    for section in file.sections() {
        if let Ok(linker::SECTION_NAME) = section.name() {
            let bytes = section.uncompressed_data().ok()?;
            // Some platforms (e.g. macOS) start the manganis section with a null byte, we need to filter that out before we deserialize the JSON
            return Some(
                std::str::from_utf8(&bytes)
                    .ok()?
                    .chars()
                    .filter(|c| !c.is_control())
                    .collect::<String>(),
            );
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
        warnings: &mut Vec<TailwindWarning>,
    ) -> String;
}

fn deserialize_assets(json: &str) -> Vec<AssetType> {
    let deserializer = serde_json::Deserializer::from_str(json);
    deserializer
        .into_iter::<AssetType>()
        .map(|x| x.unwrap())
        .collect()
}

impl AssetManifestExt for AssetManifest {
    fn load(executable: &Path) -> Self {
        let binary_data = fs::read(executable).unwrap();
        let file = object::File::parse(&*binary_data).unwrap();

        let Some(manganis_data) = get_string_manganis(&file) else {
            // If we don't see any manganis assets used in the binary, just return an empty manifest
            return Self::default();
        };

        let assets = deserialize_assets(&manganis_data);

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
