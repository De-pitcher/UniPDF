use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Failed to read file: {path}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("Failed to write PDF: {path}")]
    FileWrite {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("PDF generation failed: {0}")]
    PdfGeneration(String),

    #[error("Font loading failed: {0}")]
    FontError(String),
}

pub type Result<T> = std::result::Result<T, ConversionError>;
