mod encode;
mod iter;

pub use self::iter::Iter;

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
    B1([u8; 1]),
    B2([u8; 2]),
    B3([u8; 3]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modrm(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sib(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Disp {
    B1([u8; 1]),
    B4([u8; 4]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Imm {
    B1([u8; 1]),
    B2([u8; 2]),
    B4([u8; 4]),
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
