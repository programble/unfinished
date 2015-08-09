//! Acronym and Word model.

use std::ops::Add;
use itertools::Itertools;

/// A word in an acronym, which can have zero or more of its initial letters appear.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Word(pub String, pub usize);

impl Word {
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
    /// let a = Word(String::from("example"), 1);
    /// let b = a.clone().with_count(2);
    /// assert_eq!("EX", b.initial());
    /// ```
    pub fn with_count(mut self, count: usize) -> Self {
        assert!(count <= self.0.len());
        self.1 = count;
        self
    }

    /// Creates an uppercase `String` of the visible initial letters for this word.
    ///
    /// # Examples
    ///
    /// ```
    /// use tbd::acronym::Word;
    /// let w = Word(String::from("example"), 1);
    /// assert_eq!("E", w.initial());
    /// ```
    pub fn initial(&self) -> String {
        self.0[..self.1].to_uppercase()
    }

    /// Creates a `String` of this `Word` with the visible initial letters in uppercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use tbd::acronym::Word;
    /// let w = Word(String::from("example"), 2);
    /// assert_eq!("EXample", w.expansion());
    /// ```
    pub fn expansion(&self) -> String {
        let head = self.initial();
        let tail = self.0[self.1..].to_lowercase();
        head + &tail
    }

    /// Returns the string of this `Word`.
    pub fn string(&self) -> &str {
        &self.0[..]
    }

    /// Returns the count of this `Word`.
    pub fn count(&self) -> usize {
        self.1
    }

    /// Returns the word length.
    pub fn len(&self) -> usize {
        self.0.len()
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
///         Word(String::from("great"), 1),
///         Word(String::from("acronym"), 1),
///         Word(String::from("example"), 1),
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
    ///         Word(String::from("great"), 2),
    ///         Word(String::from("acronym"), 1),
    ///         Word(String::from("example"), 3),
    ///     ],
    /// };
    ///
    /// assert_eq!(6, a.len());
    /// ```
    pub fn len(&self) -> usize {
        self.words.iter()
            .map(|&Word(_, n)| n)
            .fold(0, usize::add)
    }

    /// Creates a `String` of the `Acronym` expansion, made up of the expansion of each `Word`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tbd::acronym::{Word, Acronym};
    ///
    /// let a = Acronym {
    ///     words: vec![
    ///         Word(String::from("great"), 2),
    ///         Word(String::from("acronym"), 1),
    ///         Word(String::from("example"), 3),
    ///     ],
    /// };
    ///
    /// assert_eq!("GReat Acronym EXAmple", a.expansion());
    /// ```
    pub fn expansion(&self) -> String {
        self.words.iter()
            .map(Word::expansion)
            .join(" ")
    }
}

impl ToString for Acronym {
    fn to_string(&self) -> String {
        self.words.iter()
            .map(Word::initial)
            .fold(String::new(), |s, i| s + &i)
    }
}
