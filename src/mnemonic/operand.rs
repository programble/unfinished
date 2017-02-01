pub type Imm8 = u8;
pub type Imm16 = u16;
pub type Imm32 = u32;
pub type Imm64 = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoRexR8 {
    Al,
    Bl,
    Cl,
    Dl,
    Ah,
    Bh,
    Ch,
    Dh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R8 {
    Al,
    Bl,
    Cl,
    Dl,
    Dil,
    Sil,
    Bpl,
    Spl,
    R8l,
    R9l,
    R10l,
    R11l,
    R12l,
    R13l,
    R14l,
    R15l,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R16 {
    Ax,
    Bx,
    Cx,
    Dx,
    Di,
    Si,
    Bp,
    Sp,
    R8w,
    R9w,
    R10w,
    R11w,
    R12w,
    R13w,
    R14w,
    R15w,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R32 {
    Eax,
    Ebx,
    Ecx,
    Edx,
    Edi,
    Esi,
    Ebp,
    Esp,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R64 {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rdi,
    Rsi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Memory<B = R64, I = Index>(pub Option<SegmentSelector>, pub Offset<B, I>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SegmentSelector {
    Cs,
    Ss,
    Ds,
    Es,
    Fs,
    Gs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Offset<B = R64, I = Index> {
    Displacement(Displacement),
    Index(I, Scale),
    IndexDisplacement(I, Scale, Displacement),
    Base(B),
    BaseDisplacement(B, Displacement),
    BaseIndex(B, I, Scale),
    BaseIndexDisplacement(B, I, Scale, Displacement),
    RipDisplacement(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoRexBase {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rdi,
    Rsi,
    Rbp,
    Rsp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoRexIndex {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rdi,
    Rsi,
    Rbp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Index {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rdi,
    Rsi,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scale {
    X1,
    X2,
    X4,
    X8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Displacement {
    Disp8(u8),
    Disp16(u16),
    Disp32(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoRexRm8 {
    R8(NoRexR8),
    M8(Memory<NoRexBase, NoRexIndex>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm8 {
    R8x(R8),
    M8(Memory),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm16 {
    R16(R16),
    M16(Memory),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm32 {
    R32(R32),
    M32(Memory),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm64 {
    R64(R64),
    M64(Memory),
}
