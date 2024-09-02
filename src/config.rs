use anyhow::anyhow;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// Update the master README if fields change.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Additional arguments to pass to `aur_helper` when removing a package.
    pub aur_rm_args: Vec<String>,
    /// Install Flatpak packages system-wide
    pub flatpak_systemwide: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            aur_rm_args: vec![],
            flatpak_systemwide: true,
        }
    }
}

impl Config {
    /// Load the config file from a users pacdef config folder.
    pub fn load(pacdef_dir: &Path) -> Result<Self> {
        let config_file_path = pacdef_dir.join("config.toml");

        if !config_file_path.is_file() {
            return Err(anyhow!("config file not found at: {config_file_path:?}"));
        }

        toml::from_str(&std::fs::read_to_string(config_file_path).context("reading config file")?)
            .context("parsing toml config")
    }
}
