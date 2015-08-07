use std::collections::{HashSet, HashMap};
use std::ascii::AsciiExt;
use acronym::{Word, Acronym};

/// Transforms "word is" sequences into "word's".
///
/// # Example
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::{Mutate, ApostropheS};
///
/// let a = Acronym {
///     words: vec![
///         Word(String::from("gnu"), 1),
///         Word(String::from("is"), 1),
///         Word(String::from("not"), 1),
///         Word(String::from("unix"), 1),
///     ],
/// };
/// let m = ApostropheS::mutate(&a).into_iter().next().unwrap();
///
/// assert_eq!(Word(String::from("gnu's"), 1), m.words[0]);
/// ```
pub struct ApostropheS;

impl ApostropheS {
    fn mutate_from(set: &mut HashSet<Acronym>, i: usize, acronym: &Acronym) {
        if acronym.words.len() <= i + 1 {
            return;
        }

        let Word(ref word_s, word_n) = acronym.words[i];
        let Word(ref next_s, _) = acronym.words[i + 1];

        if next_s.to_ascii_lowercase() == "is" {
            let mut mutated = acronym.clone();
            mutated.words.remove(i + 1);
            mutated.words[i] = Word(word_s.clone() + "'s", word_n);

            Self::mutate_from(set, i + 1, &mutated);
            set.insert(mutated);
        }

        Self::mutate_from(set, i + 1, acronym);
    }
}

impl super::Mutate for ApostropheS {
    fn mutate(acronym: &Acronym) -> HashSet<Acronym> {
        let mut set = HashSet::new();
        Self::mutate_from(&mut set, 0, acronym);
        set
    }
}

/// Combines words into contractions.
pub struct Contract;

impl super::Mutate for Contract {
    fn mutate(acronym: &Acronym) -> HashSet<Acronym> {
        HashSet::new()
    }
}

/// Expands contractions into separate words.
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::{Mutate, Expand};
///
/// let a = Acronym {
///     words: vec![
///         Word(String::from("YAML"), 1),
///         Word(String::from("ain't"), 1),
///         Word(String::from("markup"), 1),
///         Word(String::from("language"), 1),
///     ],
/// };
/// let ms: Vec<String> = Expand::mutate(&a).iter()
///     .map(Acronym::to_string)
///     .collect();
///
/// assert!(ms.contains(&String::from("YINML")));
/// ```
pub struct Expand;

impl Expand {
    fn mutate_from(set: &mut HashSet<Acronym>, i: usize, acronym: &Acronym) {
        if i == acronym.words.len() {
            return;
        }

        let current = acronym.words[i].0.to_ascii_lowercase();
        if let Some(expansions) = CONTRACTIONS.get(&*current) {
            for expansion in expansions {
                let mut mutated = acronym.clone();
                mutated.words.remove(i);
                for &word in expansion.iter().rev() {
                    mutated.words.insert(i, Word(String::from(word), 1));
                }

                Self::mutate_from(set, i + 1, &mutated);
                set.insert(mutated);
            }
        }

        Self::mutate_from(set, i + 1, acronym);
    }
}

impl super::Mutate for Expand {
    fn mutate(acronym: &Acronym) -> HashSet<Acronym> {
        let mut set = HashSet::new();
        Self::mutate_from(&mut set, 0, acronym);
        set
    }
}

macro_rules! contractions {
    (
        $(
            $contraction:expr => [ $( [ $( $expansion:expr ),+ ] ),+ ],
        )*
    ) => {{
        let mut m = HashMap::new();
        $(
            m.insert($contraction, vec![ $( vec![ $( $expansion ),* ] ),* ]);
        )*
        m
    }}
}

