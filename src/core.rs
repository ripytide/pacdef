use std::fs::{self, read_to_string, File};
use std::path::Path;

use color_eyre::eyre::{eyre, Context, ContextCompat};
use color_eyre::Result;
use dialoguer::Confirm;
use toml_edit::{Array, DocumentMut, Item, Value};

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

        let group_dir = config_dir.join("groups/");

        let config = Config::load(&config_dir).wrap_err("loading config file")?;
        let groups =
            Groups::load(&group_dir, &hostname, &config).wrap_err("failed to load groups")?;

        if groups.is_empty() {
            log::warn!("no group files found");
        }

        match self.subcommand {
            MainSubcommand::Clean(clean) => clean.run(&groups, &config),
            MainSubcommand::Add(add) => add.run(&group_dir, &groups),
            MainSubcommand::Review(review) => review.run(&groups, &config),
            MainSubcommand::Sync(sync) => sync.run(&groups, &config),
            MainSubcommand::Unmanaged(unmanaged) => unmanaged.run(&groups, &config),
        }
    }
}

impl CleanPackage {
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

impl AddPackage {
    fn run(self, group_dir: &Path, groups: &Groups) -> Result<()> {
        let containing_group_files = groups.contains(self.backend, &self.package);
        if !containing_group_files.is_empty() {
            log::info!("the {} package for the {} backend is already installed in the {containing_group_files:?} group files", self.package, self.backend);
        }

        let group_file = group_dir.join(&self.group).with_extension("toml");

        log::info!("parsing group file: {}@{group_file:?}", &self.group);

        if !group_file.is_file() {
            File::create_new(&group_file).wrap_err(eyre!(
                "creating an empty group file {}@{group_file:?}",
                &self.group,
            ))?;
        }

        let file_contents = read_to_string(&group_file)
            .wrap_err(eyre!("reading group file {}@{group_file:?}", &self.group))?;

        let mut doc = file_contents
            .parse::<DocumentMut>()
            .wrap_err(eyre!("parsing group file {}@{group_file:?}", &self.group))?;

        doc.entry(&self.backend.to_string().to_lowercase())
            .or_insert(Item::Value(Value::Array(Array::from_iter([self
                .package
                .clone()]))))
            .as_array_mut()
            .wrap_err(eyre!(
                "the {} backend in the {group_file:?} group file has a non-array value",
                self.backend
            ))?
            .push(self.package);

        fs::write(group_file, doc.to_string())
            .wrap_err("writing back modified group file {group_file:?}")?;

        Ok(())
    }
}

impl ReviewPackage {
    fn run(self, _: &Groups, _: &Config) -> Result<()> {
        review()
    }
}

impl SyncPackage {
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

impl UnmanagedPackage {
    fn run(self, groups: &Groups, config: &Config) -> Result<()> {
        let unmanaged = unmanaged(groups, config)?.simplified();

        if unmanaged.is_empty() {
            eprintln!("no unmanaged packages");
        } else {
            println!("{}", toml::to_string_pretty(&unmanaged)?);
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
