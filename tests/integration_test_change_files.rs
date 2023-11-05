use assert_fs::prelude::*;
use std::fs::remove_file;
mod test_helper;
use test_helper::{prepare_cmd, prepare_tmpdir, read_file_content};



#[test]
fn test_rename_file() {
    let temp_file = test_helper::prepare_tmpdir();
    temp_file.child("file").write_str("sample").unwrap();

    let mut cmd = prepare_cmd();
    cmd.arg("sample")
        .arg("test")
        .arg(temp_file.path())
        .assert()
        .success();

    let file_content: String  = read_file_content(&temp_file.child("file"));
    assert_eq!(file_content, "test");
    temp_file.close().unwrap();
}

