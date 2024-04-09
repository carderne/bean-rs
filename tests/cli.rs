use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn run_balance() {
    let mut cmd = Command::cargo_bin("bean-rs").unwrap();
    cmd.arg("balance").arg("example.bean");
    cmd.assert().success();
}

#[test]
fn run_bad_file() {
    let mut cmd = Command::cargo_bin("bean-rs").unwrap();
    cmd.arg("balance").arg("doesntexist.bean");
    cmd.assert().failure();
}

#[test]
fn run_check() {
    let mut cmd = Command::cargo_bin("bean-rs").unwrap();
    cmd.arg("check").arg("example.bean");
    cmd.assert().success();
}
