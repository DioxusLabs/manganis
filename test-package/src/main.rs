use assets_cli_support::*;

fn main() {
    Config::current()
        .with_assets_serve_location("/assets")
        .save();
    std::fs::remove_dir_all("./assets").unwrap_or_default();

    let class = assets::classes!("p-10");
    assert_eq!(class, "p-10");
    let path = assets::file!("./test-package-dependency/src/asset.txt");
    println!("{}", path);
    assert!(path.starts_with("/assets/asset"));
    let assets = AssetManifest::load();

    assert!(contains_tailwind_asset(&assets, "p-10"));
    for i in 1..=5 {
        let required = format!("flex flex-col p-{}", i);
        assert!(contains_tailwind_asset(&assets, &required));
    }
    for i in 1..=5 {
        let required = format!("flex flex-row p-{}", i);
        assert!(contains_tailwind_asset(&assets, &required));
    }

    println!("{:#?}", assets);

    let include_preflight = false;
    let mut warnings = Vec::new();
    let css = create_tailwind_css(&assets, include_preflight, &mut warnings);

    println!("{}", css);
    println!("{:#?}", warnings);

    assets.copy_static_assets_to("./assets").unwrap();
}

fn contains_tailwind_asset(assets: &AssetManifest, required_classes: &str) -> bool {
    for asset in assets.assets() {
        for asset in asset.assets() {
            if let AssetType::Tailwind(classes) = asset {
                if classes.classes() == required_classes {
                    return true;
                }
            }
        }
    }

    false
}
