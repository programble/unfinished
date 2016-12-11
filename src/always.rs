use std::ops::{FnOnce, FnMut, Fn};

use ignore::{Ignore1, Ignore2, Ignore3, ignore1, ignore2, ignore3};

/// See [`always`](fn.always.html).
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

/// Returns a function which takes one argument and always returns `a`.
///
/// # Examples
///
/// ```
/// # use pointfree::always1;
/// let always_true = always1(true);
/// assert!(always_true('a'));
/// ```
pub fn always1<A>(a: A) -> Ignore1<Always<A>> {
    ignore1(always(a))
}

/// Returns a function which takes two arguments and always returns `a`.
///
/// # Examples
///
/// ```
/// # use pointfree::always2;
/// let always_true = always2(true);
/// assert!(always_true('a', 'b'));
/// ```
pub fn always2<A>(a: A) -> Ignore2<Always<A>> {
    ignore2(always(a))
}

/// Returns a function which takes three arguments and always returns `a`.
///
/// # Examples
///
/// ```
/// # use pointfree::always3;
/// let always_true = always3(true);
/// assert!(always_true('a', 'b', 'c'));
/// ```
pub fn always3<A>(a: A) -> Ignore3<Always<A>> {
    ignore3(always(a))
}
