# LOC Counter

LOC Counter is a command-line tool designed to analyze source code files in a given directory and provide detailed statistics about the lines of code (LOC). It also includes COCOMO (Constructive Cost Model) estimates to help developers understand the effort, time, and resources required for software development projects.

## Features

- **Line Counting**: Counts total lines, blank lines, comment lines, and code lines for supported file types.
- **Language Support**: Supports multiple programming languages with customizable comment syntax.
- **COCOMO Estimation**: Provides effort, time, and developer estimates based on the total lines of code.

## Supported Languages

The following file types are currently supported:

| File Extension | Description         |
|----------------|---------------------|
| `.rs`          | Rust source files   |
| `.txt`         | Text files          |
| `.md`          | Markdown files      |

To add support for additional languages, update the `LANGUAGES` map in `src/languages.rs`.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/iv4n-ga6l/loc-counter.git
   cd loc-counter
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be available in the `target/release` directory.

## Usage

Run the tool by providing the starting directory as an argument:

```bash
./target/release/loc-counter <starting_directory>
```

For example:

```bash
./target/release/loc-counter ./src
```

### Sample Output

```
───────────────────────────────────────────────────────────────────────────────
File                                     Lines     Blanks    Comments      Code
───────────────────────────────────────────────────────────────────────────────
src/main.rs                                100         20          30         50
src/lib.rs                                  50         10          15         25

───────────────────────────────────────────────────────────────────────────────
COCOMO Estimates:
Total Lines of Code (LOC): 75
Effort (person-months): 0.34
Time (months): 0.88
Number of Developers: 0.39
```

## How It Works

1. **File Scanning**: The tool recursively scans the specified directory for files with supported extensions. This is handled by the `file_scanner` module.
2. **Line Counting**: For each file, the `line_counter` module analyzes the content to count total lines, blank lines, comment lines, and code lines.
3. **COCOMO Estimation**: The `cocomo` module calculates effort, time, and developer estimates based on the total lines of code.

## Adding Support for New Languages

To add a new language, update the `LANGUAGES` map in `src/languages.rs` with the appropriate comment syntax:

```rust
"ext" => LanguageConfig {
    single_line_comment: Some("//"),
    multi_line_comment_start: Some("/*"),
    multi_line_comment_end: Some("*/"),
},
```

Replace `"ext"` with the file extension and provide the comment syntax for the language.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
