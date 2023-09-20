#![allow(unused)]

use anyhow::{Context, Result};
use clap::ArgAction;
use clap::Parser;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Stdout;
use std::path::PathBuf;
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
    let content = std::fs::read_to_string(&args.path).with_context(|| {
        format!(
            "could not read file on path `{}`",
            &args.path.to_string_lossy()
        )
    })?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            write_to_console(&line.to_string(), &args.path);
        }
    }

    Ok(())
}

fn read_buf(args: &Cli) -> Result<()> {
    let f = File::open(&args.path).with_context(|| {
        format!(
            "could not find file on path `{}`",
            &args.path.to_string_lossy()
        )
    })?;

    let mut reader = BufReader::new(f);

    for line in reader.lines() {
        let res = line.with_context(|| {
            format!(
                "could not read line on path `{}`",
                &args.path.to_string_lossy()
            )
        })?;

        if res.contains(&args.pattern) {
            write_to_console(&res, &args.path);
        }
    }

    Ok(())
}

fn write_to_console(str: &String, path: &PathBuf) -> Result<()> {
    let stdout = io::stdout();

    let mut handle = io::BufWriter::new(stdout);

    writeln!(handle, "{}", str)
        .with_context(|| format!("could not print line on path `{}`", path.to_string_lossy()))?;

    Ok(())
}
