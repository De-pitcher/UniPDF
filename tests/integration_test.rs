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

mod generate_docx_fixtures;

#[test]
fn test_convert_docx_file() {
    let input = "test_files/sample.docx";
    let output = "test_outputs/sample_docx.pdf";

    std::fs::create_dir_all("test_files").ok();
    std::fs::create_dir_all("test_outputs").ok();

    // Generate fixture if not present
    generate_docx_fixtures::create_test_docx(Path::new(input));

    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    cmd.arg("convert")
        .arg(input)
        .arg("--output")
        .arg(output)
        .assert()
        .success();

    assert!(Path::new(output).exists());
    let metadata = std::fs::metadata(output).unwrap();
    assert!(metadata.len() > 0, "Generated DOCX PDF is empty");

    // Verify PDF signature (%PDF-)
    let bytes = std::fs::read(output).unwrap();
    assert!(bytes.starts_with(b"%PDF-"), "Output is not a valid PDF file");

    std::fs::remove_file(output).ok();
}

#[test]
fn test_convert_complex_docx_stress() {
    let input = "test_files/complex_stress_test.docx";
    let output = "test_outputs/complex_stress_docx.pdf";

    std::fs::create_dir_all("test_files").ok();
    std::fs::create_dir_all("test_outputs").ok();

    // Generate complex multi-page DOCX fixture
    generate_docx_fixtures::create_complex_docx(Path::new(input));

    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    cmd.arg("convert")
        .arg(input)
        .arg("--output")
        .arg(output)
        .assert()
        .success();

    assert!(Path::new(output).exists());
    let metadata = std::fs::metadata(output).unwrap();
    assert!(metadata.len() > 1000, "Generated complex DOCX PDF is too small");

    let bytes = std::fs::read(output).unwrap();
    assert!(bytes.starts_with(b"%PDF-"), "Output is not a valid PDF file");

    std::fs::remove_file(output).ok();
}

#[test]
fn test_convert_xlsx_file() {
    let input = "test_files/sample.xlsx";
    let output = "test_outputs/sample_xlsx_test.pdf";

    std::fs::create_dir_all("test_files").ok();
    std::fs::create_dir_all("test_outputs").ok();

    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    cmd.arg("convert")
        .arg(input)
        .arg("--output")
        .arg(output)
        .assert()
        .success();

    assert!(Path::new(output).exists());
    let metadata = std::fs::metadata(output).unwrap();
    assert!(metadata.len() > 0, "Generated XLSX PDF is empty");

    let bytes = std::fs::read(output).unwrap();
    assert!(bytes.starts_with(b"%PDF-"), "Output is not a valid PDF file");

    std::fs::remove_file(output).ok();
}

#[test]
fn test_convert_complex_xlsx_stress() {
    let input = "test_files/complex_stress_test.xlsx";
    let output = "test_outputs/complex_stress_xlsx_test.pdf";

    std::fs::create_dir_all("test_files").ok();
    std::fs::create_dir_all("test_outputs").ok();

    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    cmd.arg("convert")
        .arg(input)
        .arg("--output")
        .arg(output)
        .assert()
        .success();

    assert!(Path::new(output).exists());
    let metadata = std::fs::metadata(output).unwrap();
    assert!(metadata.len() > 1000, "Generated complex XLSX PDF is too small");

    let bytes = std::fs::read(output).unwrap();
    assert!(bytes.starts_with(b"%PDF-"), "Output is not a valid PDF file");

    std::fs::remove_file(output).ok();
}


