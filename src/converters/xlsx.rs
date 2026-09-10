use crate::error::{ConversionError, Result};
use crate::pdf::builder::{PdfBuilder, XlsxRow, XlsxSheet};
use calamine::{open_workbook_auto, Data, Reader};
use std::path::Path;

/// Convert an Excel spreadsheet (.xlsx, .xls, .ods) to PDF
pub fn convert(input_path: &Path, output_path: &Path, include_header: bool, include_footer: bool) -> Result<()> {
    let mut workbook = open_workbook_auto(input_path).map_err(|e| {
        ConversionError::UnsupportedFormat(format!("Failed to open spreadsheet {:?}: {:?}", input_path, e))
    })?;

    let filename = input_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("spreadsheet.xlsx");

    let sheet_names = workbook.sheet_names().to_vec();
    if sheet_names.is_empty() {
        log::warn!("Spreadsheet has no sheets: {:?}", input_path);
    }

    let mut sheets = Vec::new();

    for sheet_name in &sheet_names {
        if let Ok(range) = workbook.worksheet_range(sheet_name) {
            let mut rows = Vec::new();

            for r in range.rows() {
                // Check if row is completely empty
                let is_empty = r.iter().all(|cell| matches!(cell, Data::Empty));
                if is_empty {
                    continue;
                }

                let cells: Vec<String> = r
                    .iter()
                    .map(|cell| match cell {
                        Data::Empty => String::new(),
                        Data::String(s) => s.trim().to_string(),
                        Data::Float(f) => {
                            if f.fract() == 0.0 {
                                format!("{:.0}", f)
                            } else {
                                format!("{:.2}", f)
                            }
                        }
                        Data::Int(i) => i.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::DateTime(d) => format!("{:.2}", d),
                        Data::DateTimeIso(s) | Data::DurationIso(s) => s.clone(),
                        Data::Error(e) => format!("#ERR:{:?}", e),
                    })
                    .collect();

                rows.push(XlsxRow { cells });
            }

            if !rows.is_empty() {
                sheets.push(XlsxSheet {
                    name: sheet_name.clone(),
                    rows,
                });
            }
        }
    }

    if sheets.is_empty() {
        sheets.push(XlsxSheet {
            name: "Sheet1".to_string(),
            rows: vec![XlsxRow {
                cells: vec!["(Empty Worksheet)".to_string()],
            }],
        });
    }

    let mut pdf = PdfBuilder::new(&format!("Spreadsheet: {}", filename))?;
    pdf.add_xlsx_pages(&sheets, filename, include_header, include_footer)?;
    pdf.save(output_path)?;

    Ok(())
}
