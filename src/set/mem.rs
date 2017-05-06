use core::fmt::{Display, Formatter, Error};

use super::reg::{R8l, R8, R16, R32l, R32, R64l, R64, Sreg};

/// 32-bit indexing registers without REX prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index32l {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp
}

/// 32-bit indexing registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d
}

/// 64-bit indexing registers without REX prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index64l {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp
}

/// 64-bit indexing registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index64 {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, R8, R9, R10, R11, R12, R13, R14, R15
}

/// Index scale factors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scale {
    X1, X2, X4, X8
}

/// Displacements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Disp {
    /// 8-bit displacement.
    Disp8(i8),

    /// 32-bit displacement.
    Disp32(i32),
}

/// Memory offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Offset<Base, Index> {
    /// Base.
    Base(Base),

    /// Base + Displacement.
    BaseDisp(Base, Disp),

    /// Base + (Index * Scale).
    BaseIndex(Base, Index, Scale),

    /// Base + (Index * Scale) + Displacement.
    BaseIndexDisp(Base, Index, Scale, Disp),

    /// (Index * Scale) + Displacement.
    IndexDisp(Index, Scale, i32),

    /// Displacement.
    Disp(i32),

    /// RIP + displacement.
    RipDisp(i32),
}

/// Memory operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mem<B32 = R32, I32 = Index32, B64 = R64, I64 = Index64> {
    /// 32-bit offset.
    Offset32(Option<Sreg>, Offset<B32, I32>),

    /// 64-bit offset.
    Offset64(Option<Sreg>, Offset<B64, I64>),
}

/// Direct memory offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Moffs {
    /// 32-bit offset.
    Moffset32(Option<Sreg>, u32),

    /// 64-bit offset.
    Moffset64(Option<Sreg>, u64),
}

/// Register or memory operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm<R, B32 = R32, I32 = Index32, B64 = R64, I64 = Index64> {
    R(R),
    M(Mem<B32, I32, B64, I64>),
}

/// 8-bit register or memory operand without REX prefix.
pub type Rm8l = Rm<R8l, R32l, Index32l, R64l, Index64l>;

/// 8-bit register or memory operand.
pub type Rm8 = Rm<R8>;

/// 16-bit register or memory operand.
pub type Rm16 = Rm<R16>;

/// 32-bit register or memory operand.
pub type Rm32 = Rm<R32>;

/// 64-bit register or memory operand.
pub type Rm64 = Rm<R64>;

macro_rules! impl_display {
    ($($ty:ident { $($var:ident => $str:expr,)+ },)+) => {
        $(
            impl Display for $ty {
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    f.write_str(match *self { $($ty::$var => $str),+ })
                }
            }
        )+
    }
}

impl_display! {
    Index32l {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
    },

    Index32 {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
        R8d => "r8d",
        R9d => "r9d",
        R10d => "r10d",
        R11d => "r11d",
        R12d => "r12d",
        R13d => "r13d",
        R14d => "r14d",
        R15d => "r15d",
    },

    Index64l {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
    },

    Index64 {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
        R8 => "r8",
        R9 => "r9",
        R10 => "r10",
        R11 => "r11",
        R12 => "r12",
        R13 => "r13",
        R14 => "r14",
        R15 => "r15",
    },

    Scale {
        X1 => "1",
        X2 => "2",
        X4 => "4",
        X8 => "8",
    },
}

impl Display for Disp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Disp::Disp8(disp)  => write!(f, "{:#04x}", disp),
            Disp::Disp32(disp) => write!(f, "{:#010x}", disp),
        }
    }
}

impl<Base, Index> Display for Offset<Base, Index>
where Base: Display, Index: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Offset::Base(ref b) => write!(f, "{}", b),
            Offset::BaseDisp(ref b, d) => write!(f, "{} + {}", b, d),
            Offset::BaseIndex(ref b, ref i, s) => write!(f, "{} + {} * {}", b, i, s),
            Offset::BaseIndexDisp(ref b, ref i, s, d) => write!(f, "{} + {} * {} + {}", b, i, s, d),
            Offset::IndexDisp(ref i, s, d) => write!(f, "{} * {} + {}", i, s, d),
            Offset::Disp(d) => write!(f, "{:#010x}", d),
            Offset::RipDisp(d) => write!(f, "rip + {:#010x}", d),
        }
    }
}

impl<B32, I32, B64, I64> Display for Mem<B32, I32, B64, I64>
where Offset<B32, I32>: Display, Offset<B64, I64>: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Mem::Offset32(None, ref offset) => write!(f, "[{}]", offset),
            Mem::Offset64(None, ref offset) => write!(f, "[{}]", offset),
            Mem::Offset32(Some(sreg), ref offset) => write!(f, "[{}:{}]", sreg, offset),
            Mem::Offset64(Some(sreg), ref offset) => write!(f, "[{}:{}]", sreg, offset),
        }
    }
}

impl Display for Moffs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Moffs::Moffset32(None, offset) => write!(f, "[dword {:#010x}]", offset),
            Moffs::Moffset64(None, offset) => write!(f, "[qword {:#018x}]", offset),
            Moffs::Moffset32(Some(sreg), offset) => write!(f, "[{}:dword {:#010x}]", sreg, offset),
            Moffs::Moffset64(Some(sreg), offset) => write!(f, "[{}:qword {:#018x}]", sreg, offset),
        }
    }
}

impl<R, B32, I32, B64, I64> Display for Rm<R, B32, I32, B64, I64>
where R: Display, Mem<B32, I32, B64, I64>: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm::R(ref r) => write!(f, "{}", r),
            Rm::M(ref m) => write!(f, "{}", m), // FIXME: size.
        }
    }
}
