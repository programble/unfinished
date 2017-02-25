#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm8(pub u8);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm16(pub u16);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm32(pub u32);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm64(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R8L {
    Al, Bl, Cl, Dl, Ah, Bh, Ch, Dh
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R8 {
    Al, Bl, Cl, Dl, Sil, Dil, Bpl, Spl, R8l, R9l, R10l, R11l, R12l, R13l, R14l, R15l
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R16 {
    Ax, Bx, Cx, Dx, Si, Di, Bp, Sp, R8w, R9w, R10w, R11w, R12w, R13w, R14w, R15w
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R32L {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, Esp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, Esp, R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R64L {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R64 {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp, R8, R9, R10, R11, R12, R13, R14, R15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index32L {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index64L {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index64 {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, R8, R9, R10, R11, R12, R13, R14, R15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scale {
    X1, X2, X4, X8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Disp {
    Disp8(i8),
    Disp32(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Offset<Base, Index> {
    Base(Base),
    BaseDisp(Base, Disp),
    BaseIndex(Base, Index, Scale),
    BaseIndexDisp(Base, Index, Scale, Disp),
    Index(Index, Scale),
    IndexDisp(Index, Scale, i32),
    Disp(i32),
    RipDisp(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sreg {
    Cs, Ds, Ss, Es, Fs, Gs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mem<B32 = R32, I32 = Index32, B64 = R64, I64 = Index64> {
    Offset32(Option<Sreg>, Offset<B32, I32>),
    Offset64(Option<Sreg>, Offset<B64, I64>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm8L {
    R8L(R8L),
    M8L(Mem<R32L, Index32L, R64L, Index64L>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm8 {
    R8(R8),
    M8(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm16 {
    R16(R16),
    M16(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm32 {
    R32(R32),
    M32(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm64 {
    R64(R64),
    M64(Mem),
}
