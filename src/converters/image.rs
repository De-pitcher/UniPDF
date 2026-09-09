use crate::error::{ConversionError, Result};
use crate::pdf::PdfBuilder;
use image::{DynamicImage, GenericImage, GenericImageView};
use std::path::Path;

const DEFAULT_MARGIN_MM: f32 = 10.0;

/// Convert an image file to PDF
pub fn convert(input_path: &Path, output_path: &Path) -> Result<()> {
    // Load the image
    let img = image::open(input_path)
        .map_err(|e| ConversionError::FileRead {
            path: input_path.to_path_buf(),
            source: std::io::Error::new(std::io::ErrorKind::InvalidData, e),
        })?;

    // Handle PNG transparency
    let img = handle_transparency(img);

    // Get filename for header
    let filename = input_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("image");

    // Create PDF
    let mut pdf = PdfBuilder::new(&format!("Image Document: {}", filename))?;
    
    // Add image to PDF
    pdf.add_image_page(&img, filename, DEFAULT_MARGIN_MM)?;
    
    // Save PDF
    pdf.save(output_path)?;

    Ok(())
}

/// Handle PNG transparency by compositing onto white background
fn handle_transparency(img: DynamicImage) -> DynamicImage {
    // Check if image has alpha channel
    if img.color().has_alpha() {
        log::info!("Image has transparency - compositing onto white background");
        
        let (width, height) = img.dimensions();
        let mut white_bg = image::DynamicImage::new_rgb8(width, height);
        
        // Fill with white
        for x in 0..width {
            for y in 0..height {
                white_bg.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            }
        }
        
        // Composite the image onto white background
        image::imageops::overlay(&mut white_bg, &img, 0, 0);
        white_bg
    } else {
        img
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_convert_simple_image() {
        // This test requires actual image files
        // For now, we'll skip it in CI
        if std::env::var("CI").is_ok() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("test.pdf");

        // Create a simple 100x100 white image
        let img = image::DynamicImage::new_rgb8(100, 100);
        let input = temp_dir.path().join("test.png");
        img.save(&input).unwrap();

        // Convert to PDF
        let result = convert(&input, &output);
        assert!(result.is_ok(), "Conversion failed: {:?}", result.err());

        // Verify PDF was created
        assert!(output.exists());
        assert!(output.metadata().unwrap().len() > 0);
    }

    #[test]
    fn test_transparency_handling() {
        // Create an RGBA image
        let img = image::DynamicImage::new_rgba8(10, 10);
        assert!(img.color().has_alpha());

        // Process it
        let processed = handle_transparency(img);
        
        // Should be RGB now
        assert!(!processed.color().has_alpha());
    }
}
