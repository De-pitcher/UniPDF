# Changelog

All notable changes to UniPDF will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Phase 0.2.0 - Image Support
**Status:** ⏳ Planned  
**Goal:** Convert images to PDF with proper sizing and rotation.

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
