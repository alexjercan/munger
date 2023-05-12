use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
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

fn generate_subsequences<K, V>(mappings: &Vec<(K, Vec<V>)>, index: usize) -> Vec<HashMap<K, V>>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    let mut subsequences = vec![];

    if index == mappings.len() {
        subsequences.push(HashMap::new());
    } else {
        let (key, values) = &mappings[index];

        let subsubsequences = generate_subsequences(mappings, index + 1);

        for value in values {
            for subsubsequence in &subsubsequences {
                let mut subsequence = subsubsequence.clone();
                subsequence.insert(key.clone(), value.clone());
                subsequences.push(subsequence);
            }
        }

        subsequences.extend(subsubsequences);
    }

    return subsequences;
}

trait MungeBlock {
    fn munge(&self, word: &str) -> Vec<String>;
}

struct LeetSpeak<'a> {
    mappings: Vec<HashMap<&'a str, &'a str>>,
}

impl<'a> LeetSpeak<'a> {
    fn new() -> Self {
        let mappings: Vec<(_, Vec<_>)> = vec![
            ("e", "3"),
            ("a", "4"),
            ("a", "@"),
            ("o", "0"),
            ("i", "1"),
            ("i", "!"),
            ("l", "1"),
            ("s", "5"),
            ("s", "$"),
        ]
        .into_iter()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (k, v)| {
            acc.entry(k).and_modify(|e| e.push(v)).or_insert(vec![k, v]);
            return acc;
        })
        .into_iter()
        .collect();

        let mappings = generate_subsequences(&mappings, 0);

        return LeetSpeak { mappings };
    }
}

impl<'a> MungeBlock for LeetSpeak<'a> {
    fn munge(&self, word: &str) -> Vec<String> {
        return self
            .mappings
            .iter()
            .map(|mapping| {
                mapping
                    .iter()
                    .fold(word.to_owned(), |acc, (k, v)| acc.replace(k, v))
            })
            .collect();
    }
}

struct Capitalization;

impl Capitalization {
    fn new() -> Self {
        return Capitalization {};
    }
}

impl MungeBlock for Capitalization {
    fn munge(&self, word: &str) -> Vec<String> {
        return vec![
            word.to_string(),
            word.to_lowercase(),
            word.to_uppercase(),
            word[0..1].to_uppercase() + &word[1..],
        ];
    }
}

struct Munger {
    blocks: Vec<Box<dyn MungeBlock>>,
}

impl MungeBlock for Munger {
    fn munge(&self, word: &str) -> Vec<String> {
        let mut words = vec![word.to_string()];

        for block in &self.blocks {
            words.extend(block.munge(word));
        }

        return words
            .drain(..)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
    }
}

macro_rules! munger {
    ( $( $x:expr ),* ) => {
        Munger {
            blocks: vec![ $( Box::new($x) ),* ],
        }
    };
}

fn main() {
    let args = Args::parse();

    let words = read_words(&args.wordlist).unwrap();

    let munger = munger![LeetSpeak::new(), Capitalization::new()];

    let words = words.iter().flat_map(|w| munger.munge(w)).collect();

    write_words(&words, &args.output).unwrap();
}
