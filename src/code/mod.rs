/// Instruction encoding.
pub mod encode;

/// Register encoding.
pub mod reg;

use core::fmt::{Display, Formatter, Error};
use core::iter::{self, Cloned, Fuse, Once};
use core::slice;

/// Group 1 instruction prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Prefix1 {
    Lock = 0xf0,
    Repne = 0xf2,
    Rep = 0xf3,
}

/// Group 2 instruction prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Prefix2 {
    Cs = 0x2e,
    Ss = 0x36,
    Ds = 0x3e,
    Es = 0x26,
    Fs = 0x64,
    Gs = 0x65,
}

/// Group 3 instruction prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Prefix3 {
    Osz = 0x66,
}

/// Group 4 instruction prefixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Prefix4 {
    Asz = 0x67,
}

/// REX prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rex(pub u8); // Wants to be NonZero.

/// Primary opcode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    B1([u8; 1]),
    B2([u8; 2]),
    B3([u8; 3]),
}

/// ModR/M byte.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modrm(pub u8);

/// SIB byte.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sib(pub u8);

/// Displacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Disp {
    B1([u8; 1]),
    B4([u8; 4]),
    B8([u8; 8]),
}

/// Immediate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Imm {
    B1([u8; 1]),
    B2([u8; 2]),
    B4([u8; 4]),
    B8([u8; 8]),
}

/// Instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Inst {
    pub prefix1: Option<Prefix1>,
    pub prefix2: Option<Prefix2>,
    pub prefix3: Option<Prefix3>,
    pub prefix4: Option<Prefix4>,
    pub rex: Option<Rex>,
    pub opcode: Opcode,
    pub modrm: Option<Modrm>,
    pub sib: Option<Sib>,
    pub disp: Option<Disp>,
    pub imm: Option<Imm>,
}

impl Default for Rex {
    #[inline]
    fn default() -> Self {
        Rex(0x40)
    }
}

impl Rex {
    pub fn set_w(self) -> Self { Rex(self.0 | 0b1000) }
    pub fn set_r(self) -> Self { Rex(self.0 | 0b0100) }
    pub fn set_x(self) -> Self { Rex(self.0 | 0b0010) }
    pub fn set_b(self) -> Self { Rex(self.0 | 0b0001) }
}

impl Opcode {
    pub fn plus(self, r: u8) -> Self {
        match self {
            Opcode::B1(c) => Opcode::B1([c[0] + r]),
            Opcode::B2(c) => Opcode::B2([c[0], c[1] + r]),
            Opcode::B3(c) => Opcode::B3([c[0], c[1], c[2] + r]),
        }
    }
}

impl Modrm {
    pub fn set_mod(self, mod_: u8) -> Self {
        debug_assert_eq!(0, mod_ & !0b11);
        Modrm(self.0 & !0b11_000_000 | mod_ << 6)
    }

    pub fn set_reg(self, reg: u8) -> Self {
        debug_assert_eq!(0, reg & !0b111);
        Modrm(self.0 & !0b00_111_000 | reg << 3)
    }

    pub fn set_rm(self, rm: u8) -> Self {
        debug_assert_eq!(0, rm & !0b111);
        Modrm(self.0 & !0b00_000_000 | rm)
    }
}

impl Sib {
    pub fn set_scale(self, scale: u8) -> Self {
        debug_assert_eq!(0, scale & !0b11);
        Sib(self.0 & !0b11_000_000 | scale << 6)
    }

    pub fn set_index(self, index: u8) -> Self {
        debug_assert_eq!(0, index & !0b111);
        Sib(self.0 & !0b00_111_000 | index << 3)
    }

    pub fn set_base(self, base: u8) -> Self {
        debug_assert_eq!(0, base & !0b111);
        Sib(self.0 & !0b00_000_000 | base)
    }
}

impl From<i8> for Disp {
    #[inline]
    fn from(i: i8) -> Self {
        Disp::B1([i as u8])
    }
}

impl From<i32> for Disp {
    #[inline]
    fn from(i: i32) -> Self {
        Disp::B4([
            i as u8,
            (i >> 8) as u8,
            (i >> 16) as u8,
            (i >> 24) as u8,
        ])
    }
}

impl From<i64> for Disp {
    #[inline]
    fn from(i: i64) -> Self {
        Disp::B8([
            i as u8,
            (i >> 8) as u8,
            (i >> 16) as u8,
            (i >> 24) as u8,
            (i >> 32) as u8,
            (i >> 40) as u8,
            (i >> 48) as u8,
            (i >> 56) as u8,
        ])
    }
}

impl From<i8> for Imm {
    #[inline]
    fn from(i: i8) -> Self {
        Imm::B1([i as u8])
    }
}

impl From<i16> for Imm {
    #[inline]
    fn from(i: i16) -> Self {
        Imm::B2([
            i as u8,
            (i >> 8) as u8,
        ])
    }
}

impl From<i32> for Imm {
    #[inline]
    fn from(i: i32) -> Self {
        Imm::B4([
            i as u8,
            (i >> 8) as u8,
            (i >> 16) as u8,
            (i >> 24) as u8,
        ])
    }
}

impl From<i64> for Imm {
    #[inline]
    fn from(i: i64) -> Self {
        Imm::B8([
            i as u8,
            (i >> 8) as u8,
            (i >> 16) as u8,
            (i >> 24) as u8,
            (i >> 32) as u8,
            (i >> 40) as u8,
            (i >> 48) as u8,
            (i >> 56) as u8,
        ])
    }
}

impl<'a> IntoIterator for &'a Prefix1 {
    type Item = u8;
    type IntoIter = Once<u8>;
    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(*self as u8)
    }
}

