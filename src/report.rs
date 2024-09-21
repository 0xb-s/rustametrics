use crate::cli::Cli;
use crate::metrics::CodeMetrics;
use crate::performance::BenchmarkResult;
use log::debug;
use serde::Serialize;
use serde_json;
use serde_yaml;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Report<'a> {
    code_metrics: &'a CodeMetrics,
    benchmarks: Option<&'a Vec<BenchmarkResult>>,
}

pub fn generate_reports(
    metrics: &CodeMetrics,
    benchmarks: Option<Vec<BenchmarkResult>>,
    cli: &Cli,
) -> Result<(), String> {
    let report = Report {
        code_metrics: metrics,
        benchmarks: benchmarks.as_ref(),
    };

    // Generate JSON report
    if let Some(json_path) = &cli.json {
        match serde_json::to_string_pretty(&report) {
            Ok(json) => {
                let mut file = File::create(json_path).map_err(|e| e.to_string())?;
                file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
                debug!("JSON report generated at {}", json_path);
            }
            Err(e) => return Err(format!("Failed to serialize JSON: {}", e)),
        }
    }

    // Generate YAML report
    if let Some(yaml_path) = &cli.yaml {
        match serde_yaml::to_string(&report) {
            Ok(yaml) => {
                let mut file = File::create(yaml_path).map_err(|e| e.to_string())?;
                file.write_all(yaml.as_bytes()).map_err(|e| e.to_string())?;
                debug!("YAML report generated at {}", yaml_path);
            }
            Err(e) => return Err(format!("Failed to serialize YAML: {}", e)),
        }
    }

    // Generate HTML report
    if let Some(html_path) = &cli.html {
        let html_content = generate_html(&report);
        let mut file = File::create(html_path).map_err(|e| e.to_string())?;
        file.write_all(html_content.as_bytes())
            .map_err(|e| e.to_string())?;
        debug!("HTML report generated at {}", html_path);
    }

    Ok(())
}

/// Generates an HTML report from the metrics and benchmarks.
fn generate_html(report: &Report) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><meta charset=\"UTF-8\"><title>RustaMetrics Report</title></head><body>");
    html.push_str("<h1>RustaMetrics Report</h1>");

    // Code Metrics
    html.push_str("<h2>Code Metrics</h2><ul>");
    html.push_str(&format!(
        "<li>Total Files: {}</li>",
        report.code_metrics.total_files
    ));
    html.push_str(&format!(
        "<li>Total Functions: {}</li>",
        report.code_metrics.total_functions
    ));
    html.push_str(&format!(
        "<li>Total Structs: {}</li>",
        report.code_metrics.total_structs
    ));
    html.push_str(&format!(
        "<li>Total Enums: {}</li>",
        report.code_metrics.total_enums
    ));
    html.push_str(&format!(
        "<li>Total Traits: {}</li>",
        report.code_metrics.total_traits
    ));
    html.push_str(&format!(
        "<li>Total Impls: {}</li>",
        report.code_metrics.total_impls
    ));
    html.push_str(&format!(
        "<li>Lines of Code: {}</li>",
        report.code_metrics.lines_of_code
    ));
    html.push_str(&format!(
        "<li>Comments: {}</li>",
        report.code_metrics.comments
    ));
    html.push_str("</ul>");

    // Benchmarks
    if let Some(benchmarks) = report.benchmarks {
        html.push_str("<h2>Performance Benchmarks</h2><ul>");
        for bench in benchmarks {
            html.push_str(&format!(
                "<li>{}: {} iterations, Average Time: {} ns</li>",
                bench.name, bench.iterations, bench.average_time_ns
            ));
        }
        html.push_str("</ul>");
    }

    html.push_str("</body></html>");
    html
}
