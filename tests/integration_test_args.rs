use assert_fs::prelude::*;
use predicates::prelude::*;
mod test_helper;
use test_helper::{prepare_cmd, prepare_tmpdir, read_file_content};

#[test]
fn test_no_args() {
    let mut cmd = prepare_cmd();

    // Fail if no args are present
    cmd.assert().failure();
}

#[test]
fn test_third_args_with_existing_path() {
    let mut cmd = prepare_cmd();
    let temp_path = test_helper::prepare_tmpdir();
    temp_path.child("sample_path").touch().unwrap();

    // Assert that 3 args are should pass if third arg exist
    cmd.arg("replace")
        .arg("replace")
        .arg(temp_path.path())
        .assert()
        .success();
}

#[test]
fn test_third_args_with_non_existing_path() {
    let mut cmd = prepare_cmd();

    // Fail if the third argument is not a real path
    cmd.arg("replace")
        .arg("replace")
        .arg("path_that_does_not_exist")
        .assert()
        .failure();
}

#[test]
fn test_arg_dry_run_rename_file() {
    let temp_file = prepare_tmpdir();
    temp_file.child("file").write_str("sample").unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--dry-run")
        .arg("sample")
        .arg("test")
        .arg(temp_file.path())
        .assert()
        .success();

    let file_content: String = read_file_content(&temp_file.child("file"));
    assert_eq!(file_content, "sample");
    temp_file.close().unwrap();
}

#[test]
fn test_arg_dry_run_rename_path() {
    let temp_path = prepare_tmpdir();
    temp_path.child("sample_path").touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--dry-run")
        .arg("sample")
        .arg("test")
        .arg(temp_path.path())
        .assert()
        .success();

    temp_path
        .child("sample_path")
        .assert(predicate::path::exists());
    temp_path.close().unwrap();
}

#[test]
fn test_arg_all_cases_lowercase() {
    let temp_path = prepare_tmpdir();
    temp_path.child("sample_path").touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--all-cases")
        .arg("SAMPLE")
        .arg("TEST")
        .arg(temp_path.path())
        .assert()
        .success();

    temp_path
        .child("test_path")
        .assert(predicate::path::exists());
    temp_path.close().unwrap();
}

#[test]
fn test_arg_all_cases_uppercase() {
    let temp_path = prepare_tmpdir();
    temp_path.child("SAMPLE_PATH").touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--all-cases")
        .arg("sample")
        .arg("test")
        .arg(temp_path.path())
        .assert()
        .success();

    temp_path
        .child("TEST_PATH")
        .assert(predicate::path::exists());
    temp_path.close().unwrap();
}

#[test]
fn test_arg_all_cases_capitalize() {
    let temp_path = prepare_tmpdir();
    temp_path.child("Sample_path").touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--all-cases")
        .arg("sample")
        .arg("test")
        .arg(temp_path.path())
        .assert()
        .success();

    temp_path
        .child("Test_path")
        .assert(predicate::path::exists());
    temp_path.close().unwrap();
}

#[test]
fn test_arg_lowercase() {
    let temp_path = prepare_tmpdir();
    temp_path.child("sample_path").touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("SAMPLE")
        .arg("TEST")
        .arg(temp_path.path())
        .assert()
        .success();

    temp_path
        .child("sample_path")
        .assert(predicate::path::exists());
    temp_path.close().unwrap();
}

#[test]
fn test_arg_uppercase() {
    let temp_path = prepare_tmpdir();
    temp_path.child("SAMPLE_path").touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp_path.path())
        .assert()
        .success();

    temp_path
        .child("SAMPLE_path")
        .assert(predicate::path::exists());
    temp_path.close().unwrap();
}

#[test]
fn test_arg_capitalize() {
    let temp_path = prepare_tmpdir();
    temp_path.child("Sample_path").touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp_path.path())
        .assert()
        .success();

    temp_path
        .child("Sample_path")
        .assert(predicate::path::exists());
    temp_path.close().unwrap();
}
