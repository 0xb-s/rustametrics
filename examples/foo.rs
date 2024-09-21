use rustametrics::document::ParsedFile;

use rustametrics::metrics::compute_metrics;

fn main() {
    let parsed_files = vec![ParsedFile {
        path: std::path::PathBuf::from("examples/foo.rs"),
        syntax: syn::parse_file(
            r#"
                fn main() {
                    println!("Hello, world!");
                }

                struct Example {
                    field: i32,
                }

                enum Color {
                    Red,
                    Green,
                    Blue,
                }
                "#,
        )
        .unwrap(),
    }];

    let metrics = compute_metrics(&parsed_files);
    assert_eq!(metrics.total_files, 1);
    assert_eq!(metrics.total_functions, 1);
    assert_eq!(metrics.total_structs, 1);
    assert_eq!(metrics.total_enums, 1);
}
