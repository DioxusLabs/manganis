use assets_common::{AssetManifest, AssetType};
pub use railwind::warning::Warning as TailwindWarning;

pub fn create_tailwind_css(
    manifest: &AssetManifest,
    include_preflight: bool,
    warnings: &mut Vec<TailwindWarning>,
) -> String {
    let mut all_classes = String::new();

    for package in manifest.assets() {
        for asset in package.assets() {
            if let AssetType::Tailwind(classes) = asset {
                all_classes.push_str(classes.classes());
                all_classes.push(' ');
            }
        }
    }

    let source = railwind::Source::String(all_classes, railwind::CollectionOptions::String);

    let css = railwind::parse_to_string(source, include_preflight, warnings);

    crate::file::minify_css(&css)
}
