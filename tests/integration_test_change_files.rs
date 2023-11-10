use assert_fs::prelude::*;
use std::fs::remove_file;
mod test_helper;
use test_helper::{prepare_cmd, prepare_tmpdir, read_file_content};

/// Test that the file content will change when its content is passed as the first argument
#[test]
fn test_rename_file_content() {
    let temp_file = prepare_tmpdir();
    temp_file.child("file").write_str("sample").unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp_file.path())
        .assert()
        .success();

    let file_content: String = read_file_content(&temp_file.child("file"));
    assert_eq!(file_content, "test");
    temp_file.close().unwrap();
}

/// Test that a lowercase file content will change when its uppercase content is passed as the first argument and the --lowercase option is used
#[test]
fn test_rename_file_content_lowercase() {
    let temp_file = prepare_tmpdir();
    temp_file.child("file").write_str("sample").unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--lowercase")
        .arg("SAMPLE")
        .arg("TEST")
        .arg(temp_file.path())
        .assert()
        .success();

    let file_content: String = read_file_content(&temp_file.child("file"));
    assert_eq!(file_content, "test");
    temp_file.close().unwrap();
}

/// Test that a uppercase file content will change when its lowercase content is passed as the first argument and the --uppercase option is used
#[test]
fn test_rename_file_content_uppercase() {
    let temp_file = prepare_tmpdir();
    temp_file.child("file").write_str("SAMPLE").unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--uppercase")
        .arg("sample")
        .arg("test")
        .arg(temp_file.path())
        .assert()
        .success();

    let file_content: String = read_file_content(&temp_file.child("file"));
    assert_eq!(file_content, "TEST");
    temp_file.close().unwrap();
}

/// Test that a capitalize file content will change when its lowercase content is passed as the first argument and the --capitalize option is used
#[test]
fn test_rename_file_content_capitalize() {
    let temp_file = prepare_tmpdir();
    temp_file.child("file").write_str("Sample").unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("--capitalize")
        .arg("sample")
        .arg("test")
        .arg(temp_file.path())
        .assert()
        .success();

    let file_content: String = read_file_content(&temp_file.child("file"));
    assert_eq!(file_content, "Test");
    temp_file.close().unwrap();
}
