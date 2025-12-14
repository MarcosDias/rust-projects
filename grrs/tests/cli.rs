use assert_cmd::cargo_bin_cmd;
use assert_fs::fixture::FileWriteStr;
use predicates::prelude::*;

#[test]
fn file_doesnt_exist() -> anyhow::Result<()> {
    let mut cmd = cargo_bin_cmd!("grrs");
    cmd.arg("pattern").arg("non_existent_file.txt");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("failed to read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("grrs");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}