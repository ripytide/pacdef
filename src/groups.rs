use crate::prelude::*;
use anyhow::{anyhow, Context, Result};
use strum::IntoEnumIterator;
use walkdir::{DirEntry, WalkDir};

use std::{fs::read_to_string, path::Path};

pub fn load_groups(group_dir: &Path) -> Result<BTreeSet<AnyPackageInstall>> {
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

        let packages: AnyPackageInstallOptions =
            parse_group_file(&group_name, &file_contents).context("parsing group file")?;

        groups.insert(group_name, packages);
    }
    Ok(groups)
}

fn parse_group_file(group_name: &str, contents: &str) -> Result<AnyPackageInstallOptions> {
    let mut packages_install = AnyPackageInstallOptions::default();

    let toml = toml::from_str::<toml::Table>(contents)?;

    for (key, value) in toml.iter() {
        match AnyBackend::iter().find(|x| x.to_string().to_lowercase() == key.to_lowercase()) {
            Some(backend) => {
                let packages = value.as_array().context(
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

    Ok(packages_install)
}
