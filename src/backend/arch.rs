use anyhow::Result;
use std::collections::BTreeMap;

use crate::cmd::{command_found, run_args, run_args_for_stdout};
use crate::prelude::*;

#[derive(Debug, Clone, Copy, derive_more::Display)]
pub struct Arch;

#[derive(Debug, Clone)]
pub struct ArchQueryInfo {
    pub explicit: bool,
}

pub struct ArchMakeImplicit;

impl Backend for Arch {
    type PackageId = String;
    type RemoveOptions = ();
    type InstallOptions = ();
    type QueryInfo = ArchQueryInfo;
    type Modification = ArchMakeImplicit;

    fn query_installed_packages(_: &Config) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        if !command_found("pacman") {
            return Ok(BTreeMap::new());
        }

        let explicit = run_args_for_stdout(["pacman", "--query", "--explicit", "--quiet"])?;
        let dependency = run_args_for_stdout(["pacman", "--query", "--deps", "--quiet"])?;

        Ok(dependency
            .lines()
            .map(|x| (x.to_string(), ArchQueryInfo { explicit: false }))
            .chain(
                explicit
                    .lines()
                    .map(|x| (x.to_string(), ArchQueryInfo { explicit: true })),
            )
            .collect())
    }

    fn install_packages(
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        run_args(
            [config.aur_helper.as_str(), "--sync"]
                .into_iter()
                .chain(Some("--no_confirm").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
        )
    }

    fn modify_packages(
        packages: &BTreeMap<Self::PackageId, Self::Modification>,
        config: &Config,
    ) -> Result<()> {
        run_args(
            [config.aur_helper.as_str(), "--database", "--asdeps"]
                .into_iter()
                .chain(packages.keys().map(String::as_str)),
        )
    }

    fn remove_packages(
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        run_args(
            [config.aur_helper.as_str(), "--remove", "--recursive"]
                .into_iter()
                .chain(config.aur_rm_args.iter().map(String::as_str))
                .chain(Some("--no_confirm").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
        )
    }
}
