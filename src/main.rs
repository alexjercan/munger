use std::{
    collections::HashSet,
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

fn leet_speak(word: &str) -> Vec<String> {
    // TODO: with combinations
    return vec![
        word.to_string(),
        word.replace("e", "3"),
        word.replace("a", "4"),
        word.replace("o", "0"),
        word.replace("i", "!"),
        word.replace("i", "1"),
        word.replace("l", "1"),
        word.replace("a", "@"),
        word.replace("s", "$"),
        word.replace("s", "5"),
        word.replace("t", "7"),
    ];
}

fn word_capitalization(word: &str) -> Vec<String> {
    return vec![
        word.to_string(),
        word.to_lowercase(),
        word.to_uppercase(),
        word[0..1].to_uppercase() + &word[1..],
    ];
}

fn munge_word(word: &str) -> Vec<String> {
    let mut words = vec![word.to_string()];

    words.extend(leet_speak(word));
    words.extend(word_capitalization(word));

    return words
        .drain(..)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
}

fn main() {
    let args = Args::parse();

    let words = read_words(&args.wordlist).unwrap();

    let words = words.iter().flat_map(|w| munge_word(w)).collect();

    write_words(&words, &args.output).unwrap();
}
