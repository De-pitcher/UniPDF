# Changelog

All notable changes to UniPDF will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2026-09-10

### Phase 0.5.0 - Native XLSX Support & Fallback System
**Status:** ✅ Completed  
**Goal:** Convert Excel spreadsheets (`.xlsx`, `.xls`, `.ods`) to PDF natively with dynamic column scaling, multi-sheet rendering, and external LibreOffice detection.

#### Added
- [x] Fast, zero-dependency spreadsheet parser engine via `calamine`
- [x] Multi-sheet extraction (`.xlsx`, `.xls`, `.ods`) with sheet titles and vector rule separators
- [x] Dynamic proportional column width calculation according to cell content length
- [x] Table cell multiline wrapping and automatic page splitting across large datasets
- [x] Bold header row styling with horizontal borders and vertical cell dividing lines
- [x] External host LibreOffice/soffice detection via `which` to notify users for complex legacy file fallbacks
- [x] Unit and integration test coverage for simple and multi-sheet stress test spreadsheets

---

## [0.4.0] - 2026-09-10

### Phase 0.4.0 - Native DOCX Support
**Status:** ✅ Completed  
**Goal:** Convert Microsoft Word (`.docx`) documents to PDF using 100% native Rust without requiring Microsoft Word or LibreOffice.

#### Added
- [x] Native OpenXML / DOCX parsing engine via `docx-rs`
- [x] Paragraph extraction supporting bold, italic, line breaks, and hyperlinks
- [x] Heading hierarchy detection (H1 to H4, Title, Subtitle) with proportional typographic scales and vector rules
- [x] Full table layout engine: automatic column widths, multiline cell wrapping, bold header rows, row borders, and vertical cell dividing lines
- [x] Dynamic multi-page splitting for large documents and long tables
- [x] Command-line auto-detection and routing for `.docx` files
- [x] Comprehensive integration and stress tests verifying native conversion of complex documents

---

## [0.3.0] - 2026-09-10

### Phase 0.3.0 - Markdown Support
**Status:** ✅ Completed  
**Goal:** Convert Markdown to beautifully formatted, publication-ready PDF with zero external dependencies.

#### Added
- [x] CommonMark parsing engine integrated via `pulldown-cmark`
- [x] Headings (H1 to H6) with hierarchy, bold Helvetica fonts, and H1 divider
- [x] Rich text formatting: **bold**, *italic*, ***bold-italic***, and `inline code`
- [x] Unordered bullet lists (`-`, `*`) with bullet points and indentation
- [x] Ordered lists (`1.`, `2.`) with dynamic counters and indentation
- [x] Monospace fenced code blocks with language tag headers
- [x] Blockquotes with italic typography and vertical accent bars
- [x] Horizontal rules (`---`) with vector dividers
- [x] Word wrapping preserving word boundaries across mixed styles
- [x] Dynamic page overflow handling with `Page X of Y` running headers and footers
- [x] Unit and integration tests for markdown parsing and conversion

---

## [0.2.0] - 2026-05-12

### Phase 0.2.0 - Image Support
**Status:** ✅ Completed  
**Goal:** Convert images to PDF with proper sizing and rotation.

#### Added
- [x] Image file support (PNG, JPG, JPEG, GIF, BMP, WEBP)
- [x] Automatic image scaling to fit A4 page with margins
- [x] Centered image placement on page
- [x] PNG transparency handling (composite on white background)
- [x] Image format detection by file extension
- [x] Header with filename on image pages
- [x] Aspect ratio preservation during scaling
- [x] RGB8 color space conversion for PDF compatibility

#### Changed
- Updated CLI to auto-detect file type (text vs image)
- Enhanced error messages for unsupported file types
- Added `embedded_images` feature to printpdf dependency

#### Technical Details
- Uses `image` crate v0.24 (matching printpdf dependency)
- Implements transparency compositing for RGBA images
- Maximum image size constrained by A4 dimensions minus margins

---

## [0.1.0] - 2026-05-12

### Phase 0.1.0 - Foundation & Text Support
**Status:** ✅ Completed  
**Goal:** Convert plain text files to PDF. Establish CLI structure.

#### Added
- [x] Basic CLI with `convert` command using `clap`
- [x] Text file reader with UTF-8 support
- [x] PDF generator using `printpdf` (A4 page size)
- [x] Monospace font rendering (Courier built-in font)
- [x] Pagination with headers (filename + page numbers)
- [x] Footer with generation timestamp
- [x] Line wrapping at 80 characters per line
- [x] 45 lines per page with proper spacing
- [x] Error handling with `anyhow` and custom error types
- [x] Comprehensive unit tests (8 tests passing)
- [x] Integration tests for CLI and conversion (4 tests passing)

#### User Experience
```bash
unipdf convert notes.txt notes.pdf --output custom.pdf
# ✅ Successfully converted to "custom.pdf"
```

#### Technical Details
- Dependencies: `clap`, `anyhow`, `thiserror`, `printpdf`, `chrono`
- Binary size: ~3.5MB (debug), expected ~1.5MB (release)
- Performance: <50ms for typical text files

---

## Future Phases

### Phase 0.2.0 - Image Support
**Status:** ⏳ Planned  
**Goal:** Convert images to PDF with proper sizing and rotation.

### Phase 0.3.0 - Markdown Support
**Status:** ⏳ Planned  
**Goal:** Convert Markdown to beautifully formatted PDF.

### Phase 0.4.0 - DOCX Support
**Status:** ⏳ Planned  
**Goal:** Support DOCX without external dependencies.

### Phase 0.5.0 - XLSX & Fallback System
**Status:** ⏳ Planned  
**Goal:** XLSX support + graceful LibreOffice fallback.

### Phase 0.6.0 - Batch & Watch Mode
**Status:** ⏳ Planned  
**Goal:** Advanced workflows for power users.

### Phase 1.0.0 - Production Ready
**Status:** ⏳ Planned  
**Goal:** Stable, well-tested, documented release.

---

## Version History

_No releases yet._

---

## Legend
- 🚧 In Progress
- ⏳ Planned
- ✅ Completed
- ⚠️ Blocked/On Hold
