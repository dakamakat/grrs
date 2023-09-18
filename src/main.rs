#![allow(unused)]

use anyhow::{Context, Result};
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

fn read_no_buf(args: &Cli) -> Result<()> {
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.to_string_lossy()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn read_buf(args: &Cli) -> Result<()> {
    let f = File::open(&args.path)?;

    let mut reader = BufReader::new(f);

    for line in reader.lines() {
        let res = line
            .with_context(|| format!("could not read line on path `{}`", &args.path.to_string_lossy()))?;

        if res.contains(&args.pattern) {
            println!("{}", res);
        }
    }

    Ok(())
}
