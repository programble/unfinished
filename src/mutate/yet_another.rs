use acronym::{Word, Acronym};

/// Adds "Yet Another" to the beginning of acronyms.
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::YetAnother;
///
/// let a = Acronym {
///     words: vec![
///         Word(String::from("markup"), 1),
///         Word(String::from("language"), 1),
///     ],
/// };
/// let mut m = YetAnother::new(&a);
///
/// assert_eq!("YAML", m.next().map(|a| a.to_string()).unwrap());
/// ```
pub struct YetAnother<'a> {
    acronym: &'a Acronym,
    done: bool,
}

impl<'a> YetAnother<'a> {
    pub fn new(acronym: &'a Acronym) -> YetAnother {
        YetAnother { done: false, acronym: acronym }
    }
}

impl<'a> Iterator for YetAnother<'a> {
    type Item = Acronym;

    fn next(&mut self) -> Option<Acronym> {
        if self.done { return None; }

        let mut next = self.acronym.clone();
        next.words.insert(0, Word(String::from("yet"), 1));
        next.words.insert(1, Word(String::from("another"), 1));

        self.done = true;
        Some(next)
    }
}
