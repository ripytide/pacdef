use color_eyre::Result;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::collections::BTreeMap;

use crate::cmd::{command_found, run_command, run_command_for_stdout};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Arch;

#[derive(Debug, Clone)]
pub struct ArchQueryInfo {}

#[serde_inline_default]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ArchInstallOptions {
    #[serde_inline_default(ArchInstallOptions::default().optional_deps)]
    pub optional_deps: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ArchModificationOptions {
    pub make_implicit: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchRemoveOptions {}

impl Backend for Arch {
    type QueryInfo = ArchQueryInfo;
    type InstallOptions = ArchInstallOptions;
    type ModificationOptions = ArchModificationOptions;
    type RemoveOptions = ArchRemoveOptions;

    fn query_installed_packages(config: &Config) -> Result<BTreeMap<String, Self::QueryInfo>> {
        if !command_found(&config.arch_package_manager) {
            return Ok(BTreeMap::new());
        }

        let explicit_packages = run_command_for_stdout(
            [
                &config.arch_package_manager,
                "--query",
                "--explicit",
                "--quiet",
            ],
            Perms::Same,
        )?;

        let mut result = BTreeMap::new();

        for package in explicit_packages.lines() {
            result.insert(package.to_string(), ArchQueryInfo {});
        }

        Ok(result)
    }

    fn install_packages(
        packages: &BTreeMap<String, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        run_command(
            [&config.arch_package_manager, "--sync"]
                .into_iter()
                .chain(Some("--no_confirm").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str))
                .chain(packages.values().flat_map(|dependencies| {
                    dependencies.optional_deps.iter().map(String::as_str)
                })),
            Perms::AsRoot,
        )
    }

    fn modify_packages(
        packages: &BTreeMap<String, Self::ModificationOptions>,
        config: &Config,
    ) -> Result<()> {
        run_command(
            [&config.arch_package_manager, "--database", "--asdeps"]
                .into_iter()
                .chain(
                    packages
                        .iter()
                        .filter(|(_, m)| m.make_implicit)
                        .map(|(p, _)| p.as_str()),
                ),
            Perms::AsRoot,
        )
    }

    fn remove_packages(
        packages: &BTreeMap<String, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        run_command(
            [&config.arch_package_manager, "--remove", "--recursive"]
                .into_iter()
                .chain(Some("--no_confirm").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
        )
    }
}
