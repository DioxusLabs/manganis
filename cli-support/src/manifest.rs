pub use railwind::warning::Warning as TailwindWarning;
use std::path::{Path, PathBuf};

use cargo_lock::{
    dependency::{self, graph::NodeIndex},
    Lockfile,
};
use manganis_common::{
    cache::asset_cache_dir, cache::package_identifier, AssetManifest, AssetType, PackageAssets,
};
use petgraph::visit::EdgeRef;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    cache::{current_cargo_toml, lock_path},
    file::process_file,
};

/// An extension trait CLI support for the asset manifest
pub trait AssetManifestExt {
    /// Loads the asset manifest for the current working directory
    fn load() -> Self;

    /// Loads the asset manifest from the cargo toml and lock file
    fn load_from_path(cargo_toml: PathBuf, cargo_lock: PathBuf) -> Self;

    /// Copies all static assets to the given location
    fn copy_static_assets_to(&self, location: impl Into<PathBuf>) -> anyhow::Result<()>;

    /// Collects all tailwind classes from all assets and outputs the CSS file
    fn collect_tailwind_css(
        &self,
        include_preflight: bool,
        warnings: &mut Vec<TailwindWarning>,
    ) -> String;
}

impl AssetManifestExt for AssetManifest {
    fn load() -> Self {
        let lock_path = lock_path();
        let cargo_toml = current_cargo_toml();
        Self::load_from_path(cargo_toml, lock_path)
    }

    fn load_from_path(cargo_toml: PathBuf, cargo_lock: PathBuf) -> Self {
        let lockfile = Lockfile::load(cargo_lock).unwrap();

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

        Self::new(all_assets)
    }

    fn copy_static_assets_to(&self, location: impl Into<PathBuf>) -> anyhow::Result<()> {
        let location = location.into();
        std::fs::create_dir_all(&location)?;
        self.packages().par_iter().try_for_each(|package| {
            package.assets().par_iter().try_for_each(|asset| {
                if let AssetType::File(file_asset) = asset {
                    process_file(file_asset, &location)?;
                }
                Ok::<(), anyhow::Error>(())
            })?;
            Ok::<(), anyhow::Error>(())
        })?;

        Ok(())
    }

    fn collect_tailwind_css(
        self: &AssetManifest,
        include_preflight: bool,
        warnings: &mut Vec<TailwindWarning>,
    ) -> String {
        let mut all_classes = String::new();

        for package in self.packages() {
            for asset in package.assets() {
                if let AssetType::Tailwind(classes) = asset {
                    all_classes.push_str(classes.classes());
                    all_classes.push(' ');
                }
            }
        }

        let source = railwind::Source::String(all_classes, railwind::CollectionOptions::String);

        let css = railwind::parse_to_string(source, include_preflight, warnings);

        crate::file::minify_css(&css)
    }
}

fn collect_dependencies(
    tree: &cargo_lock::dependency::tree::Tree,
    root_package_id: NodeIndex,
    cache_dir: &Path,
    all_assets: &mut Vec<PackageAssets>,
) {
    let mut packages_to_visit = vec![root_package_id];
    while let Some(package_id) = packages_to_visit.pop(){
        let package = tree.graph().node_weight(package_id).unwrap();
        // Add the assets for this dependency
        let mut dependency_path = cache_dir.join(package_identifier(
            package.name.as_str(),
            &package.version.to_string(),
        ));
        dependency_path.push("assets.toml");
        if dependency_path.exists() {
            match std::fs::read_to_string(&dependency_path) {
                Ok(contents) => {
                    match toml::from_str(&contents) {
                        Ok(package_assets) => {
                            all_assets.push(package_assets);
                        }
                        Err(err) => {
                            log::error!("Failed to parse asset manifest for dependency: {}", err);
                        }
                    };
                }
                Err(err) => {
                    log::error!("Failed to read asset manifest for dependency: {}", err);
                }
            }
        }
    
        // Then recurse into its dependencies
        let dependencies = tree.graph().edges(package_id);
        for dependency in dependencies {
            let dependency_index = dependency.target();
            packages_to_visit.push(dependency_index);
        }
    }
}
