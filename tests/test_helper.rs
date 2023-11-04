use assert_cmd::Command;

pub fn prepare_cmd() -> assert_cmd::Command {
    return Command::cargo_bin("rename_all").unwrap();
}

pub fn prepare_tmpdir() -> assert_fs::TempDir {
    return assert_fs::TempDir::new().unwrap();
}
