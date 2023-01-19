use serde::Deserialize;

use std::{fs::File, io::Read};

fn default_ip() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u32 {
    8080
}

/// User-configurable settings.
#[derive(Deserialize, Clone)]
pub struct Settings {
    /// IP to bind to.
    #[serde(default = "default_ip")]
    pub ip: String,

    /// Port to listen on.
    #[serde(default = "default_port")]
    pub port: u32,
}

impl Settings {
    /// Load user settings from `config.toml`.
    pub fn new() -> Option<Self> {
        Self::from_file("config.toml")
    }

    /// Load user settings from the file at `path`.
    pub fn from_file(path: &str) -> Option<Self> {
        let mut settings_toml = String::new();
        if let Ok(mut file) = File::open(path) {
            file.read_to_string(&mut settings_toml).ok()?;
        }

        Self::from_str(&settings_toml)
    }

    /// Load user settings from a string.
    pub fn from_str(toml: &str) -> Option<Self> {
        toml::from_str(toml).ok()
    }
}
