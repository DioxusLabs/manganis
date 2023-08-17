use serde::{Deserialize, Serialize};

use crate::cache::config_path;

fn default_assets_serve_location() -> String {
    "dist/".to_string()
}

/// The configuration for collecting assets
#[derive(Serialize, Deserialize, Debug)]
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
        let config = toml::to_string(&self).unwrap();
        std::fs::write(config_path(), config).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            assets_serve_location: default_assets_serve_location(),
        }
    }
}
