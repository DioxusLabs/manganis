use assets::{asset, AssetManifest};

fn main() {
    asset!("p-10");
    let assets = assets::AssetManifest::load();

    assert!(assets_contains_tailwind_asset(&assets, "p-10"));
    for i in 1..=5 {
        let required = format!("flex flex-col p-{}", i);
        assert!(assets_contains_tailwind_asset(&assets, &required));
    }
    for i in 1..=5 {
        let required = format!("flex flex-row p-{}", i);
        assert!(assets_contains_tailwind_asset(&assets, &required));
    }

    println!("{:#?}", assets);

    let include_preflight = false;
    let mut warnings = Vec::new();
    let css = assets.tailwind_css(include_preflight, &mut warnings);

    println!("{}", css);
    println!("{:#?}", warnings);
}

fn assets_contains_tailwind_asset(assets: &AssetManifest, required_classes: &str) -> bool {
    for asset in assets.assets() {
        for asset in asset.assets() {
            if let assets::AssetType::Tailwind(classes) = asset {
                if classes.classes() == required_classes {
                    return true;
                }
            }
        }
    }

    false
}
