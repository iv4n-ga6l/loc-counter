use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Scans the given directory and returns a list of files with the specified extension.
///
/// # Arguments
/// * `starting_directory` - The directory to start scanning from.
/// * `extension` - The file extension to filter by (e.g., "rs" for Rust files).
///
/// # Returns
/// A `Result` containing a vector of file paths as strings if successful, or an `io::Error` otherwise.
pub fn scan_directory_for_source_files(
    starting_directory: &str,
    extension: &str,
) -> io::Result<Vec<String>> {
    let mut source_files = Vec::new();
    let start_path = Path::new(starting_directory);

    if !start_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Provided path is not a directory",
        ));
    }

    // Recursively traverse the directory
    for entry in fs::read_dir(start_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recurse into subdirectories
            match scan_directory_for_source_files(path.to_str().unwrap(), extension) {
                Ok(mut sub_files) => source_files.append(&mut sub_files),
                Err(e) => return Err(e),
            }
        } else if let Some(ext) = path.extension() {
            if ext == extension {
                source_files.push(path.to_string_lossy().to_string());
            }
        }
    }

    source_files.sort(); // Sort the files for consistent output
    Ok(source_files)
}