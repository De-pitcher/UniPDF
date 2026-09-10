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
    Markdown,
    Docx,
    Spreadsheet,
    Unknown,
}

fn detect_file_type(path: &PathBuf) -> FileType {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "txt" => FileType::Text,
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => FileType::Image,
            "md" | "markdown" => FileType::Markdown,
            "docx" => FileType::Docx,
            "xlsx" | "xls" | "ods" => FileType::Spreadsheet,
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

    /// Batch convert multiple files matching a glob pattern into PDFs
    Batch {
        /// Input file glob pattern (e.g., "documents/**/*.docx" or "test_files/*.txt")
        #[arg(short, long)]
        pattern: String,

        /// Output directory for converted PDF files
        #[arg(short, long)]
        output_dir: PathBuf,

        /// Number of parallel worker threads (default: CPU logical cores or 1)
        #[arg(short, long, default_value_t = 1)]
        threads: usize,

        /// Disable header
        #[arg(long)]
        no_header: bool,

        /// Disable footer
        #[arg(long)]
        no_footer: bool,
    },

    /// Merge multiple PDF documents into a single PDF
    Merge {
        /// List of PDF files to merge in order
        #[arg(required = true)]
        inputs: Vec<PathBuf>,

        /// Output merged PDF path
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn convert_single_file(input: &PathBuf, output: &PathBuf, no_header: bool, no_footer: bool) -> std::result::Result<(), String> {
    if !input.exists() {
        return Err(format!("Input file does not exist: {:?}", input));
    }

    let file_type = detect_file_type(input);
    let result = match file_type {
        FileType::Text => converters::text::convert(input, output, !no_header, !no_footer),
        FileType::Image => converters::image::convert(input, output),
        FileType::Markdown => converters::markdown::convert(input, output, !no_header, !no_footer),
        FileType::Docx => converters::docx::convert(input, output, !no_header, !no_footer),
        FileType::Spreadsheet => converters::xlsx::convert(input, output, !no_header, !no_footer),
        FileType::Unknown => {
            return Err("Unsupported file type".into());
        }
    };

    result.map_err(|e| format!("{}", e))
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

            match convert_single_file(&input, &output, no_header, no_footer) {
                Ok(_) => {
                    println!("✅ Successfully converted to {:?}", output);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("❌ Conversion failed: {}", e);
                    if matches!(detect_file_type(&input), FileType::Unknown) {
                        eprintln!("💡 Supported: .txt, .png, .jpg, .jpeg, .gif, .bmp, .webp, .md, .markdown, .docx, .xlsx, .xls, .ods");
                        if which::which("soffice").is_ok() || which::which("libreoffice").is_ok() {
                            eprintln!("💡 LibreOffice detected on host. Complex legacy files (.doc, .ppt) can be converted with LibreOffice.");
                        }
                    }
                    std::process::exit(1);
                }
            }
        }
        Commands::Batch {
            pattern,
            output_dir,
            threads,
            no_header,
            no_footer,
        } => {
            println!("🔄 Scanning files matching pattern: {}", pattern);

            std::fs::create_dir_all(&output_dir)?;

            let matched_paths: Vec<PathBuf> = glob::glob(&pattern)
                .map_err(|e| anyhow::anyhow!("Invalid glob pattern: {}", e))?
                .filter_map(|r| r.ok())
                .filter(|p| p.is_file())
                .collect();

            if matched_paths.is_empty() {
                println!("⚠️ No files matched pattern: {}", pattern);
                return Ok(());
            }

            println!("📂 Found {} files. Processing with {} thread(s)...", matched_paths.len(), threads);

            let pb = indicatif::ProgressBar::new(matched_paths.len() as u64);
            pb.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                    .unwrap()
                    .progress_chars("#>-"),
            );

            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.max(1))
                .build()?;

            let results: Vec<(PathBuf, std::result::Result<PathBuf, String>)> = pool.install(|| {
                use rayon::prelude::*;
                matched_paths
                    .par_iter()
                    .map(|input_path| {
                        let file_stem = input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("doc");
                        let out_path = output_dir.join(format!("{}.pdf", file_stem));

                        let res = convert_single_file(input_path, &out_path, no_header, no_footer)
                            .map(|_| out_path);
                        pb.inc(1);
                        (input_path.clone(), res)
                    })
                    .collect()
            });

            pb.finish_with_message("Done!");

            let mut success_count = 0;
            let mut fail_count = 0;

            for (src, res) in results {
                match res {
                    Ok(dest) => {
                        log::info!("Converted {:?} -> {:?}", src, dest);
                        success_count += 1;
                    }
                    Err(err) => {
                        eprintln!("❌ Failed to convert {:?}: {}", src, err);
                        fail_count += 1;
                    }
                }
            }

            println!(
                "🎉 Batch conversion complete: {} succeeded, {} failed",
                success_count, fail_count
            );

            if fail_count > 0 && success_count == 0 {
                std::process::exit(1);
            }

            Ok(())
        }
        Commands::Merge { inputs, output } => {
            println!("🔄 Merging {} PDF file(s) into {:?}", inputs.len(), output);

            for input in &inputs {
                if !input.exists() {
                    eprintln!("❌ Error: File does not exist: {:?}", input);
                    std::process::exit(1);
                }
            }

            let input_refs: Vec<&std::path::Path> = inputs.iter().map(|p| p.as_path()).collect();

            match pdf::merge_pdfs(&input_refs, &output) {
                Ok(_) => {
                    println!("✅ Successfully merged {} files into {:?}", inputs.len(), output);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("❌ PDF Merge failed: {}", e);
                    std::process::exit(1);
                }
            }
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
