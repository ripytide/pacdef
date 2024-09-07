use std::collections::{BTreeMap, BTreeSet};

use crate::prelude::*;
use anyhow::Result;

macro_rules! append {
    ($($backend:ident),*) => {
        pub fn append(&mut self, other: &mut Self) {
            $(
                self.$backend.append(&mut other.$backend);
            )*
        }
    };
}
macro_rules! is_empty {
    ($($backend:ident),*) => {
        pub fn is_empty(&self) ->bool {
            $(
                self.$backend.is_empty() &&
            )* true
        }
    };
}
macro_rules! difference {
    ($($backend:ident),*) => {
        pub fn difference(&self, other: &Self) -> Self {
            Self {
                $(
                    $backend: self.$backend.difference(&other.$backend).cloned().collect(),
                )*
            }
        }
    };
}
macro_rules! to_packages_ids {
    ($($backend:ident),*) => {
        pub fn to_packages_ids(&self) -> PackageIds {
            PackageIds {
                $(
                    $backend: self.$backend.keys().cloned().collect(),
                )*
            }
        }
    };
}

macro_rules! x {
    ($($backend:ident),*) => {
        #[derive(Debug, Clone, Default)]
        pub struct PackageIds {
            $(
                $backend: BTreeSet<<$backend as Backend>::PackageId>,
            )*
        }
        impl PackageIds {
            append!($($backend),*);
            is_empty!($($backend),*);
            difference!($($backend),*);

            pub fn to_install_options(self) -> InstallOptions {
                InstallOptions {
                    $(
                        $backend: self.$backend.iter().map(|x| (x.clone(), <$backend as Backend>::InstallOptions::default())).collect(),
                    )*
                }
            }
            pub fn to_remove_options(self) -> RemoveOptions {
                RemoveOptions {
                    $(
                        $backend: self.$backend.iter().map(|x| (x.clone(), <$backend as Backend>::RemoveOptions::default())).collect(),
                    )*
                }
            }
        }

        #[derive(Debug, Clone, Default)]
        pub struct QueryInfos {
            $(
                $backend: BTreeMap<<$backend as Backend>::PackageId, <$backend as Backend>::QueryInfo>,
            )*
        }
        impl QueryInfos {
            append!($($backend),*);
            is_empty!($($backend),*);
            to_packages_ids!($($backend),*);

            pub fn query_installed_packages(config: &Config) -> Result<Self> {
                Ok(Self {
                    $(
                        $backend: $backend.query_installed_packages(config)?,
                    )*
                })
            }
        }

        #[derive(Debug, Clone, Default)]
        pub struct InstallOptions {
            $(
                $backend: BTreeMap<<$backend as Backend>::PackageId, <$backend as Backend>::InstallOptions>,
            )*
        }
        impl InstallOptions {
            append!($($backend),*);
            is_empty!($($backend),*);
            to_packages_ids!($($backend),*);

            pub fn install_packages(self, no_confirm: bool, config: &Config) -> Result<()> {
                $(
                    $backend.install_packages(&self.$backend, no_confirm, config)?;
                )*

                Ok(())
            }
        }

        #[derive(Debug, Clone, Default)]
        pub struct Modifications {
            $(
                $backend: BTreeMap<<$backend as Backend>::PackageId, <$backend as Backend>::Modification>,
            )*
        }
        impl Modifications {
            append!($($backend),*);
            is_empty!($($backend),*);
            to_packages_ids!($($backend),*);

            pub fn modify_packages(self, config: &Config) -> Result<()> {
                $(
                    $backend.modify_packages(&self.$backend, config)?;
                )*

                Ok(())
            }
        }

        #[derive(Debug, Clone, Default)]
        pub struct RemoveOptions {
            $(
                $backend: BTreeMap<<$backend as Backend>::PackageId, <$backend as Backend>::RemoveOptions>,
            )*
        }
        impl RemoveOptions {
            append!($($backend),*);
            is_empty!($($backend),*);
            to_packages_ids!($($backend),*);

            pub fn remove_packages(self, no_confirm: bool, config: &Config) -> Result<()> {
                $(
                    $backend.remove_packages(&self.$backend, no_confirm, config)?;
                )*

                Ok(())
            }
        }
    };
}
#[allow(non_snake_case)]
apply_public_backends!(x);
