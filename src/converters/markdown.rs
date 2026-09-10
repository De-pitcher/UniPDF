use crate::error::{ConversionError, Result};
use crate::pdf::builder::{MdBlock, MdSpan, PdfBuilder};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};
use std::path::Path;

/// Convert a Markdown file to PDF
pub fn convert(input_path: &Path, output_path: &Path, include_header: bool, include_footer: bool) -> Result<()> {
    // Read markdown file content
    let content = std::fs::read_to_string(input_path)
        .map_err(|e| ConversionError::FileRead {
            path: input_path.to_path_buf(),
            source: e,
        })?;

    if content.is_empty() {
        log::warn!("Input Markdown file is empty: {:?}", input_path);
    }

    // Get filename for header
    let filename = input_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("document.md");

    // Parse Markdown into structured blocks
    let blocks = parse_markdown(&content);

    // Create PDF with built-in fonts
    let mut pdf = PdfBuilder::new(&format!("Markdown Document: {}", filename))?;

    // Add Markdown pages
    pdf.add_markdown_pages(&blocks, filename, include_header, include_footer)?;

    // Save PDF
    pdf.save(output_path)?;

    Ok(())
}

/// Parse CommonMark text into a stream of structured blocks
pub fn parse_markdown(content: &str) -> Vec<MdBlock> {
    let parser = Parser::new(content);
    let mut blocks = Vec::new();

    let mut current_heading: Option<(u8, String)> = None;
    let mut current_spans: Vec<MdSpan> = Vec::new();
    let mut in_paragraph = false;
    let mut in_blockquote = false;
    let mut in_code_block: Option<(Option<String>, String)> = None;
    let mut list_stack: Vec<(bool, u64)> = Vec::new(); // (is_ordered, next_number)
    let mut in_item = false;

    let mut is_bold = false;
    let mut is_italic = false;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    let lvl = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    current_heading = Some((lvl, String::new()));
                }
                Tag::Paragraph => {
                    in_paragraph = true;
                    current_spans.clear();
                }
                Tag::BlockQuote(_) => {
                    in_blockquote = true;
                    current_spans.clear();
                }
                Tag::CodeBlock(kind) => {
                    let lang = match kind {
                        CodeBlockKind::Fenced(l) => {
                            let s = l.trim().to_string();
                            if s.is_empty() {
                                None
                            } else {
                                Some(s)
                            }
                        }
                        CodeBlockKind::Indented => None,
                    };
                    in_code_block = Some((lang, String::new()));
                }
                Tag::List(first_num) => {
                    list_stack.push((first_num.is_some(), first_num.unwrap_or(1)));
                }
                Tag::Item => {
                    in_item = true;
                    current_spans.clear();
                }
                Tag::Strong => is_bold = true,
                Tag::Emphasis => is_italic = true,
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) => {
                    if let Some((lvl, text)) = current_heading.take() {
                        blocks.push(MdBlock::Heading { level: lvl, text });
                    }
                }
                TagEnd::Paragraph => {
                    in_paragraph = false;
                    if !current_spans.is_empty() {
                        if in_item {
                            // Retain spans for list item completion in TagEnd::Item
                        } else if in_blockquote {
                            blocks.push(MdBlock::BlockQuote {
                                spans: std::mem::take(&mut current_spans),
                            });
                        } else {
                            blocks.push(MdBlock::Paragraph {
                                spans: std::mem::take(&mut current_spans),
                            });
                        }
                    }
                }
                TagEnd::BlockQuote => {
                    in_blockquote = false;
                    if !current_spans.is_empty() {
                        blocks.push(MdBlock::BlockQuote {
                            spans: std::mem::take(&mut current_spans),
                        });
                    }
                }
                TagEnd::CodeBlock => {
                    if let Some((lang, code)) = in_code_block.take() {
                        let lines = code.lines().map(|s| s.to_string()).collect();
                        blocks.push(MdBlock::CodeBlock { lang, lines });
                    }
                }
                TagEnd::List(_) => {
                    list_stack.pop();
                }
                TagEnd::Item => {
                    in_item = false;
                    let indent = list_stack.len().saturating_sub(1);
                    if let Some((is_ordered, next_num)) = list_stack.last_mut() {
                        if *is_ordered {
                            let num = *next_num;
                            *next_num += 1;
                            blocks.push(MdBlock::OrderedItem {
                                number: num,
                                indent_level: indent,
                                spans: std::mem::take(&mut current_spans),
                            });
                        } else {
                            blocks.push(MdBlock::BulletItem {
                                indent_level: indent,
                                spans: std::mem::take(&mut current_spans),
                            });
                        }
                    } else {
                        blocks.push(MdBlock::BulletItem {
                            indent_level: 0,
                            spans: std::mem::take(&mut current_spans),
                        });
                    }
                }
                TagEnd::Strong => is_bold = false,
                TagEnd::Emphasis => is_italic = false,
                _ => {}
            },
            Event::Text(t) => {
                if let Some((_, ref mut h_text)) = current_heading {
                    h_text.push_str(&t);
                } else if let Some((_, ref mut code)) = in_code_block {
                    code.push_str(&t);
                } else {
                    current_spans.push(MdSpan {
                        text: t.to_string(),
                        is_bold,
                        is_italic,
                        is_code: false,
                    });
                }
            }
            Event::Code(c) => {
                if let Some((_, ref mut h_text)) = current_heading {
                    h_text.push_str(&c);
                } else {
                    current_spans.push(MdSpan {
                        text: c.to_string(),
                        is_bold,
                        is_italic,
                        is_code: true,
                    });
                }
            }
            Event::Rule => {
                blocks.push(MdBlock::Rule);
            }
            Event::SoftBreak => {
                if let Some((_, ref mut h_text)) = current_heading {
                    h_text.push(' ');
                } else if let Some((_, ref mut code)) = in_code_block {
                    code.push('\n');
                } else if in_paragraph || in_item || in_blockquote {
                    current_spans.push(MdSpan {
                        text: " ".to_string(),
                        is_bold,
                        is_italic,
                        is_code: false,
                    });
                }
            }
            Event::HardBreak => {
                if let Some((_, ref mut code)) = in_code_block {
                    code.push('\n');
                } else if in_paragraph || in_item || in_blockquote {
                    current_spans.push(MdSpan {
                        text: "\n".to_string(),
                        is_bold,
                        is_italic,
                        is_code: false,
                    });
                }
            }
            _ => {}
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_markdown_headings() {
        let md = "# Title\n\n## Subtitle\n\n### Section";
        let blocks = parse_markdown(md);
        assert_eq!(blocks.len(), 3);
        match &blocks[0] {
            MdBlock::Heading { level, text } => {
                assert_eq!(*level, 1);
                assert_eq!(text, "Title");
            }
            _ => panic!("Expected Heading"),
        }
        match &blocks[1] {
            MdBlock::Heading { level, text } => {
                assert_eq!(*level, 2);
                assert_eq!(text, "Subtitle");
            }
            _ => panic!("Expected Heading"),
        }
    }

    #[test]
    fn test_parse_markdown_lists() {
        let md = "- Item 1\n- Item 2\n\n1. First\n2. Second";
        let blocks = parse_markdown(md);
        assert_eq!(blocks.len(), 4);
        match &blocks[0] {
            MdBlock::BulletItem { spans, .. } => {
                assert_eq!(spans[0].text, "Item 1");
            }
            _ => panic!("Expected BulletItem"),
        }
        match &blocks[2] {
            MdBlock::OrderedItem { number, spans, .. } => {
                assert_eq!(*number, 1);
                assert_eq!(spans[0].text, "First");
            }
            _ => panic!("Expected OrderedItem"),
        }
    }

    #[test]
    fn test_parse_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let blocks = parse_markdown(md);
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            MdBlock::CodeBlock { lang, lines } => {
                assert_eq!(lang.as_deref(), Some("rust"));
                assert_eq!(lines, &vec!["fn main() {}".to_string()]);
            }
            _ => panic!("Expected CodeBlock"),
        }
    }

    #[test]
    fn test_convert_markdown_file() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("test.md");
        let output = temp_dir.path().join("test.pdf");

        std::fs::write(&input, "# Hello Markdown\n\nThis is **bold** and *italic* text.").unwrap();

        let result = convert(&input, &output, true, true);
        assert!(result.is_ok(), "Markdown conversion failed: {:?}", result.err());
        assert!(output.exists());
        assert!(output.metadata().unwrap().len() > 0);
    }
}
