//! Bit iteration.
//!
//! # Examples
//!
//! ```
//! use tbd::bits::Bits;
//!
//! let x = 0xABu8;
//! let bits: Vec<bool> = x.bits().collect();
//!
//! assert_eq!(vec![true, true, false, true, false, true, false, true], bits);
//! ```

use std::ops::{Shl, BitAnd};

/// Abstracts over a pile of bits (unsigned integers).
///
/// Based on `bit-vec` `BitBlock`.
pub trait Bits: Copy + Shl<Self, Output=Self> + BitAnd<Self, Output=Self> + Eq {
    /// Returns `0`.
    fn zero() -> Self;

    /// Returns `1`.
    fn one() -> Self;

    /// Returns an `Iterator` over the bits as `bool`.
    fn bits(self) -> Iter<Self> {
        Iter {
            value: self,
            mask: Self::one(),
        }
    }
}

/// Iterator over bits as `bool` from least-significant to most-significant.
pub struct Iter<T: Bits> {
    value: T,
    mask: T,
}

impl<T: Bits> Iterator for Iter<T> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.mask == T::zero() { return None; }
        let masked = self.value & self.mask;
        self.mask = self.mask << T::one();
        Some(masked != T::zero())
    }
}

macro_rules! impl_bits {
    ($t:ty) => {
        impl Bits for $t {
            fn zero() -> Self { 0 }
            fn one() -> Self { 1 }
        }
    }
}

impl_bits!(u8);
impl_bits!(u16);
impl_bits!(u32);
impl_bits!(u64);
impl_bits!(usize);
