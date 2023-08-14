use std::path::Path;

use cargo_lock::{
    dependency::{self, graph::NodeIndex},
    Lockfile,
};
use petgraph::visit::EdgeRef;
pub use railwind::warning::Warning as TailwindWarning;

use crate::{
    cache::{asset_cache_dir, current_cargo_toml, lock_path, package_identifier},
    package::PackageAssets,
    AssetType,
};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct AssetManifest {
    assets: Vec<PackageAssets>,
}

impl AssetManifest {
    pub fn load() -> Self {
        let lock_path = lock_path();
        let lockfile = Lockfile::load(lock_path).unwrap();
        let cargo_toml = current_cargo_toml();

        let cargo_toml = cargo_toml::Manifest::from_path(cargo_toml).unwrap();
        let this_package = cargo_toml.package.unwrap();

        let mut all_assets = Vec::new();
        let cache_dir = asset_cache_dir();
        let tree = dependency::tree::Tree::new(&lockfile).unwrap();

        let Some(this_package_lock) = tree.roots().into_iter().find(|&p| {
            let package = tree.graph().node_weight(p).unwrap();
            package.name.as_str() == this_package.name
        }) else {
            return Self::default();
        };

        collect_dependencies(&tree, this_package_lock, &cache_dir, &mut all_assets);

        Self { assets: all_assets }
    }

    pub fn assets(&self) -> &Vec<PackageAssets> {
        &self.assets
    }

    pub fn tailwind_css(
        &self,
        include_preflight: bool,
        warnings: &mut Vec<TailwindWarning>,
    ) -> String {
        let mut all_classes = String::new();

        for package in &self.assets {
            for asset in package.assets() {
                if let AssetType::Tailwind(classes) = asset {
                    all_classes.push_str(classes.classes());
                    all_classes.push(' ');
                }
            }
        }

        let source = railwind::Source::String(all_classes, railwind::CollectionOptions::String);

        railwind::parse_to_string(source, include_preflight, warnings)
    }
}

fn collect_dependencies(
    tree: &cargo_lock::dependency::tree::Tree,
    package_id: NodeIndex,
    cache_dir: &Path,
    all_assets: &mut Vec<PackageAssets>,
) {
    let package = tree.graph().node_weight(package_id).unwrap();
    // Add the assets for this dependency
    let mut dependency_path = cache_dir.join(package_identifier(
        package.name.as_str(),
        &package.version.to_string(),
    ));
    dependency_path.push("assets.toml");
    if dependency_path.exists() {
        let contents = std::fs::read_to_string(&dependency_path).unwrap();
        let package_assets: PackageAssets = toml::from_str(&contents).unwrap();
        all_assets.push(package_assets);
    }

    // Then recurse into its dependencies
    let dependencies = tree.graph().edges(package_id);
    for dependency in dependencies {
        let dependency_index = dependency.target();
        collect_dependencies(tree, dependency_index, cache_dir, all_assets);
    }
}
