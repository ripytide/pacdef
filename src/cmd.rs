use std::process::Command;

use anyhow::Result;

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

pub fn run_args_for_stdout<S>(args: impl Iterator<Item = S>) -> Result<String>
where
    String: From<S>,
{
    let sudo = if unsafe { libc::geteuid() } == 0 {
        Some("sudo".to_string())
    } else {
        None
    };

    let args = sudo
        .into_iter()
        .chain(args.map(String::from))
        .collect::<Vec<_>>();

    let mut cmd = Command::new(args.first().expect("cannot run empty args"));
    cmd.args(args.iter().skip(1));

    let output = cmd.output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(anyhow::anyhow!("command failed: {args:?}"))
    }
}

pub fn run_args<S>(args: impl Iterator<Item = S>) -> Result<()>
where
    String: From<S>,
{
    run_args_for_stdout(args).map(|_| ())
}
