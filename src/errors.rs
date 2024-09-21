use std::io::Error as IoError;
use syn::Error as SynError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustaMetricsError {
    #[error("Syn parsing error: {0}")]
    SynError(#[from] SynError),

    #[error("IO error: {0}")]
    IoError(#[from] IoError),

    #[error("WalkDir error: {0}")]
    WalkDirError(#[from] walkdir::Error),

    #[error("Benchmarking error: {0}")]
    BenchmarkError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
