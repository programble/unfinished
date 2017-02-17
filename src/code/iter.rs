use code::*;

use core::slice;
use core::iter::{self, Cloned, Once};

macro_rules! impl_into_iterator_once_cast {
    ($ty:ty) => {
        impl<'a> IntoIterator for &'a $ty {
            type Item = u8;
            type IntoIter = Once<u8>;

            #[inline]
            fn into_iter(self) -> Once<u8> {
                iter::once(*self as u8)
            }
        }
    }
}

macro_rules! impl_into_iterator_once_tuple {
    ($ty:ty) => {
        impl<'a> IntoIterator for &'a $ty {
            type Item = u8;
            type IntoIter = Once<u8>;

            #[inline]
            fn into_iter(self) -> Once<u8> {
                iter::once(self.0)
            }
        }
    }
}

macro_rules! impl_into_iterator_cloned_slice {
    ($ty:ident { $($var:ident),+ }) => {
        impl<'a> IntoIterator for &'a $ty {
            type Item = u8;
            type IntoIter = Cloned<slice::Iter<'a, u8>>;

            #[inline]
            fn into_iter(self) -> Cloned<slice::Iter<'a, u8>> {
                match *self {
                    $($ty::$var(ref a) => a.into_iter().cloned()),+
                }
            }
        }
    }
}

impl_into_iterator_once_cast!(Prefix1);
impl_into_iterator_once_cast!(Prefix2);
impl_into_iterator_once_cast!(Prefix3);
impl_into_iterator_once_cast!(Prefix4);

impl_into_iterator_once_tuple!(Rex);
impl_into_iterator_once_tuple!(Modrm);
impl_into_iterator_once_tuple!(Sib);

impl_into_iterator_cloned_slice!(Opcode { B1, B2, B3 });
impl_into_iterator_cloned_slice!(Disp { B1, B4 });
impl_into_iterator_cloned_slice!(Imm { B1, B2, B4});
