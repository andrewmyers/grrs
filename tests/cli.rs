use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut command = Command::cargo_bin("grrs")?;
    command.arg("test").arg(file.path());
    command.assert()
        .success()
        .stdout(predicate::str::contains("1: A test\n4: Another test"));

    Ok(())
}

#[test]
fn pattern_is_empty() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;

    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut command = Command::cargo_bin("grrs")?;
    command.arg("").arg(file.path());
    command.assert()
        .success()
        .stdout(predicate::str::contains("1: A test\n2: Actual content\n3: More content\n4: Another test"));

    Ok(())
}
