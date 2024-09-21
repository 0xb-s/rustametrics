use crate::document::ParsedFile;
use crate::errors::RustaMetricsError;
use log::{debug, error};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Parses all Rust source files in the given directory.
pub fn parse_directory(dir: &str) -> Result<Vec<ParsedFile>, RustaMetricsError> {
    let mut parsed_files = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
    {
        let path = entry.path();
        debug!("Parsing file: {:?}", path);
        match parse_file(path) {
            Ok(parsed) => parsed_files.push(parsed),
            Err(e) => error!("Failed to parse {:?}: {}", path, e),
        }
    }

    Ok(parsed_files)
}

/// Parses a single Rust source file.
fn parse_file(path: &Path) -> Result<ParsedFile, RustaMetricsError> {
    let content = fs::read_to_string(path)?;
    let syntax = syn::parse_file(&content)?;
    Ok(ParsedFile {
        path: path.to_path_buf(),
        syntax,
    })
}
