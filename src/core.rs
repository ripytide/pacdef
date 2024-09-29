use color_eyre::eyre::{eyre, Context};
use color_eyre::Result;
use dialoguer::Confirm;

use crate::prelude::*;
use crate::review::review;

impl MainArguments {
    pub fn run(self) -> Result<()> {
        let hostname = if let Some(x) = self.hostname {
            x
        } else {
            hostname::get()?
                .into_string()
                .or(Err(eyre!("getting hostname")))?
        };

        let config_dir = if let Some(x) = self.config_dir {
            x
        } else {
            dirs::config_dir()
                .map(|path| path.join("pacdef/"))
                .ok_or(eyre!("getting the default pacdef config directory"))?
        };

        let config = Config::load(&config_dir).wrap_err("loading config file")?;
        let groups =
            Groups::load(&config_dir, &hostname, &config).wrap_err("failed to load groups")?;

        if groups.is_empty() {
            log::warn!("no group files found");
        }

        match self.subcommand {
            MainSubcommand::Clean(clean) => clean.run(&groups, &config),
            MainSubcommand::Review(review) => review.run(&groups, &config),
            MainSubcommand::Sync(sync) => sync.run(&groups, &config),
            MainSubcommand::Unmanaged(unmanaged) => unmanaged.run(&groups, &config),
        }
    }
}

impl CleanPackageAction {
    fn run(self, groups: &Groups, config: &Config) -> Result<()> {
        let unmanaged = unmanaged(groups, config)?;

        if unmanaged.is_empty() {
            log::info!("nothing to do since there are no unmanaged packages");
            return Ok(());
        }

        println!("would remove the following packages:\n\n{unmanaged}");

        if self.no_confirm {
            log::info!("proceeding without confirmation");
        } else if !Confirm::new()
            .with_prompt("do you want to continue?")
            .default(true)
            .show_default(true)
            .interact()
            .wrap_err("getting user confirmation")?
        {
            return Ok(());
        }

        unmanaged
            .to_remove_options()
            .remove_packages(self.no_confirm, config)
    }
}

impl ReviewPackageAction {
    fn run(self, _: &Groups, _: &Config) -> Result<()> {
        review()
    }
}

impl SyncPackageAction {
    fn run(self, groups: &Groups, config: &Config) -> Result<()> {
        let missing = missing(groups, config)?;

        if missing.is_empty() {
            log::info!("nothing to do as there are no missing packages");
            return Ok(());
        }

        println!("would install the following packages:\n\n{missing}");

        if self.no_confirm {
            log::info!("proceeding without confirmation");
        } else if !Confirm::new()
            .with_prompt("do you want to continue?")
            .default(true)
            .show_default(true)
            .interact()
            .wrap_err("getting user confirmation")?
        {
            return Ok(());
        }

        missing
            .to_install_options()
            .install_packages(self.no_confirm, config)
    }
}

impl UnmanagedPackageAction {
    fn run(self, groups: &Groups, config: &Config) -> Result<()> {
        let unmanaged = unmanaged(groups, config)?;

        if unmanaged.is_empty() {
            eprintln!("no unmanaged packages");
        } else {
            println!("{unmanaged}");
        }

        Ok(())
    }
}

fn unmanaged(groups: &Groups, config: &Config) -> Result<PackageIds> {
    Ok(QueryInfos::query_installed_packages(config)?
        .to_package_ids()
        .difference(&groups.to_package_ids()))
}
fn missing(groups: &Groups, config: &Config) -> Result<PackageIds> {
    Ok(groups
        .to_package_ids()
        .difference(&QueryInfos::query_installed_packages(config)?.to_package_ids()))
}
