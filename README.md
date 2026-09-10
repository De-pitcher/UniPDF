# 📄 UniPDF - Universal File to PDF Converter

[![CI](https://github.com/De-pitcher/UniPDF/actions/workflows/ci.yml/badge.svg)](https://github.com/De-pitcher/UniPDF/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Version: 1.0.0](https://img.shields.io/badge/Version-1.0.0-green.svg)](CHANGELOG.md)

A fast, zero-dependency command-line tool that converts common document and image files into professional PDFs. Built with 100% pure Rust for developers and teams who need reliable document rendering without requiring Microsoft Office, LibreOffice, or headless Chrome.

---

## ✨ Philosophy

**One binary. Zero external runtime dependencies. Works everywhere.**

Most document converters require 300MB–1GB+ runtimes (LibreOffice, Chromium, or MS Word COM automation). UniPDF is a statically compiled Rust binary that parses native OpenXML, CommonMark, and spreadsheet ASTs directly into vector PDF streams.

---

## 🎯 Supported Formats

| Category | Extensions | Status | Native Rust Engine |
|:---|:---|:---|:---|
| **Text Documents** | `.txt` | ✅ Native | Dynamic line wrapping, UTF-8 unicode |
| **Images** | `.png`, `.jpg`, `.jpeg`, `.gif`, `.bmp`, `.webp` | ✅ Native | Aspect-ratio preservation, alpha composite blending (`image`) |
| **Markdown** | `.md`, `.markdown` | ✅ Native | CommonMark AST, headings, inline bold/italic, lists, code blocks (`pulldown-cmark`) |
| **Word Documents** | `.docx` | ✅ Native | Heading hierarchy, styled runs, auto-sized cell table grids (`docx-rs`) |
| **Spreadsheets** | `.xlsx`, `.xls`, `.ods` | ✅ Native | Multi-sheet rendering, dynamic proportional column widths (`calamine`) |
| **PDF Operations** | `.pdf` | ✅ Native | Lossless multi-file PDF concatenation & merging (`lopdf`) |
| **Legacy Office** | `.doc`, `.ppt`, `.rtf` | 💡 Fallback | Automatic detection of host LibreOffice (`which`) |

---

## 🚀 Quick Start

### 1. Convert Single Files
```bash
# Convert plain text
unipdf convert notes.txt --output notes.pdf

# Convert Markdown with styling
unipdf convert README.md --output docs.pdf

# Convert Word document (no MS Word required)
unipdf convert report.docx --output report.pdf

# Convert multi-sheet Excel spreadsheet
unipdf convert financials.xlsx --output financials.pdf

# Disable headers or footers
unipdf convert document.docx --no-header --no-footer
```

### 2. Batch Conversion (Multi-Threaded)
Convert hundreds of files matching a glob pattern using parallel workers:
```bash
unipdf batch --pattern "documents/**/*.docx" --output-dir ./converted_pdfs --threads 4
```

### 3. Merge Multiple PDFs
Combine several PDF files into a single consolidated document in order:
```bash
unipdf merge cover.pdf report.pdf appendix.pdf --output full_dossier.pdf
```

### 4. Hot-Folder Watch Mode
Watch a directory and automatically convert newly created or modified files on the fly:
```bash
unipdf watch --dir ./incoming --output-dir ./ready_pdfs
```

---

## 📦 Installation

### From Source
```bash
git clone https://github.com/De-pitcher/UniPDF.git
cd UniPDF
cargo build --release
```
The compiled static executable will be available at `./target/release/unipdf`.

---

## 🛠️ Architecture & Tech Stack

| Component | Library / Engine | Purpose |
|:---|:---|:---|
| **CLI Framework** | `clap` v4 (derive) | Type-safe command line interface and flag parsing |
| **PDF Stream Engine** | `printpdf` | Pure-Rust low-level vector PDF generator |
| **Word Document Parser** | `docx-rs` | OpenXML AST decomposition without external binaries |
| **Spreadsheet Parser** | `calamine` | High-throughput Excel/ODS sheet decoder |
| **Markdown Parser** | `pulldown-cmark` | CommonMark event parser |
| **Image Processing** | `image` | Pixel format conversions & alpha channel blending |
| **Parallel Batching** | `rayon` + `indicatif` | Worker pool scheduling and real-time progress bars |
| **PDF Merging** | `lopdf` | Object renumbering and dictionary concatenation |
| **Live File Watcher** | `notify` | Cross-platform file system event notifications |

---

## 📝 License

Dual-licensed under MIT or Apache-2.0. See [LICENSE](LICENSE) for details.

- [image-rs](https://github.com/image-rs/image) - Image processing
