use assert_cmd::Command;

fn prepare_cmd() -> assert_cmd::Command {
    return Command::cargo_bin("rename_all").unwrap();
}

#[test]
fn test_no_args() {
    let mut cmd = prepare_cmd();

    // Fail if no args are present
    cmd.assert().failure();
}

#[test]
fn test_3_args_with_existing_path() {
    let mut cmd = prepare_cmd();

    // Assert that 3 args are shouuld pass if third argd
    cmd.arg("search").arg("replace").arg(".").assert().success();
}

#[test]
fn test_3_args_with_non_existing_path() {
    let mut cmd = prepare_cmd();

    // Fail if the third argument is not a real path
    cmd.arg("search")
        .arg("replace")
        .arg("path_that_does_not_exist")
        .assert()
        .failure();
}
