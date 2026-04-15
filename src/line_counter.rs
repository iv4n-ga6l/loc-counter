use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Counts the total lines, blank lines, comment lines, and code lines in a given file.
///
/// # Arguments
/// * `file_path` - The path to the file to be analyzed.
///
/// # Returns
/// A `Result` containing a tuple with the total number of lines, the number of blank lines,
/// the number of comment lines, and the number of code lines, or an `io::Error` if the file cannot be read.
pub fn count_lines(file_path: &str) -> io::Result<(usize, usize, usize, usize)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut total_lines = 0;
    let mut blank_lines = 0;
    let mut comment_lines = 0;
    let mut code_lines = 0;

    let mut in_multiline_comment = false;

    for line in reader.lines() {
        let line = line?;
        total_lines += 1;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            blank_lines += 1;
        } else if in_multiline_comment {
            // If already inside a multi-line comment, count as a comment line
            comment_lines += 1;
            if trimmed.ends_with("*/") {
                in_multiline_comment = false;
            }
        } else if trimmed.starts_with("//") {
            // Single-line comment
            comment_lines += 1;
        } else if let Some(start_idx) = trimmed.find("/*") {
            // Line contains the start of a multi-line comment
            comment_lines += 1;
            if !trimmed.ends_with("*/") {
                in_multiline_comment = true;
            }

            // Check if there's code before the start of the multi-line comment
            if start_idx > 0 {
                code_lines += 1;
            }
        } else {
            // Regular code line
            code_lines += 1;
        }
    }

    Ok((total_lines, blank_lines, comment_lines, code_lines))
}
