//! Main program for `pacdef`.

#![warn(
    clippy::as_conversions,
    clippy::option_if_let_else,
    clippy::redundant_pub_crate,
    clippy::semicolon_if_nothing_returned,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::use_self,
    clippy::wildcard_dependencies,
    missing_docs
)]

use anyhow::{Context, Result};

use clap::Parser;
use pacdef::{Config, Groups, MainArguments};

fn main() -> Result<()> {
    pretty_env_logger::init();

    let main_arguments = MainArguments::parse();

    let pacdef_dir = dirs::config_dir()
        .map(|path| path.join("pacdef/"))
        .context("getting the pacdef config directory")?;
    let config = Config::load(&pacdef_dir).context("loading config file")?;
    let groups = Groups::load(&pacdef_dir).context("failed to load groups")?;

    if groups.is_empty() {
        log::warn!("no group files found");
    }

    main_arguments.run(&groups, &config)?;

    Ok(())
}
