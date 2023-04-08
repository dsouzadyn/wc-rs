use std::{fs, io, path::PathBuf};

use clap::Parser;

/// Print different counts for FILE, when no file is specified data is read from stdin
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// file to be read, when no file is specified data is read from stdin
    file: Option<PathBuf>,
    /// print the byte counts
    #[arg(short, long)]
    bytes: bool,
    /// print the character counts
    #[arg(short, long)]
    chars: bool,
    /// print the newline counts
    #[arg(short, long)]
    lines: bool,
    /// print the maximum display width
    #[arg(short, long)]
    max_line_length: bool,
    /// print the word counts
    #[arg(short, long)]
    words: bool,
}

fn print_default(buffer: &mut String) {
    let bytes = buffer.len();
    let new_lines = buffer.lines().count();
    let words = buffer.split_ascii_whitespace().count();
    println!("newline: {} words: {} bytes: {}", new_lines, words, bytes);
}

fn main() {
    let cli = Cli::parse();
    let mut reader: Box<dyn io::Read> = match cli.file {
        None => Box::new(io::stdin()),
        Some(file) => Box::new(fs::File::open(file).unwrap()),
    };
    let mut result = String::new();
    let mut buffer = String::new();
    let _size = reader.read_to_string(&mut buffer);
    if cli.bytes {
        let bytes = buffer.len();
        result.push_str(&format!("bytes: {} ", bytes));
    }
    if cli.lines {
        let new_lines = buffer.lines().count();
        result.push_str(&format!("newlines: {} ", new_lines));
    }
    if cli.words {
        let words = buffer.split_ascii_whitespace().count();
        result.push_str(&format!("words: {} ", words));
    }
    if cli.max_line_length {
        let max_line_length = buffer.lines().map(|line| line.len()).max().unwrap_or(0);
        result.push_str(&format!("max line length: {}", max_line_length));
    }
    if result.len() > 0 {
        println!("{}", result);
    } else {
        print_default(&mut buffer);
    }
}
