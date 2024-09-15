use std::collections::BTreeMap;
use std::io::ErrorKind::NotFound;

use anyhow::anyhow;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use serde_json::Value;

use crate::cmd::{command_found, run_command};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Cargo;

#[serde_inline_default]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CargoInstallOptions {
    version: Option<String>,
    git: Option<String>,
    #[serde_inline_default(CargoInstallOptions::default().all_features)]
    all_features: bool,
    #[serde_inline_default(CargoInstallOptions::default().no_default_features)]
    no_default_features: bool,
    #[serde_inline_default(CargoInstallOptions::default().features)]
    features: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CargoQueryInfo {
    version: String,
    git: Option<String>,
    all_features: bool,
    no_default_features: bool,
    features: Vec<String>,
}

impl Backend for Cargo {
    type PackageId = String;
    type QueryInfo = CargoQueryInfo;
    type InstallOptions = CargoInstallOptions;
    type ModificationOptions = ();
    type RemoveOptions = ();

    fn query_installed_packages(_: &Config) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        if !command_found("cargo") {
            return Ok(BTreeMap::new());
        }

        let file = home::cargo_home()
            .context("getting the cargo home directory")?
            .join(".crates2.json");

        let contents = match std::fs::read_to_string(file) {
            Ok(string) => string,
            Err(err) if err.kind() == NotFound => {
                log::warn!("no crates file found for cargo. Assuming no crates installed yet.");
                return Ok(BTreeMap::new());
            }
            Err(err) => bail!(err),
        };

        extract_packages(&contents).context("extracting packages from crates file")
    }

    fn install_packages(
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        _: bool,
        _: &Config,
    ) -> Result<()> {
        for (package, options) in packages {
            run_command(
                ["cargo", "install"]
                    .into_iter()
                    .chain(Some("--git").into_iter().filter(|_| options.git.is_some()))
                    .chain(options.git.as_deref())
                    .chain(
                        Some("--all-features")
                            .into_iter()
                            .filter(|_| options.all_features),
                    )
                    .chain(
                        Some("--no-default-features")
                            .into_iter()
                            .filter(|_| options.no_default_features),
                    )
                    .chain(
                        Some("--features")
                            .into_iter()
                            .filter(|_| !options.features.is_empty()),
                    )
                    .chain(options.features.iter().map(|feature| feature.as_str()))
                    .chain([package.as_str()]),
                Perms::AsRoot,
            )?;
        }
        Ok(())
    }

    fn modify_packages(
        _: &BTreeMap<Self::PackageId, Self::ModificationOptions>,
        _: &Config,
    ) -> Result<()> {
        unimplemented!()
    }

    fn remove_packages(
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        _: bool,
        _: &Config,
    ) -> Result<()> {
        run_command(
            ["cargo", "uninstall"]
                .into_iter()
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
        )
    }

    fn try_parse_toml_package(
        toml: &toml::Value,
    ) -> Result<(Self::PackageId, Self::InstallOptions)> {
        match toml {
            toml::Value::String(x) => Ok((x.to_string(), Default::default())),
            toml::Value::Table(x) => Ok((
                x.clone().try_into::<StringPackageStruct>()?.package,
                x.clone().try_into()?,
            )),
            _ => Err(anyhow!("cargo packages must be either a string or a table")),
        }
    }
}

fn extract_packages(contents: &str) -> Result<BTreeMap<String, CargoQueryInfo>> {
    let json: Value = serde_json::from_str(contents).context("parsing JSON from crates file")?;

    let result: BTreeMap<String, CargoQueryInfo> = json
        .get("installs")
        .context("get 'installs' field from json")?
        .as_object()
        .context("getting object")?
        .into_iter()
        .map(|(name, value)| {
            let value = value.as_object().unwrap();

            let (name, version_source) = name.split_once(' ').unwrap();
            let (version, source) = version_source.split_once(' ').unwrap();

            let git = if source.starts_with("(git+") {
                Some(
                    source.split("+").collect::<Vec<_>>()[1]
                        .split("#")
                        .next()
                        .unwrap()
                        .to_string(),
                )
            } else {
                None
            };

            let all_features = value.get("all_features").unwrap().as_bool().unwrap();
            let no_default_features = value.get("no_default_features").unwrap().as_bool().unwrap();
            let features = value
                .get("features")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_string())
                .collect();

            (
                name.to_string(),
                CargoQueryInfo {
                    version: version.to_string(),
                    git,
                    all_features,
                    no_default_features,
                    features,
                },
            )
        })
        .collect();

    Ok(result)
}
