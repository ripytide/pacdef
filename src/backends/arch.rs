use color_eyre::Result;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::collections::{BTreeMap, BTreeSet};

use crate::cmd::{command_found, run_command, run_command_for_stdout};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Arch;

#[derive(Debug, Clone)]
pub struct ArchQueryInfo {
    pub explicit: bool,
    pub dependencies: BTreeSet<String>,
}
impl PossibleQueryInfo for ArchQueryInfo {
    fn explicit(&self) -> Option<bool> {
        Some(self.explicit)
    }
}

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

        let packages = run_command_for_stdout(
            [
                &config.arch_package_manager,
                "--query",
                "--info",
            ],
            Perms::Same,
        )?;

        let mut packages: Vec<&str> = packages.split("\n\n").collect();
        //there's an expty double line at the end of the output
        packages.pop();

        let mut result = BTreeMap::new();

        for package in packages {
            let parts: BTreeMap<&str, &str> = package.lines().map(|x| {
                let (name, value) = x.split_once(":").unwrap();
                (name.trim(), value.trim())
            }).collect();

            let explicit = parts.get("Install Reason").unwrap().contains("Explicitly");
            let dependencies = parts.get("Depends On").unwrap().split_ascii_whitespace().map(|x| x.split_once(delimiter))

            ArchQueryInfo {
                explicit,
                dependencies,
            }
        }

        Ok(BTreeMap::default())
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
                .chain(config.arch_rm_args.iter().map(String::as_str))
                .chain(Some("--no_confirm").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
        )
    }
}
