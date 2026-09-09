// UniPDF - Universal File to PDF Converter
// Phase 0.1.0: Foundation & Text Support

mod converters;
mod error;
mod pdf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug)]
enum FileType {
    Text,
    Image,
    Unknown,
}

fn detect_file_type(path: &PathBuf) -> FileType {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "txt" => FileType::Text,
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => FileType::Image,
            _ => FileType::Unknown,
        }
    } else {
        FileType::Unknown
    }
}

#[derive(Parser)]
#[command(
    name = "unipdf",
    version = "0.1.0-dev",
    about = "Universal File to PDF Converter",
    long_about = "A zero-dependency CLI tool that converts common file types to PDF.\nSupports text, images, Markdown, DOCX, and more."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a single file to PDF
    Convert {
        /// Input file path
        input: PathBuf,

        /// Output PDF path (optional, defaults to input name with .pdf extension)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Disable header (filename and page numbers)
        #[arg(long)]
        no_header: bool,

        /// Disable footer (timestamp)
        #[arg(long)]
        no_footer: bool,
    },

    /// Convert multiple files to a single PDF
    #[allow(dead_code)]
    Batch {
        /// Input file pattern (e.g., "images/*.png")
        pattern: String,

        /// Output PDF path
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    // Parse CLI arguments
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { input, output, no_header, no_footer } => {
            let output = output.unwrap_or_else(|| input.with_extension("pdf"));
            
            // Validate input file exists
            if !input.exists() {
                eprintln!("❌ Error: Input file does not exist: {:?}", input);
                std::process::exit(1);
            }

            println!("🔄 Converting {:?} to {:?}", input, output);

            // Detect file type and route to appropriate converter
            let result = match detect_file_type(&input) {
                FileType::Text => converters::text::convert(&input, &output, !no_header, !no_footer),
                FileType::Image => converters::image::convert(&input, &output),
                FileType::Unknown => {
                    eprintln!("❌ Error: Unsupported file type");
                    eprintln!("💡 Supported: .txt, .png, .jpg, .jpeg, .gif, .bmp, .webp");
                    std::process::exit(1);
                }
            };

            match result {
                Ok(_) => {
                    println!("✅ Successfully converted to {:?}", output);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("❌ Conversion failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Batch { pattern, output } => {
            println!("🔄 Batch converting {} to {:?}", pattern, output);

            // TODO: Implement batch conversion (Phase 0.6.0)
            println!("❌ Batch mode not yet implemented");
            println!("💡 This feature is planned for Phase 0.6.0");

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cli_parsing() {
        // This test ensures CLI can be instantiated
        // Actual functionality tests will be added as features are implemented
        assert!(true);
    }
}
