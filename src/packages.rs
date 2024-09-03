use anyhow::Result;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

use crate::prelude::*;

macro_rules! append {
    ($($name:ident)*) => {
        pub fn append(&mut self, other: &mut Self) {
            for (backend, packages) in self.0.iter_mut() {
                if let Some(other_packages) = other.0.get_mut(backend) {
                    packages.append(other_packages)
                }
            }
        }
    };
}
macro_rules! is_empty {
    ($($name:ident)*) => {
        pub fn is_empty(&self) -> bool {
            self.0.values().all(|x| x.is_empty())
        }
    };
}
macro_rules! into_packages_ids {
    ($($name:ident)*) => {
        pub fn into_packages_ids(self) -> PackagesIds {
            PackageIds(
                self.0
                    .iter()
                    .map(|(x, y)| (*x, y.into_keys().collect()))
                    .collect(),
            )
        }
    };
}

#[derive(Debug, Clone, Default)]
pub struct PackageIds(BTreeMap<AnyBackend, BTreeSet<AnyPackageId>>);
impl PackageIds {
    append!();
    is_empty!();

    pub fn difference(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter_map(|(x, y)| match other.0.get(x) {
                    Some(z) => Some((x, y, z)),
                    None => None,
                })
                .map(|(x, y, z)| (*x, y.difference(z).cloned().collect()))
                .collect(),
        )
    }

    pub fn insert(&mut self, backend: AnyBackend, package_id: AnyPackageId) {
        self.0.entry(backend).or_default().insert(package_id);
    }

    pub fn missing(groups: &Groups, config: &Config) -> Result<Self> {
        let requested = groups.to_packages_install();

        let installed = PackagesQuery::installed(config)?;

        let missing = requested
            .into_packages_ids()
            .difference(&installed.into_packages_ids());

        Ok(missing)
    }
    pub fn unmanaged(groups: &Groups, config: &Config) -> Result<Self> {
        let requested = groups.to_packages_install();

        let installed = PackagesQuery::installed(config)?;

        Ok(installed
            .into_packages_ids()
            .difference(&requested.into_packages_ids()))
    }
}
impl Display for PackageIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (backend, packages) in self.0.iter() {
            write!(
                f,
                "{backend}:\n{}",
                itertools::Itertools::intersperse(
                    packages.iter().map(|x| x.to_string()),
                    "\n".to_string()
                )
                .collect::<String>()
            );
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct PackagesInstall(BTreeMap<AnyBackend, BTreeMap<AnyPackageId, AnyPackageInstall>>);
impl PackagesInstall {
    append!();
    is_empty!();
    into_packages_ids!();

            pub fn install(self, no_confirm: bool, config: &Config) -> Result<()> {
                self.0.iter(|(x, y)| x.install_packages());
                Ok(())
                    $( .and($backend::install_packages(&self.$name, no_confirm, config)) )*
            }

            pub fn from_packages_ids_defaults(packages_ids: &PackagesIds) -> Self {
                Self {
                    $( $name: packages_ids.$name.iter().map(|x| (x.clone(), <$backend as Backend>::InstallOptions::default())).collect(), )*
                }
            }

            pub fn from_toml(toml_packages: BTreeMap<AnyBackend, Vec<toml::Value>>) -> Result<Self> {
                Ok(Self {
                    $( $name: toml_packages.get(&$backend.into()).into_iter().flatten().map(|x| <$backend as Backend>::package_from_toml(x)), )*
                })
            }
}

#[derive(Debug, Clone, Default)]
pub struct PackagesQuery(BTreeMap<AnyBackend, BTreeMap<AnyPackageId, AnyPackageQuery>>);
impl PackagesQuery {
    append!();
    is_empty!();
    into_packages_ids!();

            pub fn installed(config: &Config) -> Result<Self> {
                Ok(Self {
                    $( $name: $backend::query_installed_packages(config)?, )*
                })
            }
}

#[derive(Debug, Clone, Default)]
pub struct PackagesRemove(BTreeMap<AnyBackend, BTreeMap<AnyPackageId, AnyPackageRemove>>);
impl PackagesRemove {
    append!();
    is_empty!();
    into_packages_ids!();

            pub fn remove(self, no_confirm: bool, config: &Config) -> Result<()> {
                Ok(())
                    $( .and($backend::remove_packages(&self.$name, no_confirm, config)) )*
            }

            pub fn from_packages_ids_defaults(packages_ids: &PackagesIds) -> Self {
                Self {
                    $( $name: packages_ids.$name.iter().map(|x| (x.clone(), <$backend as Backend>::RemoveOptions::default())).collect(), )*
                }
            }
}

