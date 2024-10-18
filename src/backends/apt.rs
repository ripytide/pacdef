use std::collections::BTreeMap;

use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::cmd::{command_found, run_command, run_command_for_stdout};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Apt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AptQueryInfo {}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AptInstallOptions {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AptModificationOptions {
    make_implicit: bool,
}

impl Backend for Apt {
    type QueryInfo = AptQueryInfo;
    type InstallOptions = AptInstallOptions;
    type ModificationOptions = AptModificationOptions;
    type RemoveOptions = ();

    fn query_installed_packages(_: &Config) -> Result<BTreeMap<String, Self::QueryInfo>> {
        if !command_found("apt-mark") {
            return Ok(BTreeMap::new());
        }

        // See https://askubuntu.com/questions/2389/how-to-list-manually-installed-packages
        // for a run-down of methods for finding lists of
        // explicit/dependency packages. It doesn't seem as if apt was
        // designed with this use-case in mind so there are lots and
        // lots of different methods all of which seem to have
        // caveats.
        let explicit =
            run_command_for_stdout(["apt-mark", "showmanual"], Perms::Same, ShouldPrint::Hide)?;

        Ok(explicit
            .lines()
            .map(|x| (x.to_string(), AptQueryInfo {}))
            .collect())
    }

    fn install_packages(
        packages: &BTreeMap<String, Self::InstallOptions>,
        _: &Config,
    ) -> Result<()> {
        run_command(
            ["apt-get", "install", "--yes"]
                .into_iter()
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
            ShouldPrint::Print,
        )
    }

    fn modify_packages(
        packages: &BTreeMap<String, Self::ModificationOptions>,
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
            ShouldPrint::Print,
        )
    }

    fn remove_packages(packages: &BTreeMap<String, Self::RemoveOptions>, _: &Config) -> Result<()> {
        run_command(
            ["apt-get", "remove", "--yes"]
                .into_iter()
                .chain(packages.keys().map(String::as_str)),
            Perms::AsRoot,
            ShouldPrint::Print,
        )
    }
}
