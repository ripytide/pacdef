use std::collections::BTreeMap;
use std::io::ErrorKind::NotFound;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cmd::{command_found, run_args};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, derive_more::Display)]
pub struct Cargo;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CargoInstallOptions {
    git: Option<String>,
    all_features: bool,
    no_default_features: bool,
}

impl Backend for Cargo {
    type PackageId = String;
    type RemoveOptions = ();
    type InstallOptions = CargoInstallOptions;
    type QueryInfo = CargoInstallOptions;
    type Modification = ();

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
        run_args(
            ["cargo", "install"].into_iter().chain(
                packages
                    .iter()
                    .filter(|(_, opts)| {
                        opts.git.is_none() && !opts.all_features && !opts.no_default_features
                    })
                    .map(|(name, _)| name)
                    .map(String::as_str),
            ),
        )?;
        for (package, options) in packages {
            match options {
                CargoInstallOptions {
                    git: _,
                    all_features: true,
                    no_default_features: true,
                } => {
                    bail!("Invalid config parameters for {package}");
                }

                CargoInstallOptions {
                    git: None,
                    all_features: false,
                    no_default_features: false,
                } => {} // cannot be matched

                CargoInstallOptions {
                    git: Some(remote),
                    all_features: false,
                    no_default_features: false,
                } => run_args(["cargo", "install", "--git", remote, package])?,

                CargoInstallOptions {
                    git: Some(remote),
                    all_features: true,
                    no_default_features: false,
                } => run_args([
                    "cargo",
                    "install",
                    "--all-features",
                    "--git",
                    remote,
                    package,
                ])?,

                CargoInstallOptions {
                    git: Some(remote),
                    all_features: false,
                    no_default_features: true,
                } => run_args([
                    "cargo",
                    "install",
                    "--no-default-features",
                    "--git",
                    remote,
                    package,
                ])?,

                CargoInstallOptions {
                    git: None,
                    all_features: true,
                    no_default_features: false,
                } => run_args(["cargo", "install", "--all-features", package])?,

                CargoInstallOptions {
                    git: None,
                    all_features: false,
                    no_default_features: true,
                } => run_args(["cargo", "install", "--no-default-features", package])?,
            }
        }
        Ok(())
    }

    fn modify_packages(
        _: &BTreeMap<Self::PackageId, Self::Modification>,
        _: &Config,
    ) -> Result<()> {
        unimplemented!()
    }

    fn remove_packages(
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
            (
                name.to_string(),
                CargoInstallOptions {
                    git: git_repo.split_once('#').map(|(repo, _)| repo.to_string()),
                    all_features,
                    no_default_features,
                },
            )
        })
        .collect();

    Ok(result)
}
