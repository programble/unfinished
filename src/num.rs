use std::fmt::Debug;
use std::hash::Hash;

/// Integer types.
pub trait Num: Debug + Copy + Eq + Ord + Hash {
    /// Returns zero.
    fn zero() -> Self;
}

macro_rules! impl_num {
    ($n:ty) => (
        impl Num for $n {
            fn zero() -> Self { 0 }
        }
    )
}

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);
impl_num!(usize);

impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);
impl_num!(isize);
