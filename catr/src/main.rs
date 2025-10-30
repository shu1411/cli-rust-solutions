use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_values_t = ["-".to_string()])]
    files: Vec<String>,

    /// Number all output lines
    #[arg(short('n'), long("number"), conflicts_with = "number_nonblank_lines")]
    number_lines: bool,

    /// Number nonempty output lines
    #[arg(short('b'), long("number-nonblank"), conflicts_with = "number_lines")]
    number_nonblank_lines: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let mut prev_num = 0;
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),

            Ok(file) => {
                for line in file.lines() {
                    let line = line?;

                    if args.number_lines {
                        prev_num += 1;
                        println!("{prev_num:>6}\t{line}");
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{prev_num:>6}\t{line}");
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
