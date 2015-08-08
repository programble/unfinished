//! Acronym mutation.

use acronym::Acronym;

/// An `Acronym` mutator.
pub trait Mutate: Iterator<Item = Acronym> {
    /// Creates an `Iterator` over mutated acronyms.
    fn new(acronym: &Acronym) -> Self;
}

/// A mutator that yields nothing.
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::{Mutate, Nop};
///
/// let mut m = Nop::new(&Acronym {
///     words: vec![
///         Word(String::from("nop"), 1),
///         Word(String::from("example"), 1),
///     ]
/// });
///
/// assert_eq!(None, m.next());
/// ```
pub struct Nop;

impl Mutate for Nop {
    fn new(_acronym: &Acronym) -> Self {
        return Nop;
    }
}

impl Iterator for Nop {
    type Item = Acronym;

    fn next(&mut self) -> Option<Acronym> {
        None
    }
}

//pub use self::yet_another::YetAnother;
//mod yet_another;
//
//pub use self::recursive::Recursive;
//mod recursive;
//
//pub use self::permutation::Permutation;
//mod permutation;
//
//pub use self::apostrophe::{WordApostropheS, WordIs};
//mod apostrophe;
//
//pub use self::contraction::{Contract, Expand};
//mod contraction;
