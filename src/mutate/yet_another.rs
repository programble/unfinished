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
///         Word::new(String::from("Markup")),
///         Word::new(String::from("Language")),
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
        mutated.words.insert(0, Word::new(String::from("Yet")));
        mutated.words.insert(1, Word::new(String::from("Another")));

        let mut set = HashSet::new();
        set.insert(mutated);

        set
    }
}
