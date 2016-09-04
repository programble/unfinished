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

/// Number storage types.
pub trait Num { }

impl Num for u8 { }
impl Num for u16 { }
impl Num for u32 { }
impl Num for u64 { }
impl Num for usize { }

impl Num for i8 { }
impl Num for i16 { }
impl Num for i32 { }
impl Num for i64 { }
impl Num for isize { }

/// A fixed-point number stored in `N`, scaled by `Base` to `Exponent`.
pub struct Fix<N: Num, Base: NonZero + Unsigned, Exponent: Integer> {
    num: N,
    base: PhantomData<Base>,
    exponent: PhantomData<Exponent>,
}
