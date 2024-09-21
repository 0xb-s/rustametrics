use std::path::PathBuf;
use syn::File;

/// Represents a parsed Rust source file.
pub struct ParsedFile {
    pub path: PathBuf,
    pub syntax: File,
}
