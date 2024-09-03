use anyhow::{Context, Result};
use dialoguer::Confirm;

use crate::prelude::*;
use crate::review::review;

impl MainArguments {
    pub fn run(self, groups: &Groups, config: &Config) -> Result<()> {
        match self.subcommand {
            MainSubcommand::Clean(clean) => clean.run(groups, config),
            MainSubcommand::Review(review) => review.run(groups, config),
            MainSubcommand::Sync(sync) => sync.run(groups, config),
            MainSubcommand::Unmanaged(unmanaged) => unmanaged.run(groups, config),
            MainSubcommand::Version(version) => version.run(config),
        }
    }
}

impl VersionArguments {
    fn run(self, _: &Config) -> Result<()> {
        println!("pacdef, version: {}\n", env!("CARGO_PKG_VERSION"));

        Ok(())
    }
}

impl CleanPackageAction {
    fn run(self, groups: &Groups, config: &Config) -> Result<()> {
        let unmanaged = AnyPackageIds::unmanaged(groups, config)?;

        if unmanaged.is_empty() {
            log::info!("nothing to do since there are no unmanaged packages");
            return Ok(());
        }

        println!("Would remove the following packages:\n\n{unmanaged}\n");

        if self.no_confirm {
            log::info!("proceeding without confirmation");
        } else if !Confirm::new()
            .with_prompt("Do you want to continue?")
            .default(true)
            .show_default(true)
            .interact()
            .context("getting user confirmation")?
        {
            return Ok(());
        }

        let packages_to_remove = unmanaged.;

        packages_to_remove.remove(self.no_confirm, config)
    }
}

impl ReviewPackageAction {
    fn run(self, _: &Groups, _: &Config) -> Result<()> {
        review()
    }
}

impl SyncPackageAction {
    fn run(self, groups: &Groups, config: &Config) -> Result<()> {
        let missing = AnyPackageIds::missing(groups, config)?;

        if missing.is_empty() {
            log::info!("nothing to do as there are no missing packages");
            return Ok(());
        }

        println!("Would install the following packages:\n\n{missing}\n");

        if self.no_confirm {
            log::info!("proceeding without confirmation");
        } else if !Confirm::new()
            .with_prompt("Do you want to continue?")
            .default(true)
            .show_default(true)
            .interact()
            .context("getting user confirmation")?
        {
            return Ok(());
        }

        let packages_to_install = AnyPackageInstallOptions::from_package_ids_defaults(&missing);

        packages_to_install.install(self.no_confirm, config)
    }
}

impl UnmanagedPackageAction {
    fn run(self, groups: &Groups, config: &Config) -> Result<()> {
        let unmanaged = AnyPackageIds::unmanaged(groups, config)?;

        if unmanaged.is_empty() {
            println!("no unmanaged packages");
        } else {
            println!("unmanaged packages:\n\n{unmanaged}");
        }

        Ok(())
    }
}

fn unmanaged(groups: &Groups, config: &Config) -> 
