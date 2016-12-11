use std::ops::{FnOnce, FnMut, Fn};

/// See [`partial1`](fn.partial1.html).
pub struct Partial1<F, A> {
    f: F,
    a: A,
}

/// See [`partial2`](fn.partial2.html).
pub struct Partial2<F, A> {
    f: F,
    a: A,
}

impl<F, A, B, C> FnOnce<(B,)> for Partial1<F, A>
where F: FnOnce(A, B) -> C {
    type Output = C;
    extern "rust-call" fn call_once(self, args: (B,)) -> C {
        (self.f)(self.a, args.0)
    }
}

impl<'a, F, A, B, C> FnMut<(B,)> for Partial1<F, &'a mut A>
where F: FnMut(&mut A, B) -> C {
    extern "rust-call" fn call_mut(&mut self, args: (B,)) -> C {
        (self.f)(self.a, args.0)
    }
}

impl<'a, F, A, B, C> FnMut<(B,)> for Partial1<F, &'a A>
where F: FnMut(&A, B) -> C {
    extern "rust-call" fn call_mut(&mut self, args: (B,)) -> C {
        (self.f)(self.a, args.0)
    }
}

impl<'a, F, A, B, C> Fn<(B,)> for Partial1<F, &'a A>
where F: Fn(&A, B) -> C {
    extern "rust-call" fn call(&self, args: (B,)) -> C {
        (self.f)(self.a, args.0)
    }
}

impl<F, A, B, C, D> FnOnce<(B, C)> for Partial2<F, A>
where F: FnOnce(A, B, C) -> D {
    type Output = D;
    extern "rust-call" fn call_once(self, args: (B, C)) -> D {
        (self.f)(self.a, args.0, args.1)
    }
}

impl<'a, F, A, B, C, D> FnMut<(B, C)> for Partial2<F, &'a mut A>
where F: FnMut(&mut A, B, C) -> D {
    extern "rust-call" fn call_mut(&mut self, args: (B, C)) -> D {
        (self.f)(self.a, args.0, args.1)
    }
}

impl<'a, F, A, B, C, D> FnMut<(B, C)> for Partial2<F, &'a A>
where F: FnMut(&A, B, C) -> D {
    extern "rust-call" fn call_mut(&mut self, args: (B, C)) -> D {
        (self.f)(self.a, args.0, args.1)
    }
}

impl<'a, F, A, B, C, D> Fn<(B, C)> for Partial2<F, &'a A>
where F: Fn(&A, B, C) -> D {
    extern "rust-call" fn call(&self, args: (B, C)) -> D {
        (self.f)(self.a, args.0, args.1)
    }
}

/// Returns...
///
/// # Examples
///
/// ```
/// # use pointfree::partial1;
/// let mut vec = Vec::new();
/// {
///     let mut push = partial1(Vec::push, &mut vec);
///     push(1);
///     push(2);
///     push(3);
/// }
/// assert_eq!([1, 2, 3], vec.as_slice());
/// ```
///
/// ```
/// # use pointfree::partial1;
/// let vec = vec![1, 2, 3];
/// let get = partial1(<[_]>::get, vec.as_slice());
/// assert_eq!(Some(&2), get(1));
/// ```
pub fn partial1<F, A>(f: F, a: A) -> Partial1<F, A> {
    Partial1 { f: f, a: a }
}

/// Returns...
pub fn partial2<F, A>(f: F, a: A) -> Partial2<F, A> {
    Partial2 { f: f, a: a }
}
