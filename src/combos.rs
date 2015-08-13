//! Combinations generator.

/// Endlessly repeating iterator that indicates when a new cycle begins.
///
/// Wraps an `Iterator` and yields tuples of the original value and a boolean indicating the
/// beginning of a cycle.
///
/// # Examples
///
/// ```
/// use tbd::combos::FlagCycle;
///
/// let mut i = FlagCycle::new(0..3);
///
/// assert_eq!(Some((0, true)),  i.next());
/// assert_eq!(Some((1, false)), i.next());
/// assert_eq!(Some((2, false)), i.next());
/// assert_eq!(Some((0, true)),  i.next());
/// assert_eq!(Some((1, false)), i.next());
/// ```
pub struct FlagCycle<I> {
    orig: I,
    iter: I,
    zero: bool,
}

impl<I> FlagCycle<I> where I: Iterator + Clone {
    pub fn new(iter: I) -> Self {
        FlagCycle { orig: iter.clone(), iter: iter, zero: true }
    }
}

impl<I> Iterator for FlagCycle<I> where I: Iterator + Clone {
    type Item = (I::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => {
                self.iter = self.orig.clone();
                self.iter.next().map(|v| (v, true))
            },

            Some(v) => if self.zero {
                // Treat the beginning of the first cycle as any other.
                self.zero = false;
                Some((v, true))
            } else {
                Some((v, false))
            },
        }
    }
}

/// Digit iterator that carries to another digit on cycle.
///
/// Wraps an `Iterator` in a `FlagCycle` and forms a linked list of digits to carry to. Stores its
/// current value for later access and yields unit.
///
/// Iteration ends when a `Digit` without a carry `Digit` reaches the end of its wrapped
/// `Iterator`.
///
/// # Examples
///
/// ```
/// use tbd::combos::Digit;
///
/// let a = Digit::new(0..3, None);
/// let b = Digit::new(0..4, Some(a));
///
/// assert_eq!(12, b.count());
/// ```
pub struct Digit<I> where I: Iterator + Clone {
    iter: FlagCycle<I>,
    carry: Option<Box<Digit<I>>>,

    /// Last yielded value of wrapped `Iterator`.
    pub current: Option<I::Item>,
}

impl<I> Digit<I> where I: Iterator + Clone {
    pub fn new(iter: I, carry: Option<Self>) -> Self {
        Digit {
            iter: FlagCycle::new(iter),
            current: None,
            carry: carry.map(Box::new),
        }
    }

    pub fn iter(&self) -> DigitIter<I> {
        DigitIter { next_digit: Some(self) }
    }
}

impl<I> Iterator for Digit<I> where I: Iterator + Clone {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((v, false)) => {
                self.current = Some(v);
                Some(())
            },

            Some((v, true)) => {
                let previous = self.current.take();
                self.current = Some(v);

                if let Some(ref mut carry) = self.carry {
                    carry.next()
                } else {
                    // We are the last digit on a cycle boundary. If we had no previous value, this
                    // is the first iteration and we should return Some. Otherwise, we've exhausted
                    // all combinations and should return None.
                    if previous.is_none() { Some(()) } else { None }
                }
            },

            None => None,
        }
    }
}

/// Linked list iterator for `Digit`.
pub struct DigitIter<'a, I> where I: Iterator + Clone + 'a, I::Item: 'a {
    next_digit: Option<&'a Digit<I>>,
}

impl<'a, I> Iterator for DigitIter<'a, I> where I: Iterator + Clone + 'a, I::Item: 'a {
    type Item = &'a Digit<I>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_digit.map(|digit| {
            // Get a reference to the boxed Digit inside an Option.
            self.next_digit = digit.carry.as_ref().map(|b| &**b);
            digit
        })
    }
}

/// Iterator combinations iterator.
///
/// Wraps a vector of `Iterator` and yields vectors of each combination of their values.
///
/// # Examples
///
/// ```
/// use tbd::combos::Combos;
///
/// let i = Combos::new(vec![0..3, 0..2]);
///
/// assert_eq!(
///     vec![
///         vec![0, 0],
///         vec![1, 0],
///         vec![2, 0],
///         vec![0, 1],
///         vec![1, 1],
///         vec![2, 1],
///     ],
///     i.collect::<Vec<_>>()
/// );
/// ```
pub struct Combos<I> where I: Iterator + Clone {
    head: Digit<I>,
}

impl<I> Combos<I> where I: Iterator + Clone {
    pub fn new(iters: Vec<I>) -> Self {
        // Construct a linked list of Digits from the iterators.
        let head = iters.into_iter()
            .rev()
            .fold(None, |h, i| Some(Digit::new(i, h)))
            .unwrap();

        Combos { head: head }
    }
}

impl<I> Iterator for Combos<I> where I: Iterator + Clone, I::Item: Clone {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.next().map(|_| {
            self.head.iter()
                .map(|d| d.current.clone().unwrap())
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn combos() {
        let i = super::Combos::new(vec![0..2, 0..3, 0..4]);

        assert_eq!(
            vec![
                vec![0, 0, 0],
                vec![1, 0, 0],
                vec![0, 1, 0],
                vec![1, 1, 0],
                vec![0, 2, 0],
                vec![1, 2, 0],
                vec![0, 0, 1],
                vec![1, 0, 1],
                vec![0, 1, 1],
                vec![1, 1, 1],
                vec![0, 2, 1],
                vec![1, 2, 1],
                vec![0, 0, 2],
                vec![1, 0, 2],
                vec![0, 1, 2],
                vec![1, 1, 2],
                vec![0, 2, 2],
                vec![1, 2, 2],
                vec![0, 0, 3],
                vec![1, 0, 3],
                vec![0, 1, 3],
                vec![1, 1, 3],
                vec![0, 2, 3],
                vec![1, 2, 3],
            ],
            i.collect::<Vec<_>>()
        );
    }
}
