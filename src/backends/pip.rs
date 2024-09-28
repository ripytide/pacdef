use color_eyre::eyre::eyre;
use color_eyre::Result;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::cmd::command_found;
use crate::cmd::run_command;
use crate::cmd::run_command_for_stdout;
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Pip;

#[derive(Debug, Clone)]
pub struct PipQueryInfo {
    pub explicit: bool,
}

impl Backend for Pip {
    type PackageId = String;
    type QueryInfo = PipQueryInfo;
    type InstallOptions = ();
    type ModificationOptions = ();
    type RemoveOptions = ();

    fn query_installed_packages(_: &Config) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        if !command_found("pip") {
            return Ok(BTreeMap::new());
        }

        let all = extract_package_names(run_command_for_stdout(
            ["pip", "list", "--format", "json"],
            Perms::Same,
        )?)?;
        let implicit = extract_package_names(run_command_for_stdout(
            ["pip", "list", "--format", "json", "--not-required"],
            Perms::Same,
        )?)?;

        let explicit = all.difference(&implicit);

        Ok(implicit
            .iter()
            .map(|x| (x.to_string(), PipQueryInfo { explicit: false }))
            .chain(explicit.map(|x| (x.to_string(), PipQueryInfo { explicit: true })))
            .collect())
    }

    fn install_packages(
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        _: bool,
        _: &Config,
    ) -> Result<()> {
        run_command(
            ["pip", "install"]
                .into_iter()
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
        )
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
            ["pip", "uninstall"]
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
            _ => Err(eyre!("pip packages must be a string")),
        }
    }
}

fn extract_package_names(stdout: String) -> Result<BTreeSet<String>> {
    let value: Value = serde_json::from_str(&stdout)?;

    Ok(value
        .as_array()
        .ok_or(eyre!("getting inner json array"))?
        .iter()
        .map(|node| node["name"].as_str().expect("should always be a string"))
        .map(String::from)
        .collect())
}
