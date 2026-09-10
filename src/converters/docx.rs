use crate::error::{ConversionError, Result};
use crate::pdf::builder::{DocxBlock, DocxTableCell, DocxTableRow, MdSpan, PdfBuilder};
use docx_rs::*;
use std::path::Path;

/// Convert a DOCX file to PDF
pub fn convert(input_path: &Path, output_path: &Path, include_header: bool, include_footer: bool) -> Result<()> {
    // Read docx file into bytes
    let bytes = std::fs::read(input_path).map_err(|e| ConversionError::FileRead {
        path: input_path.to_path_buf(),
        source: e,
    })?;

    if bytes.is_empty() {
        log::warn!("Input DOCX file is empty: {:?}", input_path);
    }

    // Parse DOCX using native docx-rs
    let docx = read_docx(&bytes).map_err(|e| {
        ConversionError::UnsupportedFormat(format!("Failed to parse DOCX file: {:?}", e))
    })?;

    let filename = input_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("document.docx");

    let blocks = extract_docx_blocks(&docx);

    let mut pdf = PdfBuilder::new(&format!("DOCX Document: {}", filename))?;
    pdf.add_docx_pages(&blocks, filename, include_header, include_footer)?;
    pdf.save(output_path)?;

    Ok(())
}

fn extract_docx_blocks(docx: &Docx) -> Vec<DocxBlock> {
    let mut blocks = Vec::new();

    for child in &docx.document.children {
        match child {
            DocumentChild::Paragraph(p) => {
                if let Some(block) = parse_paragraph(p) {
                    blocks.push(block);
                }
            }
            DocumentChild::Table(t) => {
                let table_block = parse_table(t);
                if !table_block.rows.is_empty() {
                    blocks.push(DocxBlock::Table(table_block));
                }
            }
            _ => {}
        }
    }

    blocks
}

fn parse_paragraph(p: &Paragraph) -> Option<DocxBlock> {
    let style = p.property.style.as_ref().map(|s| s.val.to_lowercase());
    let spans = extract_spans_from_paragraph(p);

    if spans.is_empty() {
        return None;
    }

    // Check heading level by style
    let heading_level = if let Some(ref s) = style {
        if s.contains("heading 1") || s == "heading1" || s == "title" {
            Some(1)
        } else if s.contains("heading 2") || s == "heading2" || s == "subtitle" {
            Some(2)
        } else if s.contains("heading 3") || s == "heading3" {
            Some(3)
        } else if s.contains("heading 4") || s == "heading4" {
            Some(4)
        } else {
            None
        }
    } else {
        None
    };

    if let Some(lvl) = heading_level {
        let full_text = spans.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join("");
        Some(DocxBlock::Heading {
            level: lvl,
            text: full_text,
        })
    } else {
        Some(DocxBlock::Paragraph { spans })
    }
}

fn extract_spans_from_paragraph(p: &Paragraph) -> Vec<MdSpan> {
    let mut spans = Vec::new();
    extract_spans_from_paragraph_children(&p.children, &mut spans);
    spans
}

fn extract_spans_from_paragraph_children(children: &[ParagraphChild], spans: &mut Vec<MdSpan>) {
    for child in children {
        match child {
            ParagraphChild::Run(r) => {
                let is_bold = r.run_property.bold.is_some();
                let is_italic = r.run_property.italic.is_some();

                for rc in &r.children {
                    match rc {
                        RunChild::Text(t) => {
                            if !t.text.is_empty() {
                                spans.push(MdSpan {
                                    text: t.text.clone(),
                                    is_bold,
                                    is_italic,
                                    is_code: false,
                                });
                            }
                        }
                        RunChild::Break(_) => {
                            spans.push(MdSpan {
                                text: "\n".to_string(),
                                is_bold: false,
                                is_italic: false,
                                is_code: false,
                            });
                        }
                        RunChild::Tab(_) => {
                            spans.push(MdSpan {
                                text: "    ".to_string(),
                                is_bold: false,
                                is_italic: false,
                                is_code: false,
                            });
                        }
                        _ => {}
                    }
                }
            }
            ParagraphChild::Hyperlink(h) => {
                extract_spans_from_paragraph_children(&h.children, spans);
            }
            _ => {}
        }
    }
}

fn parse_table(t: &Table) -> crate::pdf::builder::DocxTable {
    let mut rows = Vec::new();

    for child in &t.rows {
        let TableChild::TableRow(tr) = child;
        let mut cells = Vec::new();

        for cell_child in &tr.cells {
            let TableRowChild::TableCell(tc) = cell_child;
            let mut cell_spans = Vec::new();

            for content in &tc.children {
                if let TableCellContent::Paragraph(p) = content {
                    let spans = extract_spans_from_paragraph(p);
                    if !cell_spans.is_empty() && !spans.is_empty() {
                        cell_spans.push(MdSpan {
                            text: " ".to_string(),
                            is_bold: false,
                            is_italic: false,
                            is_code: false,
                        });
                    }
                    cell_spans.extend(spans);
                }
            }

            cells.push(DocxTableCell { spans: cell_spans });
        }

        rows.push(DocxTableRow { cells });
    }

    crate::pdf::builder::DocxTable { rows }
}
