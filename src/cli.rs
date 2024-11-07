// src/cli.rs

use clap::{ArgGroup, Parser};


#[derive(Parser, Debug)]
#[command(name = "RustaMetrics")]
#[command(author = "0xb-s")]
#[command(version = "0.1.0")]
#[command(about = "Measure code metrics and performance of Rust projects", long_about = None)]
#[command(group(
    ArgGroup::new("output")
        .required(false)
        .args(&["json", "yaml", "html"]),
))]
pub struct Cli {
    /// Input directory containing Rust source files
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    pub input_dir: String,

    /// Output JSON report file
    #[arg(
        short = 'j',
        long,
        value_name = "JSON_FILE",
        conflicts_with = "yaml",
        conflicts_with = "html"
    )]
    pub json: Option<String>,

    /// Output YAML report file
    #[arg(
        short = 'y',
        long,
        value_name = "YAML_FILE",
        conflicts_with = "json",
        conflicts_with = "html"
    )]
    pub yaml: Option<String>,

    /// Output HTML report file
    #[arg(
        short = 'h',
        long,
        value_name = "HTML_FILE",
        conflicts_with = "json",
        conflicts_with = "yaml"
    )]
    pub html: Option<String>,

    /// Enable verbose logging
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Run performance benchmarks
    #[arg(short = 'b', long, action = clap::ArgAction::SetTrue)]
    pub benchmarks: bool,
}
