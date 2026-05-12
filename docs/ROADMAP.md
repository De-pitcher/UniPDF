# 🗺️ UniPDF Development Roadmap

This document outlines the detailed development phases for UniPDF, from MVP to production-ready release.

## 📋 Phase Overview

| Phase | Version | Timeline | Complexity | Status |
|-------|---------|----------|------------|--------|
| Foundation | 0.1.0 | 2-3 weeks | ⭐ Low | 🚧 In Progress |
| Images | 0.2.0 | 1-2 weeks | ⭐⭐ Medium | ⏳ Planned |
| Markdown | 0.3.0 | 2-3 weeks | ⭐⭐ Medium | ⏳ Planned |
| DOCX | 0.4.0 | 3-4 weeks | ⭐⭐⭐ High | ⏳ Planned |
| XLSX + Fallback | 0.5.0 | 3-4 weeks | ⭐⭐⭐ High | ⏳ Planned |
| Batch/Watch | 0.6.0 | 2 weeks | ⭐⭐ Medium | ⏳ Planned |
| Production | 1.0.0 | 2-3 weeks | ⭐⭐⭐ High | ⏳ Planned |

---

## Version 0.1.0 - Foundation & Text Support

**Goal:** Convert plain text files to PDF. Establish CLI structure.

### Deliverables

#### CLI Structure
- [x] Initialize Rust project with `cargo init`
- [ ] Set up `clap` with derive macros
- [ ] Implement `convert` subcommand
- [ ] Add `--output` flag (optional, defaults to input name with .pdf)
- [ ] Add `--help` and `--version` flags
- [ ] Error handling with `anyhow` and `thiserror`

#### Text Processing
- [ ] UTF-8 file reader with error handling
- [ ] Line wrapping at 80 characters
- [ ] Page breaks at 60 lines
- [ ] Support for CR, LF, CRLF line endings

#### PDF Generation
- [ ] Integrate `printpdf` crate
- [ ] Embed Liberation Mono font (or fallback to system monospace)
- [ ] A4 page size (210mm × 297mm)
- [ ] Margins: 25mm top/bottom, 20mm left/right
- [ ] Header: filename (left), page number (right)
- [ ] Footer: generation timestamp (center)

#### Testing
- [ ] Unit tests for line wrapping logic
- [ ] Integration test: convert `test_files/sample.txt`
- [ ] Verify output PDF is valid (using `lopdf` or `pdf-extract`)

### User Experience
```bash
unipdf convert notes.txt notes.pdf
# Works instantly. Output is readable but not beautiful.
```

### Technical Details
- **Dependencies:** `clap`, `anyhow`, `thiserror`, `printpdf`
- **Binary Size Target:** <5MB (statically linked)
- **Performance Target:** <10ms for 10KB text file

### Acceptance Criteria
- [ ] CLI accepts a text file and produces a valid PDF
- [ ] Long lines wrap correctly
- [ ] Page numbers appear on each page
- [ ] UTF-8 characters render correctly
- [ ] Error messages are helpful (e.g., "File not found: notes.txt")

---

## Version 0.2.0 - Image Support

**Goal:** Convert images to PDF with proper sizing and rotation.

### Deliverables

#### Image Formats
- [ ] PNG support (with transparency handling)
- [ ] JPEG/JPG support
- [ ] GIF support
- [ ] BMP support
- [ ] WEBP support

#### PDF Layout
- [ ] Automatic scaling to fit page (maintain aspect ratio)
- [ ] Center image on page
- [ ] `--page-size` flag (A4, Letter, custom)
- [ ] `--margin` flag (default: 10mm)
- [ ] `--fit-to-page` flag (cover, contain, stretch)

#### PNG Transparency Mitigation
- [ ] Detect alpha channel
- [ ] Composite onto white background before embedding
- [ ] Warn user if transparency was removed

#### Batch Conversion
- [ ] Accept multiple image files
- [ ] Combine into single PDF (one image per page)
- [ ] Preserve input order

### User Experience
```bash
unipdf convert photo.png photo.pdf --page-size A4 --margin 10
unipdf batch *.jpg --output album.pdf --fit-to-page cover
```

### Technical Details
- **New Dependencies:** `image`, `imageproc`
- **Pitfall Mitigated:** PNG transparency is composited onto white background

### Acceptance Criteria
- [ ] All common image formats convert without errors
- [ ] Images fit within page margins
- [ ] Batch conversion creates multi-page PDF
- [ ] Transparent PNGs render with white background (no missing content)

---

## Version 0.3.0 - Markdown Support

**Goal:** Convert Markdown to beautifully formatted PDF.

### Deliverables

#### Markdown Parsing
- [ ] Integrate `pulldown-cmark` (CommonMark compliant)
- [ ] Support headers (H1-H6)
- [ ] Support **bold**, *italic*, `code`
- [ ] Support lists (ordered, unordered, nested)
- [ ] Support blockquotes
- [ ] Support code blocks with language tags

#### Syntax Highlighting
- [ ] Integrate `syntect` for code blocks
- [ ] Support common languages (Rust, Python, JS, etc.)
- [ ] Fallback to monospace for unknown languages

