use assert_fs::prelude::*;
use predicates::prelude::*;
mod test_helper;
use test_helper::{prepare_cmd, prepare_tmpdir, read_file_content};

/// Test that the command will fail if no arguments are present
#[test]
fn test_no_args() {
    let mut cmd = prepare_cmd();

    // Fail if no args are present
    cmd.assert().failure();
}

/// Test that the command will work with 3 arguments, the last argument being a real path
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

/// Test that the command will fail with 3 arguments, if the last argument is not a real path
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

/// Test that a lowercase file name will not change using uppercase arguments when the lowercase option is not used
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

/// Test that a uppercase file name will not change when the uppercase option is not used
#[test]
fn test_no_arg_no_change_uppercase() {
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

/// Test that a capitalized file name will not change when the capitalize option is not used
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

/// Test that the file content will not change on a dry run
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

/// Test that the file name will not change on a dry run
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

/// Test that the lowercase argument will replace the file content when using --all-cases
#[test]
fn test_arg_rename_content_all_cases_lowercase() {
    let temp_file = prepare_tmpdir();
    temp_file.child("file").write_str("sample").unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--all-cases")
        .arg("SAMPLE")
        .arg("TEST")
        .arg(temp_file.path())
        .assert()
        .success();

    let file_content: String = read_file_content(&temp_file.child("file"));
    assert_eq!(file_content, "test");
    temp_file.close().unwrap();
}

/// Test that the uppercase argument will replace the file content when using --all-cases
#[test]
fn test_arg_rename_content_all_cases_uppercase() {
    let temp_file = prepare_tmpdir();
    temp_file.child("file").write_str("SAMPLE").unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--all-cases")
        .arg("sample")
        .arg("test")
        .arg(temp_file.path())
        .assert()
        .success();

    let file_content: String = read_file_content(&temp_file.child("file"));
    assert_eq!(file_content, "TEST");
    temp_file.close().unwrap();
}

/// Test that the capitalized argument will replace the file content when using --all-cases
#[test]
fn test_arg_rename_content_all_cases_capitalize() {
    let temp_file = prepare_tmpdir();
    temp_file.child("file").write_str("Sample").unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--all-cases")
        .arg("sample")
        .arg("test")
        .arg(temp_file.path())
        .assert()
        .success();

    let file_content: String = read_file_content(&temp_file.child("file"));
    assert_eq!(file_content, "Test");
    temp_file.close().unwrap();
}

/// Test that the lowercase argument will replace the file name when using --all-cases
#[test]
fn test_arg_rename_path_all_cases_lowercase() {
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

/// Test that the uppercase argument will replace the file name when using --all-cases
#[test]
fn test_arg_rename_path_all_cases_uppercase() {
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

/// Test that the capitalized argument will replace the file name when using --all-cases
#[test]
fn test_arg_rename_path_all_cases_capitalize() {
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
