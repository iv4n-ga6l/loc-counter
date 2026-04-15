use std::env;
use std::process;

mod file_scanner;
mod line_counter;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <starting_directory>", args[0]);
        process::exit(1);
    }

    let starting_directory = &args[1];

    // Scan the directory for source files
    match file_scanner::scan_directory_for_source_files(starting_directory, "rs") {
        Ok(files) => {
            println!("───────────────────────────────────────────────────────────────────────────────");
            println!("File                                     Lines     Blanks    Comments      Code");
            println!("───────────────────────────────────────────────────────────────────────────────");

            for file in files {
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
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
            process::exit(1);
        }
    }
}
