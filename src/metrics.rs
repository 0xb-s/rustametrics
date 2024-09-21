use crate::document::ParsedFile;
use serde::Serialize;
use syn::Item;
use syn::__private::ToTokens;

#[derive(Debug, Serialize)]
pub struct CodeMetrics {
    pub total_files: usize,
    pub total_functions: usize,
    pub total_structs: usize,
    pub total_enums: usize,
    pub total_traits: usize,
    pub total_impls: usize,
    pub lines_of_code: usize,
    pub comments: usize,
    // Add more metrics as needed
}

pub fn compute_metrics(parsed_files: &[ParsedFile]) -> CodeMetrics {
    let mut metrics = CodeMetrics {
        total_files: parsed_files.len(),
        total_functions: 0,
        total_structs: 0,
        total_enums: 0,
        total_traits: 0,
        total_impls: 0,
        lines_of_code: 0,
        comments: 0,
    };

    for file in parsed_files {
        let loc = count_lines(&file.syntax);
        let comm = count_comments(&file.syntax);
        metrics.lines_of_code += loc;
        metrics.comments += comm;

        for item in &file.syntax.items {
            match item {
                Item::Fn(_) => metrics.total_functions += 1,
                Item::Struct(_) => metrics.total_structs += 1,
                Item::Enum(_) => metrics.total_enums += 1,
                Item::Trait(_) => metrics.total_traits += 1,
                Item::Impl(_) => metrics.total_impls += 1,
                _ => {}
            }
        }
    }

    metrics
}

/// Counts the number of lines of code in the syntax tree.
fn count_lines(file: &syn::File) -> usize {
    file.to_token_stream().to_string().lines().count()
}

/// Counts the number of comments in the syntax tree.
fn count_comments(file: &syn::File) -> usize {
    let tokens = file.to_token_stream().to_string();
    tokens.matches("//").count() + tokens.matches("/*").count()
}
