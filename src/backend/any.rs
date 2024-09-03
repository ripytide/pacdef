use crate::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

pub type PackageIds = BTreeSet<AnyPackageId>;
pub type InstallOptions = BTreeMap<AnyPackageId, AnyInstallOptions>;
pub type QueryInfos = BTreeMap<AnyPackageId, AnyQueryInfo>;
pub type RemoveOptions = BTreeMap<AnyPackageId, AnyRemoveOptions>;
pub type Modifications = BTreeMap<AnyPackageId, AnyModification>;

macro_rules! generate_anys {
    ($($backend:ident),*) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, strum::EnumIter,  derive_more::Display, derive_more::From)]
        pub enum AnyBackend {
            $(
                $backend,
            )*
        }
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, strum::EnumDiscriminants, derive_more::Display)]
        #[strum_discriminants(derive(derive_more::Display))]
        pub enum AnyPackageId {
            $(
                $backend(<$backend as Backend>::PackageId),
            )*
        }
        impl AnyPackageId {
            pub fn default_install_options(&self) -> AnyInstallOptions {
                match self {
                    $( AnyPackageId::$backend(_) => AnyInstallOptions::$backend(Default::default()), )*
                }
            }
            pub fn default_remove_options(&self) -> AnyRemoveOptions {
                match self {
                    $( AnyPackageId::$backend(_) => AnyRemoveOptions::$backend(Default::default()), )*
                }
            }
        }
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
