use std::process::Command;

use color_eyre::{eyre::eyre, Result};

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

pub fn run_command_for_stdout<I, S>(command: I, perms: Perms) -> Result<String>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    let we_are_root = {
        let uid = unsafe { libc::geteuid() };
        uid == 0
    };

    let args: Vec<String> = command.into_iter().map(Into::into).collect::<Vec<_>>();

    if args.is_empty() {
        return Err(eyre!("cannot run an empty command"));
    }

    let args = Some("sudo".to_string())
        .filter(|_| matches!(perms, Perms::AsRoot) && !we_are_root)
        .into_iter()
        .chain(args)
        .collect::<Vec<_>>();

    let (command, args) = args.split_first().unwrap();

    let output = Command::new(command).args(args).output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(eyre!("command failed: {:?}", args))
    }
}

pub fn run_command<I, S>(command: I, perms: Perms) -> Result<()>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    run_command_for_stdout(command, perms).map(|_| ())
}
