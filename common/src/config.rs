use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::cache::config_path;

fn default_assets_serve_location() -> String {
    #[cfg(target_os = "wasm")]
    {
        "/".to_string()
    }
    #[cfg(not(target_os = "wasm"))]
    {
        "./assets/".to_string()
    }
}

/// The configuration for collecting assets
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    #[serde(default = "default_assets_serve_location")]
    assets_serve_location: String,
}

impl Config {
    /// The location where assets will be served from. On web applications, this should be the URL path to the directory where assets are served from.
    pub fn with_assets_serve_location(&self, assets_serve_location: impl Into<String>) -> Self {
        Self {
            assets_serve_location: assets_serve_location.into(),
        }
    }

    /// The location where assets will be served from. On web applications, this should be the URL path to the directory where assets are served from.
    pub fn assets_serve_location(&self) -> &str {
        &self.assets_serve_location
    }

    #[doc(hidden)]
    /// Returns the path to the config
    /// This is only used in the macro
    pub fn config_path() -> PathBuf {
        config_path()
    }

    /// Returns the current config
    pub fn current() -> Self {
        std::fs::read(config_path())
            .ok()
            .and_then(|config| toml::from_str(&String::from_utf8_lossy(&config)).ok())
            .unwrap_or_default()
    }

    /// Saves the config globally. This must be run before compiling the application you are collecting assets from.
    ///
    /// The assets macro will read the config from the global config file and set the assets serve location to the value in the config.
    pub fn save(&self) {
        let current = Self::current();
        if current == *self {
            return;
        }

        let config = toml::to_string(&self).unwrap();
        let config_path = config_path();
        std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        std::fs::write(config_path, config).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            assets_serve_location: default_assets_serve_location(),
        }
    }
}
