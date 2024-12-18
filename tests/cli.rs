use anyhow::Ok;
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), anyhow::Error> {
    let mut cli = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let file_path: &str = "test/file/does/not/exist";
    cli.arg("Patten").arg(&file_path);

    cli.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Could not read file: {}",
            file_path
        )));

    Ok(())
}

#[test]
fn should_return_matched_lines() -> Result<(), anyhow::Error> {
    let temp_file = assert_fs::NamedTempFile::new("test/file/name")?;
    temp_file.write_str("a test\n newTe\nanother test")?;

    let mut cli = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cli.arg("test").arg(temp_file.path());

    cli.assert()
        .success()
        .stdout(predicate::str::contains("a test\nanother test"));

    Ok(())
}
