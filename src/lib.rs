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

use std::marker::PhantomData;

use typenum::{NonZero, Unsigned, Integer};

pub use typenum::consts;

pub mod aliases;

mod num;
pub use num::Num;

/// A fixed-point number stored in `N`, scaled by `Base ^ Exponent`.
pub struct Fix<N: Num, Base: NonZero + Unsigned, Exponent: Integer> {
    num: N,
    base: PhantomData<Base>,
    exponent: PhantomData<Exponent>,
}
