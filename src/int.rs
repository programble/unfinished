use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;

/// Integer types.
pub trait Int: Debug + Copy + Eq + Ord + Hash + Add<Output = Self> {
    /// Returns zero.
    fn zero() -> Self;
}

macro_rules! impl_int {
    ($n:ty) => (
        impl Int for $n {
            fn zero() -> Self { 0 }
        }
    )
}

impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(usize);

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(isize);
