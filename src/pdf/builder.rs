use crate::error::{ConversionError, Result};
use printpdf::{PdfDocument, PdfDocumentReference, PdfPageIndex, PdfLayerIndex, PdfLayerReference};
use printpdf::{Mm, BuiltinFont, IndirectFontRef, Image, Line, Point, Rgb, Color};
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
    pub font_helvetica: IndirectFontRef,
    pub font_helvetica_bold: IndirectFontRef,
    pub font_helvetica_oblique: IndirectFontRef,
    pub font_helvetica_bold_oblique: IndirectFontRef,
    pub font_courier: IndirectFontRef,
    pub font_courier_bold: IndirectFontRef,
    first_page: PdfPageIndex,
    first_layer: PdfLayerIndex,
}

impl PdfBuilder {
    pub fn new(title: &str) -> Result<Self> {
        // Create PDF document
        let (doc, page1, layer1) = PdfDocument::new(title, Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
        
        // Load built-in fonts
        let font_courier = doc.add_builtin_font(BuiltinFont::Courier)
            .map_err(|e| ConversionError::FontError(format!("Failed to load font: {:?}", e)))?;
        let font_courier_bold = doc.add_builtin_font(BuiltinFont::CourierBold)
            .map_err(|e| ConversionError::FontError(format!("Failed to load font: {:?}", e)))?;
        let font_helvetica = doc.add_builtin_font(BuiltinFont::Helvetica)
            .map_err(|e| ConversionError::FontError(format!("Failed to load font: {:?}", e)))?;
        let font_helvetica_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
            .map_err(|e| ConversionError::FontError(format!("Failed to load font: {:?}", e)))?;
        let font_helvetica_oblique = doc.add_builtin_font(BuiltinFont::HelveticaOblique)
            .map_err(|e| ConversionError::FontError(format!("Failed to load font: {:?}", e)))?;
        let font_helvetica_bold_oblique = doc.add_builtin_font(BuiltinFont::HelveticaBoldOblique)
            .map_err(|e| ConversionError::FontError(format!("Failed to load font: {:?}", e)))?;

        Ok(Self {
            doc,
            font: font_courier.clone(),
            font_helvetica,
            font_helvetica_bold,
            font_helvetica_oblique,
            font_helvetica_bold_oblique,
            font_courier,
            font_courier_bold,
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

    pub fn add_markdown_pages(
        &mut self,
        blocks: &[MdBlock],
        filename: &str,
        include_header: bool,
        include_footer: bool,
    ) -> Result<()> {
        let mut pages: Vec<(PdfPageIndex, PdfLayerIndex)> = vec![(self.first_page, self.first_layer)];
        let mut current_page_idx = 0;

        let top_margin = if include_header { MARGIN_TOP_MM } else { MARGIN_TOP_MM - 10.0 };
        let bottom_margin = if include_footer { MARGIN_BOTTOM_MM } else { MARGIN_BOTTOM_MM - 10.0 };
        let mut current_y = A4_HEIGHT_MM - top_margin - 10.0;
        let max_content_width = A4_WIDTH_MM - MARGIN_LEFT_MM - MARGIN_RIGHT_MM;

        for block in blocks {
            match block {
                MdBlock::Heading { level, text } => {
                    let (font_size, line_h, margin_before, margin_after) = match level {
                        1 => (18.0, 7.5, 6.0, 3.0),
                        2 => (14.5, 6.0, 5.0, 2.5),
                        3 => (12.0, 5.0, 4.0, 2.0),
                        _ => (10.5, 4.5, 3.0, 1.5),
                    };

                    let max_chars = match level {
                        1 => 42,
                        2 => 52,
                        3 => 62,
                        _ => 72,
                    };
                    let wrapped_lines = wrap_line(text, max_chars);

                    let needed_h = margin_before + (wrapped_lines.len() as f32 * line_h) + margin_after;
                    if current_y - needed_h < bottom_margin {
                        let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                        pages.push((new_p, new_l));
                        current_page_idx += 1;
                        current_y = A4_HEIGHT_MM - top_margin - 10.0;
                    }

                    current_y -= margin_before;
                    let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);

                    for line in &wrapped_lines {
                        layer.use_text(line, font_size, Mm(MARGIN_LEFT_MM), Mm(current_y), &self.font_helvetica_bold);
                        current_y -= line_h;
                    }

                    if *level == 1 {
                        let line_y = current_y + line_h - 1.5;
                        self.draw_rule(&layer, MARGIN_LEFT_MM, A4_WIDTH_MM - MARGIN_RIGHT_MM, line_y, 0.4, 0.8);
                    }

                    current_y -= margin_after;
                }
                MdBlock::Paragraph { spans } => {
                    let font_size = 10.0;
                    let line_h = 4.5;
                    let lines = wrap_spans(spans, max_content_width, font_size);

                    for line in lines {
                        if current_y - line_h < bottom_margin {
                            let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                            pages.push((new_p, new_l));
                            current_page_idx += 1;
                            current_y = A4_HEIGHT_MM - top_margin - 10.0;
                        }

                        let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);
                        render_styled_line(
                            &layer,
                            &line,
                            MARGIN_LEFT_MM,
                            current_y,
                            font_size,
                            &self.font_helvetica,
                            &self.font_helvetica_bold,
                            &self.font_helvetica_oblique,
                            &self.font_helvetica_bold_oblique,
                            &self.font_courier,
                            &self.font_courier_bold,
                        );
                        current_y -= line_h;
                    }
                    current_y -= 2.5;
                }
                MdBlock::BulletItem { indent_level, spans } => {
                    let font_size = 10.0;
                    let line_h = 4.5;
                    let base_indent = MARGIN_LEFT_MM + (*indent_level as f32 * 5.0);
                    let content_x = base_indent + 5.0;
                    let max_w = A4_WIDTH_MM - MARGIN_RIGHT_MM - content_x;

                    let lines = wrap_spans(spans, max_w, font_size);

                    for (i, line) in lines.iter().enumerate() {
                        if current_y - line_h < bottom_margin {
                            let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                            pages.push((new_p, new_l));
                            current_page_idx += 1;
                            current_y = A4_HEIGHT_MM - top_margin - 10.0;
                        }

                        let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);
                        if i == 0 {
                            layer.use_text("-", font_size, Mm(base_indent), Mm(current_y), &self.font_helvetica_bold);
                        }
                        render_styled_line(
                            &layer,
                            line,
                            content_x,
                            current_y,
                            font_size,
                            &self.font_helvetica,
                            &self.font_helvetica_bold,
                            &self.font_helvetica_oblique,
                            &self.font_helvetica_bold_oblique,
                            &self.font_courier,
                            &self.font_courier_bold,
                        );
                        current_y -= line_h;
                    }
                    current_y -= 1.5;
                }
                MdBlock::OrderedItem { number, indent_level, spans } => {
                    let font_size = 10.0;
                    let line_h = 4.5;
                    let base_indent = MARGIN_LEFT_MM + (*indent_level as f32 * 5.0);
                    let prefix = format!("{}. ", number);
                    let prefix_w = prefix.len() as f32 * 2.2;
                    let content_x = base_indent + prefix_w.max(6.0);
                    let max_w = A4_WIDTH_MM - MARGIN_RIGHT_MM - content_x;

                    let lines = wrap_spans(spans, max_w, font_size);

                    for (i, line) in lines.iter().enumerate() {
                        if current_y - line_h < bottom_margin {
                            let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                            pages.push((new_p, new_l));
                            current_page_idx += 1;
                            current_y = A4_HEIGHT_MM - top_margin - 10.0;
                        }

                        let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);
                        if i == 0 {
                            layer.use_text(&prefix, font_size, Mm(base_indent), Mm(current_y), &self.font_helvetica_bold);
                        }
                        render_styled_line(
                            &layer,
                            line,
                            content_x,
                            current_y,
                            font_size,
                            &self.font_helvetica,
                            &self.font_helvetica_bold,
                            &self.font_helvetica_oblique,
                            &self.font_helvetica_bold_oblique,
                            &self.font_courier,
                            &self.font_courier_bold,
                        );
                        current_y -= line_h;
                    }
                    current_y -= 1.5;
                }
                MdBlock::BlockQuote { spans } => {
                    let font_size = 9.5;
                    let line_h = 4.5;
                    let content_x = MARGIN_LEFT_MM + 6.0;
                    let max_w = A4_WIDTH_MM - MARGIN_RIGHT_MM - content_x;

                    let lines = wrap_spans(spans, max_w, font_size);
                    let needed_h = (lines.len() as f32 * line_h) + 2.0;

                    if current_y - needed_h < bottom_margin {
                        let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                        pages.push((new_p, new_l));
                        current_page_idx += 1;
                        current_y = A4_HEIGHT_MM - top_margin - 10.0;
                    }

                    let quote_top = current_y + 1.0;
                    let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);

                    for line in &lines {
                        render_styled_line(
                            &layer,
                            line,
                            content_x,
                            current_y,
                            font_size,
                            &self.font_helvetica_oblique,
                            &self.font_helvetica_bold_oblique,
                            &self.font_helvetica_oblique,
                            &self.font_helvetica_bold_oblique,
                            &self.font_courier,
                            &self.font_courier_bold,
                        );
                        current_y -= line_h;
                    }

                    let quote_bottom = current_y + 1.5;
                    self.draw_vertical_bar(&layer, MARGIN_LEFT_MM + 2.0, quote_bottom, quote_top, 1.2, 0.6);
                    current_y -= 2.5;
                }
                MdBlock::CodeBlock { lang, lines } => {
                    let font_size = 8.5;
                    let line_h = 3.8;
                    current_y -= 2.0;

                    let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);
                    if let Some(ref l) = lang {
                        layer.use_text(&format!("[{}]", l), 7.5, Mm(MARGIN_LEFT_MM + 4.0), Mm(current_y), &self.font_helvetica_bold);
                        current_y -= 3.2;
                    }

                    for line in lines {
                        if current_y - line_h < bottom_margin {
                            let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                            pages.push((new_p, new_l));
                            current_page_idx += 1;
                            current_y = A4_HEIGHT_MM - top_margin - 10.0;
                        }

                        let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);
                        layer.use_text(line, font_size, Mm(MARGIN_LEFT_MM + 4.0), Mm(current_y), &self.font_courier);
                        current_y -= line_h;
                    }
                    current_y -= 3.0;
                }
                MdBlock::Rule => {
                    if current_y - 6.0 < bottom_margin {
                        let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                        pages.push((new_p, new_l));
                        current_page_idx += 1;
                        current_y = A4_HEIGHT_MM - top_margin - 10.0;
                    }

                    current_y -= 2.5;
                    let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);
                    self.draw_rule(&layer, MARGIN_LEFT_MM, A4_WIDTH_MM - MARGIN_RIGHT_MM, current_y, 0.5, 0.75);
                    current_y -= 3.5;
                }
            }
        }

        // Add headers and footers across all created pages
        let total_pages = pages.len();
        for (page_num, (p_idx, l_idx)) in pages.iter().enumerate() {
            let layer = self.doc.get_page(*p_idx).get_layer(*l_idx);
            if include_header {
                self.add_header(&layer, filename, page_num + 1, total_pages)?;
            }
            if include_footer {
                self.add_footer(&layer)?;
            }
        }

        Ok(())
    }

    pub fn add_docx_pages(
        &mut self,
        blocks: &[DocxBlock],
        filename: &str,
        include_header: bool,
        include_footer: bool,
    ) -> Result<()> {
        let mut pages: Vec<(PdfPageIndex, PdfLayerIndex)> = vec![(self.first_page, self.first_layer)];
        let mut current_page_idx = 0;

        let top_margin = if include_header { MARGIN_TOP_MM } else { MARGIN_TOP_MM - 10.0 };
        let bottom_margin = if include_footer { MARGIN_BOTTOM_MM } else { MARGIN_BOTTOM_MM - 10.0 };
        let mut current_y = A4_HEIGHT_MM - top_margin - 10.0;
        let max_content_width = A4_WIDTH_MM - MARGIN_LEFT_MM - MARGIN_RIGHT_MM;

        for block in blocks {
            match block {
                DocxBlock::Heading { level, text } => {
                    let (font_size, line_h, margin_before, margin_after) = match level {
                        1 => (18.0, 7.5, 6.0, 3.0),
                        2 => (14.5, 6.0, 5.0, 2.5),
                        3 => (12.0, 5.0, 4.0, 2.0),
                        _ => (10.5, 4.5, 3.0, 1.5),
                    };

                    let max_chars = match level {
                        1 => 42,
                        2 => 52,
                        3 => 62,
                        _ => 72,
                    };
                    let wrapped_lines = wrap_line(text, max_chars);

                    let needed_h = margin_before + (wrapped_lines.len() as f32 * line_h) + margin_after;
                    if current_y - needed_h < bottom_margin {
                        let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                        pages.push((new_p, new_l));
                        current_page_idx += 1;
                        current_y = A4_HEIGHT_MM - top_margin - 10.0;
                    }

                    current_y -= margin_before;
                    let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);

                    for line in &wrapped_lines {
                        layer.use_text(line, font_size, Mm(MARGIN_LEFT_MM), Mm(current_y), &self.font_helvetica_bold);
                        current_y -= line_h;
                    }

                    if *level == 1 {
                        let line_y = current_y + line_h - 1.5;
                        self.draw_rule(&layer, MARGIN_LEFT_MM, A4_WIDTH_MM - MARGIN_RIGHT_MM, line_y, 0.4, 0.8);
                    }

                    current_y -= margin_after;
                }
                DocxBlock::Paragraph { spans } => {
                    let font_size = 10.0;
                    let line_h = 4.5;
                    let lines = wrap_spans(spans, max_content_width, font_size);

                    for line in lines {
                        if current_y - line_h < bottom_margin {
                            let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                            pages.push((new_p, new_l));
                            current_page_idx += 1;
                            current_y = A4_HEIGHT_MM - top_margin - 10.0;
                        }

                        let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);
                        render_styled_line(
                            &layer,
                            &line,
                            MARGIN_LEFT_MM,
                            current_y,
                            font_size,
                            &self.font_helvetica,
                            &self.font_helvetica_bold,
                            &self.font_helvetica_oblique,
                            &self.font_helvetica_bold_oblique,
                            &self.font_courier,
                            &self.font_courier_bold,
                        );
                        current_y -= line_h;
                    }
                    current_y -= 2.5;
                }
                DocxBlock::Table(table) => {
                    if table.rows.is_empty() {
                        continue;
                    }

                    // Count max columns
                    let col_count = table.rows.iter().map(|r| r.cells.len()).max().unwrap_or(0);
                    if col_count == 0 {
                        continue;
                    }

                    let col_width = max_content_width / (col_count as f32);
                    let cell_font_size = 9.0;
                    let cell_line_h = 4.0;
                    let cell_padding = 2.0;
                    let text_max_w = (col_width - (cell_padding * 2.0)).max(5.0);

                    // Add spacing before table
                    current_y -= 3.0;

                    for (row_idx, row) in table.rows.iter().enumerate() {
                        // Pre-wrap each cell in this row to compute row height
                        let mut row_wrapped_cells: Vec<Vec<Vec<StyledWord>>> = Vec::new();
                        let mut max_cell_lines = 1;

                        for col_idx in 0..col_count {
                            if let Some(cell) = row.cells.get(col_idx) {
                                let mut spans = cell.spans.clone();
                                // Bold the first row (header row)
                                if row_idx == 0 {
                                    for s in &mut spans {
                                        s.is_bold = true;
                                    }
                                }
                                let wrapped = wrap_spans(&spans, text_max_w, cell_font_size);
                                max_cell_lines = max_cell_lines.max(wrapped.len().max(1));
                                row_wrapped_cells.push(wrapped);
                            } else {
                                row_wrapped_cells.push(Vec::new());
                            }
                        }

                        let row_height = (max_cell_lines as f32 * cell_line_h) + (cell_padding * 2.0);

                        // Check if row fits on current page
                        if current_y - row_height < bottom_margin {
                            let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                            pages.push((new_p, new_l));
                            current_page_idx += 1;
                            current_y = A4_HEIGHT_MM - top_margin - 10.0;
                        }

                        let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);

                        let row_top = current_y;
                        let row_bottom = current_y - row_height;

                        // Draw top border if first row
                        if row_idx == 0 {
                            self.draw_rule(&layer, MARGIN_LEFT_MM, MARGIN_LEFT_MM + max_content_width, row_top, 0.8, 0.4);
                        }

                        // Render cell contents
                        for (col_idx, cell_lines) in row_wrapped_cells.iter().enumerate() {
                            let cell_x = MARGIN_LEFT_MM + (col_idx as f32 * col_width) + cell_padding;
                            let mut cell_y = row_top - cell_padding - (cell_line_h * 0.75);

                            for line in cell_lines {
                                render_styled_line(
                                    &layer,
                                    line,
                                    cell_x,
                                    cell_y,
                                    cell_font_size,
                                    &self.font_helvetica,
                                    &self.font_helvetica_bold,
                                    &self.font_helvetica_oblique,
                                    &self.font_helvetica_bold_oblique,
                                    &self.font_courier,
                                    &self.font_courier_bold,
                                );
                                cell_y -= cell_line_h;
                            }
                        }

                        // Draw bottom border of row
                        let border_thickness = if row_idx == 0 { 0.6 } else { 0.3 };
                        let border_gray = if row_idx == 0 { 0.5 } else { 0.75 };
                        self.draw_rule(&layer, MARGIN_LEFT_MM, MARGIN_LEFT_MM + max_content_width, row_bottom, border_thickness, border_gray);

                        // Draw vertical cell dividing lines
                        for col_idx in 0..=col_count {
                            let vert_x = MARGIN_LEFT_MM + (col_idx as f32 * col_width);
                            self.draw_vertical_bar(&layer, vert_x, row_bottom, row_top, 0.3, 0.75);
                        }

                        current_y = row_bottom;
                    }

                    // Spacing after table
                    current_y -= 4.0;
                }
            }
        }

        // Add headers and footers across all created pages
        let total_pages = pages.len();
        for (page_num, (p_idx, l_idx)) in pages.iter().enumerate() {
            let layer = self.doc.get_page(*p_idx).get_layer(*l_idx);
            if include_header {
                self.add_header(&layer, filename, page_num + 1, total_pages)?;
            }
            if include_footer {
                self.add_footer(&layer)?;
            }
        }

        Ok(())
    }

    pub fn add_xlsx_pages(
        &mut self,
        sheets: &[XlsxSheet],
        filename: &str,
        include_header: bool,
        include_footer: bool,
    ) -> Result<()> {
        let mut pages: Vec<(PdfPageIndex, PdfLayerIndex)> = vec![(self.first_page, self.first_layer)];
        let mut current_page_idx = 0;

        let top_margin = if include_header { MARGIN_TOP_MM } else { MARGIN_TOP_MM - 10.0 };
        let bottom_margin = if include_footer { MARGIN_BOTTOM_MM } else { MARGIN_BOTTOM_MM - 10.0 };
        let mut current_y = A4_HEIGHT_MM - top_margin - 10.0;
        let max_content_width = A4_WIDTH_MM - MARGIN_LEFT_MM - MARGIN_RIGHT_MM;

        for sheet in sheets {
            if sheet.rows.is_empty() {
                continue;
            }

            // Sheet Title
            let title_font_size = 13.0;
            let title_h = 7.0;
            if current_y - title_h < bottom_margin {
                let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                pages.push((new_p, new_l));
                current_page_idx += 1;
                current_y = A4_HEIGHT_MM - top_margin - 10.0;
            }

            let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);
            layer.use_text(
                &format!("Sheet: {}", sheet.name),
                title_font_size,
                Mm(MARGIN_LEFT_MM),
                Mm(current_y),
                &self.font_helvetica_bold,
            );
            current_y -= 4.0;
            self.draw_rule(&layer, MARGIN_LEFT_MM, MARGIN_LEFT_MM + max_content_width, current_y, 0.5, 0.6);
            current_y -= 4.0;

            // Determine maximum columns in this sheet
            let col_count = sheet.rows.iter().map(|r| r.cells.len()).max().unwrap_or(0);
            if col_count == 0 {
                continue;
            }

            // Estimate proportional column widths based on maximum string lengths in each column
            let mut max_lens: Vec<usize> = vec![1; col_count];
            for row in &sheet.rows {
                for (col_idx, cell) in row.cells.iter().enumerate() {
                    if col_idx < col_count {
                        max_lens[col_idx] = max_lens[col_idx].max(cell.chars().count().max(1));
                    }
                }
            }

            let total_len: usize = max_lens.iter().sum();
            let col_widths: Vec<f32> = max_lens
                .iter()
                .map(|&l| {
                    let ratio = (l as f32) / (total_len as f32);
                    // Bound column width between 15mm and max_content_width * 0.5
                    (max_content_width * ratio).max(16.0)
                })
                .collect();

            // Rescale widths so sum exactly equals max_content_width
            let sum_widths: f32 = col_widths.iter().sum();
            let scaled_col_widths: Vec<f32> = col_widths
                .iter()
                .map(|&w| (w / sum_widths) * max_content_width)
                .collect();

            let cell_font_size = 8.5;
            let cell_line_h = 3.8;
            let cell_padding = 1.8;

            for (row_idx, row) in sheet.rows.iter().enumerate() {
                // Wrap cell texts for this row
                let mut row_wrapped_cells: Vec<Vec<Vec<StyledWord>>> = Vec::new();
                let mut max_cell_lines = 1;

                for col_idx in 0..col_count {
                    let col_w = scaled_col_widths[col_idx];
                    let text_max_w = (col_w - (cell_padding * 2.0)).max(4.0);
                    let cell_text = row.cells.get(col_idx).cloned().unwrap_or_default();
                    let is_header_row = row_idx == 0;

                    let span = MdSpan {
                        text: cell_text,
                        is_bold: is_header_row,
                        is_italic: false,
                        is_code: false,
                    };

                    let wrapped = wrap_spans(&[span], text_max_w, cell_font_size);
                    max_cell_lines = max_cell_lines.max(wrapped.len().max(1));
                    row_wrapped_cells.push(wrapped);
                }

                let row_height = (max_cell_lines as f32 * cell_line_h) + (cell_padding * 2.0);

                // Check page split
                if current_y - row_height < bottom_margin {
                    let (new_p, new_l) = self.doc.add_page(Mm(A4_WIDTH_MM), Mm(A4_HEIGHT_MM), "Layer 1");
                    pages.push((new_p, new_l));
                    current_page_idx += 1;
                    current_y = A4_HEIGHT_MM - top_margin - 10.0;
                }

                let layer = self.doc.get_page(pages[current_page_idx].0).get_layer(pages[current_page_idx].1);
                let row_top = current_y;
                let row_bottom = current_y - row_height;

                // Top border for first row
                if row_idx == 0 {
                    self.draw_rule(&layer, MARGIN_LEFT_MM, MARGIN_LEFT_MM + max_content_width, row_top, 0.8, 0.4);
                }

                // Render cell texts
                let mut current_col_x = MARGIN_LEFT_MM;
                for (col_idx, cell_lines) in row_wrapped_cells.iter().enumerate() {
                    let col_w = scaled_col_widths[col_idx];
                    let cell_x = current_col_x + cell_padding;
                    let mut cell_y = row_top - cell_padding - (cell_line_h * 0.75);

                    for line in cell_lines {
                        render_styled_line(
                            &layer,
                            line,
                            cell_x,
                            cell_y,
                            cell_font_size,
                            &self.font_helvetica,
                            &self.font_helvetica_bold,
                            &self.font_helvetica_oblique,
                            &self.font_helvetica_bold_oblique,
                            &self.font_courier,
                            &self.font_courier_bold,
                        );
                        cell_y -= cell_line_h;
                    }

                    current_col_x += col_w;
                }

                // Row bottom border
                let border_thickness = if row_idx == 0 { 0.6 } else { 0.25 };
                let border_gray = if row_idx == 0 { 0.4 } else { 0.8 };
                self.draw_rule(&layer, MARGIN_LEFT_MM, MARGIN_LEFT_MM + max_content_width, row_bottom, border_thickness, border_gray);

                // Vertical column borders
                let mut vert_x = MARGIN_LEFT_MM;
                self.draw_vertical_bar(&layer, vert_x, row_bottom, row_top, 0.25, 0.8);
                for col_w in &scaled_col_widths {
                    vert_x += col_w;
                    self.draw_vertical_bar(&layer, vert_x, row_bottom, row_top, 0.25, 0.8);
                }

                current_y = row_bottom;
            }

            // Spacing after each sheet
            current_y -= 6.0;
        }

        // Add headers and footers across all created pages
        let total_pages = pages.len();
        for (page_num, (p_idx, l_idx)) in pages.iter().enumerate() {
            let layer = self.doc.get_page(*p_idx).get_layer(*l_idx);
            if include_header {
                self.add_header(&layer, filename, page_num + 1, total_pages)?;
            }
            if include_footer {
                self.add_footer(&layer)?;
            }
        }

        Ok(())
    }

    fn draw_rule(&self, layer: &PdfLayerReference, x_start: f32, x_end: f32, y: f32, thickness: f32, gray: f32) {
        let line = Line {
            points: vec![
                (Point::new(Mm(x_start), Mm(y)), false),
                (Point::new(Mm(x_end), Mm(y)), false),
            ],
            is_closed: false,
        };
        layer.set_outline_color(Color::Rgb(Rgb::new(gray, gray, gray, None)));
        layer.set_outline_thickness(thickness);
        layer.add_line(line);
    }

    fn draw_vertical_bar(&self, layer: &PdfLayerReference, x: f32, y_bottom: f32, y_top: f32, thickness: f32, gray: f32) {
        let line = Line {
            points: vec![
                (Point::new(Mm(x), Mm(y_bottom)), false),
                (Point::new(Mm(x), Mm(y_top)), false),
            ],
            is_closed: false,
        };
        layer.set_outline_color(Color::Rgb(Rgb::new(gray, gray, gray, None)));
        layer.set_outline_thickness(thickness);
        layer.add_line(line);
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

#[derive(Debug, Clone)]
pub struct XlsxSheet {
    pub name: String,
    pub rows: Vec<XlsxRow>,
}

#[derive(Debug, Clone)]
pub struct XlsxRow {
    pub cells: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum DocxBlock {
    Heading { level: u8, text: String },
    Paragraph { spans: Vec<MdSpan> },
    Table(DocxTable),
}

#[derive(Debug, Clone)]
pub struct DocxTable {
    pub rows: Vec<DocxTableRow>,
}

#[derive(Debug, Clone)]
pub struct DocxTableRow {
    pub cells: Vec<DocxTableCell>,
}

#[derive(Debug, Clone)]
pub struct DocxTableCell {
    pub spans: Vec<MdSpan>,
}

#[derive(Debug, Clone)]
pub enum MdBlock {
    Heading { level: u8, text: String },
    Paragraph { spans: Vec<MdSpan> },
    BulletItem { indent_level: usize, spans: Vec<MdSpan> },
    OrderedItem { number: u64, indent_level: usize, spans: Vec<MdSpan> },
    CodeBlock { lang: Option<String>, lines: Vec<String> },
    BlockQuote { spans: Vec<MdSpan> },
    Rule,
}

#[derive(Debug, Clone)]
pub struct MdSpan {
    pub text: String,
    pub is_bold: bool,
    pub is_italic: bool,
    pub is_code: bool,
}

#[derive(Debug, Clone)]
struct StyledWord {
    text: String,
    is_bold: bool,
    is_italic: bool,
    is_code: bool,
    width_mm: f32,
}

fn wrap_spans(spans: &[MdSpan], max_width_mm: f32, font_size: f32) -> Vec<Vec<StyledWord>> {
    let mut words = Vec::new();
    let pt_to_mm = 0.3527778;

    for span in spans {
        for w in span.text.split_whitespace() {
            let char_count = w.chars().count() as f32;
            let width_mm = if span.is_code {
                char_count * (font_size * 0.6 * pt_to_mm)
            } else if span.is_bold {
                char_count * (font_size * 0.55 * pt_to_mm)
            } else {
                char_count * (font_size * 0.52 * pt_to_mm)
            };

            words.push(StyledWord {
                text: w.to_string(),
                is_bold: span.is_bold,
                is_italic: span.is_italic,
                is_code: span.is_code,
                width_mm,
            });
        }
    }

    let mut lines = Vec::new();
    let mut current_line: Vec<StyledWord> = Vec::new();
    let mut current_line_width: f32 = 0.0;

    for word in words {
        let space_w = if word.is_code {
            font_size * 0.6 * pt_to_mm
        } else {
            font_size * 0.28 * pt_to_mm
        };
        let needed = if current_line.is_empty() {
            word.width_mm
        } else {
            space_w + word.width_mm
        };

        if !current_line.is_empty() && current_line_width + needed > max_width_mm {
            lines.push(std::mem::take(&mut current_line));
            current_line_width = word.width_mm;
            current_line.push(word);
        } else {
            current_line_width += needed;
            current_line.push(word);
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn render_styled_line(
    layer: &PdfLayerReference,
    words: &[StyledWord],
    x_start: f32,
    y: f32,
    font_size: f32,
    font_helvetica: &IndirectFontRef,
    font_helvetica_bold: &IndirectFontRef,
    font_helvetica_oblique: &IndirectFontRef,
    font_helvetica_bold_oblique: &IndirectFontRef,
    font_courier: &IndirectFontRef,
    font_courier_bold: &IndirectFontRef,
) {
    let pt_to_mm = 0.3527778;
    let mut x = x_start;

    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            let space_w = if word.is_code {
                font_size * 0.6 * pt_to_mm
            } else {
                font_size * 0.28 * pt_to_mm
            };
            x += space_w;
        }

        let font = if word.is_code {
            if word.is_bold {
                font_courier_bold
            } else {
                font_courier
            }
        } else if word.is_bold && word.is_italic {
            font_helvetica_bold_oblique
        } else if word.is_bold {
            font_helvetica_bold
        } else if word.is_italic {
            font_helvetica_oblique
        } else {
            font_helvetica
        };

        layer.use_text(&word.text, font_size, Mm(x), Mm(y), font);
        x += word.width_mm;
    }
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
