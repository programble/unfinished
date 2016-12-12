use std::ops::{FnOnce, FnMut, Fn};

/// See [`ignore1`](fn.ignore1.html).
pub struct Ignore1<F> {
    f: F,
}

/// See [`ignore2`](fn.ignore2.html).
pub struct Ignore2<F> {
    f: F,
}

/// See [`ignore3`](fn.ignore3.html).
pub struct Ignore3<F> {
    f: F,
}

impl<F, A, B> FnOnce<(A,)> for Ignore1<F> where F: FnOnce() -> B {
    type Output = B;
    extern "rust-call" fn call_once(self, _: (A,)) -> B {
        (self.f)()
    }
}

impl<F, A, B> FnMut<(A,)> for Ignore1<F> where F: FnMut() -> B {
    extern "rust-call" fn call_mut(&mut self, _: (A,)) -> B {
        (self.f)()
    }
}

impl<F, A, B> Fn<(A,)> for Ignore1<F> where F: Fn() -> B {
    extern "rust-call" fn call(&self, _: (A,)) -> B {
        (self.f)()
    }
}

impl<F, A, B, C> FnOnce<(A, B)> for Ignore2<F> where F: FnOnce() -> C {
    type Output = C;
    extern "rust-call" fn call_once(self, _: (A, B)) -> C {
        (self.f)()
    }
}

impl<F, A, B, C> FnMut<(A, B)> for Ignore2<F> where F: FnMut() -> C {
    extern "rust-call" fn call_mut(&mut self, _: (A, B)) -> C {
        (self.f)()
    }
}

impl<F, A, B, C> Fn<(A, B)> for Ignore2<F> where F: Fn() -> C {
    extern "rust-call" fn call(&self, _: (A, B)) -> C {
        (self.f)()
    }
}

impl<F, A, B, C, D> FnOnce<(A, B, C)> for Ignore3<F> where F: FnOnce() -> D {
    type Output = D;
    extern "rust-call" fn call_once(self, _: (A, B, C)) -> D {
        (self.f)()
    }
}

impl<F, A, B, C, D> FnMut<(A, B, C)> for Ignore3<F> where F: FnMut() -> D {
    extern "rust-call" fn call_mut(&mut self, _: (A, B, C)) -> D {
        (self.f)()
    }
}

impl<F, A, B, C, D> Fn<(A, B, C)> for Ignore3<F> where F: Fn() -> D {
    extern "rust-call" fn call(&self, _: (A, B, C)) -> D {
        (self.f)()
    }
}

/// Wraps a nullary function in a unary function which ignores its argument.
///
/// # Examples
///
/// ```
/// # use pointfree::{ignore1, always};
/// let always_true = ignore1(always(true));
/// assert!(always_true('a'));
/// ```
pub fn ignore1<F>(f: F) -> Ignore1<F> {
    Ignore1 { f: f }
}

/// Wraps a nullary function in a binary function which ignores its arguments.
///
/// # Examples
///
/// ```
/// # use pointfree::{ignore2, always};
/// let always_true = ignore2(always(true));
/// assert!(always_true('a', 'b'));
/// ```
pub fn ignore2<F>(f: F) -> Ignore2<F> {
    Ignore2 { f: f }
}

/// Wraps a nullary function in a ternary function which ignores its arguments.
///
/// # Examples
///
/// ```
/// # use pointfree::{ignore3, always};
/// let always_true = ignore3(always(true));
/// assert!(always_true('a', 'b', 'c'));
/// ```
pub fn ignore3<F>(f: F) -> Ignore3<F> {
    Ignore3 { f: f }
}