impl<'a> IntoIterator for &'a Prefix2 {
    type Item = u8;
    type IntoIter = Once<u8>;
    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(*self as u8)
    }
}

impl<'a> IntoIterator for &'a Prefix3 {
    type Item = u8;
    type IntoIter = Once<u8>;
    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(*self as u8)
    }
}

impl<'a> IntoIterator for &'a Prefix4 {
    type Item = u8;
    type IntoIter = Once<u8>;
    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(*self as u8)
    }
}

impl<'a> IntoIterator for &'a Rex {
    type Item = u8;
    type IntoIter = Once<u8>;
    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self.0)
    }
}

impl<'a> IntoIterator for &'a Modrm {
    type Item = u8;
    type IntoIter = Once<u8>;
    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self.0)
    }
}

impl<'a> IntoIterator for &'a Sib {
    type Item = u8;
    type IntoIter = Once<u8>;
    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self.0)
    }
}

impl<'a> IntoIterator for &'a Opcode {
    type Item = u8;
    type IntoIter = Cloned<slice::Iter<'a, u8>>;
    #[inline]
    fn into_iter(self) -> Cloned<slice::Iter<'a, u8>> {
        match *self {
            Opcode::B1(ref arr) => arr.into_iter().cloned(),
            Opcode::B2(ref arr) => arr.into_iter().cloned(),
            Opcode::B3(ref arr) => arr.into_iter().cloned(),
        }
    }
}

impl<'a> IntoIterator for &'a Disp {
    type Item = u8;
    type IntoIter = Cloned<slice::Iter<'a, u8>>;
    #[inline]
    fn into_iter(self) -> Cloned<slice::Iter<'a, u8>> {
        match *self {
            Disp::B1(ref arr) => arr.into_iter().cloned(),
            Disp::B4(ref arr) => arr.into_iter().cloned(),
            Disp::B8(ref arr) => arr.into_iter().cloned(),
        }
    }
}

impl<'a> IntoIterator for &'a Imm {
    type Item = u8;
    type IntoIter = Cloned<slice::Iter<'a, u8>>;
    #[inline]
    fn into_iter(self) -> Cloned<slice::Iter<'a, u8>> {
        match *self {
            Imm::B1(ref arr) => arr.into_iter().cloned(),
            Imm::B2(ref arr) => arr.into_iter().cloned(),
            Imm::B4(ref arr) => arr.into_iter().cloned(),
            Imm::B8(ref arr) => arr.into_iter().cloned(),
        }
    }
}

/// Iterator of encoded bytes.
pub struct Iter<'a> {
    prefix1: Option<Fuse<Once<u8>>>,
    prefix2: Option<Fuse<Once<u8>>>,
    prefix3: Option<Fuse<Once<u8>>>,
    prefix4: Option<Fuse<Once<u8>>>,
    rex: Option<Fuse<Once<u8>>>,
    opcode: Fuse<Cloned<slice::Iter<'a, u8>>>,
    modrm: Option<Fuse<Once<u8>>>,
    sib: Option<Fuse<Once<u8>>>,
    disp: Option<Fuse<Cloned<slice::Iter<'a, u8>>>>,
    imm: Option<Fuse<Cloned<slice::Iter<'a, u8>>>>,
}

impl<'a> IntoIterator for &'a Inst {
    type Item = u8;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        fn fuse<'a, I>(x: &'a Option<I>) -> Option<Fuse<<&'a I as IntoIterator>::IntoIter>>
        where &'a I: IntoIterator {
            x.as_ref().map(IntoIterator::into_iter).map(Iterator::fuse)
        }
        Iter {
            prefix1: fuse(&self.prefix1),
            prefix2: fuse(&self.prefix2),
            prefix3: fuse(&self.prefix3),
            prefix4: fuse(&self.prefix4),
            rex:     fuse(&self.rex),
            opcode:  self.opcode.into_iter().fuse(),
            modrm:   fuse(&self.modrm),
            sib:     fuse(&self.sib),
            disp:    fuse(&self.disp),
            imm:     fuse(&self.imm),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        if let Some(ref mut prefix1) = self.prefix1 { for b in prefix1 { return Some(b) } }
        if let Some(ref mut prefix2) = self.prefix2 { for b in prefix2 { return Some(b) } }
        if let Some(ref mut prefix3) = self.prefix3 { for b in prefix3 { return Some(b) } }
        if let Some(ref mut prefix4) = self.prefix4 { for b in prefix4 { return Some(b) } }
        if let Some(ref mut rex) = self.rex { for b in rex { return Some(b) } }
        for b in &mut self.opcode { return Some(b) }
        if let Some(ref mut modrm) = self.modrm { for b in modrm { return Some(b) } }
        if let Some(ref mut sib) = self.sib { for b in sib { return Some(b) } }
        if let Some(ref mut disp) = self.disp { for b in disp { return Some(b) } }
        if let Some(ref mut imm) = self.imm { for b in imm { return Some(b) } }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        fn len<I>(i: &Option<I>) -> usize where I: ExactSizeIterator {
            i.as_ref().map(ExactSizeIterator::len).unwrap_or(0)
        }
        let size = len(&self.prefix1)
            + len(&self.prefix2)
            + len(&self.prefix3)
            + len(&self.prefix4)
            + len(&self.rex)
            + self.opcode.len()
            + len(&self.modrm)
            + len(&self.sib)
            + len(&self.disp)
            + len(&self.imm);
        (size, Some(size))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> { }

impl Inst {
    #[inline]
    pub fn iter(&self) -> Iter {
        self.into_iter()
    }
}

impl Display for Inst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut iter = self.iter();
        write!(f, "{:02x}", iter.next().unwrap())?;
        for b in iter {
            write!(f, " {:02x}", b)?;
        }
        Ok(())
    }
}
