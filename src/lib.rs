//! Fixed point numeric types.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
)]

extern crate typenum;

pub mod aliases;

pub use typenum::consts;

mod num;
pub use num::Num;

use std::cmp::Ordering;
use std::fmt::{Debug, Error as FmtError, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use typenum::{NonZero, Unsigned, Integer, Z0};

/// Marker trait alias for base.
pub trait Base: NonZero + Unsigned { }
impl<T: NonZero + Unsigned> Base for T { }

/// Marker trait alias for exponent.
pub trait Exponent: Integer { }
impl<T: Integer> Exponent for T { }

/// A fixed-point number stored in `N`, scaled by `B ^ E`.
pub struct Fix<N: Num, B: Base, E: Exponent> {
    num: N,
    marker: PhantomData<(B, E)>,
}

impl<N: Num, B: Base, E: Exponent> Debug for Fix<N, B, E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{:?}x{}^{}", self.num, B::to_u64(), E::to_i64())
    }
}

// TODO: Implement Display.

impl<N: Num, B: Base, E: Exponent> Clone for Fix<N, B, E> {
    fn clone(&self) -> Self {
        Fix {
            num: self.num,
            marker: PhantomData,
        }
    }
}

impl<N: Num, B: Base, E: Exponent> Copy for Fix<N, B, E> { }

impl<N: Num, B: Base, E: Exponent> PartialEq for Fix<N, B, E> {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl<N: Num, B: Base, E: Exponent> Eq for Fix<N, B, E> { }

// TODO: Implement PartialEq/Eq against other exponents.

impl<N: Num, B: Base, E: Exponent> PartialOrd for Fix<N, B, E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.num.partial_cmp(&other.num)
    }
}

impl<N: Num, B: Base, E: Exponent> Ord for Fix<N, B, E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num.cmp(&other.num)
    }
}

// TODO: Implement PartialOrd/Ord against other exponents.

impl<N: Num, B: Base, E: Exponent> Hash for Fix<N, B, E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.num.hash(state);
    }
}

/// Returns zero.
impl<N: Num, B: Base, E: Exponent> Default for Fix<N, B, E> {
    fn default() -> Self {
        Fix {
            num: N::zero(),
            marker: PhantomData,
        }
    }
}

impl<N: Num, B: Base> From<N> for Fix<N, B, Z0> {
    fn from(num: N) -> Self {
        Fix {
            num: num,
            marker: PhantomData,
        }
    }
}
