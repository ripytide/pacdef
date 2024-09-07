use crate::cmd::command_found;
use crate::cmd::run_args;
use crate::cmd::run_args_for_stdout;
use crate::prelude::*;
use anyhow::Result;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
pub struct Rustup;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RustupQueryInfo {
    pub components: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RustupInstallOptions {
    pub components: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RustupModificationOptions {
    pub add_components: Vec<String>,
    pub remove_components: Vec<String>,
}

impl Backend for Rustup {
    type PackageId = String;
    type QueryInfo = RustupQueryInfo;
    type InstallOptions = RustupInstallOptions;
    type ModificationOptions = RustupModificationOptions;
    type RemoveOptions = ();

    fn query_installed_packages(_: &Config) -> Result<BTreeMap<Self::PackageId, Self::QueryInfo>> {
        if !command_found("rustup") {
            return Ok(BTreeMap::new());
        }

        let mut packages = BTreeMap::new();

        let toolchains_stdout = run_args_for_stdout(["rustup", "toolchain", "list"])?;
        let toolchains = toolchains_stdout.lines().map(|x| {
            x.split(' ')
                .next()
                .expect("output shouldn't contain empty lines")
                .to_string()
        });

        for toolchain in toolchains {
            let components_stdout = run_args_for_stdout([
                "rustup",
                "component",
                "list",
                "--installed",
                "--toolchain",
                toolchain.as_str(),
            ])?;

            packages.insert(
                toolchain,
                RustupQueryInfo {
                    components: components_stdout.lines().map(|x| x.to_string()).collect(),
                },
            );
        }

        Ok(packages)
    }

    fn install_packages(
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        _: bool,
        _: &Config,
    ) -> Result<()> {
        for (toolchain, rustup_install_options) in packages.iter() {
            run_args(["rustup", "toolchain", "install", toolchain.as_str()])?;

            if !rustup_install_options.components.is_empty() {
                run_args(
                    [
                        "rustup",
                        "component",
                        "add",
                        "--toolchain",
                        toolchain.as_str(),
                    ]
                    .into_iter()
                    .chain(rustup_install_options.components.iter().map(|x| x.as_str())),
                )?;
            }
        }

        Ok(())
    }

    fn modify_packages(
        packages: &BTreeMap<Self::PackageId, Self::ModificationOptions>,
        _: &Config,
    ) -> Result<()> {
        for (toolchain, rustup_modification_options) in packages.iter() {
            if !rustup_modification_options
                .add_components
                .iter()
                .chain(rustup_modification_options.remove_components.iter())
                .all_unique()
            {
                log::warn!("component in both add_components and remove_components for the {toolchain} toolchain modification")
            }

            if !rustup_modification_options.remove_components.is_empty() {
                run_args(
                    [
                        "rustup",
                        "component",
                        "remove",
                        "--toolchain",
                        toolchain.as_str(),
                    ]
                    .into_iter()
                    .chain(
                        rustup_modification_options
                            .remove_components
                            .iter()
                            .map(|x| x.as_str()),
                    ),
                )?;
            }
            if !rustup_modification_options.add_components.is_empty() {
                run_args(
                    [
                        "rustup",
                        "component",
                        "add",
                        "--toolchain",
                        toolchain.as_str(),
                    ]
                    .into_iter()
                    .chain(
                        rustup_modification_options
                            .add_components
                            .iter()
                            .map(|x| x.as_str()),
                    ),
                )?;
            }
        }

        Ok(())
    }

    fn remove_packages(
        packages: &BTreeMap<Self::PackageId, Self::RemoveOptions>,
        _: bool,
        _: &Config,
    ) -> Result<()> {
        for toolchain in packages.keys() {
            run_args(["rustup", "toolchain", "remove", toolchain.as_str()])?;
        }

        Ok(())
    }
}
