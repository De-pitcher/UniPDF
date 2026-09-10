// Integration tests for UniPDF
// These tests verify end-to-end conversion functionality

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("zero-dependency CLI tool"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_convert_text_file() {
    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    let input = "test_files/sample.txt";
    let output = "test_outputs/sample.pdf";

    // Ensure test_outputs directory exists
    std::fs::create_dir_all("test_outputs").ok();

    cmd.arg("convert")
        .arg(input)
        .arg("--output")
        .arg(output)
        .assert()
        .success();

    // Verify output file was created
    assert!(Path::new(output).exists());
    
    // Verify file has content
    let metadata = std::fs::metadata(output).unwrap();
    assert!(metadata.len() > 0, "PDF file is empty");

    // Cleanup
    std::fs::remove_file(output).ok();
}

#[test]
fn test_convert_nonexistent_file() {
    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    
    cmd.arg("convert")
        .arg("nonexistent.txt")
        .assert()
        .failure()
        .code(1);
}

#[test]
fn test_convert_markdown_file() {
    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    let input = "test_files/sample.md";
    let output = "test_outputs/sample_md.pdf";

    std::fs::create_dir_all("test_outputs").ok();

    cmd.arg("convert")
        .arg(input)
        .arg("--output")
        .arg(output)
        .assert()
        .success();

    assert!(Path::new(output).exists());
    let metadata = std::fs::metadata(output).unwrap();
    assert!(metadata.len() > 0, "Generated Markdown PDF is empty");

    std::fs::remove_file(output).ok();
}
