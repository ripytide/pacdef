use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

use crate::prelude::*;
use color_eyre::Result;

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
macro_rules! to_package_ids {
    ($($backend:ident),*) => {
        pub fn to_package_ids(&self) -> PackageIds {
            PackageIds {
                $(
                    $backend: self.$backend.keys().cloned().collect(),
                )*
            }
        }
    };
}

macro_rules! package_ids {
    ($($backend:ident),*) => {
        #[derive(Debug, Clone, Default, Serialize)]
        #[allow(non_snake_case)]
        pub struct PackageIds {
            $(
                pub $backend: BTreeSet<<$backend as Backend>::PackageId>,
            )*
        }
        impl PackageIds {
            append!($($backend),*);
            is_empty!($($backend),*);

            pub fn difference(&self, other: &Self) -> Self {
                Self {
                    $(
                        $backend: self.$backend.difference(&other.$backend).cloned().collect(),
                    )*
                }
            }

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
        impl std::fmt::Display for PackageIds {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut lists: Vec<String> = Vec::new();

                $(
                    if !self.$backend.is_empty() {
                        lists.push(
                            format!("[{}]\n{}",
                                $backend,
                                itertools::Itertools::intersperse(
                                    self.$backend.iter().map(ToString::to_string),
                                    "\n".to_string()
                                ).collect::<String>()
                            )
                        );
                    }
                )*

                write!(f, "{}",
                    itertools::Itertools::intersperse(lists.into_iter(), "\n\n".to_string()).collect::<String>()
                )
            }
        }
    }
}
apply_public_backends!(package_ids);

macro_rules! query_infos {
    ($($backend:ident),*) => {
        #[derive(Debug, Clone, Default)]
        #[allow(non_snake_case)]
        pub struct QueryInfos {
            $(
                pub $backend: BTreeMap<<$backend as Backend>::PackageId, <$backend as Backend>::QueryInfo>,
            )*
        }
        impl QueryInfos {
            append!($($backend),*);
            is_empty!($($backend),*);
            to_package_ids!($($backend),*);

            pub fn query_installed_packages(config: &Config) -> Result<Self> {
                Ok(Self {
                    $(
                        $backend:
                            if is_enabled(&$backend.to_string(), config) {
                                $backend::query_installed_packages(config)?
                            } else {
                                Default::default()
                            },
                    )*
                })
            }
        }
    }
}
apply_public_backends!(query_infos);

macro_rules! install_options {
    ($($backend:ident),*) => {
        #[derive(Debug, Clone, Default)]
        #[allow(non_snake_case)]
        pub struct InstallOptions {
            $(
                pub $backend: BTreeMap<<$backend as Backend>::PackageId, <$backend as Backend>::InstallOptions>,
            )*
        }
        impl InstallOptions {
            append!($($backend),*);
            is_empty!($($backend),*);
            to_package_ids!($($backend),*);

            pub fn install_packages(self, no_confirm: bool, config: &Config) -> Result<()> {
                $(
                    if is_enabled(&$backend.to_string(), config) {
                        $backend::install_packages(&self.$backend, no_confirm, config)?;
                    }
                )*

                Ok(())
            }
        }
    }
}
apply_public_backends!(install_options);

macro_rules! modification_options {
    ($($backend:ident),*) => {
        #[derive(Debug, Clone, Default)]
        #[allow(non_snake_case)]
        pub struct ModificationOptions {
            $(
                pub $backend: BTreeMap<<$backend as Backend>::PackageId, <$backend as Backend>::ModificationOptions>,
            )*
        }
        impl ModificationOptions {
            append!($($backend),*);
            is_empty!($($backend),*);
            to_package_ids!($($backend),*);

            pub fn modify_packages(self, config: &Config) -> Result<()> {
                $(
                    if is_enabled(&$backend.to_string(), config) {
                        $backend::modify_packages(&self.$backend, config)?;
                    }
                )*

                Ok(())
            }
        }
    }
}
apply_public_backends!(modification_options);

macro_rules! remove_options {
    ($($backend:ident),*) => {
        #[derive(Debug, Clone, Default)]
        #[allow(non_snake_case)]
        pub struct RemoveOptions {
            $(
                pub $backend: BTreeMap<<$backend as Backend>::PackageId, <$backend as Backend>::RemoveOptions>,
            )*
        }
        impl RemoveOptions {
            append!($($backend),*);
            is_empty!($($backend),*);
            to_package_ids!($($backend),*);

            pub fn remove_packages(self, no_confirm: bool, config: &Config) -> Result<()> {
                $(
                    if is_enabled(&$backend.to_string(), config) {
                        $backend::remove_packages(&self.$backend, no_confirm, config)?;
                    }
                )*

                Ok(())
            }
        }
    };
}
apply_public_backends!(remove_options);

fn is_enabled(backend: &str, config: &Config) -> bool {
    !config
        .disabled_backends
        .iter()
        .any(|x| x.to_lowercase() == backend.to_lowercase())
}
