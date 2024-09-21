

use regex::Regex;

/// Sanitizes a string to be used as a filename.
pub fn sanitize_filename(name: &str) -> String {
    let re = Regex::new(r"[<>:\/\\|?*]").unwrap();
    re.replace_all(name, "_").to_string()
}
