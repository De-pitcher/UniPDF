# 📄 UniPDF - Universal File to PDF Converter

A zero-dependency CLI tool that converts common file types to PDF. Built for developers who need reliable conversions without installing LibreOffice, Chrome, or other heavy dependencies.

## ✨ Philosophy

**One binary. Zero dependencies. Works everywhere.**

Most file converters require massive dependencies (LibreOffice: 200MB+, Chrome: 300MB+). UniPDF is different—it's a single statically compiled binary for common formats, with graceful fallback to system tools for complex ones.

## 🎯 Supported Formats (Realistic)

| Category | Formats | Status | Implementation |
|----------|---------|--------|----------------|
| Text | TXT | ✅ Native | Custom PDF renderer |
| Images | PNG, JPG, JPEG, GIF, BMP, WEBP | ✅ Native | `image` crate |
| Markdown | MD | ✅ Native | `pulldown-cmark` + `printpdf` |
| DOCX | .docx | ✅ Native (v0.4+) | `docx-rs` + custom renderer |
| XLSX | .xlsx | ⚠️ Native (v0.5+) | `calamine` + table layout |
| PPTX | .pptx | ⚠️ Fallback First | LibreOffice subprocess |
| Legacy Office | .doc, .odt, .ods, .odp | ⚠️ Fallback Only | LibreOffice required |

> **Note:** PPTX has no mature pure-Rust parser. It will use LibreOffice fallback until a solution emerges.

## 🚀 Quick Start

```bash
# Install via cargo
cargo install unipdf

# Convert a text file
unipdf convert notes.txt notes.pdf

# Convert all images in a folder to a single PDF
unipdf batch images/*.png --output album.pdf

# Convert a DOCX (requires no dependencies)
unipdf convert resume.docx resume.pdf
```

## 📦 Installation

### From Source
```bash
git clone https://github.com/yourusername/unipdf.git
cd unipdf
cargo build --release
sudo cp target/release/unipdf /usr/local/bin/
```

### Pre-built Binaries
Download from [Releases](https://github.com/yourusername/unipdf/releases)

## 📚 Documentation

- [Roadmap & Development Phases](docs/ROADMAP.md)
- [Contributing Guide](docs/CONTRIBUTING.md)
- [Architecture Overview](docs/ARCHITECTURE.md)
- [Changelog](CHANGELOG.md)

## 🛠️ Tech Stack

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust 1.75+ | Static binary, no runtime, excellent error handling |
| CLI parsing | `clap` (derive) | Best-in-class, compile-time validation |
| Error handling | `anyhow` + `thiserror` | Developer-friendly error messages |
| PDF generation | `printpdf` | Pure Rust, high-level API |
| Image processing | `image` | Pure Rust, format-agnostic |
| Markdown parsing | `pulldown-cmark` | Fast, CommonMark compliant |

## 📊 Current Status

**Version:** 0.1.0-dev  
**Phase:** Foundation & Text Support

See [CHANGELOG.md](CHANGELOG.md) for detailed progress.

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for development setup and guidelines.

## 📝 License

MIT or Apache-2.0 (dual-licensed)

## 🙏 Acknowledgments

- [printpdf](https://github.com/fschutt/printpdf) - PDF generation library
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown parsing
- [image-rs](https://github.com/image-rs/image) - Image processing
