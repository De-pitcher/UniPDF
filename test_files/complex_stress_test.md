# UniPDF Architecture & Stress Test Document

---

## 1. Overview & System Components
UniPDF is designed as a zero-dependency CLI document to PDF converter.
It parses documents into structured abstract syntax trees (ASTs) before streaming vector drawing commands to the PDF engine.

> **Architecture Note:**
> UniPDF uses pure Rust engines for printpdf, image, and pulldown-cmark.
> No external runtime like Chrome, Node.js, or LibreOffice is required for core formats.

---

## 2. Supported Matrix & Formats

The converter dynamically adjusts page layout according to document structure:

1. **Text Conversion Engine**
   - High-throughput streaming line buffer
   - Custom character wrap at 80 columns
   - Accurate Courier monospace font metric calculation
2. **Image Processing Pipeline**
   - Support for PNG, JPEG, WEBP, GIF, and BMP
   - Real-time alpha-channel flatten to white background
   - Automatic aspect-ratio bounded scaling
3. **CommonMark Renderer**
   - Headings H1 through H6 with proportional leading
   - Mixed emphasis: **bold**, *italic*, and regular styled spans
   - Monospace inline code spans like `fn main() -> Result<()>`
   - Deep nested bullet points and ordered lists

---

## 3. Nested Hierarchies

* Top-level system requirement
  * Memory constraint: under 512MB RAM
  * CPU constraint: single worker execution
    * Hardware profile: Intel Core i7-8650U @ 1.90GHz
* Conversion target goals
  * Less than 100ms per typical Markdown file
  * Zero memory leaks across consecutive document runs

1. Step One: Parse Document AST
2. Step Two: Calculate Available Vertical Area
3. Step Three: Stream PDF vector instructions

---

## 4. Source Code Examples

Here is the core logic in Rust demonstrating type-safe block parsing:

```rust
pub fn parse_markdown(content: &str) -> Vec<MdBlock> {
    let parser = Parser::new(content);
    let mut blocks = Vec::new();
    
    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                println!("Found heading level: {:?}", level);
            }
            Event::Text(t) => {
                println!("Text chunk: {}", t);
            }
            _ => {}
        }
    }
    blocks
}
```

And a sample Python benchmark script:

```python
import time
import subprocess

start = time.perf_counter()
res = subprocess.run(["unipdf", "convert", "sample.md", "-o", "out.pdf"])
print(f"Elapsed: {time.perf_counter() - start:.4f}s")
```

---

## 5. Security & Isolation

> "Security by default is the foundation of high-reliability document processing.
> Every untrusted buffer must be bounded, sanitized, and isolated."

All PDF output streams include:
- UTC generation timestamp
- Exact page numbers in Page X of Y format
- Built-in Type-1 PDF fonts (Helvetica, Courier)
