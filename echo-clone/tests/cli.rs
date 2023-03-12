use std::fs::{ read_to_string };

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn fails_without_args() -> TestResult {
  let mut cmd = Command::cargo_bin("echo-clone")?;
  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("USAGE"));

  Ok(())
}

#[test]
fn passes_with_args() -> TestResult {
  let mut cmd = Command::cargo_bin("echo-clone")?;
  cmd
    .args(["hello", "world"])
    .assert()
    .success()
    .stdout(predicate::str::contains("hello world"));

  Ok(())
}

#[test]
fn same_as_hello1_txt() -> TestResult {
  let path = "tests/expected/hello1.txt";
  let expected = read_to_string(path)?;

  let mut cmd = Command::cargo_bin("echo-clone")?;
  cmd
    .args(["Hello there"])
    .assert()
    .success()
    .stdout(expected);

  Ok(())
}

#[test]
fn same_as_hello1n_txt() -> TestResult {
  let path = "tests/expected/hello1.n.txt";
  let expected = read_to_string(path)?;

  let mut cmd = Command::cargo_bin("echo-clone")?;
  cmd
    .args(["Hello  there", "-n"])
    .assert()
    .success()
    .stdout(expected);

  Ok(())
}

#[test]
fn same_as_hello2_txt() -> TestResult {
  let path = "tests/expected/hello2.txt";
  let expected = read_to_string(path)?;

  let mut cmd = Command::cargo_bin("echo-clone")?;
  cmd
    .args(["Hello", "there"])
    .assert()
    .success()
    .stdout(expected);

  Ok(())
}

#[test]
fn same_as_hello2n_txt() -> TestResult {
  let path = "tests/expected/hello2.n.txt";
  let expected = read_to_string(path)?;

  let mut cmd = Command::cargo_bin("echo-clone")?;
  cmd
    .args(["-n" ,"Hello", "there"])
    .assert()
    .success()
    .stdout(expected);

  Ok(())
}
