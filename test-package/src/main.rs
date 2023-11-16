// The assets must be configured with the [CLI](cli-support/examples/cli.rs) before this example can be run.

use manganis_cli_support::AssetManifestExt;
use manganis_common::{AssetManifest, Config};
use std::path::PathBuf;
use test_package_dependency::IMAGE_ASSET;

fn main() {
    tracing_subscriber::fmt::init();

    println!("low quality preview: {:?}", test_package_dependency::AVIF_ASSET.preview());
    assert!(test_package_dependency::AVIF_ASSET.preview().is_some());

    // This is the location where the assets will be copied to in the filesystem
    let assets_file_location = "./dist/";
    // This is the location where the assets will be served from
    let assets_serve_location = "dist/";

    // First set any settings you need for the build
    Config::default()
        .with_assets_serve_location(assets_serve_location)
        .save();

    // Then collect the assets
    let manifest = AssetManifest::load();

    // Remove the old assets
    let _ = std::fs::remove_dir_all(assets_file_location);

    // And copy the static assets to the public directory
    manifest
        .copy_static_assets_to(assets_file_location)
        .unwrap();

    // Then collect the tailwind CSS
    let css = manifest.collect_tailwind_css(true, &mut Vec::new());

    // And write the CSS to the public directory
    std::fs::write(format!("{}/tailwind.css", assets_file_location), css).unwrap();

    let class = manganis::classes!("p-10");
    assert_eq!(class, "p-10");
    let path = manganis::file!("./test-package-dependency/src/asset.txt");
    println!("{}", path);
    assert!(path.starts_with("dist/asset"));
    println!("{}", IMAGE_ASSET);
    assert!(IMAGE_ASSET.starts_with("dist/rustacean"));
    let path = PathBuf::from(format!("./{IMAGE_ASSET}"));
    println!("{:?}", path);
    println!("contents: {:?}", std::fs::read(path).unwrap());
}
