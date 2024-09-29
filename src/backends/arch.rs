use color_eyre::eyre::eyre;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::collections::BTreeMap;

use crate::cmd::{command_found, run_command, run_command_for_stdout};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Arch;

pub type ArchPackageId = String;

#[derive(Debug, Clone)]
pub struct ArchQueryInfo {
    pub explicit: bool,
}

#[derive(Debug, Clone)]
pub struct ArchModificationOptions {
    pub make_implicit: bool,
}

#[serde_inline_default]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ArchInstallOptions {
    #[serde_inline_default(ArchInstallOptions::default().optional_deps)]
    pub optional_deps: Vec<ArchPackageId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchRemoveOptions {}

impl Arch {
    pub fn query_installed_packages(
        &self,
        config: &Config,
    ) -> Result<BTreeMap<ArchPackageId, ArchQueryInfo>> {
        if !command_found(&config.arch_package_manager) {
            return Ok(BTreeMap::new());
        }

        let explicit = run_command_for_stdout(
            [
                &config.arch_package_manager,
                "--query",
                "--explicit",
                "--quiet",
            ],
            Perms::Same,
        )?;
        let dependency = run_command_for_stdout(
            [&config.arch_package_manager, "--query", "--deps", "--quiet"],
            Perms::Same,
        )?;

        Ok(dependency
            .lines()
            .map(|x| (x.to_string(), ArchQueryInfo { explicit: false }))
            .chain(
                explicit
                    .lines()
                    .map(|x| (x.to_string(), ArchQueryInfo { explicit: true })),
            )
            .collect())
    }

    pub fn install_packages(
        &self,
        packages: &BTreeMap<ArchPackageId, ArchInstallOptions>,
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

    pub fn modify_packages(
        &self,
        packages: &BTreeMap<ArchPackageId, ArchModificationOptions>,
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

    pub fn remove_packages(
        &self,
        packages: &BTreeMap<ArchPackageId, ArchRemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        run_command(
            [&config.arch_package_manager, "--remove", "--recursive"]
                .into_iter()
                .chain(config.arch_rm_args.iter().map(String::as_str))
                .chain(Some("--no_confirm").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
        )
    }

    pub fn try_parse_toml_package(
        &self,
        toml: &toml::Value,
    ) -> Result<(ArchPackageId, ArchInstallOptions)> {
        match toml {
            toml::Value::String(x) => Ok((x.to_string(), Default::default())),
            toml::Value::Table(x) => Ok((
                x.clone().try_into::<StringPackageStruct>()?.package,
                x.clone().try_into()?,
            )),
            _ => Err(eyre!("arch packages must be either a string or a table")),
        }
    }
}
