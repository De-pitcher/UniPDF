use docx_rs::*;
use std::fs::File;
use std::path::Path;

pub fn create_test_docx(path: &Path) {
    let file = File::create(path).unwrap();

    let docx = Docx::new()
        .add_paragraph(
            Paragraph::new()
                .style("Heading 1")
                .add_run(Run::new().add_text("UniPDF Native DOCX Conversion")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("This document was generated and parsed completely "))
                .add_run(Run::new().bold().add_text("without Microsoft Word or LibreOffice"))
                .add_run(Run::new().add_text(". It is 100% native Rust."))
        )
        .add_paragraph(
            Paragraph::new()
                .style("Heading 2")
                .add_run(Run::new().add_text("Key Architecture Features")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().italic().add_text("UniPDF extracts paragraphs, inline formatting (bold/italic), and tables directly from OpenXML AST into printpdf vectors."))
        )
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Feature"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Status"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Implementation"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Headings"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Supported"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("H1 - H4 with dynamic styling"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Tables"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Supported"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Auto-column widths and cell grids"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Formatting"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Supported"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bold, italic, line breaks"))),
                ]),
            ])
        );

    docx.build().pack(file).unwrap();
}

pub fn create_complex_docx(path: &Path) {
    let file = File::create(path).unwrap();

    let mut docx = Docx::new()
        .add_paragraph(
            Paragraph::new()
                .style("Title")
                .add_run(Run::new().add_text("Comprehensive DOCX Stress Test Report")),
        )
        .add_paragraph(
            Paragraph::new()
                .style("Subtitle")
                .add_run(Run::new().italic().add_text("Multi-page Document with Mixed Styles and Deep Tables")),
        )
        .add_paragraph(
            Paragraph::new()
                .style("Heading 1")
                .add_run(Run::new().add_text("1. Executive Summary")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("UniPDF is evaluated against large-scale technical documents. This section verifies long paragraphs, multiline wrapping, and consecutive styled runs."))
        );

    for i in 1..=5 {
        docx = docx.add_paragraph(
            Paragraph::new()
                .style("Heading 2")
                .add_run(Run::new().add_text(format!("Section {}: Performance & Stress Metrics", i))),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. "))
                .add_run(Run::new().bold().add_text("Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. "))
                .add_run(Run::new().italic().add_text("Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."))
        );
    }

    // Add multi-column matrix table
    let mut table_rows = vec![
        TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Metric ID"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Parameter Name"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Threshold"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Result"))),
        ])
    ];

    for row_idx in 1..=12 {
        table_rows.push(TableRow::new(vec![
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(format!("MTR-{:03}", row_idx)))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text(format!("Memory footprint under workload #{}", row_idx)))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("< 128 MB"))),
            TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("PASS"))),
        ]));
    }

    docx = docx.add_table(Table::new(table_rows));

    docx.build().pack(file).unwrap();
}
