use std::collections::HashSet;
use std::ops::Range;
use std::cmp;

use acronym::Acronym;
use bits::Bits;

/// Toggles visibility of prepositions and conjunctions.
///
/// Generates each possible combination of shown/hidden using binary counting.
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::Toggle;
///
/// let a = Acronym {
///     words: vec![
///         Word(String::from("intitute"), 1),
///         Word(String::from("of"), 1),
///         Word(String::from("electric"), 1),
///         Word(String::from("and"), 1),
///         Word(String::from("electronics"), 1),
///         Word(String::from("engineers"), 1),
///     ],
/// };
/// let mut m = Toggle::new(&a);
///
/// assert!(m.any(|a| a.to_string() == "IEEE"));
/// ```
pub struct Toggle<'a> {
    acronym: &'a Acronym,
    range: Range<u64>,
}

impl<'a> Toggle<'a> {
    pub fn new(acronym: &Acronym) -> Toggle {
        let count = acronym.words.iter()
            .map(|w| w.string().to_lowercase())
            .filter(|s| TOGGLE_WORDS.contains(&s[..]))
            .count();
        let count = cmp::min(count, 63);
        let combos = 1 << count;

        Toggle {
            acronym: acronym,
            range: 0..combos,
        }
    }
}

impl<'a> Iterator for Toggle<'a> {
    type Item = Acronym;

    fn next(&mut self) -> Option<Acronym> {
        self.range.next().map(|combo| {
            let mut next = Acronym { words: Vec::new() };
            let mut bits = combo.bits();

            for word in &self.acronym.words {
                let lower = word.string().to_lowercase();

                if TOGGLE_WORDS.contains(&*lower) {
                    if bits.next().unwrap() {
                        next.words.push(word.clone().with_count(1));
                    } else {
                        next.words.push(word.clone().with_count(0));
                    }
                } else {
                    next.words.push(word.clone());
                }
            }

            next
        })
    }
}

macro_rules! set {
    ( $( $e:expr ),* ) => {{
        let mut set = HashSet::new();
        $( set.insert($e); )*
        set
    }}
}

lazy_static! {
    static ref TOGGLE_WORDS: HashSet<&'static str> = set![
        // Conjunctions
        "for", "and", "nor", "but", "or", "yet", "so",

        // Prepositions
        // https://en.wikipedia.org/wiki/List_of_English_prepositions
        "a", "abaft", "abeam", "aboard", "about", "above", "absent", "across",
        "afore", "after", "against", "along", "alongside", "amid", "amidst",
        "among", "amongst", "an", "anenst", "apropos", "apud", "around", "as",
        "aside", "astride", "at", "athwart", "atop",

        "barring", "before", "behind", "below", "beneath", "beside", "besides",
        "between", "beyond", "but", "by",

        "chez", "circa", "concerning",
        "despite", "down", "during",
        "except", "excluding",
        "failing", "following", "for", "forenenst", "from",
        "given",
        "in", "including", "inside", "into",
        "like",
        "mid", "midst", "minus", "modulo",
        "near", "next", "notwithstanding",
        "o'", "of", "off", "on", "onto", "opposite", "out", "outside", "over",
        "pace", "past", "per", "plus", "pro",
        "qua",
        "regarding", "round",
        "sans", "save", "since",

        "than", "through", "throughout", "till", "times", "to", "toward",
        "towards",

        "under", "underneath", "unlike", "until", "unto", "up", "upon",
        "versus", "via", "vice", "vis-à-vis",
        "with", "within", "without", "worth"
    ];
}

#[cfg(test)]
mod tests {
    use acronym::{Word, Acronym};

    #[test]
    fn toggle() {
        let a = Acronym {
            words: vec![
                Word(String::from("test"), 1),
                Word(String::from("and"), 1),
                Word(String::from("test"), 1),
                Word(String::from("or"), 0),
                Word(String::from("test"), 1),
                Word(String::from("between"), 1),
            ],
        };
        let r = super::Toggle::new(&a)
            .map(|a| a.expansion())
            .collect::<Vec<_>>();

        assert_eq!(8, r.len());
        assert!(r.contains(&String::from("Test and Test or Test between")));
        assert!(r.contains(&String::from("Test and Test or Test Between")));
        assert!(r.contains(&String::from("Test and Test Or Test between")));
        assert!(r.contains(&String::from("Test and Test Or Test Between")));
        assert!(r.contains(&String::from("Test And Test or Test between")));
        assert!(r.contains(&String::from("Test And Test or Test Between")));
        assert!(r.contains(&String::from("Test And Test Or Test between")));
        assert!(r.contains(&String::from("Test And Test Or Test Between")));
    }
}
