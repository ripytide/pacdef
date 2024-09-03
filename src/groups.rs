use crate::prelude::*;
use anyhow::{anyhow, Context, Result};
use strum::IntoEnumIterator;
use walkdir::{DirEntry, WalkDir};

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
    path::Path,
};

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct Groups(BTreeMap<String, InstallOptions>);

pub type InstallOptions = BTreeSet<AnyInstallOptions>;

impl Groups {
    pub fn to_install_options(&self) -> InstallOptions {
        self.0.values().flatten().cloned().collect()
    }

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

            let install_options: InstallOptions =
                parse_group_file(&group_name, &file_contents).context("parsing group file")?;

            groups.insert(group_name, install_options);
        }
        Ok(groups)
    }
}

fn parse_group_file(group_name: &str, contents: &str) -> Result<InstallOptions> {
    let mut all_install_options = InstallOptions::default();

    let toml = toml::from_str::<toml::Table>(contents)?;

    for (key, value) in toml.iter() {
        match AnyBackend::iter().find(|x| x.to_string().to_lowercase() == key.to_lowercase()) {
            Some(backend) => {
                let install_options = value.as_array().context(
                    anyhow!("the {backend} backend in the {group_name} group toml file has a non-array value")
                )?;

                // let packages = packages.into_iter().map(|x|)
            }
            None => {
                log::warn!("unrecognised backend: {key} in group file: {group_name}");
            }
        }
    }

    for backend in AnyBackend::iter() {
        let backend_name = backend.to_string();

        if let Some(value) = toml.get(&backend_name) {}
    }

    Ok(all_install_options)
}
