use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*; // Used for writing assertions

#[test]
fn help_emits_usage() -> Result<()> {
    let mut cmd = Command::cargo_bin("digdug")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn lookup_localhost() -> Result<()> {
    let mut cmd = Command::cargo_bin("digdug")?;

    cmd.arg("127.0.0.1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("localhost"));

    Ok(())
}


#[test]
fn lookup_invalid() -> Result<()> {
    let mut cmd = Command::cargo_bin("digdug")?;

    cmd.arg("not.an.ip.address");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid"));

    Ok(())
}
