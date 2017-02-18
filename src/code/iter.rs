use code::{Prefix1, Prefix2, Prefix3, Prefix4, Rex, Opcode, Modrm, Sib, Disp, Imm, Instruction};

use core::slice;
use core::iter::{self, Cloned, Fuse, Once};

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

pub struct Iter<'a> {
    prefix1: Option<Fuse<<&'a Prefix1 as IntoIterator>::IntoIter>>,
    prefix2: Option<Fuse<<&'a Prefix2 as IntoIterator>::IntoIter>>,
    prefix3: Option<Fuse<<&'a Prefix3 as IntoIterator>::IntoIter>>,
    prefix4: Option<Fuse<<&'a Prefix4 as IntoIterator>::IntoIter>>,
    rex: Option<Fuse<<&'a Rex as IntoIterator>::IntoIter>>,
    opcode: Fuse<<&'a Opcode as IntoIterator>::IntoIter>,
    modrm: Option<Fuse<<&'a Modrm as IntoIterator>::IntoIter>>,
    sib: Option<Fuse<<&'a Sib as IntoIterator>::IntoIter>>,
    disp: Option<Fuse<<&'a Disp as IntoIterator>::IntoIter>>,
    imm: Option<Fuse<<&'a Imm as IntoIterator>::IntoIter>>,
}

impl<'a> IntoIterator for &'a Instruction {
    type Item = u8;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        Iter {
            prefix1: self.prefix1.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse),
            prefix2: self.prefix2.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse),
            prefix3: self.prefix3.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse),
            prefix4: self.prefix4.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse),
            rex: self.rex.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse),
            opcode: self.opcode.into_iter().fuse(),
            modrm: self.modrm.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse),
            sib: self.sib.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse),
            disp: self.disp.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse),
            imm: self.imm.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        if let Some(ref mut prefix1) = self.prefix1 { for x in prefix1 { return Some(x); } }
        if let Some(ref mut prefix2) = self.prefix2 { for x in prefix2 { return Some(x); } }
        if let Some(ref mut prefix3) = self.prefix3 { for x in prefix3 { return Some(x); } }
        if let Some(ref mut prefix4) = self.prefix4 { for x in prefix4 { return Some(x); } }
        if let Some(ref mut rex) = self.rex { for x in rex { return Some(x); } }
        for x in &mut self.opcode { return Some(x); }
        if let Some(ref mut modrm) = self.modrm { for x in modrm { return Some(x); } }
        if let Some(ref mut sib) = self.sib { for x in sib { return Some(x); } }
        if let Some(ref mut disp) = self.disp { for x in disp { return Some(x); } }
        if let Some(ref mut imm) = self.imm { for x in imm { return Some(x); } }
        None
    }

    // TODO: Implement size_hint.
}

impl Instruction {
    #[inline]
    pub fn iter(&self) -> Iter {
        self.into_iter()
    }
}