lazy_static! {
    // https://en.wikipedia.org/wiki/Wikipedia:List_of_English_contractions
    static ref CONTRACTIONS: HashMap<&'static str, Vec<Vec<&'static str>>> = contractions! [
        "ain't"          => [["am", "not"],
                             ["is", "not"]],
        "aren't"         => [["are", "not"]],
        "can't"          => [["can", "not"]],
        "could've"       => [["could", "have"]],
        "couldn't"       => [["could", "not"]],
        "couldn't've"    => [["could", "not", "have"],
                             ["couldn't", "have"]],
        "didn't"         => [["did", "not"]],
        "doesn't"        => [["does", "not"]],
        "don't"          => [["do", "not"]],
        "hadn't"         => [["had", "not"]],
        "hadn't've"      => [["had", "not", "have"],
                             ["hadn't", "have"]],
        "hasn't"         => [["has", "not"]],
        "haven't"        => [["have", "not"]],
        "he'd"           => [["he", "had"],
                             ["he", "would"]],
        "he'd've"        => [["he", "would", "have"],
                             ["he'd", "have"],
                             ["he", "would've"]],
        "he'll"          => [["he", "shall"],
                             ["he", "will"]],
        "he's"           => [["he", "has"],
                             ["he", "is"]],
        "how'd"          => [["how", "did"],
                             ["how", "would"]],
        "how'll"         => [["how", "will"]],
        "how's"          => [["how", "has"],
                             ["how", "is"],
                             ["how", "does"]],
        "i'd"            => [["i", "had"],
                             ["i", "would"]],
        "i'd've"         => [["i", "would", "have"],
                             ["i'd", "have"],
                             ["i", "would've"]],
        "i'll"           => [["i", "shall"],
                             ["i", "will"]],
        "i'm"            => [["i", "am"]],
        "i've"           => [["i", "have"]],
        "isn't"          => [["is", "not"]],
        "it'd"           => [["it", "had"],
                             ["it", "would"]],
        "it'd've"        => [["it", "would", "have"],
                             ["it'd", "have"],
                             ["it", "would've"]],
        "it'll"          => [["it", "shall"],
                             ["it", "will"]],
        "it's"           => [["it", "has"],
                             ["it", "is"]],
        "let's"          => [["let", "us"]],
        "mightn't"       => [["might", "not"]],
        "mightn't've"    => [["might", "not", "have"],
                             ["mightn't", "have"]],
        "might've"       => [["might", "have"]],
        "mustn't"        => [["must", "not"]],
        "must've"        => [["must", "have"]],
        "needn't"        => [["need", "not"]],
        "not've"         => [["not", "have"]],
        "o'clock"        => [["of", "the", "clock"]],
        "oughtn't"       => [["ought", "not"]],
        "shan't"         => [["shall", "not"]],
        "she'd"          => [["she", "had"],
                             ["she", "would"]],
        "she'd've"       => [["she", "would", "have"],
                             ["she'd", "have"],
                             ["she", "would've"]],
        "she'll"         => [["she", "shall"],
                             ["she", "will"]],
        "she's"          => [["she", "has"],
                             ["she", "is"]],
        "should've"      => [["should", "have"]],
        "shouldn't"      => [["should", "not"]],
        "shouldn't've"   => [["should", "not", "have"],
                             ["shouldn't", "have"]],
        "somebody'd"     => [["somebody", "had"],
                             ["somebody", "would"]],
        "somebody'd've"  => [["somebody", "would", "have"],
                             ["somebody'd", "have"],
                             ["somebody", "would've"]],
        "somebody'll"    => [["somebody", "shall"],
                             ["somebody", "will"]],
        "somebody's"     => [["somebody", "has"],
                             ["somebody", "is"]],
        "someone'd"      => [["someone", "had"],
                             ["someone", "would"]],
        "someone'd've"   => [["someone", "would", "have"],
                             ["someone'd", "have"],
                             ["someone", "would've"]],
        "someone'll"     => [["someone", "shall"],
                             ["someone", "will"]],
        "someone's"      => [["someone", "has"],
                             ["someone", "is"]],
        "something'd"    => [["something", "had"],
                             ["something", "would"]],
        "something'd've" => [["something", "would", "have"],
                             ["something'd", "have"],
                             ["something", "would've"]],
        "something'll"   => [["something", "shall"],
                             ["something", "will"]],
        "something's"    => [["something", "has"],
                             ["something", "is"]],
        "that'll"        => [["that", "will"]],
        "that's"         => [["that", "has"],
                             ["that", "is"]],
        "there'd"        => [["there", "had"],
                             ["there", "would"]],
        "there'd've"     => [["there", "would", "have"],
                             ["there'd", "have"],
                             ["there", "would've"]],
        "there're"       => [["there", "are"]],
        "there's"        => [["there", "has"],
                             ["there", "is"]],
        "they'd"         => [["they", "had"],
                             ["they", "would"]],
        "they'd've"      => [["they", "would", "have"],
                             ["they'd", "have"],
                             ["they", "would've"]],
        "they'll"        => [["they", "shall"],
                             ["they", "will"]],
        "they're"        => [["they", "are"]],
        "they've"        => [["they", "have"]],
        "wasn't"         => [["was", "not"]],
        "we'd"           => [["we", "had"],
                             ["we", "would"]],
        "we'd've"        => [["we", "would", "have"],
                             ["we'd", "have"],
                             ["we", "would've"]],
        "we'll"          => [["we", "will"]],
        "we're"          => [["we", "are"]],
        "we've"          => [["we", "have"]],
        "weren't"        => [["were", "not"]],
        "what'll"        => [["what", "shall"],
                             ["what", "will"]],
        "what're"        => [["what", "are"]],
        "what's"         => [["what", "has"],
                             ["what", "is"],
                             ["what", "does"]],
        "what've"        => [["what", "have"]],
        "when's"         => [["when", "has"],
                             ["when", "is"]],
        "where'd"        => [["where", "did"]],
        "where's"        => [["where", "has"],
                             ["where", "is"]],
        "where've"       => [["where", "have"]],
        "who'd"          => [["who", "would"],
                             ["who", "had"]],
        "who'd've"       => [["who", "would", "have"],
                             ["who'd", "have"],
                             ["who", "would've"]],
        "who'll"         => [["who", "shall"],
                             ["who", "will"]],
        "who're"         => [["who", "are"]],
        "who's"          => [["who", "has"],
                             ["who", "is"]],
        "who've"         => [["who", "have"]],
        "why'll"         => [["why", "will"]],
        "why're"         => [["why", "are"]],
        "why's"          => [["why", "has"],
                             ["why", "is"]],
        "won't"          => [["will", "not"]],
        "would've"       => [["would", "have"]],
        "wouldn't"       => [["would", "not"]],
        "wouldn't've"    => [["would", "not", "have"],
                             ["wouldn't", "have"]],
        "y'all"          => [["you", "all"]],
        "y'all'll"       => [["you", "all", "will"],
                             ["y'all", "will"]],
        "y'all'd've"     => [["you", "all", "should", "have"],
                             ["you", "all", "could", "have"],
                             ["you", "all", "would", "have"],
                             ["you", "all", "should've"],
                             ["you", "all", "could've"],
                             ["you", "all", "would've"],
                             ["y'all", "should", "have"],
                             ["y'all", "could", "have"],
                             ["y'all", "would", "have"],
                             ["y'all", "should've"],
                             ["y'all", "could've"],
                             ["y'all", "would've"]],
        "you'd"          => [["you", "had"],
                             ["you", "would"]],
        "you'd've"       => [["you", "would", "have"],
                             ["you'd", "have"],
                             ["you", "would've"]],
        "you'll"         => [["you", "shall"],
                             ["you", "will"]],
        "you're"         => [["you", "are"]],
        "you've"         => [["you", "have"]],
    ];
}
