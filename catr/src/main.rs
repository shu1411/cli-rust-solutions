use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
struct Args {
    /// Input file(s)
    #[arg(default_values_t = ["-".to_string()])]
    file: Vec<String>,

    /// Number all output lines
    #[arg(short('n'), long("number"), conflicts_with = "number_nonblank_lines")]
    number_lines: bool,

    /// Number nonempty output lines
    #[arg(short('b'), long("number-nonblank"), conflicts_with = "number_lines")]
    number_nonblank_lines: bool,
}

fn main() {
    let args = Args::parse();
    println!("{args:#?}");
}

