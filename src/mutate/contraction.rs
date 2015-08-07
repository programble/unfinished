use std::collections::HashSet;
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
        Self::mutate_from(&mut set, 0, &acronym);
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
pub struct Expand;

impl super::Mutate for Expand {
    fn mutate(acronym: &Acronym) -> HashSet<Acronym> {
        HashSet::new()
    }
}
