use std::ops::Range;
use acronym::{Word, Acronym};

/// Generates all possible recursive acronyms for an `Acronym`.
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::Recursive;
///
/// let a = Acronym {
///     words: vec![
///         Word(String::from("not"), 1),
///         Word(String::from("an"), 0),
///         Word(String::from("emulator"), 1),
///     ],
/// };
/// let mut m = Recursive::new(&a, true);
///
/// assert!(m.any(|a| a.to_string() == "WINE"));
/// ```
pub struct Recursive<'a> {
    acronym: &'a Acronym,
    insert_is: bool,
    letters: Range<u8>,
}

impl<'a> Recursive<'a> {
    /// Creates a `Recursive` which optionally inserts "is" as the second word.
    pub fn new(acronym: &'a Acronym, insert_is: bool) -> Recursive {
        Recursive {
            acronym: acronym,
            insert_is: insert_is,
            letters: b'A'..b'Z',
        }
    }
}

impl<'a> Iterator for Recursive<'a> {
    type Item = Acronym;

    fn next(&mut self) -> Option<Acronym> {
        self.letters.next().map(|letter| {
            let letter = (letter as char).to_string();

            let mut next = self.acronym.clone();
            let word = Word(letter + &next.to_string(), 1);
            next.words.insert(0, word);
            if self.insert_is {
                next.words.insert(1, Word(String::from("is"), 1));
            }

            next
        })
    }
}
