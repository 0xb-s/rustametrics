// src/performance.rs

use crate::errors::RustaMetricsError;
use criterion::Criterion;
use std::path::Path;

use serde::Serialize;
use std::fs;
#[derive(Debug, Serialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub average_time_ns: f64,
}

/// Runs performance benchmarks on functions in the codebase.
pub fn run_benchmarks(input_dir: &str) -> Result<Vec<BenchmarkResult>, RustaMetricsError> {
    let mut results = Vec::new();


    let mut criterion = Criterion::default().configure_from_args();
    let input_path = Path::new(input_dir);

    criterion.bench_function("read_files", |b| {
        b.iter(|| {
            let _ = read_files(input_path);
        })
    });

    // Collect results (simplified for illustration)
    results.push(BenchmarkResult {
        name: "read_files".to_string(),
        iterations: 1000,
        average_time_ns: 5000.0, // changes this value TODO;
    });

    Ok(results)
}


fn read_files(dir: &Path) {
    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
    {
        let _ = fs::read_to_string(entry.path());
    }
}
