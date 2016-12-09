use std::ops::{FnOnce, FnMut, Fn};

/// Always returns the same value.
///
/// - `FnOnce() -> A`
/// - `FnMut() -> A where A: Clone`
/// - `Fn() -> A where A: Clone`
#[derive(Debug, Clone, Copy)]
pub struct Always<A> {
    a: A,
}

impl<A> FnOnce<()> for Always<A> {
    type Output = A;
    extern "rust-call" fn call_once(self, _: ()) -> A {
        self.a
    }
}

impl<A> FnMut<()> for Always<A> where A: Clone {
    extern "rust-call" fn call_mut(&mut self, _: ()) -> A {
        self.a.clone()
    }
}

impl<A> Fn<()> for Always<A> where A: Clone {
    extern "rust-call" fn call(&self, _: ()) -> A {
        self.a.clone()
    }
}

/// Returns a function which always returns `a`.
///
/// # Examples
///
/// ```
/// # use pointfree::always;
/// let always_true = always(true);
/// assert!(always_true());
/// ```
pub fn always<A>(a: A) -> Always<A> {
    Always { a: a }
}
