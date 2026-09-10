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
        .stdout(predicate::str::contains("1.0.0"));
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

#[test]
fn test_batch_conversion() {
    let out_dir = "test_outputs/batch_run";
    std::fs::create_dir_all(out_dir).ok();

    let mut cmd = Command::cargo_bin("unipdf").unwrap();
    cmd.arg("batch")
        .arg("--pattern")
        .arg("test_files/*.txt")
        .arg("--output-dir")
        .arg(out_dir)
        .arg("--threads")
        .arg("1")
        .assert()
        .success();

    // Verify converted files exist
    assert!(Path::new("test_outputs/batch_run/sample.pdf").exists());
    assert!(Path::new("test_outputs/batch_run/comprehensive.pdf").exists());

    // Clean up
    std::fs::remove_dir_all(out_dir).ok();
}

#[test]
fn test_merge_pdfs() {
    let pdf1 = "test_outputs/merge_in1.pdf";
    let pdf2 = "test_outputs/merge_in2.pdf";
    let merged = "test_outputs/merged_result.pdf";

    std::fs::create_dir_all("test_outputs").ok();

    // Create two simple PDFs to merge
    let mut cmd1 = Command::cargo_bin("unipdf").unwrap();
    cmd1.arg("convert").arg("test_files/short.txt").arg("--output").arg(pdf1).assert().success();

    let mut cmd2 = Command::cargo_bin("unipdf").unwrap();
    cmd2.arg("convert").arg("test_files/sample.txt").arg("--output").arg(pdf2).assert().success();

    // Execute Merge command
    let mut merge_cmd = Command::cargo_bin("unipdf").unwrap();
    merge_cmd.arg("merge")
        .arg(pdf1)
        .arg(pdf2)
        .arg("--output")
        .arg(merged)
        .assert()
        .success();

    assert!(Path::new(merged).exists());
    let metadata = std::fs::metadata(merged).unwrap();
    assert!(metadata.len() > 0, "Merged PDF is empty");

    let bytes = std::fs::read(merged).unwrap();
    assert!(bytes.starts_with(b"%PDF-"), "Merged output is not a valid PDF");

    // Clean up
    std::fs::remove_file(pdf1).ok();
    std::fs::remove_file(pdf2).ok();
    std::fs::remove_file(merged).ok();
}



