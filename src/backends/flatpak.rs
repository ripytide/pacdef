use std::collections::{BTreeMap, BTreeSet};

use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::cmd::{command_found, run_command, run_command_for_stdout};
use crate::prelude::*;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Flatpak;

#[derive(Debug, Clone)]
pub struct FlatpakQueryInfo {
    pub systemwide: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FlatpakInstallOptions {}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FlatpakModificationOptions {}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FlatpakRemoveOptions {}

impl Backend for Flatpak {
    type QueryInfo = FlatpakQueryInfo;
    type InstallOptions = FlatpakInstallOptions;
    type ModificationOptions = FlatpakModificationOptions;
    type RemoveOptions = FlatpakRemoveOptions;

    fn query_installed_packages(_: &Config) -> Result<BTreeMap<String, Self::QueryInfo>> {
        if !command_found("flatpak") {
            return Ok(BTreeMap::new());
        }

        let sys_explicit_btree = run_command_for_stdout(
            [
                "flatpak",
                "list",
                "--system",
                "--app",
                "--columns=application",
            ],
            Perms::Same,
        )?
        .lines()
        .map(String::from)
        .collect::<BTreeSet<_>>();

        let user_explicit_btree = run_command_for_stdout(
            [
                "flatpak",
                "list",
                "--user",
                "--app",
                "--columns=application",
            ],
            Perms::Same,
        )?
        .lines()
        .map(String::from)
        .collect::<BTreeSet<_>>();

        let sys_explicit = sys_explicit_btree
            .iter()
            .map(|x| (x.clone(), FlatpakQueryInfo { systemwide: true }));
        let user_explicit = user_explicit_btree
            .iter()
            .map(|x| (x.clone(), FlatpakQueryInfo { systemwide: false }));

        let all = sys_explicit.chain(user_explicit).collect();

        Ok(all)
    }

    fn install_packages(
        packages: &BTreeMap<String, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        if !packages.is_empty() {
            run_command(
                [
                    "flatpak",
                    "install",
                    if config.flatpak_systemwide {
                        "--system"
                    } else {
                        "--user"
                    },
                ]
                .into_iter()
                .chain(Some("--assumeyes").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
                Perms::Sudo,
            )?;
        }

        Ok(())
    }

    fn modify_packages(_: &BTreeMap<String, Self::ModificationOptions>, _: &Config) -> Result<()> {
        unimplemented!()
    }

    fn remove_packages(
        packages: &BTreeMap<String, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        if !packages.is_empty() {
            run_command(
                [
                    "flatpak",
                    "uninstall",
                    if config.flatpak_systemwide {
                        "--system"
                    } else {
                        "--user"
                    },
                ]
                .into_iter()
                .chain(Some("--assumeyes").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
                Perms::Sudo,
            )?;
        }

        Ok(())
    }
}
