use std::env;
use std::process;

mod file_scanner;
mod line_counter;
mod languages;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <starting_directory>", args[0]);
        process::exit(1);
    }

    let starting_directory = &args[1];

    // Collect all supported file extensions from the language configuration
    let extensions: Vec<&str> = languages::LANGUAGES.keys().cloned().collect();

    // Scan the directory for source files
    let mut all_files = Vec::new();
    for extension in extensions {
        match file_scanner::scan_directory_for_source_files(starting_directory, extension) {
            Ok(mut files) => all_files.append(&mut files),
            Err(e) => eprintln!("Error scanning for .{} files: {}", extension, e),
        }
    }

    println!("───────────────────────────────────────────────────────────────────────────────");
    println!("File                                     Lines     Blanks    Comments      Code");
    println!("───────────────────────────────────────────────────────────────────────────────");

    for file in all_files {
        match line_counter::count_lines(&file) {
            Ok((total_lines, blank_lines, comment_lines, code_lines)) => {
                println!("{:<45} {:>8} {:>8} {:>10} {:>10}", file, total_lines, blank_lines, comment_lines, code_lines);
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", file, e);
            }
        }
    }
}