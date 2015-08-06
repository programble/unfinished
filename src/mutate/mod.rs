//! Acronym mutation.

use std::collections::HashSet;
use acronym::Acronym;

/// An `Acronym` mutator.
pub trait Mutate {
    /// Creates a `HashSet` of mutated `Acronym` from an initial `Acronym`.
    fn mutate(acronym: &Acronym) -> HashSet<Acronym>;
}

/// A mutator that does nothing.
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::{Mutate, Nop};
///
/// let a = Acronym {
///     words: vec![
///         Word::new(String::from("Nop")),
///         Word::new(String::from("Example")),
///     ],
/// };
///
/// assert_eq!(0, Nop::mutate(&a).len());
/// ```
pub struct Nop;

impl Mutate for Nop {
    fn mutate(_acronym: &Acronym) -> HashSet<Acronym> {
        HashSet::new()
    }
}

pub use self::yet_another::YetAnother;

pub mod yet_another;
