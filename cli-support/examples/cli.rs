use manganis_cli_support::{AssetManifestExt, ManganisSupportGuard};
use manganis_common::{AssetManifest, Config};
use std::process::Command;

// This is the location where the assets will be copied to in the filesystem
const ASSETS_FILE_LOCATION: &str = "./assets";

// This is the location where the assets will be served from
const ASSETS_SERVE_LOCATION: &str = "/assets";

fn main() {
    // Debug
    // let data = format!("{:?}", args);
    // fs::write("./link-args.txt", data).unwrap();

    // First set any settings you need for the build.
    Config::default()
        .with_assets_serve_location(ASSETS_SERVE_LOCATION)
        .save();

    // Next, tell manganis that you support assets
    let _guard = ManganisSupportGuard::default();

    // Now check if there is data from linker, otherwise start building the project.
    if let Some((working_dir, object_files)) =
        manganis_cli_support::linker_intercept(std::env::args())
    {
        // Extract the assets
        let assets = AssetManifest::load_from_objects(object_files);

        let assets_dir = working_dir.join(working_dir.join(ASSETS_FILE_LOCATION));

        // Remove the old assets
        let _ = std::fs::remove_dir_all(&assets_dir);

        // And copy the static assets to the public directory
        assets.copy_static_assets_to(&assets_dir).unwrap();

        // Then collect the tailwind CSS
        let css = assets.collect_tailwind_css(true, &mut Vec::new());

        // And write the CSS to the public directory
        let tailwind_path = assets_dir.join("tailwind.css");
        std::fs::write(tailwind_path, css).unwrap();
    } else {
        println!("Building!");
        build();
    }
}

fn build() {
    // Then build your application
    let current_dir = std::env::current_dir().unwrap();

    let args = ["--release"]; //"--message-format=json-render-diagnostics",
    Command::new("cargo")
        .current_dir(&current_dir)
        .arg("build")
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Call the helper function to intercept the Rust linker.
    manganis_cli_support::start_linker_intercept(Some(&current_dir), args).unwrap();
}
