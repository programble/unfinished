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

mod int;
pub use int::Int;

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
pub struct Fix<I: Int, B: Base, E: Exponent> {
    int: I,
    marker: PhantomData<(B, E)>,
}

impl<I: Int, B: Base, E: Exponent> Debug for Fix<I, B, E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{:?}x{}^{}", self.int, B::to_u64(), E::to_i64())
    }
}

// TODO: Implement Display.

impl<I: Int, B: Base, E: Exponent> Clone for Fix<I, B, E> {
    fn clone(&self) -> Self {
        Fix {
            int: self.int,
            marker: PhantomData,
        }
    }
}

impl<I: Int, B: Base, E: Exponent> Copy for Fix<I, B, E> { }

impl<I: Int, B: Base, E: Exponent> PartialEq for Fix<I, B, E> {
    fn eq(&self, other: &Self) -> bool {
        self.int == other.int
    }
}

impl<I: Int, B: Base, E: Exponent> Eq for Fix<I, B, E> { }

// TODO: Implement PartialEq/Eq against other exponents.

impl<I: Int, B: Base, E: Exponent> PartialOrd for Fix<I, B, E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.int.partial_cmp(&other.int)
    }
}

impl<I: Int, B: Base, E: Exponent> Ord for Fix<I, B, E> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.int.cmp(&other.int)
    }
}

// TODO: Implement PartialOrd/Ord against other exponents.

impl<I: Int, B: Base, E: Exponent> Hash for Fix<I, B, E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.int.hash(state);
    }
}

/// Returns zero.
impl<I: Int, B: Base, E: Exponent> Default for Fix<I, B, E> {
    fn default() -> Self {
        Fix {
            int: I::zero(),
            marker: PhantomData,
        }
    }
}

impl<I: Int, B: Base, E: Exponent> Fix<I, B, E> {
    /// Creates a fixed-point number from an integer.
    pub fn from_int(int: I) -> Self {
        Fix {
            int: int,
            marker: PhantomData,
        }
    }

    /// Returns `self` as an integer.
    pub fn into_int(self) -> I {
        self.int
    }
}
