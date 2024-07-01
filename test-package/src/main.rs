// The assets must be configured with the [CLI](cli-support/examples/cli.rs) before this example can be run.

use test_package_dependency::{
    AVIF_ASSET, COMFORTAA_FONT, CSS_ASSET, HTML_ASSET, IMAGE_ASSET, JPEG_ASSET, PNG_ASSET,
    RESIZED_AVIF_ASSET, RESIZED_JPEG_ASSET, RESIZED_PNG_ASSET, RESIZED_WEBP_ASSET, ROBOTO_FONT,
    ROBOTO_FONT_LIGHT_FONT, TEXT_ASSET, WEBP_ASSET,
};

const TEXT_FILE: &str = manganis::mg!(file("./test-package-dependency/src/asset.txt"));

const ALL_ASSETS: &[&str] = &[
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
];

fn main() {
    tracing_subscriber::fmt::init();

    let cwd = std::env::current_dir().unwrap();

    // Make sure the macro paths match with the paths that actually exist
    for path in ALL_ASSETS {
        let path = cwd.join(format!(".{path}"));
        println!("{}", path.display());
        assert!(path.exists());
    }
}
