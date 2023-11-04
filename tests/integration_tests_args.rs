use assert_fs::prelude::*;
use predicates::prelude::*;
mod test_helper;
use test_helper::{prepare_cmd, prepare_tmpdir};

#[test]
fn test_no_args() {
    let mut cmd = prepare_cmd();

    // Fail if no args are present
    cmd.assert().failure();
}

#[test]
fn test_third_args_with_existing_path() {
    let mut cmd = prepare_cmd();

    // Assert that 3 args are should pass if third arg exist
    cmd.arg("search").arg("replace").arg(".").assert().success();
}

#[test]
fn test_third_args_with_non_existing_path() {
    let mut cmd = prepare_cmd();

    // Fail if the third argument is not a real path
    cmd.arg("search")
        .arg("replace")
        .arg("path_that_does_not_exist")
        .assert()
        .failure();
}

#[test]
fn test_arg_dry_run() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("sample_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--dry-run")
        .arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("sample_path").assert(predicate::path::exists());

    temp.close().unwrap();
}

#[test]
fn test_arg_all_cases_lowercase() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("sample_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--all-cases")
        .arg("SAMPLE")
        .arg("TEST")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("test_path").assert(predicate::path::exists());
    temp.close().unwrap();
}

#[test]
fn test_arg_all_cases_uppercase() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("SAMPLE_PATH");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--all-cases")
        .arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("TEST_PATH").assert(predicate::path::exists());

    temp.close().unwrap();
}

#[test]
fn test_arg_all_cases_capitalize() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("Sample_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--all-cases")
        .arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("Test_path").assert(predicate::path::exists());

    temp.close().unwrap();
}

#[test]
fn test_arg_lowercase() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("sample_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("SAMPLE")
        .arg("TEST")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("sample_path").assert(predicate::path::exists());

    temp.close().unwrap();
}

#[test]
fn test_arg_uppercase() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("SAMPLE_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("SAMPLE_path").assert(predicate::path::exists());

    temp.close().unwrap();
}

#[test]
fn test_arg_capitalize() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("Sample_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("Sample_path").assert(predicate::path::exists());

    temp.close().unwrap();
}
