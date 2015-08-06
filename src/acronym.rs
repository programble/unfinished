//! Acronym and Word model.

use std::ops::Add;
use std::ascii::AsciiExt;

/// A word in an acronym, which can have zero or more of its initial letters appear.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Word {
    string: String,
    initial_count: usize,
}

impl Word {
    /// Creates a new `Word` with its first letter visible.
    pub fn new(s: String) -> Word {
        Word {
            string: s,
            initial_count: 1,
        }
    }

    /// Sets the number of visible initial letters.
    ///
    /// # Panics
    ///
    /// If `count` is more than the word length.
    ///
    /// # Examples
    ///
    /// ```
    /// use tbd::acronym::Word;
    /// let mut w = Word::new(String::from("example")).with_count(2);
    /// assert_eq!("EX", w.initial());
    /// ```
    pub fn with_count(mut self, count: usize) -> Self {
        assert!(count <= self.string.len());
        self.initial_count = count;
        self
    }

    /// Returns the number of visible initial letters.
    pub fn initial_count(&self) -> usize {
        self.initial_count
    }

    /// Creates an uppercase `String` of the visible initial letters for this word.
    ///
    /// # Examples
    ///
    /// ```
    /// use tbd::acronym::Word;
    /// let w = Word::new(String::from("example"));
    /// assert_eq!("E", w.initial());
    /// ```
    pub fn initial(&self) -> String {
        self.string[..self.initial_count].to_ascii_uppercase()
    }

    /// Creates a `String` of this `Word` with the visible initial letters in uppercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use tbd::acronym::Word;
    /// let w = Word::new(String::from("example")).with_count(2);
    /// assert_eq!("EXample", w.expansion());
    /// ```
    pub fn expansion(&self) -> String {
        let head = self.initial();
        let tail = self.string[self.initial_count..].to_ascii_lowercase();
        head + &tail
    }

    /// Returns the word length.
    pub fn len(&self) -> usize {
        self.string.len()
    }
}

/// An acronym.
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
///
/// let a = Acronym {
///     words: vec![
///         Word::new(String::from("Great")),
///         Word::new(String::from("Acronym")),
///         Word::new(String::from("Example")),
///     ],
/// };
///
/// assert_eq!("GAE", a.to_string());
/// ```
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Acronym {
    pub words: Vec<Word>,
}

impl Acronym {
    /// Returns the acronym length.
    ///
    /// # Examples
    ///
    /// ```
    /// use tbd::acronym::{Word, Acronym};
    ///
    /// let a = Acronym {
    ///     words: vec![
    ///         Word::new(String::from("Great")).with_count(2),
    ///         Word::new(String::from("Acronym")),
    ///         Word::new(String::from("Example")).with_count(3),
    ///     ],
    /// };
    ///
    /// assert_eq!(6, a.len());
    /// ```
    pub fn len(&self) -> usize {
        self.words.iter()
            .map(Word::initial_count)
            .fold(0, usize::add)
    }
}

impl ToString for Acronym {
    fn to_string(&self) -> String {
        self.words.iter()
            .map(Word::initial)
            .fold(String::new(), |s, i| s + &i)
    }
}
