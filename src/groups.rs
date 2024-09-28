use crate::prelude::*;
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use toml::{Table, Value};
use walkdir::{DirEntry, WalkDir};

use std::{collections::BTreeMap, fs::read_to_string, path::Path};

#[derive(Debug, Default, derive_more::Deref, derive_more::DerefMut)]
pub struct Groups(BTreeMap<String, InstallOptions>);

impl Groups {
    pub fn to_install_options(&self) -> InstallOptions {
        let mut install_options = InstallOptions::default();

        for x in self.0.values() {
            install_options.append(&mut x.clone())
        }

        install_options
    }
    pub fn to_package_ids(&self) -> PackageIds {
        self.to_install_options().to_package_ids()
    }

    pub fn load(group_dir: &Path, hostname: &str, config: &Config) -> Result<Self> {
        let mut groups = Self::default();

        let group_dir = group_dir.join("groups/");
        if !group_dir.is_dir() {
            return Err(eyre!(
                "the groups directory was not found in the pacdef config folder, please create it"
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
                .ok_or(eyre!("will not fail on linux"))?
                .to_string();

            log::info!("parsing group file: {group_name}@{group_file:?}");

            let file_contents = read_to_string(group_file.path()).wrap_err("reading group file")?;

            let install_options: InstallOptions =
                parse_group_file(&group_name, &file_contents).wrap_err("parsing group file")?;

            groups.insert(group_name, install_options);
        }
        Ok(groups)
    }
}

fn parse_group_file(group_name: &str, contents: &str) -> Result<InstallOptions> {
    let mut install_options = InstallOptions::default();

    let toml = toml::from_str::<Table>(contents)?;

    for (key, value) in toml.iter() {
        install_options.append(&mut parse_toml_key_value(group_name, key, value)?);
    }

    Ok(install_options)
}

fn parse_toml_key_value(group_name: &str, key: &str, value: &Value) -> Result<InstallOptions> {
    macro_rules! x {
        ($($backend:ident),*) => {
            $(
                if key.to_lowercase() == $backend.to_string().to_lowercase() {
                    let mut install_options = InstallOptions::default();

                    let packages = value.as_array().ok_or(
                        eyre!("the {} backend in the {group_name} group toml file has a non-array value", $backend)
                    )?;

                    for package in packages {
                        let (package_id, package_install_options) = $backend::try_parse_toml_package(package)?;
                        install_options.$backend.insert(package_id, package_install_options);
                    }

                    return Ok(install_options);
                }
            )*
        };
    }
    apply_public_backends!(x);

    log::warn!("unrecognised backend: {key} in group file: {group_name}");

    Ok(InstallOptions::default())
}
