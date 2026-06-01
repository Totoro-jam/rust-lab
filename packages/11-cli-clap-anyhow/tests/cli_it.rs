//! End-to-end CLI tests using assert_cmd.

use assert_cmd::Command;
use predicates::prelude::*;

fn calc() -> Command {
    Command::cargo_bin("calc").unwrap()
}

#[test]
fn add_two_numbers() {
    calc()
        .args(["add", "2", "3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));
}

#[test]
fn divide_by_zero_fails() {
    calc()
        .args(["divide", "10", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot divide by zero"));
}

#[test]
fn even_check() {
    calc()
        .args(["even", "4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("true"));

    calc()
        .args(["even", "7"])
        .assert()
        .success()
        .stdout(predicate::str::contains("false"));
}

#[test]
fn verbose_flag_accepted() {
    calc()
        .args(["--verbose", "add", "1", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2"));
}

#[test]
fn missing_args_shows_help() {
    calc()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}
