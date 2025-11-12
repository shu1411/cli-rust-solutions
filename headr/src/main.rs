use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_values_t = ["-".to_string()])]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short('n'),
        long("lines"),
        value_name = "LINES",
        default_value_t = 10,
        conflicts_with = "bytes"
    )]
    lines: u64,

    /// Number of bytes
    #[arg(short('c'), long("bytes"), value_name = "BYTES", value_parser = clap::value_parser!(u64).range(1..=u64::MAX))]
    bytes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
