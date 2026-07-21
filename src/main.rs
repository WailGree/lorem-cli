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

static WORDS: &[&str] = &[
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
    "aliquip",
    "ex",
    "ea",
    "commodo",
    "consequat",
    "duis",
    "aute",
    "irure",
    "in",
    "reprehenderit",
    "voluptate",
    "velit",
    "esse",
    "cillum",
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

fn main() {
    let args = Args::parse();

    let lorem_text = generate_lorem_ipsum(&args.unit, args.count);
    println!("{}", lorem_text);
}

fn generate_lorem_ipsum(unit: &Unit, count: usize) -> String {
    match unit {
        Unit::Word => generate_words(count).join(" "),
        Unit::Sentence => generate_sentences(count),
        Unit::Paragraph => generate_paragraphs(count),
    }
}

fn generate_words(count: usize) -> Vec<&'static str> {
    let mut words = Vec::with_capacity(count);

    for _ in 0..count {
        let word = WORDS[fastrand::usize(..WORDS.len())];
        words.push(word);
    }

    words
}

fn generate_sentences(count: usize) -> String {
    let mut sentences = Vec::with_capacity(count);

    for _ in 0..count {
        let sentence_length = fastrand::usize(5..15);
        let sentence_words = generate_words(sentence_length);

        let sentence = sentence_words.join(" ");
        sentences.push(sentence);
    }

    sentences.join(". ") + "."
}

fn generate_paragraphs(count: usize) -> String {
    let mut paragraphs = Vec::with_capacity(count);

    for _ in 0..count {
        let paragraph_sentences = fastrand::usize(3..8);
        let sentences = generate_sentences(paragraph_sentences);
        paragraphs.push(sentences);
    }

    paragraphs.join("\n\n")
}
