use std::collections::HashSet;
use std::ops::Range;
use std::cmp;
use acronym::Acronym;
use bits::Bits;

/// Generates permutations of hidden and shown prepositions.
///
/// Generates each possible combination of shown/hidden using binary counting.
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::Preposition;
///
/// let a = Acronym {
///     words: vec![
///         Word(String::from("Massachusetts"), 1),
///         Word(String::from("Institute"), 1),
///         Word(String::from("of"), 1),
///         Word(String::from("Technology"), 1),
///     ],
/// };
/// let mut m = Preposition::new(&a);
///
/// assert!(m.any(|a| a.to_string() == "MIT"));
/// ```
pub struct Preposition<'a> {
    acronym: &'a Acronym,
    range: Range<u64>,
}

impl<'a> Preposition<'a> {
    pub fn new(acronym: &Acronym) -> Preposition {
        let count = acronym.words.iter()
            .map(|w| w.string().to_lowercase())
            .filter(|s| PREPOSITIONS.contains(&s[..]))
            .count();
        let count = cmp::min(count, 63);
        let combos = 2u64.pow(count as u32);

        Preposition {
            acronym: acronym,
            range: 0..combos,
        }
    }
}

impl<'a> Iterator for Preposition<'a> {
    type Item = Acronym;

    fn next(&mut self) -> Option<Acronym> {
        self.range.next().map(|combo| {
            let mut next = Acronym { words: Vec::new() };
            let mut bits = combo.bits();

            for word in &self.acronym.words {
                let lower = word.string().to_lowercase();

                if PREPOSITIONS.contains(&lower[..]) {
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
    (
        $($e:expr),*
    ) => {{
        let mut s = HashSet::new();
        $( s.insert($e); )*
        s
    }}
}

lazy_static! {
    // https://en.wikipedia.org/wiki/List_of_English_prepositions
    static ref PREPOSITIONS: HashSet<&'static str> = set! [
        "a", "abaft", "abeam", "aboard", "about", "above", "absent", "across",
        "afore", "after", "against", "along", "alongside", "amid", "amidst",
        "among", "amongst", "an", "anenst", "apropos", "apud", "around", "as",
        "aside", "astride", "at", "athwart", "atop", "barring", "before", "behind",
        "below", "beneath", "beside", "besides", "between", "beyond", "but", "by",
        "chez", "circa", "concerning", "despite", "down", "during", "except",
        "excluding", "failing", "following", "for", "forenenst", "from", "given",
        "in", "including", "inside", "into", "like", "mid", "midst", "minus",
        "modulo", "near", "next", "notwithstanding", "o'", "of", "off", "on",
        "onto", "opposite", "out", "outside", "over", "pace", "past", "per",
        "plus", "pro", "qua", "regarding", "round", "sans", "save", "since",
        "than", "through", "throughout", "till", "times", "to", "toward",
        "towards", "under", "underneath", "unlike", "until", "unto", "up", "upon",
        "versus", "via", "vice", "vis-à-vis", "with", "within", "without", "worth"
    ];
}

#[cfg(test)]
mod tests {
    use acronym::{Word, Acronym};

    #[test]
    fn preposition() {
        let a = Acronym {
            words: vec![
                Word(String::from("test"), 1),
                Word(String::from("a"), 1),
                Word(String::from("test"), 1),
                Word(String::from("of"), 1),
                Word(String::from("testing"), 1),
                Word(String::from("via"), 0),
                Word(String::from("tests"), 1),
            ],
        };
        let r = super::Preposition::new(&a)
            .map(|a| a.expansion())
            .collect::<Vec<_>>();

        assert_eq!(8, r.len());
        assert!(r.contains(&String::from("Test a Test of Testing via Tests")));
        assert!(r.contains(&String::from("Test a Test of Testing Via Tests")));
        assert!(r.contains(&String::from("Test a Test Of Testing via Tests")));
        assert!(r.contains(&String::from("Test a Test Of Testing Via Tests")));
        assert!(r.contains(&String::from("Test A Test of Testing via Tests")));
        assert!(r.contains(&String::from("Test A Test of Testing Via Tests")));
        assert!(r.contains(&String::from("Test A Test Of Testing via Tests")));
        assert!(r.contains(&String::from("Test A Test Of Testing Via Tests")));
    }
}
