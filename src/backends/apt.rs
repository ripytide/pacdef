use std::collections::BTreeMap;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::cmd::{command_found, run_command, run_command_for_stdout};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Apt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AptQueryInfo {
    pub explicit: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AptModificationOptions {
    make_implicit: bool,
}

impl Backend for Apt {
    type PackageId = String;
    type QueryInfo = AptQueryInfo;
    type InstallOptions = ();
    type ModificationOptions = AptModificationOptions;
    type RemoveOptions = ();

    fn query_installed_packages(_: &Config) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        if !command_found("apt-mark") {
            return Ok(BTreeMap::new());
        }

        // See https://askubuntu.com/questions/2389/how-to-list-manually-installed-packages
        // for a run-down of methods for finding lists of
        // explicit/dependency packages. It doesn't seem as if apt was
        // designed with this use-case in mind so there are lots and
        // lots of different methods all of which seem to have
        // caveats.
        let explicit = run_command_for_stdout(["apt-mark", "showmanual"], Perms::Same)?;
        let dependency = run_command_for_stdout(["apt-mark", "showauto"], Perms::Same)?;

        Ok(dependency
            .lines()
            .map(|x| (x.to_string(), AptQueryInfo { explicit: false }))
            .chain(
                explicit
                    .lines()
                    .map(|x| (x.to_string(), AptQueryInfo { explicit: true })),
            )
            .collect())
    }

    fn install_packages(
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        _: &Config,
    ) -> Result<()> {
        run_command(
            ["apt-get", "install"]
                .into_iter()
                .chain(Some("--yes").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
        )
    }

    fn modify_packages(
        packages: &BTreeMap<Self::PackageId, Self::ModificationOptions>,
        _: &Config,
    ) -> Result<()> {
        run_command(
            ["apt-mark", "auto"].into_iter().chain(
                packages
                    .iter()
                    .filter(|(_, m)| m.make_implicit)
                    .map(|(p, _)| p.as_str()),
            ),
            Perms::AsRoot,
        )
    }

    fn remove_packages(
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        _: &Config,
    ) -> Result<()> {
        run_command(
            ["apt-get", "remove"]
                .into_iter()
                .chain(Some("--yes").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
        )
    }

    fn try_parse_toml_package(
        toml: &toml::Value,
    ) -> Result<(Self::PackageId, Self::InstallOptions)> {
        match toml {
            toml::Value::String(x) => Ok((x.to_string(), Default::default())),
            _ => Err(eyre!("apt packages must be a string")),
        }
    }
}
