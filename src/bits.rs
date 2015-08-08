//! Bit access.

use std::mem;

/// Access bits of `self` as `bool`.
///
/// # Examples
///
/// ```
/// use tbd::bits::Bits;
///
/// let x = 0xF0u8;
///
/// assert_eq!(false, x.get_bit(3).unwrap());
/// assert_eq!(true, x.get_bit(4).unwrap());
/// ```
pub trait Bits {
    /// Gets the value of bit `n`, or `None` if bit does not exist.
    fn get_bit(&self, n: usize) -> Option<bool>;
}

macro_rules! impl_bits {
    ($t:ty) => {
        impl Bits for $t {
            fn get_bit(&self, n: usize) -> Option<bool> {
                if n > mem::size_of::<$t>() * 8 - 1 {
                    None
                } else {
                    Some(self >> n & 1 == 1)
                }
            }
        }
    }
}

impl_bits!(u8);
impl_bits!(u16);
impl_bits!(u32);
impl_bits!(u64);
impl_bits!(usize);
