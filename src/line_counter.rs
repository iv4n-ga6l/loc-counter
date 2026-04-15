use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Counts the total lines and blank lines in a given file.
///
/// # Arguments
/// * `file_path` - The path to the file to be analyzed.
///
/// # Returns
/// A `Result` containing a tuple with the total number of lines and the number of blank lines,
/// or an `io::Error` if the file cannot be read.
pub fn count_lines(file_path: &str) -> io::Result<(usize, usize)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut total_lines = 0;
    let mut blank_lines = 0;

    for line in reader.lines() {
        total_lines += 1;
        if line?.trim().is_empty() {
            blank_lines += 1;
        }
    }

    Ok((total_lines, blank_lines))
}
