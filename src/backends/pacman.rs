use crate::prelude::*;
use color_eyre::Result;
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
    type ModificationOptions = ArchModificationOptions;
    type RemoveOptions = ArchRemoveOptions;

    fn query_installed_packages(
        config: &Config,
    ) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        Self::PACMAN.query_installed_packages(config)
    }
    fn install_packages(
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        Self::PACMAN.install_packages(packages, no_confirm, config)
    }
    fn modify_packages(
        packages: &BTreeMap<Self::PackageId, Self::ModificationOptions>,
        config: &Config,
    ) -> Result<()> {
        Self::PACMAN.modify_packages(packages, config)
    }
    fn remove_packages(
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        Self::PACMAN.remove_packages(packages, no_confirm, config)
    }
    fn try_parse_toml_package(
        toml: &toml::Value,
    ) -> Result<(Self::PackageId, Self::InstallOptions)> {
        Self::PACMAN.try_parse_toml_package(toml)
    }
}
