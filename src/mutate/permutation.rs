use acronym::Acronym;

/// Generates all possible permutations of an `Acronym`.
///
/// # Examples
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::Permutation;
///
/// let a = Acronym {
///     words: vec![
///         Word(String::from("tempting"), 1),
///         Word(String::from("pretty"), 1),
///         Word(String::from("and"), 1),
///         Word(String::from("hot"), 1),
///     ],
/// };
/// let mut m = Permutation::new(&a);
///
/// assert!(m.any(|a| a.to_string() == "PHAT"));
/// ```
pub struct Permutation {
    next: Acronym,
    done: bool,
}

impl Permutation {
    pub fn new(acronym: &Acronym) -> Permutation {
        let mut first = acronym.clone();
        first.words.sort();

        Permutation {
            next: first,
            done: false,
        }
    }
}

impl Iterator for Permutation {
    type Item = Acronym;

    fn next(&mut self) -> Option<Acronym> {
        if self.done { return None; }

        let current = self.next.clone();
        self.done = !next_permutation(&mut self.next.words);

        Some(current)
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
