use std::ops::Range;
use std::cmp;
use acronym::{Word, Acronym};
use bits::Bits;

/// Generates permutations of expanded and contracted "word is" / "word's" forms.
///
/// Starts by contracting all occurrences of "is", then generates each possible combination of
/// expansion/contraction using binary counting.
///
/// ```
/// use tbd::acronym::{Word, Acronym};
/// use tbd::mutate::IsContraction;
///
/// let a = Acronym {
///     words: vec![
///         Word(String::from("gnu"), 1),
///         Word(String::from("is"), 1),
///         Word(String::from("not"), 1),
///         Word(String::from("unix"), 1),
///     ],
/// };
/// let mut m = IsContraction::new(&a);
///
/// assert!(m.any(|a| a.expansion() == "Gnu's Not Unix"));
/// ```
pub struct IsContraction {
    acronym: Acronym,
    range: Range<u64>,
}

fn contract_all(acronym: &Acronym) -> (Acronym, u32) {
    let mut count = 0u32;
    let mut contracted = Acronym { words: Vec::new() };

    let mut iter = acronym.words.iter().peekable();

    while let Some(current) = iter.next() {
        if current.string().ends_with("'s") { count += 1; }

        let current_is = current.string().to_lowercase() == "is";
        let next_is = iter.peek()
            .map(|w| w.string().to_lowercase() == "is")
            .unwrap_or(false);

        if next_is {
            let contraction = String::from(current.string()) + "'s";
            contracted.words.push(Word(contraction, 1));
            count += 1;
        } else if !current_is {
            contracted.words.push(current.clone());
        }
    }

    (contracted, count)
}

impl IsContraction {
    pub fn new(acronym: &Acronym) -> IsContraction {
        let (contracted, count) = contract_all(acronym);
        let count = cmp::min(count, 63);
        let combos = 2u64.pow(count);

        IsContraction {
            acronym: contracted,
            range: 0..combos,
        }
    }
}

impl Iterator for IsContraction {
    type Item = Acronym;

    fn next(&mut self) -> Option<Acronym> {
        self.range.next().map(|combo| {
            let mut next = Acronym { words: Vec::new() };
            let mut bits = combo.bits();

            for word in self.acronym.words.iter() {
                if word.string().ends_with("'s") && bits.next().unwrap() {
                    let apostrophe = word.len() - 2;
                    let bare = String::from(&word.string()[..apostrophe]);

                    next.words.push(Word(bare, 1));
                    next.words.push(Word(String::from("is"), 1));

                } else {
                    next.words.push(word.clone());
                }
            }

            next
        })
    }
}

#[cfg(test)]
mod tests {
    use acronym::{Word, Acronym};

    #[test]
    fn contract_all() {
        let expanded = Acronym {
            words: vec![
                Word(String::from("test"), 1),
                Word(String::from("is"), 1),
                Word(String::from("what"), 1),
                Word(String::from("test"), 1),
                Word(String::from("is"), 1),
                Word(String::from("not"), 1),
                Word(String::from("test's"), 1),
            ],
        };
        let (contracted, count) = super::contract_all(&expanded);
        assert_eq!(3, count);
        assert_eq!("Test's What Test's Not Test's", contracted.expansion());
    }

    #[test]
    fn is_contraction() {
        let a = Acronym {
            words: vec![
                Word(String::from("test"), 1),
                Word(String::from("is"), 1),
                Word(String::from("what"), 1),
                Word(String::from("test"), 1),
                Word(String::from("is"), 1),
                Word(String::from("not"), 1),
                Word(String::from("test's"), 1),
            ],
        };
        let r = super::IsContraction::new(&a)
            .map(|a| a.expansion())
            .collect::<Vec<_>>();

        assert_eq!(8, r.len());
        assert!(r.contains(&String::from("Test's What Test's Not Test's")));
        assert!(r.contains(&String::from("Test's What Test's Not Test Is")));
        assert!(r.contains(&String::from("Test's What Test Is Not Test's")));
        assert!(r.contains(&String::from("Test's What Test Is Not Test Is")));
        assert!(r.contains(&String::from("Test Is What Test's Not Test's")));
        assert!(r.contains(&String::from("Test Is What Test's Not Test Is")));
        assert!(r.contains(&String::from("Test Is What Test Is Not Test's")));
        assert!(r.contains(&String::from("Test Is What Test Is Not Test Is")));
    }
}
