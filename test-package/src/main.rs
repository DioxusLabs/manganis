// The assets must be configured with the [CLI](cli-support/examples/cli.rs) before this example can be run.

use std::path::PathBuf;

use test_package_dependency::*;

const TEXT_FILE: &str = manganis::mg!("/test-package-dependency/src/asset.txt");
const VIDEO_FILE: &str = manganis::mg!("/test.mp4");

const ALL_ASSETS: &[&str] = &[
    VIDEO_FILE,
    TEXT_FILE,
    TEXT_ASSET,
    IMAGE_ASSET,
    HTML_ASSET,
    CSS_ASSET,
    PNG_ASSET,
    RESIZED_PNG_ASSET.path(),
    JPEG_ASSET.path(),
    RESIZED_JPEG_ASSET.path(),
    AVIF_ASSET.path(),
    RESIZED_AVIF_ASSET.path(),
    WEBP_ASSET.path(),
    RESIZED_WEBP_ASSET.path(),
    ROBOTO_FONT,
    COMFORTAA_FONT,
    ROBOTO_FONT_LIGHT_FONT,
    SCRIPT,
    DATA,
    FOLDER,
];

fn main() {
    tracing_subscriber::fmt::init();

    let external_paths_should_exist: bool = option_env!("MANGANIS_SUPPORT").is_some();

    // Make sure the macro paths match with the paths that actually exist
    for path in ALL_ASSETS {
        // Skip remote assets
        if path.starts_with("http") {
            continue;
        }
        let path = PathBuf::from(path);
        println!("{:?}", path);
        assert!(!external_paths_should_exist || path.exists());
    }
}
