use std::collections::BTreeMap;
use std::process::Command;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use regex::Regex;

use crate::cmd::{command_found, run_command, run_command_for_stdout};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Xbps;

#[derive(Debug, Clone)]
pub struct XbpsModificationOptions {
    make_implicit: bool,
}

impl Backend for Xbps {
    type PackageId = String;
    type QueryInfo = ();
    type InstallOptions = ();
    type ModificationOptions = XbpsModificationOptions;
    type RemoveOptions = ();

    fn query_installed_packages(
        _: &Config,
    ) -> Result<std::collections::BTreeMap<Self::PackageId, Self::QueryInfo>> {
        if !command_found("xbps-query") {
            return Ok(BTreeMap::new());
        }

        let mut cmd = Command::new("xbps-query");
        cmd.args(["-l"]);
        let stdout = run_command_for_stdout(["xbps-query", "-l"], Perms::Same)?;

        // Removes the package status and description from output
        let re1 = Regex::new(r"^ii |^uu |^hr |^\?\? | .*")?;
        // Removes the package version from output
        let re2 = Regex::new(r"-[^-]*$")?;

        let packages = stdout
            .lines()
            .map(|line| {
                let mid_result = re1.replace_all(line, "");

                (re2.replace_all(&mid_result, "").to_string(), ())
            })
            .collect();

        Ok(packages)
    }

    fn install_packages(
        packages: &std::collections::BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        _: &Config,
    ) -> Result<()> {
        run_command(
            ["xbps-install", "-S"]
                .into_iter()
                .chain(Some("-y").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
        )
    }

    fn remove_packages(
        packages: &std::collections::BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        _: &Config,
    ) -> Result<()> {
        run_command(
            ["xbps-remove", "-R"]
                .into_iter()
                .chain(Some("-y").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
        )
    }

    fn modify_packages(
        packages: &std::collections::BTreeMap<Self::PackageId, Self::ModificationOptions>,
        _: &Config,
    ) -> Result<()> {
        run_command(
            ["xbps-pkgdb", "-m", "auto"].into_iter().chain(
                packages
                    .iter()
                    .filter(|(_, m)| m.make_implicit)
                    .map(|(p, _)| p.as_str()),
            ),
            Perms::AsRoot,
        )
    }

    fn try_parse_toml_package(
        toml: &toml::Value,
    ) -> Result<(Self::PackageId, Self::InstallOptions)> {
        match toml {
            toml::Value::String(x) => Ok((x.to_string(), Default::default())),
            _ => Err(eyre!("xbps packages must be a string")),
        }
    }
}
