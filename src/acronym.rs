//! Acronym and Word model.

use std::ascii::AsciiExt;

/// A word in an acronym, which can have zero or more of its initial letters appear.
#[derive(Clone, Debug)]
pub struct Word {
    string: String,
    initial_count: usize,
}

impl Word {
    /// Creates a new `Word` showing its first letter.
    pub fn new(s: String) -> Word {
        Word {
            string: s,
            initial_count: 1,
        }
    }

    /// Creates an uppercase `String` of the initial letters shown for this word.
    ///
    /// # Examples
    ///
    /// ```
    /// use tbd::acronym::Word;
    /// let w = Word::new("example".to_string());
    /// assert_eq!("E", w.initial());
    /// ```
    pub fn initial(&self) -> String {
        self.string[0..self.initial_count].to_ascii_uppercase()
    }

    /// Sets the number of initials to show for this word.
    ///
    /// # Panics
    ///
    /// If `count` is more than the word length.
    ///
    /// # Examples
    ///
    /// ```
    /// use tbd::acronym::Word;
    /// let mut w = Word::new("example".to_string());
    /// w.set_initial_count(2);
    /// assert_eq!("EX", w.initial());
    /// ```
    pub fn set_initial_count(&mut self, count: usize) {
        assert!(count <= self.string.len());
        self.initial_count = count;
    }

    /// Returns the word length.
    pub fn len(&self) -> usize {
        self.string.len()
    }
}
