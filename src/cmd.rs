use std::{
    fmt::Write,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

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

#[derive(Debug, Clone, Copy)]
pub enum ShouldPrint {
    Print,
    Hide,
}

pub fn run_command_for_stdout<I, S>(
    args: I,
    perms: Perms,
    should_print_stdout: ShouldPrint,
) -> Result<String>
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
    let mut child = command
        .args(remaining_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let child_out = BufReader::new(child.stdout.as_mut().unwrap());

    let mut stdout: String = String::new();

    for line in child_out.lines() {
        let line = line?;

        write!(&mut stdout, "{line}")?;

        if let ShouldPrint::Print = should_print_stdout {
            print!("{line}");
        }
    }

    if child.wait()?.success() {
        Ok(stdout)
    } else {
        Err(eyre!("command failed: {:?}", args))
    }
}

pub fn run_command<I, S>(command: I, perms: Perms, should_print_stdout: ShouldPrint) -> Result<()>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    run_command_for_stdout(command, perms, should_print_stdout).map(|_| ())
}
