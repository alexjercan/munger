use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

use clap::Parser;

/// Simple program to munge a wordlist into a password list
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the wordlist to munge; will use stdin if not specified
    #[arg(short, long)]
    wordlist: Option<String>,

    /// Path to the output file; will use stdout if not specified
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

    return BufReader::new(input).lines().into_iter().collect();
}

fn write_words(words: &Vec<String>, file: &Option<String>) -> Result<(), io::Error> {
    let output: Box<dyn io::Write> = match file {
        Some(file) => Box::new(File::create(file)?),
        None => Box::new(io::stdout()),
    };

    let mut stream = BufWriter::new(output);
    words
        .iter()
        .map(|word| stream.write_all(format!("{word}\n").as_bytes()))
        .collect::<Result<_, _>>()?;
    return stream.flush();
}

fn main() {
    let args = Args::parse();

    let words = read_words(&args.wordlist).unwrap();

    write_words(&words, &args.output).unwrap();
}
