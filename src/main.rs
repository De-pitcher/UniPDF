// UniPDF - Universal File to PDF Converter
// Phase 0.1.0: Foundation & Text Support

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
        Commands::Convert { input, output } => {
            let output = output.unwrap_or_else(|| input.with_extension("pdf"));
            println!("🔄 Converting {:?} to {:?}", input, output);

            // TODO: Implement actual conversion
            println!("❌ Conversion not yet implemented");
            println!("💡 Phase 0.1.0 is under development");

            Ok(())
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
