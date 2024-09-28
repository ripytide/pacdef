use color_eyre::eyre::{eyre, Context};
use serde_inline_default::serde_inline_default;
use std::{collections::BTreeMap, path::Path};

use color_eyre::Result;
use serde::{Deserialize, Serialize};

// Update README if fields change.
#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub aur_rm_args: Vec<String>,
    #[serde_inline_default(true)]
    pub flatpak_systemwide: bool,
    #[serde(default)]
    pub hostnames: BTreeMap<String, Vec<String>>,
}

impl Config {
    pub fn load(pacdef_dir: &Path) -> Result<Self> {
        let config_file_path = pacdef_dir.join("config.toml");

        if !config_file_path.is_file() {
            return Err(eyre!("config file not found at: {config_file_path:?}"));
        }

        toml::from_str(&std::fs::read_to_string(config_file_path).wrap_err("reading config file")?)
            .wrap_err("parsing toml config")
    }
}
