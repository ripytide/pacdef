use crate::prelude::*;
use anyhow::{anyhow, Context, Result};
use walkdir::{DirEntry, WalkDir};

use std::{collections::BTreeMap, fs::read_to_string, path::Path};

/// A type representing a users group files with all their packages
#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
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

    /// Loads [`Groups`] from a users pacdef config folder.
    pub fn load(group_dir: &Path) -> Result<Self> {
        let mut groups = Self::default();

        let group_dir = group_dir.join("groups/");
        if !group_dir.is_dir() {
            return Err(anyhow!(
                "The groups directory was not found in the pacdef config folder, please create it"
            ));
        }

        let group_files: Vec<DirEntry> = WalkDir::new(&group_dir)
            .follow_links(true)
            .into_iter()
            .collect::<Result<_, _>>()?;

        for group_file in group_files.iter().filter(|path| path.path().is_file()) {
            let group_name = group_file
                .path()
                .strip_prefix(&group_dir)?
                .to_str()
                .ok_or(anyhow!("Will not fail on Linux"))?
                .to_string();

            log::info!("parsing group file: {group_name}@{group_file:?}");

            let file_contents = read_to_string(group_file.path()).context("Reading group file")?;

            let packages: PackagesInstall = toml::from_str(&file_contents)?;
            groups.insert(group_name, packages);
        }
        Ok(groups)
    }
}
