#![allow(unused)]

use clap::ArgAction;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
    /// Use BufReader or not
    #[clap(long, short)]
    no_bufferize: bool,
}
fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let now = Instant::now();

    // Code block to measure.
    {
        println!(
            "pattern: {0} , path: {1}",
            args.pattern,
            args.path.display()
        );

        if args.no_bufferize {
            read_no_buf(&args);
        } else {
            read_buf(&args);
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

fn read_no_buf(args: &Cli) {
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

fn read_buf(args: &Cli) -> std::io::Result<()> {
    let f = File::open(&args.path)?;

    let mut reader = BufReader::new(f);

    for line in reader.lines() {
        let res = match line {
            Ok(line) => line,
            Err(e) => return Err(e),
        };

        if res.contains(&args.pattern) {
            println!("{}", res);
        }
    }

    Ok(())
}
