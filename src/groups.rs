use crate::prelude::*;
use anyhow::Result;

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

/// A type representing a users group files with all their packages
#[derive(Default)]
pub struct Groups(BTreeMap<String, PackagesInstall>);

impl Groups {
    /// Convert to [`PackagesInstall`] using defaults for the backends' `InstallOptions`
    pub fn to_packages_install(&self) -> PackagesInstall {
        let mut packages = PackagesInstall::default();

        for group in self.values() {
            packages.append(&mut group.clone());
        }

        packages
    }

    /// Return a new, empty Group
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Loads Groups from a users pacdef config folder.
    ///
    /// # Errors
    ///  - If the Group config file cannot be found.
    pub fn load() -> Result<Self> {
        todo!()
    }
}

impl Deref for Groups {
    type Target = BTreeMap<String, PackagesInstall>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Groups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
