use crate::prelude::*;
use anyhow::Result;
use std::collections::BTreeMap;

macro_rules! generate_anys {
    ($($backend:ident),*) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, strum::EnumIter, derive_more::Display, derive_more::From)]
        pub enum AnyBackend {
            $(
                $backend,
            )*
        }
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
        pub enum AnyPackageId {
            $(
                $backend(<$backend as Backend>::PackageId),
            )*
        }
        //todo rename all to match trait associated types
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum AnyInstallOptions {
            $(
                $backend(<$backend as Backend>::InstallOptions),
            )*
        }
        #[derive(Debug, Clone)]
        pub enum AnyQueryInfo {
            $(
                $backend(<$backend as Backend>::QueryInfo),
            )*
        }
        #[derive(Debug, Clone)]
        pub enum AnyRemoveOptions {
            $(
                $backend(<$backend as Backend>::RemoveOptions),
            )*
        }
        #[derive(Debug, Clone)]
        pub enum AnyModification {
            $(
                $backend(<$backend as Backend>::Modification),
            )*
        }
    };

}
generate_anys!(Apt, Cargo, Dnf, Flatpak, Pacman, Paru, Pip, Pipx, Rustup, Xbps, Yay);

impl Backend for AnyBackend {
    type PackageId = AnyPackageId;
    type InstallOptions = AnyInstallOptions;
    type RemoveOptions = AnyRemoveOptions;
    type QueryInfo = AnyQueryInfo;
    type Modification = AnyModification;

    fn query_installed_packages(
        &self,
        config: &Config,
    ) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        todo!()
    }

    fn install_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        todo!()
    }

    fn modify_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::Modification>,
        config: &Config,
    ) -> Result<()> {
        todo!()
    }

    fn remove_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        todo!()
    }
}
