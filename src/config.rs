use anyhow::anyhow;
use serde_inline_default::serde_inline_default;
use std::{collections::BTreeMap, path::Path};

use anyhow::{Context, Result};
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
            return Err(anyhow!("config file not found at: {config_file_path:?}"));
        }

        dbg!(toml::from_str(
            &std::fs::read_to_string(config_file_path).context("reading config file")?
        )
        .context("parsing toml config"))
    }
}
