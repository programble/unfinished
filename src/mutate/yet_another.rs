use std::collections::HashSet;
use acronym::{Word, Acronym};

/// Adds "Yet Another" to the beginning of acronyms.
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::{Mutate, YetAnother};
///
/// let a = Acronym {
///     words: vec![
///         Word(String::from("markup"), 1),
///         Word(String::from("language"), 1),
///     ],
/// };
/// let ms = YetAnother::mutate(&a);
///
/// assert_eq!("YAML", ms.iter().next().unwrap().to_string());
/// ```
pub struct YetAnother;

impl super::Mutate for YetAnother {
    fn mutate(acronym: &Acronym) -> HashSet<Acronym> {
        let mut mutated = acronym.clone();
        mutated.words.insert(0, Word(String::from("yet"), 1));
        mutated.words.insert(1, Word(String::from("another"), 1));

        let mut set = HashSet::new();
        set.insert(mutated);

        set
    }
}
