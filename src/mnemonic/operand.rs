#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm8(pub u8);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm16(pub u16);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm32(pub u32);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm64(pub u64);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rel8(pub i8);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rel32(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cc {
    A,
    Ae,
    B,
    Be,
    C,
    E,
    G,
    Ge,
    L,
    Le,
    Na,
    Nae,
    Nb,
    Nbe,
    Nc,
    Ne,
    Ng,
    Nge,
    Nl,
    Nle,
    No,
    Np,
    Ns,
    Nz,
    O,
    P,
    Pe,
    Po,
    S,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R8l {
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
pub enum R32l {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, Esp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, Esp, R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R64l {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R64 {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp, R8, R9, R10, R11, R12, R13, R14, R15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sti {
    St0, St1, St2, St3, St4, St5, St6, St7
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cr {
    Cr0, Cr2, Cr3, Cr4, Cr8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dr {
    Dr0, Dr1, Dr2, Dr3, Dr4, Dr5, Dr6, Dr7
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index32l {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index64l {
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
pub enum Moffs {
    Moffset32(Option<Sreg>, u32),
    Moffset64(Option<Sreg>, u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm8l {
    R8l(R8l),
    M8l(Mem<R32l, Index32l, R64l, Index64l>),
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
