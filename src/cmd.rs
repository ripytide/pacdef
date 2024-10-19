use std::process::{Command, Stdio};

use color_eyre::{eyre::eyre, Result};
use itertools::Itertools;

pub fn command_found(command: &str) -> bool {
    if let Ok(path) = std::env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, command);
            if std::fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, Clone, Copy)]
pub enum Perms {
    AsRoot,
    Same,
}

pub fn run_command_for_stdout<I, S>(args: I, perms: Perms) -> Result<String>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    let we_are_root = {
        let uid = unsafe { libc::geteuid() };
        uid == 0
    };

    let args: Vec<String> = args.into_iter().map(Into::into).collect::<Vec<_>>();

    if args.is_empty() {
        return Err(eyre!("cannot run an empty command"));
    }

    let args = Some("sudo".to_string())
        .filter(|_| matches!(perms, Perms::AsRoot) && !we_are_root)
        .into_iter()
        .chain(args)
        .collect::<Vec<_>>();

    let (first_arg, remaining_args) = args.split_first().unwrap();

    let mut command = Command::new(first_arg);
    let status = command
        .args(remaining_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()?;

    if status.status.success() {
        Ok(String::from_utf8(status.stdout)?)
    } else {
        Err(eyre!("command failed: {:?}", args.into_iter().join(" ")))
    }
}

pub fn run_command<I, S>(args: I, perms: Perms) -> Result<()>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    let we_are_root = {
        let uid = unsafe { libc::geteuid() };
        uid == 0
    };

    let args: Vec<String> = args.into_iter().map(Into::into).collect::<Vec<_>>();

    if args.is_empty() {
        return Err(eyre!("cannot run an empty command"));
    }

    let args = Some("sudo".to_string())
        .filter(|_| matches!(perms, Perms::AsRoot) && !we_are_root)
        .into_iter()
        .chain(args)
        .collect::<Vec<_>>();

    let (first_arg, remaining_args) = args.split_first().unwrap();

    let mut command = Command::new(first_arg);
    let status = command
        .args(remaining_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(eyre!("command failed: {:?}", args.into_iter().join(" ")))
    }
}
