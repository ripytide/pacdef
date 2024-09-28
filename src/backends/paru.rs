use crate::prelude::*;
use color_eyre::Result;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Paru;
impl Paru {
    const PARU: Arch = Arch { command: "paru" };
}

impl Backend for Paru {
    type PackageId = ArchPackageId;
    type QueryInfo = ArchQueryInfo;
    type InstallOptions = ArchInstallOptions;
    type ModificationOptions = ArchModificationOptions;
    type RemoveOptions = ArchRemoveOptions;

    fn query_installed_packages(
        config: &Config,
    ) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        Self::PARU.query_installed_packages(config)
    }
    fn install_packages(
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        Self::PARU.install_packages(packages, no_confirm, config)
    }
    fn modify_packages(
        packages: &BTreeMap<Self::PackageId, Self::ModificationOptions>,
        config: &Config,
    ) -> Result<()> {
        Self::PARU.modify_packages(packages, config)
    }
    fn remove_packages(
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        Self::PARU.remove_packages(packages, no_confirm, config)
    }
    fn try_parse_toml_package(
        toml: &toml::Value,
    ) -> Result<(Self::PackageId, Self::InstallOptions)> {
        Self::PARU.try_parse_toml_package(toml)
    }
}
