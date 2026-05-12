# Contributing to UniPDF

Thank you for your interest in contributing to UniPDF! This guide will help you get started.

## 🚀 Development Setup

### Prerequisites
- **Rust:** 1.75 or later
- **Git:** For version control
- **Optional:** LibreOffice (for testing fallback system)

### Clone & Build
```bash
git clone https://github.com/yourusername/unipdf.git
cd unipdf
cargo build
cargo test
```

### Run Locally
```bash
cargo run -- convert test_files/sample.txt output.pdf
```

## 📂 Project Structure

```
unipdf/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── cli.rs               # clap command definitions
│   ├── converters/          # Conversion implementations
│   │   ├── mod.rs
│   │   ├── text.rs          # Text to PDF
│   │   ├── image.rs         # Image to PDF
│   │   ├── markdown.rs      # Markdown to PDF
│   │   ├── docx.rs          # DOCX to PDF
│   │   └── xlsx.rs          # XLSX to PDF
│   ├── pdf/                 # PDF generation helpers
│   │   ├── mod.rs
│   │   ├── builder.rs       # PDF document builder
│   │   └── fonts.rs         # Font embedding
│   ├── fallback/            # LibreOffice fallback system
│   │   ├── mod.rs
│   │   └── libreoffice.rs
│   └── error.rs             # Custom error types
├── tests/
│   ├── integration/         # End-to-end tests
│   └── test_files/          # Sample files for testing
├── benches/                 # Performance benchmarks
├── docs/                    # Documentation
│   ├── ROADMAP.md
│   ├── CONTRIBUTING.md
│   └── ARCHITECTURE.md
├── CHANGELOG.md
├── Cargo.toml
└── README.md
```

## 🛠️ Development Workflow

### 1. Pick an Issue
Browse [open issues](https://github.com/yourusername/unipdf/issues) and pick one labeled `good first issue` or `help wanted`.

### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name
```

### 3. Write Code
- Follow Rust idioms and best practices
- Add tests for new functionality
- Update documentation as needed

### 4. Run Tests
```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

### 5. Commit & Push
```bash
git add .
git commit -m "feat: add TIFF image support"
git push origin feature/your-feature-name
```

### 6. Open a Pull Request
- Describe what your PR does
- Reference any related issues
- Wait for review

## 🧪 Testing Guidelines

### Unit Tests
Place unit tests in the same file as the code:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_wrapping() {
        // Test implementation
    }
}
```

### Integration Tests
Place integration tests in `tests/`:
```rust
#[test]
fn test_convert_text_to_pdf() {
    let output = Command::new("unipdf")
        .args(&["convert", "tests/test_files/sample.txt", "output.pdf"])
        .output()
        .expect("Failed to execute");
    
    assert!(output.status.success());
    assert!(Path::new("output.pdf").exists());
}
```

### Test Files
Add sample files to `tests/test_files/` for integration testing.

## 📝 Commit Message Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

### Examples
```
feat(image): add TIFF support

Implement TIFF image conversion using the image crate.
Includes tests for grayscale and RGB TIFF files.

Closes #42
```

```
fix(docx): handle missing fonts gracefully

Previously crashed when DOCX specified unavailable font.
Now falls back to Liberation Sans with a warning.
```

## 🎯 Good First Issues

Here are some beginner-friendly tasks:

### Easy
- Add support for TIFF images
- Improve error messages for common failure modes
- Add shell completions (bash, zsh, fish)
- Write integration tests for existing converters

### Medium
- Implement PDF/A compliance
- Add EPUB to PDF conversion
- Improve table rendering in Markdown converter
- Add custom page size support

### Hard
- Implement bidirectional text support (RTL)
- Add font subsetting to reduce PDF size
- Optimize memory usage for large batches
- Implement incremental PDF writing for streaming

## 📚 Resources

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### PDF Format
- [PDF Reference (1.7)](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf)
- [printpdf docs](https://docs.rs/printpdf/)

### Office Formats
- [DOCX specification](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/)
- [docx-rs crate](https://docs.rs/docx-rs/)

## 🐛 Reporting Bugs

When reporting bugs, please include:
- **OS and version:** (e.g., Ubuntu 22.04, macOS 13, Windows 11)
- **Rust version:** (run `rustc --version`)
- **UniPDF version:** (run `unipdf --version`)
- **Command executed:** (e.g., `unipdf convert file.docx out.pdf`)
- **Expected behavior:** What you expected to happen
- **Actual behavior:** What actually happened
- **Sample file:** If possible, attach the file that caused the issue

## 💬 Community

- **Discussions:** [GitHub Discussions](https://github.com/yourusername/unipdf/discussions)
- **Chat:** [Discord](https://discord.gg/yourserver)
- **Issues:** [GitHub Issues](https://github.com/yourusername/unipdf/issues)

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT or Apache-2.0 license (dual-licensed).
