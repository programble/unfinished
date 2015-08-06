use std::collections::HashSet;
use acronym::Acronym;

/// Generates all possible permutations of an `Acronym`.
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::{Mutate, Permutation};
///
/// let a = Acronym {
///     words: vec![
///         Word::new(String::from("Tempting")),
///         Word::new(String::from("Pretty")),
///         Word::new(String::from("And")),
///         Word::new(String::from("Hot")),
///     ],
/// };
/// let ms: Vec<String> = Permutation::mutate(&a).iter()
///     .map(Acronym::to_string)
///     .collect();
///
/// assert!(ms.contains(&String::from("PHAT")));
/// ```
pub struct Permutation;

impl super::Mutate for Permutation {
    fn mutate(acronym: &Acronym) -> HashSet<Acronym> {
        let mut set = HashSet::new();

        let mut permutation = acronym.clone();
        permutation.words.sort();

        set.insert(permutation.clone());

        while next_permutation(&mut permutation.words) {
            set.insert(permutation.clone());
        }

        set
    }
}

/// Mutates the slice to the next lexicographic permutation.
///
/// Returns `true` if successful and `false` if the slice is at the
/// last-ordered permutation.
// https://github.com/rust-lang/rust/blob/d10ff632ceaab93410fd57de6e41adca2ef89688/src/libcore/slice.rs#L472
fn next_permutation<T>(slice: &mut [T]) -> bool where T: Ord {
    // These cases only have 1 permutation each, so we can't do anything.
    if slice.len() < 2 { return false; }

    // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
    let mut i = slice.len() - 1;
    while i > 0 && slice[i-1] >= slice[i] {
        i -= 1;
    }

    // If that is the entire vector, this is the last-ordered permutation.
    if i == 0 {
        return false;
    }

    // Step 2: Find the rightmost element larger than the pivot (i-1)
    let mut j = slice.len() - 1;
    while j >= i && slice[j] <= slice[i-1]  {
        j -= 1;
    }

    // Step 3: Swap that element with the pivot
    slice.swap(j, i-1);

    // Step 4: Reverse the (previously) weakly decreasing part
    slice[i..].reverse();

    true
}
