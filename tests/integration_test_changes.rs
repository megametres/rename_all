use assert_cmd::Command;
use assert_fs::prelude::*;
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

#[test]
fn test_rename_uppercase() {
    let temp = prepare_tmpdir();
    let input_parent_folder = temp.child("SAMPLE_path_parent").child("SAMPLE_path_child");
    input_parent_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("TEST_path_parent/TEST_path_child")
        .assert(predicate::path::exists());

    temp.close().unwrap();
}

#[test]
fn test_rename_capitalize() {
    let temp = prepare_tmpdir();
    let input_parent_folder = temp.child("Sample_path_parent").child("Sample_path_child");
    input_parent_folder.touch().unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp.path())
        .assert()
        .success();

    temp.child("Test_path_parent/Test_path_child")
        .assert(predicate::path::exists());

    temp.close().unwrap();
}