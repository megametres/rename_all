use assert_cmd::Command;
use assert_fs::prelude::*;
use std::fs::{File,remove_file};
use predicates::prelude::*;

fn prepare_cmd() -> assert_cmd::Command {
    return Command::cargo_bin("rename_all").unwrap();
}

fn prepare_tmpdir() -> assert_fs::TempDir {
    return assert_fs::TempDir::new().unwrap();
}

#[test]
fn test_rename_path() {
    let temp = prepare_tmpdir();
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

#[test]
fn test_rename_relative_path() {
    {
        let _ = File::create("sample_file");
    }
    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg("sample_file");

    let cmd_result = remove_file("test_file");
    match cmd_result {
        Ok(_) => assert!(true),
        Err(_) => {
            let _ = remove_file("sample_file");
            assert!(false)
        }
    }
}

#[test]
fn test_rename_uppercase_path() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("SAMPLE_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp.path());

    temp.child("TEST_path")
        .assert(predicate::path::exists());

    temp.close().unwrap();
}

#[test]
fn test_rename_capitalize() {
    let temp = prepare_tmpdir();
    let input_folder = temp.child("Sample_path");
    input_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp.path());

    temp.child("Test_path")
        .assert(predicate::path::exists());

    temp.close().unwrap();
}

#[test]
fn test_rename_path_recursive() {
    let temp = prepare_tmpdir();
    let input_parent_folder = temp.child("sample_path_parent").child("sample_path_child");
    input_parent_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp.path());

    temp.child("test_path_parent/test_path_child")
        .assert(predicate::path::exists());

    temp.close().unwrap();
}