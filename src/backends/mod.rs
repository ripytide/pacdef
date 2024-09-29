pub mod all;
pub mod apt;
pub mod arch;
pub mod cargo;
pub mod dnf;
pub mod flatpak;
pub mod pipx;
pub mod rustup;
pub mod xbps;

use std::collections::BTreeMap;

use crate::prelude::*;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

macro_rules! apply_public_backends {
    ($macro:ident) => {
        $macro! { Arch, Apt, Cargo, Dnf, Flatpak, Pipx, Rustup, Xbps }
    };
}
pub(crate) use apply_public_backends;

#[derive(Debug, Serialize, Deserialize)]
pub struct StringPackageStruct {
    pub package: String,
}

pub trait Backend {
    type PackageId;
    type QueryInfo;
    type InstallOptions;
    type ModificationOptions;
    type RemoveOptions;

    fn query_installed_packages(
        config: &Config,
    ) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>>;

    fn install_packages(
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()>;

    fn modify_packages(
        packages: &BTreeMap<Self::PackageId, Self::ModificationOptions>,
        config: &Config,
    ) -> Result<()>;

    fn remove_packages(
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()>;
}
