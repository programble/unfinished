use std::ops::{FnOnce, FnMut, Fn};

/// Takes one argument and calls `F` with none.
///
/// - `FnOnce(A) -> B where F: FnOnce() -> B`
/// - `FnMut(A) -> B where F: FnMut() -> B`
/// - `Fn(A) -> B where F: Fn() -> B`
#[derive(Debug, Clone, Copy)]
pub struct Ignore1<F> {
    f: F,
}

/// Takes two arguments and calls `F` with none.
///
/// - `FnOnce(A, B) -> C where F: FnOnce() -> C`
/// - `FnMut(A, B) -> C where F: FnMut() -> C`
/// - `Fn(A, B) -> C where F: Fn() -> C`
#[derive(Debug, Clone, Copy)]
pub struct Ignore2<F> {
    f: F,
}

/// Takes three arguments and calls `F` with none.
///
/// - `FnOnce(A, B, C) -> D where F: FnOnce() -> D`
/// - `FnMut(A, B, C) -> D where F: FnMut() -> D`
/// - `Fn(A, B, C) -> D where F: Fn() -> D`
#[derive(Debug, Clone, Copy)]
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

/// Returns a function which takes one argument and calls `f` with none.
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

/// Returns a function which takes two arguments and calls `f` with none.
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

/// Returns a function which takes three arguments and calls `f` with none.
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
