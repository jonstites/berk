use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use tempfile::{tempdir, tempfile};

#[test]
fn init_dir_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    let dir = tempdir()?;
    let test_file = dir.path().join("test/file/doesnt/exist");
    cmd.arg("init")
        .arg(test_file);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));

    Ok(())
}

#[test]
fn init_repo() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::main_binary()?;
    let dir = tempdir()?;
    cmd.arg("init")
        .arg(dir.path());
    cmd.assert()
        .success();
    
    Ok(())
}
