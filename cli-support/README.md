# Manganis CLI Support

This crate provides utilities to collect assets that integrate with the Manganis macro. It makes it easy to integrate an asset collection and optimization system into a build tool.

```rust, no_run
use manganis_cli_support::{AssetManifestExt, ManganisSupportGuard};
use manganis_common::{AssetManifest, Config};

fn main() {
    use std::process::Command;

    // This is the location where the assets will be copied to in the filesystem
    let assets_file_location = "./assets";
    // This is the location where the assets will be served from
    let assets_serve_location = "/assets";

    // First set any settings you need for the build
    Config::default()
        .with_assets_serve_location(assets_serve_location)
        .save();

    // Next, tell manganis that you support assets
    let _guard = ManganisSupportGuard::default();

    // Then build your application
    Command::new("cargo")
        .args(["build"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // the location where cargo built your binary.
    // you can use the `cargo-metadata` crate for this
    let executable_path = std::path::PathBuf::from("./target/debug/my_binary");

    // Then collect the assets
    let manifest = AssetManifest::load(&executable_path);

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
}
```
