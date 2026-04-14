use std::env;
use std::process;

mod file_scanner;

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
            for file in files {
                println!("{}", file);
            }
        }
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
            process::exit(1);
        }
    }
}