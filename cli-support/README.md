# Collect Assets CLI Support

This crate provides utilities to collect assets that integrate with the collect assets macro. It makes it easy to integrate an asset collection and optimization system into a build tool.

```rust
#![allow(unused)]
use assets_cli_support::AssetManifestExt;
use assets_common::{Config, AssetManifest};

fn main() {
    use std::process::Command;

    // This is the location where the assets will be copied to in the filesystem
    let assets_file_location = "./assets";
    // This is the location where the assets will be served from
    let assets_serve_location = "/assets";

    // First set any settings you need for the build
    let config = Config::default()
        .with_assets_serve_location(assets_serve_location).save();

    /// build the application
    Command::new("cargo")
        .args(["build"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Then collect the assets
    let manifest = AssetManifest::load();

    // And copy the static assets to the public directory
    manifest.copy_static_assets_to(assets_file_location).unwrap();

    // Then collect the tailwind CSS
    let css = manifest.collect_tailwind_css(true, &mut Vec::new());

    // And write the CSS to the public directory
    std::fs::write(format!("{}/tailwind.css", assets_file_location), css).unwrap();
}
```