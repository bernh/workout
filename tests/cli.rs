use assert_cmd::Command;

#[test]
fn cli_workout() {
    let mut cmd = Command::cargo_bin("workout").unwrap();
    cmd.arg("-c")
        .arg("tests/paces.toml")
        .arg("-w")
        .arg("10E + 3 * (2 min I + 2 min rst)");
    cmd.assert()
        .success()
        .stdout("11.9 km, 1:08 h, 5:45 min/km\n");
}
