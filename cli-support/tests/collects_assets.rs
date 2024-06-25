use manganis_cli_support::{AssetManifestExt, ManganisSupportGuard};
use manganis_common::{AssetManifest, AssetType, Config};
use std::path::PathBuf;
use std::process::Stdio;

use std::process::Command;

#[test]
fn collects_assets() {
    tracing_subscriber::fmt::init();

    // This is the location where the assets will be served from
    let assets_serve_location = "/";

    // First set any settings you need for the build
    Config::default()
        .with_assets_serve_location(assets_serve_location)
        .save();

    // Next, tell manganis that you support assets
    let _guard = ManganisSupportGuard::default();

    // Find the test package directory which is up one directory from this package
    let mut test_package_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    test_package_dir.push("test-package");

    println!("running the CLI from {test_package_dir:?}");

    Config::default().save();

    // Then build your application
    let args = ["--release"]; //"--message-format=json-render-diagnostics",
    let mut command = Command::new("cargo")
        .arg("build")
        .args(args)
        .current_dir(&test_package_dir)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let reader = std::io::BufReader::new(command.stdout.take().unwrap());
    let path = get_executable_location(reader);

    // Then collect the assets
    let assets = AssetManifest::load(&path);

    let all_assets = assets.assets();

    println!("{:#?}", all_assets);

    let locations = all_assets
        .iter()
        .filter_map(|a| match a {
            AssetType::File(f) => Some(f.location()),
            _ => None,
        })
        .collect::<Vec<_>>();

    // Make sure the right number of assets were collected
    assert_eq!(locations.len(), 16);

    // Then copy the assets to a temporary directory and run the application
    let assets_dir = PathBuf::from("./assets");
    assets.copy_static_assets_to(assets_dir).unwrap();

    // Then run the application
    let status = Command::new(path).stdout(Stdio::piped()).status().unwrap();

    // Make sure the application exited successfully
    assert!(status.success());
}
