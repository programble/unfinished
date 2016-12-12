use std::ops::{FnOnce, FnMut, Fn};

/// See [`flip2`](fn.flip2.html).
pub struct Flip2<F> {
    f: F,
}

impl<F, A, B, C> FnOnce<(B, A)> for Flip2<F>
where F: FnOnce(A, B) -> C {
    type Output = C;
    extern "rust-call" fn call_once(self, args: (B, A)) -> C {
        (self.f)(args.1, args.0)
    }
}

impl<F, A, B, C> FnMut<(B, A)> for Flip2<F>
where F: FnMut(A, B) -> C {
    extern "rust-call" fn call_mut(&mut self, args: (B, A)) -> C {
        (self.f)(args.1, args.0)
    }
}

impl<F, A, B, C> Fn<(B, A)> for Flip2<F>
where F: Fn(A, B) -> C {
    extern "rust-call" fn call(&self, args: (B, A)) -> C {
        (self.f)(args.1, args.0)
    }
}

/// Wraps a binary function in a function which flips its arguments.
///
/// # Examples
///
/// ```
/// # use pointfree::flip2;
/// let is_digit = flip2(char::is_digit);
/// assert!(is_digit(10, '9'));
/// ```
pub fn flip2<F>(f: F) -> Flip2<F> {
    Flip2 { f: f }
}
