pub mod cli;
pub mod document;
pub mod errors;
pub mod metrics;
pub mod parser;
pub mod performance;
pub mod report;
pub mod utils;
use crate::cli::Cli;

use clap::Parser;
use env_logger;
use log::{error, info};

pub fn run() {
    // Initialize the logger
    env_logger::init();

    // Parse CLI arguments
    let cli = Cli::parse();

    if cli.verbose {
        info!("Starting RustaMetrics with configuration: {:?}", cli);
    }

    // Parse Rust source files
    let parsed_files = match parser::parse_directory(&cli.input_dir) {
        Ok(files) => {
            if cli.verbose {
                info!("Parsed {} files.", files.len());
            }
            files
        }
        Err(e) => {
            error!("Failed to parse source files: {}", e);
            return;
        }
    };

    // Compute metrics
    let metrics = metrics::compute_metrics(&parsed_files);

    if cli.verbose {
        info!("Computed code metrics.");
    }

    // Optionally run benchmarks
    let benchmarks = if cli.benchmarks {
        match performance::run_benchmarks(&cli.input_dir) {
            Ok(results) => {
                if cli.verbose {
                    info!("Benchmarking completed.");
                }
                Some(results)
            }
            Err(e) => {
                error!("Benchmarking failed: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Generate reports
    if let Err(e) = report::generate_reports(&metrics, benchmarks, &cli) {
        error!("Failed to generate reports: {}", e);
    } else {
        if cli.verbose {
            info!("Reports generated successfully.");
        }
    }
}
