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

use std::fmt::{Debug, Error as FmtError, Formatter};
use std::marker::PhantomData;

use typenum::{NonZero, Unsigned, Integer};

pub use typenum::consts;

pub mod aliases;

mod num;
pub use num::Num;

/// Marker trait alias for base.
pub trait Base: NonZero + Unsigned { }
impl<T: NonZero + Unsigned> Base for T { }

/// Marker trait alias for exponent.
pub trait Exponent: Integer { }
impl<T: Integer> Exponent for T { }

/// A fixed-point number stored in `N`, scaled by `B ^ E`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fix<N: Num, B: Base, E: Exponent> {
    num: N,
    marker: PhantomData<(B, E)>,
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

impl<N: Num, B: Base, E: Exponent> Debug for Fix<N, B, E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{:?}x{}^{}", self.num, B::to_u64(), E::to_i64())
    }
}
