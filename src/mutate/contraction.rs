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

lazy_static! {
    // https://en.wikipedia.org/wiki/Wikipedia:List_of_English_contractions
    static ref CONTRACTIONS: HashMap<&'static str, Vec<Vec<&'static str>>> = {
        let mut m = HashMap::with_capacity(107);

        m.insert("ain't", vec![vec!["am", "not"], vec!["is", "not"]]);
        m.insert("aren't", vec![vec!["are", "not"]]);
        m.insert("can't", vec![vec!["can", "not"]]);
        m.insert("could've", vec![vec!["could", "have"]]);
        m.insert("couldn't", vec![vec!["could", "not"]]);
        m.insert("couldn't've", vec![vec!["could", "not", "have"], vec!["couldn't", "have"]]);
        m.insert("didn't", vec![vec!["did", "not"]]);
        m.insert("doesn't", vec![vec!["does", "not"]]);
        m.insert("don't", vec![vec!["do", "not"]]);
        m.insert("hadn't", vec![vec!["had", "not"]]);
        m.insert("hadn't've", vec![vec!["had", "not", "have"], vec!["hadn't", "have"]]);
        m.insert("hasn't", vec![vec!["has", "not"]]);
        m.insert("haven't", vec![vec!["have", "not"]]);
        m.insert("he'd", vec![vec!["he", "had"], vec!["he", "would"]]);
        m.insert("he'd've", vec![vec!["he", "would", "have"], vec!["he'd", "have"]]);
        m.insert("he'll", vec![vec!["he", "shall"], vec!["he", "will"]]);
        m.insert("he's", vec![vec!["he", "has"], vec!["he", "is"]]);
        m.insert("how'd", vec![vec!["how", "did"], vec!["how", "would"]]);
        m.insert("how'll", vec![vec!["how", "will"]]);
        m.insert("how's", vec![vec!["how", "has"], vec!["how", "is"], vec!["how", "does"]]);
        m.insert("i'd", vec![vec!["i", "had"], vec!["i", "would"]]);
        m.insert("i'd've", vec![vec!["i", "would", "have"], vec!["i'd", "have"]]);
        m.insert("i'll", vec![vec!["i", "shall"], vec!["i", "will"]]);
        m.insert("i'm", vec![vec!["i", "am"]]);
        m.insert("i've", vec![vec!["i", "have"]]);
        m.insert("isn't", vec![vec!["is", "not"]]);
        m.insert("it'd", vec![vec!["it", "had"], vec!["it", "would"]]);
        m.insert("it'd've", vec![vec!["it", "would", "have"], vec!["it'd", "have"]]);
        m.insert("it'll", vec![vec!["it", "shall"], vec!["it", "will"]]);
        m.insert("it's", vec![vec!["it", "has"], vec!["it", "is"]]);
        m.insert("let's", vec![vec!["let", "us"]]);
        m.insert("mightn't", vec![vec!["might", "not"]]);
        m.insert("mightn't've", vec![vec!["might", "not", "have"], vec!["mightn't", "have"]]);
        m.insert("might've", vec![vec!["might", "have"]]);
        m.insert("mustn't", vec![vec!["must", "not"]]);
        m.insert("must've", vec![vec!["must", "have"]]);
        m.insert("needn't", vec![vec!["need", "not"]]);
        m.insert("not've", vec![vec!["not", "have"]]);
        m.insert("o'clock", vec![vec!["of", "the", "clock"]]);
        m.insert("oughtn't", vec![vec!["ought", "not"]]);
        m.insert("shan't", vec![vec!["shall", "not"]]);
        m.insert("she'd", vec![vec!["she", "had"], vec!["she", "would"]]);
        m.insert("she'd've", vec![vec!["she", "would", "have"], vec!["she'd", "have"]]);
        m.insert("she'll", vec![vec!["she", "shall"], vec!["she", "will"]]);
        m.insert("she's", vec![vec!["she", "has"], vec!["she", "is"]]);
        m.insert("should've", vec![vec!["should", "have"]]);
        m.insert("shouldn't", vec![vec!["should", "not"]]);
        m.insert("shouldn't've", vec![vec!["should", "not", "have"], vec!["shouldn't", "have"]]);
        m.insert("somebody'd", vec![vec!["somebody", "had"], vec!["somebody", "would"]]);
        m.insert("somebody'd've", vec![vec!["somebody", "would", "have"], vec!["somebody'd", "have"]]);
        m.insert("somebody'll", vec![vec!["somebody", "shall"], vec!["somebody", "will"]]);
        m.insert("somebody's", vec![vec!["somebody", "has"], vec!["somebody", "is"]]);
        m.insert("someone'd", vec![vec!["someone", "had"], vec!["someone", "would"]]);
        m.insert("someone'd've", vec![vec!["someone", "would", "have"], vec!["someone'd", "have"]]);
        m.insert("someone'll", vec![vec!["someone", "shall"], vec!["someone", "will"]]);
        m.insert("someone's", vec![vec!["someone", "has"], vec!["someone", "is"]]);
        m.insert("something'd", vec![vec!["something", "had"], vec!["something", "would"]]);
        m.insert("something'd've", vec![vec!["something", "would", "have"], vec!["something'd", "have"]]);
        m.insert("something'll", vec![vec!["something", "shall"], vec!["something", "will"]]);
        m.insert("something's", vec![vec!["something", "has"], vec!["something", "is"]]);
        m.insert("that'll", vec![vec!["that", "will"]]);
        m.insert("that's", vec![vec!["that", "has"], vec!["that", "is"]]);
        m.insert("there'd", vec![vec!["there", "had"], vec!["there", "would"]]);
        m.insert("there'd've", vec![vec!["there", "would", "have"], vec!["there'd", "have"]]);
        m.insert("there're", vec![vec!["there", "are"]]);
        m.insert("there's", vec![vec!["there", "has"], vec!["there", "is"]]);
        m.insert("they'd", vec![vec!["they", "had"], vec!["they", "would"]]);
        m.insert("they'd've", vec![vec!["they", "would", "have"], vec!["they'd", "have"]]);
        m.insert("they'll", vec![vec!["they", "shall"], vec!["they", "will"]]);
        m.insert("they're", vec![vec!["they", "are"]]);
        m.insert("they've", vec![vec!["they", "have"]]);
        m.insert("wasn't", vec![vec!["was", "not"]]);
        m.insert("we'd", vec![vec!["we", "had"], vec!["we", "would"]]);
        m.insert("we'd've", vec![vec!["we", "would", "have"], vec!["we'd", "have"]]);
        m.insert("we'll", vec![vec!["we", "will"]]);
        m.insert("we're", vec![vec!["we", "are"]]);
        m.insert("we've", vec![vec!["we", "have"]]);
        m.insert("weren't", vec![vec!["were", "not"]]);
        m.insert("what'll", vec![vec!["what", "shall"], vec!["what", "will"]]);
        m.insert("what're", vec![vec!["what", "are"]]);
        m.insert("what's", vec![vec!["what", "has"], vec!["what", "is"], vec!["what", "does"]]);
        m.insert("what've", vec![vec!["what", "have"]]);
        m.insert("when's", vec![vec!["when", "has"], vec!["when", "is"]]);
        m.insert("where'd", vec![vec!["where", "did"]]);
        m.insert("where's", vec![vec!["where", "has"], vec!["where", "is"]]);
        m.insert("where've", vec![vec!["where", "have"]]);
        m.insert("who'd", vec![vec!["who", "would"], vec!["who", "had"]]);
        m.insert("who'd've", vec![vec!["who", "would", "have"], vec!["who'd", "have"]]);
        m.insert("who'll", vec![vec!["who", "shall"], vec!["who", "will"]]);
        m.insert("who're", vec![vec!["who", "are"]]);
        m.insert("who's", vec![vec!["who", "has"], vec!["who", "is"]]);
        m.insert("who've", vec![vec!["who", "have"]]);
        m.insert("why'll", vec![vec!["why", "will"]]);
        m.insert("why're", vec![vec!["why", "are"]]);
        m.insert("why's", vec![vec!["why", "has"], vec!["why", "is"]]);
        m.insert("won't", vec![vec!["will", "not"]]);
        m.insert("would've", vec![vec!["would", "have"]]);
        m.insert("wouldn't", vec![vec!["would", "not"]]);
        m.insert("wouldn't've", vec![vec!["would", "not", "have"], vec!["wouldn't", "have"]]);
        m.insert("y'all", vec![vec!["you", "all"]]);
        m.insert("y'all'll", vec![vec!["you", "all", "will"], vec!["y'all", "will"]]);
        m.insert("y'all'd've", vec![
                 vec!["you", "all", "should", "have"],
                 vec!["you", "all", "could", "have"],
                 vec!["you", "all", "would", "have"],
                 vec!["you", "all", "should've"],
                 vec!["you", "all", "could've"],
                 vec!["you", "all", "would've"],
                 vec!["y'all", "should", "have"],
                 vec!["y'all", "could", "have"],
                 vec!["y'all", "would", "have"],
                 vec!["y'all", "should've"],
                 vec!["y'all", "could've"],
                 vec!["y'all", "would've"],
        ]);
        m.insert("you'd", vec![vec!["you", "had"], vec!["you", "would"]]);
        m.insert("you'd've", vec![vec!["you", "would", "have"], vec!["you'd", "have"]]);
        m.insert("you'll", vec![vec!["you", "shall"], vec!["you", "will"]]);
        m.insert("you're", vec![vec!["you", "are"]]);
        m.insert("you've", vec![vec!["you", "have"]]);

        m
    };
}
