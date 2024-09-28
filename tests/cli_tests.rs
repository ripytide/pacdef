use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
use std::process::Command;

#[test]
fn version() {
    let mut cmd = Command::cargo_bin("pacdef").unwrap();
    cmd.arg("version");
    cmd.assert().success();
}
#[test]
fn unmanaged() {
    let mut cmd = Command::cargo_bin("pacdef").unwrap();
    cmd.arg("unmanaged");
    cmd.assert().success();
}
