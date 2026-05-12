# 🏗️ UniPDF Architecture

This document describes the internal architecture of UniPDF.

## Overview

UniPDF uses a three-layer architecture that prioritizes native conversion with graceful fallbacks:

```
┌─────────────────────────────────────────────────────────────┐
│                    CLI Layer (clap + anyhow)                │
├─────────────────────────────────────────────────────────────┤
│                    Conversion Orchestrator                  │
├─────────────────────────────────────────────────────────────┤
│ Native Backend  │    Hybrid Backend    │   Fallback Layer   │
│ (Pure Rust)     │    (Rust + helpers)  │   (Subprocess)     │
├─────────────────┼──────────────────────┼────────────────────┤
│ Text converter  │ image crate          │ LibreOffice (opt)  │
│ printpdf        │ PNG alpha compositing │ Chromium (opt)    │
│ pulldown-cmark  │ Font rendering        │ Legacy parsers     │
│ docx-rs         │                      │                    │
│ calamine        │                      │                    │
└─────────────────┴──────────────────────┴────────────────────┘
```

## Layer Details

### 1. CLI Layer

**Responsibility:** Parse command-line arguments, validate inputs, display errors.

**Key Components:**
- `clap` with derive macros for argument parsing
- `anyhow` for error propagation
- `thiserror` for custom error types

**Entry Points:**
- `convert <input> <output>` - Single file conversion
- `batch <pattern> --output <file>` - Multi-file conversion
- `watch <dir> --out <dir>` - File watcher mode
- `merge <files...> --output <file>` - PDF merging

### 2. Conversion Orchestrator

**Responsibility:** Route files to appropriate converter, handle fallbacks.

**Flow:**
```rust
pub fn convert(input: &Path, output: &Path) -> Result<()> {
    let format = detect_format(input)?;
    
    match format {
        Format::Text => converters::text::convert(input, output),
        Format::Image(ext) => converters::image::convert(input, output),
        Format::Markdown => converters::markdown::convert(input, output),
        Format::Docx => {
            match converters::docx::convert(input, output) {
                Ok(_) => Ok(()),
                Err(e) if fallback_available() => {
                    warn!("Native failed: {}. Trying fallback...", e);
                    fallback::libreoffice::convert(input, output)
                },
                Err(e) => Err(e),
            }
        },
        // ... other formats
    }
}
```

### 3. Native Converters

#### Text Converter (`src/converters/text.rs`)
```rust
pub fn convert(input: &Path, output: &Path) -> Result<()> {
    // Read file with UTF-8 validation
    let content = fs::read_to_string(input)?;
    
    // Create PDF document
    let mut doc = PdfDocument::new("Text Document");
    let font = load_mono_font()?;
    
    // Render text with pagination
    for (page_num, lines) in paginate(&content, 60) {
        let page = doc.add_page();
        render_header(&page, input.file_name(), page_num);
        render_lines(&page, &lines, &font);
    }
    
    doc.save(output)?;
    Ok(())
}
```

#### Image Converter (`src/converters/image.rs`)
```rust
pub fn convert(input: &Path, output: &Path) -> Result<()> {
    // Load image
    let img = image::open(input)?;
    
    // Handle PNG transparency
    let img = if has_alpha_channel(&img) {
        composite_on_white(img)
    } else {
        img
    };
    
    // Create PDF with image
    let mut doc = PdfDocument::new("Image Document");
    let page = doc.add_page();
    let scaled = fit_to_page(&img, A4_WIDTH, A4_HEIGHT);
    page.add_image(scaled);
    
    doc.save(output)?;
    Ok(())
}
```

#### Markdown Converter (`src/converters/markdown.rs`)
```rust
pub fn convert(input: &Path, output: &Path) -> Result<()> {
    // Parse Markdown
    let content = fs::read_to_string(input)?;
    let parser = Parser::new(&content);
    
    // Create PDF
    let mut doc = PdfDocument::new("Markdown Document");
    let mut renderer = MarkdownRenderer::new(&mut doc);
    
    // Render elements
    for event in parser {
        match event {
            Event::Start(Tag::Heading(level)) => renderer.start_heading(level),
            Event::Text(text) => renderer.add_text(&text),
            Event::Code(code) => renderer.add_inline_code(&code),
            Event::Start(Tag::CodeBlock(lang)) => renderer.start_code_block(lang),
            // ... handle all events
        }
    }
    
    doc.save(output)?;
    Ok(())
}
```

### 4. Fallback System

**When Used:**
- Native converter fails (e.g., RTL text in DOCX)
- Format not supported natively (e.g., PPTX, legacy .doc)
- User explicitly requests `--force-fallback`

**Implementation:**
```rust
pub mod fallback {
    pub fn detect_libreoffice() -> Option<PathBuf> {
        which::which("soffice").ok()
    }
    
    pub fn convert(input: &Path, output: &Path) -> Result<()> {
        let soffice = detect_libreoffice()
            .ok_or_else(|| anyhow!("LibreOffice not found"))?;
        
        let status = Command::new(soffice)
            .args(&["--headless", "--convert-to", "pdf", "--outdir", output.parent()])
            .arg(input)
            .status()?;
        
        if !status.success() {
            bail!("LibreOffice conversion failed");
        }
        
        Ok(())
    }
}
```

