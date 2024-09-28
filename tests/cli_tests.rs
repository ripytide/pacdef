use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
use std::process::Command;

#[test]
fn unmanaged() {
    let mut cmd = Command::cargo_bin("pacdef").unwrap();
    cmd.args(["--hostname", "pc", "--config-dir", ".", "unmanaged"]);
    cmd.assert().success();
}
