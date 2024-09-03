pub mod apt;
pub mod arch;
pub mod cargo;
pub mod dnf;
pub mod flatpak;
pub mod pacman;
pub mod paru;
pub mod pip;
pub mod pipx;
pub mod rustup;
pub mod xbps;
pub mod yay;
pub mod any;

use std::collections::BTreeMap;

use crate::prelude::*;
use anyhow::Result;

pub trait Backend {
    type PackageId;
    type InstallOptions;
    type RemoveOptions;
    type QueryInfo;
    type Modification;

    fn query_installed_packages(
        &self,
        config: &Config,
    ) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>>;

    fn install_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()>;

    fn modify_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::Modification>,
        config: &Config,
    ) -> Result<()>;

    fn remove_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()>;
}
