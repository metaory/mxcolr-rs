#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn scan_line(line: &str, pattern: &str) {
    if line.contains(pattern) {
        println!(":->> {}", line);
    }
}

fn main() {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.path);
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    match result {
        Ok(content) => {
            println!("File content: {}", content);
            println!("-------------------------");
            for line in content.lines() {
                scan_line(line, &args.pattern);
            }
        }
        Err(error) => {
            println!("Oh noes: {}", error);
            panic!("Can't deal with {}, lets die here", error);
        }
    }
}

// _________________________________________________________________
// #################################################################
// let pattern = std::env::args().nth(1).expect("no pattern given");
// let path = std::env::args().nth(2).expect("no path given");
// https://rust-cli.github.io/book/tutorial
