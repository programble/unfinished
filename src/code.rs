use core::iter::{self, Once};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Prefix1 {
    Lock = 0xf0,
    Repne = 0xf2,
    Rep = 0xf3,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Prefix3 {
    OperandSize = 0x66,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Prefix4 {
    AddressSize = 0x67,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rex(pub u8); // Wants to be NonZero.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    B1(u8),
    B2(u8, u8),
    B3(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modrm(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sib(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Disp {
    B1(u8),
    B4(u8, u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Imm {
    B1(u8),
    B2(u8, u8),
    B4(u8, u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction {
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

impl IntoIterator for Prefix1 {
    type Item = u8;
    type IntoIter = Once<u8>;

    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self as u8)
    }
}

impl IntoIterator for Prefix2 {
    type Item = u8;
    type IntoIter = Once<u8>;

    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self as u8)
    }
}

impl IntoIterator for Prefix3 {
    type Item = u8;
    type IntoIter = Once<u8>;

    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self as u8)
    }
}

impl IntoIterator for Prefix4 {
    type Item = u8;
    type IntoIter = Once<u8>;

    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self as u8)
    }
}

impl IntoIterator for Rex {
    type Item = u8;
    type IntoIter = Once<u8>;

    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self.0)
    }
}

impl IntoIterator for Modrm {
    type Item = u8;
    type IntoIter = Once<u8>;

    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self.0)
    }
}

impl IntoIterator for Sib {
    type Item = u8;
    type IntoIter = Once<u8>;

    #[inline]
    fn into_iter(self) -> Once<u8> {
        iter::once(self.0)
    }
}
