use crate::error::{ConversionError, Result};
use printpdf::{PdfDocument, PdfDocumentReference, PdfPageIndex, PdfLayerIndex, PdfLayerReference};
use printpdf::{Mm, BuiltinFont, IndirectFontRef, Image};
use std::path::Path;
extern crate image;

// Page dimensions in mm
const A4_WIDTH_MM: f32 = 210.0;
const A4_HEIGHT_MM: f32 = 297.0;

// Margins in mm
const MARGIN_TOP_MM: f32 = 25.0;
const MARGIN_BOTTOM_MM: f32 = 25.0;
const MARGIN_LEFT_MM: f32 = 20.0;
const MARGIN_RIGHT_MM: f32 = 20.0;

// Text settings
const FONT_SIZE: f32 = 10.0;
const LINE_HEIGHT: f32 = 4.0; // in mm
const CHARS_PER_LINE: usize = 80;

// Courier font metrics at 10pt (monospace)
// 1 point = 0.3527778 mm, Courier character width ≈ 0.6 em = 6 points
const COURIER_CHAR_WIDTH_MM: f32 = 2.117; // Accurate width per character

pub struct PdfBuilder {
    doc: PdfDocumentReference,
    font: IndirectFontRef,
    first_page: PdfPageIndex,
    first_layer: PdfLayerIndex,
}