#### Styling
- [ ] Default theme (GitHub-like)
- [ ] Custom CSS via `--style` flag
- [ ] Font size customization (`--font-size`, default: 11pt)
- [ ] Line height customization (`--line-height`, default: 1.5)

#### Advanced Features
- [ ] Table support (with borders)
- [ ] Image embedding (inline images in Markdown)
- [ ] Hyperlinks (as footnotes in PDF)

### User Experience
```bash
unipdf convert README.md docs.pdf --style github.css
unipdf convert chapter.md --font-size 12 --line-height 1.5
```

### Technical Details
- **New Dependencies:** `pulldown-cmark`, `syntect`
- **Styling Engine:** Custom CSS-like DSL or inline styles

### Acceptance Criteria
- [ ] All CommonMark features render correctly
- [ ] Code blocks are syntax-highlighted
- [ ] Tables have visible borders and proper alignment
- [ ] Custom CSS applies correctly

---

## Version 0.4.0 - DOCX Support (Native)

**Goal:** Support DOCX without any external dependencies.

### Deliverables

#### DOCX Parsing
- [ ] Integrate `docx-rs` crate
- [ ] Parse text runs with formatting (bold, italic, underline)
- [ ] Parse paragraphs with alignment
- [ ] Parse tables (basic layout)
- [ ] Parse images (embedded in document)

#### Font Handling
- [ ] Embed Liberation Sans/Serif as fallback fonts
- [ ] Respect font families from DOCX (if available)
- [ ] Handle missing fonts gracefully (warn + fallback)

#### Layout Engine
- [ ] Respect page margins from DOCX
- [ ] Implement basic text flow (paragraphs, line breaks)
- [ ] Table rendering (borders, cell padding)
- [ ] Image positioning (inline, floating)

#### Fallback Detection
- [ ] Detect RTL text (Arabic, Hebrew) → emit warning
- [ ] Detect deeply nested tables (>5 levels) → suggest fallback
- [ ] If native conversion fails, suggest LibreOffice fallback

### User Experience
```bash
unipdf convert resume.docx resume.pdf
# Output: "✅ Converted successfully using native renderer"

unipdf convert complex-rtl.docx output.pdf
# Output: "⚠️ RTL text detected - conversion may not preserve reading order"
```

### Technical Details
- **New Dependencies:** `docx-rs`
- **Known Limitations:** RTL text, complex tables, embedded fonts

### Pitfalls Mitigated

| Pitfall | Mitigation |
|---------|------------|
| RTL text | Detect and warn: `⚠️ RTL detected - use --fallback for proper rendering` |
| Deeply nested tables | Depth limit (5 levels). If exceeded, suggest fallback |
| Missing fonts | Embed Liberation fonts; warn if original font unavailable |

### Acceptance Criteria
- [ ] Simple DOCX files convert with preserved formatting
- [ ] Tables render with correct borders and alignment
- [ ] Images appear in correct positions
- [ ] RTL text is detected and warning is shown

---

## Version 0.5.0 - XLSX & Fallback Subsystem

**Goal:** XLSX support + graceful LibreOffice fallback for complex formats.

### Deliverables

#### XLSX Support
- [ ] Integrate `calamine` crate (Excel parser)
- [ ] Render one sheet per page
- [ ] Auto-fit column widths
- [ ] Respect cell formatting (bold, colors, borders)
- [ ] `--sheet` flag to select specific sheet
- [ ] `--landscape` flag for wide tables

#### Fallback System
- [ ] Detect LibreOffice installation (`soffice --version`)
- [ ] Subprocess execution for `.doc`, `.odt`, `.pptx`
- [ ] Progress reporting for long conversions
- [ ] `--force-fallback` flag to skip native converter
- [ ] Config file for custom LibreOffice path

#### Fallback Configuration
```bash
# Set custom LibreOffice path
unipdf config set libreoffice /opt/libreoffice/program/soffice

# Force fallback mode
unipdf convert old.doc output.pdf --force-fallback
```

### User Experience
```bash
unipdf convert budget.xlsx report.pdf --landscape
# Output: "✅ Converted 3 sheets to PDF"

unipdf convert old-format.doc legacy.pdf
# Output: "Native converter unavailable for .doc. Using LibreOffice fallback..."
```

### Technical Details
- **New Dependencies:** `calamine`, `which` (for detecting soffice)
- **Fallback Command:** `soffice --headless --convert-to pdf --outdir <dir> <file>`

### Acceptance Criteria
- [ ] XLSX files render as readable PDFs
- [ ] Fallback detection works on Linux, macOS, Windows
- [ ] Error message is helpful if LibreOffice not found
- [ ] Conversion progress is shown for long operations

---

## Version 0.6.0 - Batch & Watch Mode

**Goal:** Advanced workflows for power users.

### Deliverables

#### Batch Command
- [ ] Glob pattern support (`docs/**/*.docx`)
- [ ] Parallel conversion with `rayon` (4 threads default)
- [ ] `--parallel <N>` flag to control thread count
- [ ] Progress bar with `indicatif`
- [ ] Error handling (continue on failure, report at end)

