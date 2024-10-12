use color_eyre::eyre::Context;
use serde_inline_default::serde_inline_default;
use std::{collections::BTreeMap, path::Path};

use color_eyre::Result;
use serde::{Deserialize, Serialize};

// Update README if fields change.
#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde_inline_default(Config::default().arch_package_manager)]
    pub arch_package_manager: String,
    #[serde_inline_default(Config::default().arch_rm_args)]
    pub arch_rm_args: Vec<String>,
    #[serde_inline_default(Config::default().flatpak_systemwide)]
    pub flatpak_systemwide: bool,
    #[serde_inline_default(Config::default().disabled_backends)]
    pub disabled_backends: Vec<String>,
    #[serde_inline_default(Config::default().hostname_groups_enabled)]
    pub hostname_groups_enabled: bool,
    #[serde_inline_default(Config::default().hostname_groups)]
    pub hostname_groups: BTreeMap<String, Vec<String>>,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            arch_package_manager: "pacman".to_string(),
            arch_rm_args: Vec::new(),
            flatpak_systemwide: true,
            disabled_backends: Vec::new(),
            hostname_groups_enabled: false,
            hostname_groups: BTreeMap::new(),
        }
    }
}

impl Config {
    pub fn load(pacdef_dir: &Path) -> Result<Self> {
        let config_file_path = pacdef_dir.join("config.toml");

        if !config_file_path.is_file() {
            log::trace!(
                "no config file found at {config_file_path:?}, using default config instead"
            );

            Ok(Self::default())
        } else {
            toml::from_str(
                &std::fs::read_to_string(config_file_path).wrap_err("reading config file")?,
            )
            .wrap_err("parsing toml config")
        }
    }
}
