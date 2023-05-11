use std::{
    fs::File,
    io::{self, BufRead, BufReader, Stdin},
};

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

fn read_words(file: &Option<String>) -> Result<Vec<String>, io::Error> {
    let input: Box<dyn io::Read> = match file {
        Some(file) => Box::new(File::open(file)?),
        None => Box::new(io::stdin()),
    };

    return Ok(BufReader::new(input)
        .lines()
        .into_iter()
        .flatten()
        .collect());
}

fn main() {
    let args = Args::parse();

    let words = read_words(&args.wordlist).unwrap();

    println!("{:?}", words);
}
