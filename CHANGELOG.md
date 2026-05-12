# Changelog

All notable changes to UniPDF will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Phase 0.1.0 - Foundation & Text Support
**Status:** 🚧 In Progress  
**Goal:** Convert plain text files to PDF. Establish CLI structure.

#### Planned
- [ ] Basic CLI with `convert` command using `clap`
- [ ] Text file reader with UTF-8 support
- [ ] Simple PDF generator (80 chars per line, 60 lines per page)
- [ ] Monospace font embedding (Liberation Mono)
- [ ] Pagination with headers (filename + page numbers)
- [ ] Error handling with `anyhow`
- [ ] Basic unit tests

#### User Stories
```bash
# Target command
unipdf convert notes.txt notes.pdf
# Expected: Works instantly. Output is readable but not beautiful.
```

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
