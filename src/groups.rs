use crate::prelude::*;
use anyhow::{anyhow, Context, Result};
use path_absolutize::Absolutize;
use walkdir::WalkDir;

use std::{
    collections::BTreeMap,
    fs::{create_dir, read_to_string},
    ops::{Deref, DerefMut},
    path::Path,
};

/// A type representing a users group files with all their packages
#[derive(Debug, Default)]
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

    /// Loads Groups from a users pacdef config folder.
    ///
    /// # Errors
    ///  - If the Group config file cannot be found.
    pub fn load(group_dir: &Path) -> Result<Self> {
        let mut groups = Self::default();

        let group_dir = group_dir.join("groups/");
        if !group_dir.is_dir() {
            //other directories were already created with the config file
            create_dir(&group_dir).context("group directory doesn't exist, creating one")?;
        }

        let mut files = vec![];
        for file in WalkDir::new(&group_dir).follow_links(true) {
            let path = file?.path().absolutize_from(&group_dir)?.to_path_buf();
            files.push(path);
        }
        for group_file in files.iter().filter(|path| path.is_file()) {
            let group_name = group_file
                .strip_prefix(&group_dir)?
                .to_str()
                .ok_or(anyhow!("Will not fail on Linux"))?
                .to_string();
            println!("group_name: {group_name}");
            let file_contents = read_to_string(group_file).context("Reading group file")?;

            let packages: PackagesInstall = toml::from_str(&file_contents)?;
            groups.insert(group_name, packages);
        }
        Ok(groups)
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
