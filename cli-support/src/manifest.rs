pub use railwind::warning::Warning as TailwindWarning;
use rustc_hash::FxHashSet;
use std::path::{Path, PathBuf};

use cargo_lock::{
    dependency::{self, graph::NodeIndex},
    Lockfile,
};
use manganis_common::{
    cache::asset_cache_dir, cache::push_package_cache_dir, AssetManifest, AssetType, PackageAssets,
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
            tracing::error!("Failed to find this package in the lock file");
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
    // First find any assets that do have assets. The vast majority of packages will not have any so we can rule them out quickly with a hashset before touching the filesystem
    let mut packages = FxHashSet::default();
    if let Ok(read_dir) = cache_dir.read_dir() {
        for path in read_dir {
            if let Ok(path) = path {
                if path.file_type().unwrap().is_dir() {
                    let file_name = path.file_name();
                    let package_name = file_name.to_string_lossy();
                    if let Some((package_name, _)) = package_name.rsplit_once("-") {
                        packages.insert(package_name.to_string());
                    }
                }
            }
        }
    }

    let mut packages_to_visit = vec![root_package_id];
    let mut dependency_path = PathBuf::new();
    while let Some(package_id) = packages_to_visit.pop() {
        let package = tree.graph().node_weight(package_id).unwrap();
        // First make sure this package has assets
        if !packages.contains(package.name.as_str()) {
            continue;
        }

        // Add the assets for this dependency
        dependency_path.clear();
        dependency_path.push(cache_dir);
        push_package_cache_dir(
            package.name.as_str(),
            &package.version,
            &mut dependency_path,
        );
        tracing::trace!("Looking for assets in {}", dependency_path.display());
        dependency_path.push("assets.toml");
        if dependency_path.exists() {
            match std::fs::read_to_string(&dependency_path) {
                Ok(contents) => {
                    match toml::from_str(&contents) {
                        Ok(package_assets) => {
                            all_assets.push(package_assets);
                        }
                        Err(err) => {
                            tracing::error!(
                                "Failed to parse asset manifest for dependency: {}",
                                err
                            );
                        }
                    };
                }
                Err(err) => {
                    tracing::error!("Failed to read asset manifest for dependency: {}", err);
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
