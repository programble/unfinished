use std::collections::HashSet;
use acronym::{Word, Acronym};

/// Generates all possible recursive acronyms for an `Acronym`, for each letter, with or without
/// "is".
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::{Mutate, Recursive};
///
/// let a = Acronym {
///     words: vec![
///         Word::new(String::from("Not")),
///         Word::new(String::from("an")).with_count(0),
///         Word::new(String::from("Emulator")),
///     ],
/// };
/// let ms: Vec<String> = Recursive::mutate(&a).iter()
///     .map(Acronym::to_string)
///     .collect();
///
/// assert!(ms.contains(&String::from("WINE")));
/// ```
pub struct Recursive;

impl super::Mutate for Recursive {
    fn mutate(acronym: &Acronym) -> HashSet<Acronym> {
        let mut set = HashSet::new();

        let tail = acronym.to_string();
        let is_tail = String::from("I") + &tail;

        for l in (b'A'..b'Z') {
            let word = Word::new((l as char).to_string() + &tail);
            let mut mutated = acronym.clone();
            mutated.words.insert(0, word);
            set.insert(mutated);

            let word = Word::new((l as char).to_string() + &is_tail);
            let mut mutated = acronym.clone();
            mutated.words.insert(0, word);
            mutated.words.insert(1, Word::new(String::from("Is")));
            set.insert(mutated);
        }

        set
    }
}
