use crate::error::{ConversionError, Result};
use crate::pdf::PdfBuilder;
use std::path::Path;

/// Convert a text file to PDF
pub fn convert(input_path: &Path, output_path: &Path) -> Result<()> {
    // Read the text file
    let content = std::fs::read_to_string(input_path)
        .map_err(|e| ConversionError::FileRead {
            path: input_path.to_path_buf(),
            source: e,
        })?;

    // Validate UTF-8 (already done by read_to_string, but we can add custom handling)
    if content.is_empty() {
        log::warn!("Input file is empty: {:?}", input_path);
    }

    // Get filename for header
    let filename = input_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("document.txt");

    // Create PDF
    let mut pdf = PdfBuilder::new(&format!("Text Document: {}", filename))?;
    
    // Add content to PDF
    pdf.add_text_pages(&content, filename)?;
    
    // Save PDF
    pdf.save(output_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_convert_simple_text() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("test.txt");
        let output = temp_dir.path().join("test.pdf");

        // Create a simple text file
        fs::write(&input, "Hello, World!\nThis is a test.").unwrap();

        // Convert to PDF
        let result = convert(&input, &output);
        assert!(result.is_ok(), "Conversion failed: {:?}", result.err());

        // Verify PDF was created
        assert!(output.exists());
        assert!(output.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_convert_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("empty.txt");
        let output = temp_dir.path().join("empty.pdf");

        // Create an empty file
        fs::write(&input, "").unwrap();

        // Should still work
        let result = convert(&input, &output);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_convert_long_lines() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("long.txt");
        let output = temp_dir.path().join("long.pdf");

        // Create a file with a very long line
        let long_line = "This is a very long line that exceeds eighty characters and should be wrapped automatically by the PDF generator. ".repeat(5);
        fs::write(&input, long_line).unwrap();

        let result = convert(&input, &output);
        assert!(result.is_ok());
        assert!(output.exists());
    }

    #[test]
    fn test_convert_utf8_characters() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("utf8.txt");
        let output = temp_dir.path().join("utf8.pdf");

        // Test with UTF-8 characters
        fs::write(&input, "Héllo Wörld!\nCafé ☕\n日本語").unwrap();

        let result = convert(&input, &output);
        assert!(result.is_ok());
        assert!(output.exists());
    }
}
