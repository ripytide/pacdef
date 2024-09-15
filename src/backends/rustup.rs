use crate::cmd::command_found;
use crate::cmd::run_command;
use crate::cmd::run_command_for_stdout;
use crate::prelude::*;
use anyhow::anyhow;
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

        let toolchains_stdout =
            run_command_for_stdout(["rustup", "toolchain", "list"], Perms::Same)?;
        let toolchains = toolchains_stdout.lines().map(|x| {
            x.split(' ')
                .next()
                .expect("output shouldn't contain empty lines")
                .to_string()
        });

        for toolchain in toolchains {
            //due to https://github.com/rust-lang/rustup/issues/1570
            //we sometimes must interpret a failed command as no
            //components for custom toolchains
            if let Ok(components_stdout) = run_command_for_stdout(
                [
                    "rustup",
                    "component",
                    "list",
                    "--installed",
                    "--toolchain",
                    toolchain.as_str(),
                ],
                Perms::Same,
            ) {
                packages.insert(
                    toolchain,
                    RustupQueryInfo {
                        components: components_stdout.lines().map(|x| x.to_string()).collect(),
                    },
                );
            }
        }

        Ok(packages)
    }

    fn install_packages(
        packages: &BTreeMap<Self::PackageId, Self::InstallOptions>,
        _: bool,
        _: &Config,
    ) -> Result<()> {
        for (toolchain, rustup_install_options) in packages.iter() {
            run_command(
                ["rustup", "toolchain", "install", toolchain.as_str()],
                Perms::Same,
            )?;

            if !rustup_install_options.components.is_empty() {
                run_command(
                    [
                        "rustup",
                        "component",
                        "add",
                        "--toolchain",
                        toolchain.as_str(),
                    ]
                    .into_iter()
                    .chain(rustup_install_options.components.iter().map(|x| x.as_str())),
                    Perms::Same,
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
                run_command(
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
                    Perms::Same,
                )?;
            }
            if !rustup_modification_options.add_components.is_empty() {
                run_command(
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
                    Perms::Same,
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
            run_command(
                ["rustup", "toolchain", "remove", toolchain.as_str()],
                Perms::Same,
            )?;
        }

        Ok(())
    }

    fn try_parse_toml_package(
        toml: &toml::Value,
    ) -> Result<(Self::PackageId, Self::InstallOptions)> {
        match toml {
            toml::Value::String(x) => Ok((x.to_string(), Default::default())),
            toml::Value::Table(x) => Ok((
                x.clone().try_into::<StringPackageStruct>()?.package,
                x.clone().try_into()?,
            )),
            _ => Err(anyhow!(
                "rustup packages must be either a string or a table"
            )),
        }
    }
}
