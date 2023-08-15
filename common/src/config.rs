use serde::{Deserialize, Serialize};

use crate::config_path;

fn default_assets_serve_location() -> String {
    "public".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_assets_serve_location")]
    assets_serve_location: String,
}

impl Config {
    pub fn with_assets_serve_location(&self, assets_serve_location: impl Into<String>) -> Self {
        Self {
            assets_serve_location: assets_serve_location.into(),
        }
    }

    pub fn assets_serve_location(&self) -> &str {
        &self.assets_serve_location
    }

    pub fn current() -> Self {
        std::fs::read(config_path())
            .ok()
            .and_then(|config| toml::from_str(&String::from_utf8_lossy(&config)).ok())
            .unwrap_or_default()
    }

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
