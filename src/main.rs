use clap::Parser;

/// Simple program to munge a wordlist
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the wordlist to munge
    #[arg(short, long)]
    wordlist: Option<String>,

    /// Path to the output file
    #[arg(short, long)]
    output: Option<String>,

    /// Munge level
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..9))]
    level: u8,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

}
