use std::ops::{FnOnce, FnMut, Fn};

/// See [`compose1`](fn.compose1.html).
pub struct Compose1<F, G> {
    f: F,
    g: G,
}

/// See [`compose2`](fn.compose2.html).
pub struct Compose2<F, G> {
    f: F,
    g: G,
}

/// See [`compose3`](fn.compose3.html).
pub struct Compose3<F, G> {
    f: F,
    g: G,
}

impl<F, G, A, B, C> FnOnce<(A,)> for Compose1<F, G>
where G: FnOnce(A) -> B, F: FnOnce(B) -> C {
    type Output = C;
    extern "rust-call" fn call_once(self, args: (A,)) -> C {
        (self.f)((self.g)(args.0))
    }
}

impl<F, G, A, B, C> FnMut<(A,)> for Compose1<F, G>
where G: FnMut(A) -> B, F: FnMut(B) -> C {
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> C {
        (self.f)((self.g)(args.0))
    }
}

impl<F, G, A, B, C> Fn<(A,)> for Compose1<F, G>
where G: Fn(A) -> B, F: Fn(B) -> C {
    extern "rust-call" fn call(&self, args: (A,)) -> C {
        (self.f)((self.g)(args.0))
    }
}

impl<F, G, A, B, C, D> FnOnce<(A, B)> for Compose2<F, G>
where G: FnOnce(A, B) -> C, F: FnOnce(C) -> D {
    type Output = D;
    extern "rust-call" fn call_once(self, args: (A, B)) -> D {
        (self.f)((self.g)(args.0, args.1))
    }
}

impl<F, G, A, B, C, D> FnMut<(A, B)> for Compose2<F, G>
where G: FnMut(A, B) -> C, F: FnMut(C) -> D {
    extern "rust-call" fn call_mut(&mut self, args: (A, B)) -> D {
        (self.f)((self.g)(args.0, args.1))
    }
}

impl<F, G, A, B, C, D> Fn<(A, B)> for Compose2<F, G>
where G: Fn(A, B) -> C, F: Fn(C) -> D {
    extern "rust-call" fn call(&self, args: (A, B)) -> D {
        (self.f)((self.g)(args.0, args.1))
    }
}

impl<F, G, A, B, C, D, E> FnOnce<(A, B, C)> for Compose3<F, G>
where G: FnOnce(A, B, C) -> D, F: FnOnce(D) -> E {
    type Output = E;
    extern "rust-call" fn call_once(self, args: (A, B, C)) -> E {
        (self.f)((self.g)(args.0, args.1, args.2))
    }
}

impl<F, G, A, B, C, D, E> FnMut<(A, B, C)> for Compose3<F, G>
where G: FnMut(A, B, C) -> D, F: FnMut(D) -> E {
    extern "rust-call" fn call_mut(&mut self, args: (A, B, C)) -> E {
        (self.f)((self.g)(args.0, args.1, args.2))
    }
}

impl<F, G, A, B, C, D, E> Fn<(A, B, C)> for Compose3<F, G>
where G: Fn(A, B, C) -> D, F: Fn(D) -> E {
    extern "rust-call" fn call(&self, args: (A, B, C)) -> E {
        (self.f)((self.g)(args.0, args.1, args.2))
    }
}

/// Returns a function which calls `f(g(a))`.
///
/// # Examples
///
/// ```
/// # use pointfree::compose1;
/// let len_is_power_of_two = compose1(usize::is_power_of_two, Vec::len);
/// assert!(len_is_power_of_two(&vec!['a', 'b', 'c', 'd']));
/// ```
pub fn compose1<F, G>(f: F, g: G) -> Compose1<F, G> {
    Compose1 { f: f, g: g }
}

/// Returns a function which calls `f(g(a, b))`.
///
/// # Examples
///
/// ```
/// # use pointfree::compose2;
/// let remove_is_power_of_two = compose2(u32::is_power_of_two, Vec::remove);
/// assert!(remove_is_power_of_two(&mut vec![1, 2, 3, 4], 3));
/// ```
pub fn compose2<F, G>(f: F, g: G) -> Compose2<F, G> {
    Compose2 { f: f, g: g }
}

/// Returns a function which calls `f(g(a, b, c))`.
///
/// # Examples
///
/// ```
/// # use pointfree::compose3;
/// let splitn_last = compose3(Iterator::last, str::splitn);
/// assert_eq!(Some("a little lambda"), splitn_last("Mary had a little lambda", 3, ' '));
/// ```
pub fn compose3<F, G>(f: F, g: G) -> Compose3<F, G> {
    Compose3 { f: f, g: g }
}
