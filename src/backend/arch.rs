use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::cmd::{command_found, run_args, run_args_for_stdout};
use crate::prelude::*;

#[derive(Debug, Clone, Copy, derive_more::Display)]
pub struct Arch {
    pub command: &'static str,
}

pub type ArchPackageId = String;

#[derive(Debug, Clone)]
pub struct ArchQueryInfo {
    pub explicit: bool,
}

pub struct ArchModification {
    pub make_implicit: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchInstallOptions {
    pub optional_deps: Vec<ArchPackageId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArchRemoveOptions {}

impl Arch {
    pub fn query_installed_packages(
        &self,
        _: &Config,
    ) -> Result<BTreeMap<ArchPackageId, ArchQueryInfo>> {
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

    pub fn install_packages(
        &self,
        packages: &BTreeMap<ArchPackageId, ArchInstallOptions>,
        no_confirm: bool,
        _: &Config,
    ) -> Result<()> {
        run_args(
            [self.command, "--sync"]
                .into_iter()
                .chain(Some("--no_confirm").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str))
                .chain(packages.values().flat_map(|dependencies| {
                    dependencies.optional_deps.iter().map(String::as_str)
                })),
        )
    }

    pub fn modify_packages(
        &self,
        packages: &BTreeMap<ArchPackageId, ArchModification>,
        _: &Config,
    ) -> Result<()> {
        run_args(
            [self.command, "--database", "--asdeps"].into_iter().chain(
                packages
                    .iter()
                    .filter(|(_, m)| m.make_implicit)
                    .map(|(p, _)| p.as_str()),
            ),
        )
    }

    pub fn remove_packages(
        &self,
        packages: &BTreeMap<ArchPackageId, ArchRemoveOptions>,
        no_confirm: bool,
        config: &Config,
    ) -> Result<()> {
        run_args(
            [self.command, "--remove", "--recursive"]
                .into_iter()
                .chain(config.aur_rm_args.iter().map(String::as_str))
                .chain(Some("--no_confirm").filter(|_| no_confirm))
                .chain(packages.keys().map(String::as_str)),
        )
    }
}
