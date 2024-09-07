use std::collections::BTreeMap;
use std::io::ErrorKind::NotFound;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cmd::{command_found, run_args};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Cargo;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CargoInstallOptions {
    git: Option<String>,
    all_features: bool,
    no_default_features: bool,
    features: Vec<String>,
}

impl Backend for Cargo {
    type PackageId = String;
    type QueryInfo = CargoInstallOptions;
    type InstallOptions = CargoInstallOptions;
    type Modification = ();
    type RemoveOptions = ();

    fn query_installed_packages(&self, _: &Config) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
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
        &self,
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        _: bool,
        _: &Config,
    ) -> Result<()> {
        for (package, options) in packages {
            run_args(
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
            )?;
        }
        Ok(())
    }

    fn modify_packages(
        &self,
        _: &BTreeMap<Self::PackageId, Self::Modification>,
        _: &Config,
    ) -> Result<()> {
        unimplemented!()
    }

    fn remove_packages(
        &self,
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        _: bool,
        _: &Config,
    ) -> Result<()> {
        run_args(
            ["cargo", "uninstall"]
                .into_iter()
                .chain(packages.keys().map(String::as_str)),
        )
    }
}

fn extract_packages(contents: &str) -> Result<BTreeMap<String, CargoInstallOptions>> {
    let json: Value = serde_json::from_str(contents).context("parsing JSON from crates file")?;

    let result: BTreeMap<String, CargoInstallOptions> = json
        .get("installs")
        .context("get 'installs' field from json")?
        .as_object()
        .context("getting object")?
        .into_iter()
        .map(|(name, value)| {
            let (name, git_repo) = name
                .split_once('+')
                .expect("Resolve git status and name separately");

            let all_features = value
                .as_object()
                .expect("Won't fail")
                .get("all_features")
                .expect("Won't fail")
                .as_bool()
                .expect("Won't fail");

            let no_default_features = value
                .as_object()
                .expect("Won't fail")
                .get("no_default_features")
                .expect("Won't fail")
                .as_bool()
                .expect("Won't fail");

            let features = value
                .as_object()
                .expect("Won't fail")
                .get("features")
                .expect("Won't fail")
                .as_array()
                .expect("Won't fail")
                .iter()
                .map(|value| value.as_str().expect("Won't fail").to_string())
                .collect();

            (
                name.to_string(),
                CargoInstallOptions {
                    git: git_repo.split_once('#').map(|(repo, _)| repo.to_string()),
                    all_features,
                    no_default_features,
                    features,
                },
            )
        })
        .collect();

    Ok(result)
}
