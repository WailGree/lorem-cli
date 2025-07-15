use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "lorem-cli")]
#[command(about = "Generate lorem ipsum text")]
#[command(version)]
struct Args {
    /// Type of unit to count
    #[arg(short, long, value_enum, default_value_t = Unit::Sentence)]
    unit: Unit,

    /// Number of units to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

#[derive(ValueEnum, Clone, Debug)]
enum Unit {
    #[clap(alias = "w")]
    Word,
    #[clap(alias = "s")]
    Sentence,
    #[clap(alias = "p")]
    Paragraph,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let lorem_text = generate_lorem_ipsum(&args.unit, args.count)?;
    println!("{}", lorem_text);

    Ok(())
}

fn generate_lorem_ipsum(unit: &Unit, count: usize) -> Result<String> {
    let lorem_words = vec![
        "lorem",
        "ipsum",
        "dolor",
        "sit",
        "amet",
        "consectetur",
        "adipiscing",
        "elit",
        "sed",
        "do",
        "eiusmod",
        "tempor",
        "incididunt",
        "ut",
        "labore",
        "et",
        "dolore",
        "magna",
        "aliqua",
        "ut",
        "enim",
        "ad",
        "minim",
        "veniam",
        "quis",
        "nostrud",
        "exercitation",
        "ullamco",
        "laboris",
        "nisi",
        "ut",
        "aliquip",
        "ex",
        "ea",
        "commodo",
        "consequat",
        "duis",
        "aute",
        "irure",
        "dolor",
        "in",
        "reprehenderit",
        "voluptate",
        "velit",
        "esse",
        "cillum",
        "dolore",
        "eu",
        "fugiat",
        "nulla",
        "pariatur",
        "excepteur",
        "sint",
        "occaecat",
        "cupidatat",
        "non",
        "proident",
        "sunt",
        "culpa",
        "qui",
        "officia",
        "deserunt",
        "mollit",
        "anim",
        "id",
        "est",
        "laborum",
        "et",
        "dolore",
        "magna",
        "aliqua",
        "ut",
        "enim",
        "ad",
        "minim",
        "veniam",
        "quis",
        "nostrud",
        "exercitation",
        "ullamco",
        "laboris",
        "nisi",
        "ut",
        "aliquip",
        "ex",
        "ea",
        "commodo",
        "consequat",
        "duis",
        "aute",
        "irure",
        "dolor",
        "in",
        "reprehenderit",
        "voluptate",
        "velit",
        "esse",
        "cillum",
        "dolore",
        "eu",
        "fugiat",
        "nulla",
        "pariatur",
        "excepteur",
        "sint",
        "occaecat",
        "cupidatat",
        "non",
        "proident",
        "sunt",
        "culpa",
        "qui",
        "officia",
        "deserunt",
        "mollit",
        "anim",
        "id",
        "est",
        "laborum",
    ];

    match unit {
        Unit::Word => {
            let mut words = Vec::new();
            for _ in 0..count {
                let word = lorem_words[fastrand::usize(..lorem_words.len())];
                words.push(word);
            }
            Ok(words.join(" "))
        }
        Unit::Sentence => {
            let mut sentences = Vec::new();
            for _ in 0..count {
                let sentence_length = fastrand::usize(5..15);
                let mut sentence_words = Vec::new();

                for _ in 0..sentence_length {
                    let word = lorem_words[fastrand::usize(..lorem_words.len())];
                    sentence_words.push(word);
                }

                let sentence = sentence_words.join(" ");
                sentences.push(sentence);
            }
            Ok(sentences.join(". ") + ".")
        }
        Unit::Paragraph => {
            let mut paragraphs = Vec::new();
            for _ in 0..count {
                let paragraph_sentences = fastrand::usize(3..8);
                let mut sentences = Vec::new();

                for _ in 0..paragraph_sentences {
                    let sentence_length = fastrand::usize(5..15);
                    let mut sentence_words = Vec::new();

                    for _ in 0..sentence_length {
                        let word = lorem_words[fastrand::usize(..lorem_words.len())];
                        sentence_words.push(word);
                    }

                    let sentence = sentence_words.join(" ");
                    sentences.push(sentence);
                }

                let paragraph = sentences.join(". ") + ".";
                paragraphs.push(paragraph);
            }
            Ok(paragraphs.join("\n\n"))
        }
    }
}
