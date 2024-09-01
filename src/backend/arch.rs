use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::cmd::{command_found, run_args, run_args_for_stdout};
use crate::prelude::*;

#[derive(Debug, Clone, Copy, derive_more::Display)]
pub struct Arch {
    pub arch_package_manager: ArchPackageManager,
}

#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum ArchPackageManager {
    Paru,
    Pacman,
    Yay,
}
impl ArchPackageManager {
    fn as_command_str(&self) -> &str {
        match self {
            ArchPackageManager::Paru => "paru",
            ArchPackageManager::Pacman => "pacman",
            ArchPackageManager::Yay => "yay",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArchQueryInfo {
    explicit: bool,
}

pub struct ArchMakeImplicit;

#[derive(
    Debug, Clone, Default, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
struct ArchOptionalDeps(Vec<String>);

impl Backend for Arch {
    type PackageId = String;
    type RemoveOptions = ();
    type InstallOptions = ArchOptionalDeps;
    type QueryInfo = ArchQueryInfo;
    type Modification = ArchMakeImplicit;

    fn query_installed_packages(
        &self,
        _: &Config,
    ) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        if !command_found("pacman") {
            return Ok(BTreeMap::new());
        }

        let explicit = run_args_for_stdout(["pacman", "--query", "--explicit", "--quiet"])?;
        let dependency = run_args_for_stdout(["pacman", "--query", "--deps", "--quiet"])?;

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

    fn install_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        run_args(
            [self.arch_package_manager.as_str(), "--sync"]
                .into_iter()
                .chain(Some("--no_confirm").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
        )
    }

    fn modify_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::Modification>,
        config: &Config,
    ) -> Result<()> {
        run_args(
            [self.arch_package_manager.as_str(), "--database", "--asdeps"]
                .into_iter()
                .chain(packages.keys().map(String::as_str)),
        )
    }

    fn remove_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        run_args(
            [
                self.arch_package_manager.as_str(),
                "--remove",
                "--recursive",
            ]
            .into_iter()
            .chain(config.aur_rm_args.iter().map(String::as_str))
            .chain(Some("--no_confirm").filter(|_| no_confirm))
            .chain(packages.keys().map(String::as_str)),
        )
    }
}