## PDF Generation Abstraction

All converters use a common `PdfDocument` wrapper around `printpdf`:

```rust
pub struct PdfDocument {
    inner: PdfDocumentReference,
    pages: Vec<PdfPageIndex>,
}

impl PdfDocument {
    pub fn new(title: &str) -> Self;
    pub fn add_page(&mut self) -> PdfPage;
    pub fn save(&self, path: &Path) -> Result<()>;
}

pub struct PdfPage {
    layer: PdfLayerReference,
}

impl PdfPage {
    pub fn add_text(&self, text: &str, x: Mm, y: Mm, font: &Font);
    pub fn add_image(&self, img: &DynamicImage, x: Mm, y: Mm);
    pub fn add_line(&self, from: (Mm, Mm), to: (Mm, Mm));
}
```

## Error Handling Strategy

### Custom Error Types
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("PDF generation failed: {0}")]
    PdfError(String),
    
    #[error("Native conversion failed, fallback unavailable")]
    NoFallback,
}
```

### User-Facing Messages
```rust
match convert(input, output) {
    Ok(_) => println!("✅ Converted successfully"),
    Err(ConversionError::UnsupportedFormat(ext)) => {
        eprintln!("❌ Unsupported format: .{}", ext);
        eprintln!("💡 Supported formats: txt, png, jpg, md, docx, xlsx");
    },
    Err(ConversionError::NoFallback) => {
        eprintln!("❌ Native conversion failed");
        eprintln!("💡 Install LibreOffice for fallback support:");
        eprintln!("   apt-get install libreoffice  # Ubuntu/Debian");
        eprintln!("   brew install libreoffice      # macOS");
    },
    Err(e) => eprintln!("❌ Error: {}", e),
}
```

## Memory Management

### Large File Handling
```rust
// BAD: Loads entire image into memory
let img = image::open(path)?;
doc.add_all_pages(img);  // OOM if 500MB+

// GOOD: Stream processing
for chunk in ImageChunker::new(path, PAGE_SIZE) {
    let page = doc.add_page();
    page.add_image(chunk?);
    // chunk is dropped here, freeing memory
}
```

### Batch Processing
```rust
pub fn batch_convert(inputs: Vec<PathBuf>, output: &Path) -> Result<()> {
    let mut combined = PdfDocument::new("Batch");
    
    for input in inputs {
        // Convert to temp PDF
        let temp = TempFile::new("unipdf-", ".pdf")?;
        convert(&input, temp.path())?;
        
        // Merge into combined
        combined.merge_from(temp.path())?;
        
        // temp is dropped, file deleted
    }
    
    combined.save(output)?;
    Ok(())
}
```

## Performance Considerations

### Parallel Processing (v0.6.0+)
```rust
use rayon::prelude::*;

pub fn batch_parallel(inputs: Vec<PathBuf>, output_dir: &Path) -> Result<()> {
    inputs.par_iter()
        .try_for_each(|input| {
            let output = output_dir.join(input.with_extension("pdf"));
            convert(input, &output)
        })?;
    Ok(())
}
```

### Font Caching
```rust
lazy_static! {
    static ref FONT_CACHE: Mutex<HashMap<String, Font>> = Mutex::new(HashMap::new());
}

pub fn load_font(name: &str) -> Result<Font> {
    let mut cache = FONT_CACHE.lock().unwrap();
    if let Some(font) = cache.get(name) {
        return Ok(font.clone());
    }
    
    let font = Font::load(name)?;
    cache.insert(name.to_string(), font.clone());
    Ok(font)
}
```

## Testing Architecture

### Unit Tests
- In-file tests for individual functions
- Mock PDF generation for fast tests
- Property-based testing for text wrapping

### Integration Tests
```rust
#[test]
fn test_text_conversion_integration() {
    let input = "tests/test_files/sample.txt";
    let output = "/tmp/output.pdf";
    
    convert(input, output).unwrap();
    
    // Verify PDF is valid
    let pdf = PdfDocument::load(output).unwrap();
    assert!(pdf.page_count() > 0);
    
    // Verify content preserved
    let text = extract_text(&pdf);
    assert!(text.contains("expected content"));
}
```

### Benchmark Tests
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_text_conversion(c: &mut Criterion) {
    c.bench_function("convert 10KB text", |b| {
        b.iter(|| {
            convert(
                black_box("tests/test_files/10kb.txt"),
                black_box("/tmp/out.pdf")
            )
        });
    });
}

criterion_group!(benches, bench_text_conversion);
criterion_main!(benches);
```

## Future Architecture Improvements

### Plugin System (Post-1.0)
```rust
pub trait Converter: Send + Sync {
    fn supports(&self, format: &str) -> bool;
    fn convert(&self, input: &Path, output: &Path) -> Result<()>;
}

// Allow users to register custom converters
unipdf::register_converter(Box::new(MyCustomConverter));
```

### Streaming API (Post-1.0)
```rust
let converter = TextConverter::new();
let mut stream = converter.stream(input)?;

while let Some(page) = stream.next_page()? {
    // Process page-by-page without loading full document
    process(page);
}
```

## Conclusion

UniPDF's architecture balances simplicity, performance, and extensibility. The layered design allows for easy addition of new formats while maintaining backward compatibility.

For questions or suggestions, open an issue on GitHub.
