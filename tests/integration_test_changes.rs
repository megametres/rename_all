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
