// The assets must be configured with the [CLI](cli-support/examples/cli.rs) before this example can be run.

use std::path::PathBuf;
use test_package_dependency::IMAGE_ASSET;

fn main() {
    let class = collect_assets::classes!("p-10");
    assert_eq!(class, "p-10");
    let path = collect_assets::file!("./test-package-dependency/src/asset.txt");
    println!("{}", path);
    assert!(path.starts_with("/assets/asset"));
    assert!(IMAGE_ASSET.starts_with("/assets/rustacean"));
    let path = PathBuf::from(format!(".{IMAGE_ASSET}"));
    println!("{:?}", path);
    println!("contents: {:?}", std::fs::read(path).unwrap());
}
