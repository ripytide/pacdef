use crate::prelude::*;
use anyhow::Result;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Pacman;
impl Pacman {
    const PACMAN: Arch = Arch { command: "pacman" };
}

impl Backend for Pacman {
    type PackageId = ArchPackageId;
    type QueryInfo = ArchQueryInfo;
    type InstallOptions = ArchInstallOptions;
    type Modification = ArchModification;
    type RemoveOptions = ArchRemoveOptions;

    fn query_installed_packages(
        &self,
        config: &Config,
    ) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        Self::PACMAN.query_installed_packages(config)
    }
    fn install_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        Self::PACMAN.install_packages(packages, no_confirm, config)
    }
    fn modify_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::Modification>,
        config: &Config,
    ) -> Result<()> {
        Self::PACMAN.modify_packages(packages, config)
    }
    fn remove_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        Self::PACMAN.remove_packages(packages, no_confirm, config)
    }
}
