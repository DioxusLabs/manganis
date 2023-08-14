use serde::{Deserialize, Serialize};

fn default_path() -> String {
    let mut path = std::env::current_dir().unwrap();
    path.push("collect_assets.toml");
    path.to_string_lossy().to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    assets_serve_location: String,
}

impl Config {
    pub fn with_assets_serve_location(&self, assets_serve_location: String) -> Self {
        Self {
            assets_serve_location,
        }
    }

    pub fn assets_serve_location(&self) -> &str {
        &self.assets_serve_location
    }

    pub fn current() -> Self {
        let environment_variable =
            std::env::var("ASSETS_CONFIG_PATH").unwrap_or_else(|_| default_path());
        let config_path = std::path::Path::new(&environment_variable);
        let config_file = std::fs::read_to_string(config_path).unwrap_or_default();
        toml::from_str(&config_file).unwrap_or_default()
    }

    pub fn save(&self) {
        let environment_variable =
            std::env::var("ASSETS_CONFIG").unwrap_or_else(|_| default_path());
        let config_path = std::path::Path::new(&environment_variable);
        let config_file = toml::to_string(&self).unwrap();
        std::fs::write(config_path, config_file).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            assets_serve_location: "assets".to_string(),
        }
    }
}