#### Watch Mode
- [ ] File system watcher using `notify` crate
- [ ] Debounce delay (`--debounce`, default: 500ms)
- [ ] Auto-convert on file creation/modification
- [ ] Log successful conversions
- [ ] Graceful shutdown on Ctrl+C

#### Memory Management
- [ ] Process large image batches sequentially (not all at once)
- [ ] Memory limit check (reject batches exceeding 1GB total size)
- [ ] Stream processing for large files

#### PDF Merging
- [ ] `merge` subcommand
- [ ] Combine multiple PDFs into one
- [ ] Optional table of contents (`--add-toc`)

### User Experience
```bash
# Convert all docs in folder (parallel)
unipdf batch "docs/**/*.docx" --out ./pdfs --parallel 4

# Watch folder (converts as files arrive)
unipdf watch ./hotfolder --out ./converted --debounce 500ms

# Merge multiple PDFs
unipdf merge *.pdf --output combined.pdf --add-toc
```

### Technical Details
- **New Dependencies:** `rayon`, `notify`, `indicatif`, `lopdf` (for merging)
- **Memory Safety:** Process one image at a time, write page, drop from memory

### Acceptance Criteria
- [ ] Batch conversion handles 100+ files without crashes
- [ ] Watch mode responds to file changes within debounce window
- [ ] Progress bar accurately reflects completion percentage
- [ ] Memory usage stays below 512MB even for large batches

---

## Version 1.0.0 - Production Ready

**Goal:** Stable, well-tested, documented release.

### Deliverables

#### Testing
- [ ] Unit tests for all converters (>80% coverage)
- [ ] Integration tests with real files
- [ ] Content verification using `pdf-extract`
- [ ] Memory usage assertions (<500MB for typical workloads)
- [ ] Performance regression tests

#### CI/CD
- [ ] GitHub Actions for Linux, macOS, Windows
- [ ] Automated release builds
- [ ] Pre-built binaries for all platforms
- [ ] Automated changelog generation

#### Documentation
- [ ] Comprehensive README
- [ ] Man page (`unipdf.1`)
- [ ] Shell completions (bash, zsh, fish)
- [ ] API documentation (for library usage)

#### Polish
- [ ] Better error messages with suggestions
- [ ] Colored terminal output
- [ ] `--verbose` flag for debugging
- [ ] Config file support (`~/.unipdf.toml`)

### Testing Strategy
```rust
#[cfg(test)]
mod tests {
    // Unit tests for each converter
    // Integration tests with real files
    // Content verification using pdf-extract
    // Memory usage assertions under 500MB
}
```

### Acceptance Criteria
- [ ] All tests pass on Linux, macOS, Windows
- [ ] Binary size <15MB
- [ ] Startup time <50ms
- [ ] No known crashes or data loss
- [ ] Documentation is complete and accurate

---

## 🧠 Known Pitfalls & Ongoing Mitigations

| Pitfall | Impact | Mitigation | Status |
|---------|--------|------------|--------|
| **RTL/BiDi text** | Arabic/Hebrew shows left-to-right | Detect at runtime; emit warning with fallback suggestion | ✅ Mitigated in v0.4 |
| **PNG transparency** | Images disappear in PDF | Composite alpha channel onto white background | ✅ Mitigated in v0.2 |
| **Deeply nested tables** | Stack overflow crash | Depth limit (5 levels). Suggest fallback if exceeded | ✅ Mitigated in v0.4 |
| **Large file memory** | OOM on 500MB+ images | Sequential processing; reject batches exceeding 1GB | ✅ Mitigated in v0.6 |
| **Missing content verification** | Silent text loss | Integration tests with `pdf-extract` assertions | ✅ Mitigated in CI |
| **LibreOffice version skew** | Fallback may fail with old LO | Check version at runtime; suggest minimum version | ✅ Mitigated in v0.5 |
| **Font embedding failures** | PDF uses fallback fonts | Embed Liberation fonts; warn on missing | ⏳ Planned for v0.4 |

---

## 📊 Performance Expectations (Revised)

| Operation | File Size | Time (Target) | Memory (Target) |
|-----------|-----------|---------------|-----------------|
| TXT to PDF | 10KB | <10ms | 5MB |
| Markdown to PDF | 50KB | 50ms | 10MB |
| PNG to PDF (single) | 5MB | 100ms | 60MB |
| DOCX to PDF | 500KB | 1-2s | 50MB |
| XLSX to PDF (10 sheets) | 2MB | 2-3s | 100MB |
| Batch 100 images | 500MB total | 10s | 512MB (sequential) |

> **Note:** These are realistic targets. Initial implementations may be slower and can be optimized over time.

---

## 🎯 Success Metrics

### v0.1.0 Success
- [ ] CLI accepts text files and produces valid PDFs
- [ ] At least 5 community members test it successfully

### v1.0.0 Success
- [ ] 100+ GitHub stars
- [ ] Used in at least 10 production environments
- [ ] Zero critical bugs in issue tracker
- [ ] Average conversion time <2s for typical documents
