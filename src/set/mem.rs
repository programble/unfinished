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

/// 8-bit register or memory operand without REX prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm8l {
    R8l(R8l),
    M8l(Mem<R32l, Index32l, R64l, Index64l>),
}

/// 8-bit register or memory operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm8 {
    R8(R8),
    M8(Mem),
}

/// 16-bit register or memory operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm16 {
    R16(R16),
    M16(Mem),
}

/// 32-bit register or memory operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm32 {
    R32(R32),
    M32(Mem),
}

/// 64-bit register or memory operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm64 {
    R64(R64),
    M64(Mem),
}
