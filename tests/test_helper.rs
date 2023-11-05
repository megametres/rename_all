use assert_cmd::Command;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn prepare_cmd() -> assert_cmd::Command {
    return Command::cargo_bin("rename_all").unwrap();
}

pub fn prepare_tmpdir() -> assert_fs::TempDir {
    return assert_fs::TempDir::new().unwrap();
}

pub fn read_file_content(file_path: &Path) -> String {
    let mut read_file = File::open(file_path).expect("Unable to open the file");
    let mut file_content = String::new();
    let _ = read_file.read_to_string(&mut file_content);
    file_content
}
