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
        pub fn is_empty(&self) -> bool {
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
                inner: BTreeMap::from([
                    $( (AnyBackend::$backend, self.$backend.keys().cloned().collect()), )*
                ])
            }
        }
    };
}

macro_rules! any {
    ($($backend:ident),*) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, derive_more::FromStr, derive_more::Display)]
        pub enum AnyBackend {
            $($backend,)*
        }
    };
}
apply_public_backends!(any);

#[derive(Debug, Clone, Default, Serialize)]
#[serde(transparent)]
pub struct PackageIds {
    inner: BTreeMap<AnyBackend, BTreeSet<String>>,
}
impl PackageIds {
    pub fn simplified(mut self) -> Self {
        self.inner.retain(|_, x| !x.is_empty());
        self
    }

    pub fn append(&mut self, other: &mut Self) {
        for (backend, packages) in other.inner.iter_mut() {
            self.inner.entry(*backend).or_default().append(packages);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.values().all(|x| x.is_empty())
    }

    pub fn contains(&self, backend: AnyBackend, package: &String) -> bool {
        self.inner
            .get(&backend)
            .is_some_and(|x| x.contains(package))
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut output = Self::default();
        for (backend, packages) in self.inner.iter() {
            if let Some(other_packages) = other.inner.get(backend) {
                output.inner.insert(
                    *backend,
                    packages.difference(other_packages).cloned().collect(),
                );
            }
        }
        output
    }
}
impl std::fmt::Display for PackageIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (backend, packages) in self.inner.iter() {
            if !packages.is_empty() {
                writeln!(f, "[{backend}]")?;
                writeln!(
                    f,
                    "{}",
                    itertools::Itertools::intersperse(packages.iter().cloned(), "\n".to_string())
                        .collect::<String>()
                )?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
macro_rules! package_ids {
    ($($backend:ident),*) => {
        impl PackageIds {
            pub fn to_install_options(self) -> InstallOptions {
                InstallOptions {
                    $(
                        $backend: if let Some(packages) = self.inner.get(&AnyBackend::$backend) {
                            packages.iter().map(|x| (x.clone(), <$backend as Backend>::InstallOptions::default())).collect()
                        } else {
                            Default::default()
                        },
                    )*
                }
            }
            pub fn to_remove_options(self) -> RemoveOptions {
                RemoveOptions{
                    $(
                        $backend: if let Some(packages) = self.inner.get(&AnyBackend::$backend) {
                            packages.iter().map(|x| (x.clone(), <$backend as Backend>::RemoveOptions::default())).collect()
                        } else {
                            Default::default()
                        },
                    )*
                }
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
                pub $backend: BTreeMap<String, <$backend as Backend>::QueryInfo>,
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
                pub $backend: BTreeMap<String, <$backend as Backend>::InstallOptions>,
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
                pub $backend: BTreeMap<String, <$backend as Backend>::ModificationOptions>,
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
                pub $backend: BTreeMap<String, <$backend as Backend>::RemoveOptions>,
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
