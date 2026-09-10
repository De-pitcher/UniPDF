# Markdown Test Document

This is a comprehensive test document to verify the **UniPDF Markdown conversion engine**.

## Typography and Text Formatting

Markdown allows rich text formatting such as **bold text**, *italicized text*, and `inline code blocks`.
You can also combine formatting like ***bold italic*** for strong emphasis.

### Lists and Enumerations

Here is an unordered list of features:
- Fast, zero-dependency PDF generation
- Pure Rust AST parsing with `pulldown-cmark`
- Built-in vector font rendering
- Clean margins and headers

Here is an ordered workflow list:
1. Parse the markdown input into an event stream
2. Layout blocks and text spans with word wrapping
3. Calculate dynamic page boundaries
4. Render text, headings, and code blocks

### Source Code Blocks

Below is a fenced code block demonstrating Rust code:

```rust
fn main() {
    println!("Hello from UniPDF Markdown Engine!");
    let x = 42;
    if x > 0 {
        println!("Positive number: {}", x);
    }
}
```

### Quotes and Dividers

> "Simplicity is prerequisite for reliability."
> — Edsger W. Dijkstra

---

Thank you for using **UniPDF**!