impl PdfBuilder {
    pub fn new(title: &str) -> Result<Self> {
        // Create PDF document
        let (doc, page1, layer1) = PdfDocument::new(title, Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
        
        // Load built-in font (we'll use Courier for monospace)
        let font = doc.add_builtin_font(BuiltinFont::Courier)
            .map_err(|e| ConversionError::FontError(format!("Failed to load font: {:?}", e)))?;

        Ok(Self {
            doc,
            font,
            first_page: page1,
            first_layer: layer1,
        })
    }

    pub fn add_text_pages(&mut self, content: &str, filename: &str, include_header: bool, include_footer: bool) -> Result<()> {
        // Calculate dynamic margins and content area based on what's enabled
        let (top_margin, _bottom_margin, lines_per_page) = Self::calculate_layout(include_header, include_footer);
        
        let lines: Vec<&str> = content.lines().collect();
        let total_pages = (lines.len() + lines_per_page - 1) / lines_per_page;
        let total_pages = total_pages.max(1); // At least 1 page

        for (page_num, chunk) in lines.chunks(lines_per_page).enumerate() {
            let (page_idx, layer_idx) = if page_num == 0 {
                // Use the first page that was created with the document
                (self.first_page, self.first_layer)
            } else {
                // Add new pages
                self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1")
            };

            let current_layer = self.doc.get_page(page_idx).get_layer(layer_idx);
            
            // Add header (filename and page number) if enabled
            if include_header {
                self.add_header(&current_layer, filename, page_num + 1, total_pages)?;
            }

            // Add text content
            self.add_text_content(&current_layer, chunk, top_margin)?;

            // Add footer (timestamp) if enabled
            if include_footer {
                self.add_footer(&current_layer)?;
            }
        }

        Ok(())
    }

    /// Calculate optimal layout based on enabled features
    /// Returns: (top_margin, bottom_margin, lines_per_page)
    fn calculate_layout(include_header: bool, include_footer: bool) -> (f32, f32, usize) {
        // Extra space needed for header/footer
        let header_space = if include_header { 10.0 } else { 0.0 };
        let footer_space = if include_footer { 10.0 } else { 0.0 };
        
        let top_margin = MARGIN_TOP_MM - (if include_header { 0.0 } else { 10.0 });
        let bottom_margin = MARGIN_BOTTOM_MM - (if include_footer { 0.0 } else { 10.0 });
        
        // Calculate available vertical space for text content
        let total_content_height = A4_HEIGHT_MM - top_margin - bottom_margin - header_space - footer_space;
        let lines_per_page = (total_content_height / LINE_HEIGHT).floor() as usize;
        
        (top_margin, bottom_margin, lines_per_page)
    }

    fn add_header(&self, layer: &PdfLayerReference, filename: &str, page_num: usize, total_pages: usize) -> Result<()> {
        let header_y = A4_HEIGHT_MM - 15.0; // 15mm from top
        
        // Left side: filename
        layer.use_text(
            filename,
            FONT_SIZE,
            Mm(MARGIN_LEFT_MM),
            Mm(header_y),
            &self.font,
        );

        // Right side: page number
        let page_text = format!("Page {} of {}", page_num, total_pages);
        let text_width_mm = page_text.len() as f32 * COURIER_CHAR_WIDTH_MM;
        layer.use_text(
            &page_text,
            FONT_SIZE,
            Mm(A4_WIDTH_MM - MARGIN_RIGHT_MM - text_width_mm),
            Mm(header_y),
            &self.font,
        );

        Ok(())
    }

    fn add_text_content(&self, layer: &PdfLayerReference, lines: &[&str], top_margin: f32) -> Result<()> {
        let start_y = A4_HEIGHT_MM - top_margin - 10.0; // Below header if present

        let mut line_count = 0;
        for line in lines.iter() {
            // Skip wrapping if line is already short enough
            if line.len() <= CHARS_PER_LINE {
                let y = start_y - (line_count as f32 * LINE_HEIGHT);
                layer.use_text(
                    *line,
                    FONT_SIZE,
                    Mm(MARGIN_LEFT_MM),
                    Mm(y),
                    &self.font,
                );
                line_count += 1;
            } else {
                // Wrap long lines
                let wrapped = wrap_line(line, CHARS_PER_LINE);
                for wrapped_line in wrapped.iter() {
                    let y = start_y - (line_count as f32 * LINE_HEIGHT);
                    layer.use_text(
                        wrapped_line,
                        FONT_SIZE,
                        Mm(MARGIN_LEFT_MM),
                        Mm(y),
                        &self.font,
                    );
                    line_count += 1;
                }
            }
        }

        Ok(())
    }

    fn add_footer(&self, layer: &PdfLayerReference) -> Result<()> {
        let footer_y = MARGIN_BOTTOM_MM - 5.0;
        
        // Generate timestamp
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let footer_text = format!("Generated: {}", timestamp);
        
        // Center the footer using accurate character width
        let text_width_mm = footer_text.len() as f32 * COURIER_CHAR_WIDTH_MM;
        let x = (A4_WIDTH_MM - text_width_mm) / 2.0;
        
        layer.use_text(
            &footer_text,
            FONT_SIZE - 2.0, // Slightly smaller
            Mm(x),
            Mm(footer_y),
            &self.font,
        );

        Ok(())
    }

    pub fn add_image_page(&mut self, img: &image::DynamicImage, filename: &str, margin_mm: f32) -> Result<()> {
        use image::GenericImageView;
        
        // Get image dimensions
        let (img_width, img_height) = img.dimensions();
        
        // Calculate available space on page (minus margins)
        let available_width = A4_WIDTH_MM - (2.0 * margin_mm);
        let available_height = A4_HEIGHT_MM - (2.0 * margin_mm) - 20.0; // Extra space for header
        
        // Calculate scaling to fit image while maintaining aspect ratio
        let width_scale = available_width / (img_width as f32);
        let height_scale = available_height / (img_height as f32);
        let scale = width_scale.min(height_scale);
        
        let scaled_width = (img_width as f32) * scale;
        let scaled_height = (img_height as f32) * scale;
        
        // Center the image on the page
        let x = (A4_WIDTH_MM - scaled_width) / 2.0;
        let y = A4_HEIGHT_MM - margin_mm - 10.0 - scaled_height; // Below header
        
        // Convert image to RGB8 format for PDF
        let rgb_img = img.to_rgb8();
        
        // Create image object for PDF
        let image_obj = Image::from_dynamic_image(&image::DynamicImage::ImageRgb8(rgb_img));
        
        // Add image to first page (or create new page for batch)
        let current_layer = self.doc.get_page(self.first_page).get_layer(self.first_layer);
        
        // Add header
        self.add_image_header(&current_layer, filename)?;
        
        // Add the image
        image_obj.add_to_layer(
            current_layer.clone(),
            printpdf::ImageTransform {
                translate_x: Some(Mm(x)),
                translate_y: Some(Mm(y)),
                scale_x: Some(scale),
                scale_y: Some(scale),
                ..Default::default()
            },
        );
        
        Ok(())
    }

    fn add_image_header(&self, layer: &PdfLayerReference, filename: &str) -> Result<()> {
        let header_y = A4_HEIGHT_MM - 15.0;
        
        layer.use_text(
            filename,
            FONT_SIZE,
            Mm(MARGIN_LEFT_MM),
            Mm(header_y),
            &self.font,
        );
        
        Ok(())
    }

    pub fn save(self, path: &Path) -> Result<()> {
        let file = std::fs::File::create(path)
            .map_err(|e| ConversionError::FileWrite {
                path: path.to_path_buf(),
                source: e,
            })?;

        self.doc.save(&mut std::io::BufWriter::new(file))
            .map_err(|e| ConversionError::PdfGeneration(format!("Failed to save PDF: {:?}", e)))?;

        Ok(())
    }
}

// Helper function to wrap lines at character limit
fn wrap_line(line: &str, max_chars: usize) -> Vec<String> {
    // Estimate number of wrapped lines for pre-allocation
    let estimated_lines = (line.len() / max_chars) + 1;
    let mut result = Vec::with_capacity(estimated_lines);
    let mut current = String::with_capacity(max_chars);

    for word in line.split_whitespace() {
        if current.len() + word.len() + 1 > max_chars {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
                current = String::with_capacity(max_chars);
            }
            
            // If a single word is longer than max_chars, split it
            if word.len() > max_chars {
                let chars: Vec<char> = word.chars().collect();
                for chunk in chars.chunks(max_chars) {
                    result.push(chunk.iter().collect());
                }
            } else {
                current.push_str(word);
            }
        } else {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    if result.is_empty() {
        result.push(String::new());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_line_short() {
        let line = "This is a short line";
        let result = wrap_line(line, 80);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "This is a short line");
    }

    #[test]
    fn test_wrap_line_long() {
        let line = "This is a very long line that exceeds eighty characters and should be wrapped into multiple lines automatically";
        let result = wrap_line(line, 80);
        assert!(result.len() > 1);
        for wrapped in &result {
            assert!(wrapped.len() <= 80);
        }
    }

    #[test]
    fn test_wrap_line_exact() {
        let line = "1234567890".repeat(8); // Exactly 80 chars
        let result = wrap_line(&line, 80);
        assert_eq!(result.len(), 1);
    }
}
