use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs::{remove_file, File};
mod test_helper;
use test_helper::{prepare_cmd, prepare_tmpdir};

/// Test that a file name will change when its name is passed as the first argument with an absolute path
#[test]
fn test_rename_path() {
    let temp = test_helper::prepare_tmpdir();
    let input_folder = temp.child("sample_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("test_path").assert(predicate::path::exists());

    temp.close().unwrap();
}

/// Test that a file name will change when its name is passed as the first argument with an relative path
#[test]
fn test_rename_relative_path() {
    {
        let _ = File::create("sample_file");
    }
    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg("sample_file")
        .assert()
        .success();

    let cmd_result = remove_file("test_file");
    match cmd_result {
        Ok(_) => assert!(true),
        Err(_) => {
            let _ = remove_file("sample_file");
            assert!(false)
        }
    }
}

/// Test that a child file name will change when its name is passed as the first argument
#[test]
fn test_rename_path_recursive() {
    let temp = prepare_tmpdir();
    let input_parent_folder = temp.child("sample_path_parent").child("sample_path_child");
    input_parent_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("test_path_parent/test_path_child")
        .assert(predicate::path::exists());
    temp.close().unwrap();
}

/// Test that a lowercase file name will change when its uppercase name is passed as the first argument and the --lowercase option is used
#[test]
fn test_rename_lowercase_path() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("sample_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--lowercase")
        .arg("SAMPLE")
        .arg("TEST")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("test_path").assert(predicate::path::exists());
    temp.close().unwrap();
}

/// Test that a uppercase file name will change when its lowercase name is passed as the first argument and the --uppercase option is used
#[test]
fn test_rename_uppercase_path() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("SAMPLE_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--uppercase")
        .arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("TEST_path").assert(predicate::path::exists());
    temp.close().unwrap();
}

/// Test that a capitalize file name will change when its lowercase name is passed as the first argument and the --capitalize option is used
#[test]
fn test_rename_capitalize() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("Sample_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--capitalize")
        .arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("Test_path").assert(predicate::path::exists());
    temp.close().unwrap();
}
