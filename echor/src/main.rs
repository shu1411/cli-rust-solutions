use clap::{Arg, ArgAction, Command};

fn main() {
    // set up a command line interface
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("shu <shurian1411@gmail.com>")
        .about("Rust version of `echo`")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

    // extract <TEXT> argument(s)
    // We call unwrap here because we know that the program will require at least one <TEXT>
    // argument, hence get_many won't return a None.
    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();

    // extract the value of the '-n' flag.
    let omit_newline = matches.get_flag("omit_newline");

    // omit newline if '-n' flag is present.
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{ending}", text.join(" "));
}
