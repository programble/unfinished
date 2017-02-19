#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm8(pub u8);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm16(pub u16);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm32(pub u32);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm64(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg8 {
    Al, Bl, Cl, Dl, Ah, Bh, Ch, Dh
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rex8 {
    Al, Bl, Cl, Dl, Sil, Dil, Bpl, Spl, R8l, R9l, R10l, R11l, R12l, R13l, R14l, R15l
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg16 {
    Ax, Bx, Cx, Dx, Si, Di, Bp, Sp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rex16 {
    Ax, Bx, Cx, Dx, Si, Di, Bp, Sp, R8w, R9w, R10w, R11w, R12w, R13w, R14w, R15w
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, Esp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rex32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, Esp, R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reg64 {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rex64 {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp, R8, R9, R10, R11, R12, R13, R14, R15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Disp {
    Disp8(i8),
    Disp32(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scale {
    X1, X2, X4, X8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexReg32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexRex32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexReg64 {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IndexRex64 {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, R8, R9, R10, R11, R12, R13, R14, R15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Offset<Base, Index> {
    Disp(Disp),
    Index(Index, Scale),
    IndexDisp(Index, Scale, Disp),
    Base(Base),
    BaseDisp(Base, Disp),
    BaseIndex(Base, Index, Scale),
    BaseIndexDisp(Base, Index, Scale, Disp),
    RipDisp(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sreg {
    Cs, Ds, Ss, Es, Fs, Gs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Memory<Base32, Index32, Base64, Index64> {
    Offset32(Option<Sreg>, Offset<Base32, Index32>),
    Offset64(Option<Sreg>, Offset<Base64, Index64>),
}

pub type Mem = Memory<Reg32, IndexReg32, Reg64, IndexReg64>;
pub type Mex = Memory<Rex32, IndexRex32, Rex64, IndexRex64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm8 {
    Reg8(Reg8),
    Rex8(Rex8),
    Mem8(Mem),
    Mex8(Mex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm16 {
    Reg16(Reg16),
    Rex16(Rex16),
    Mem16(Mem),
    Mex16(Mex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm32 {
    Reg32(Reg32),
    Rex32(Rex32),
    Mem32(Mem),
    Mex32(Mex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm64 {
    Rex64(Rex64),
    Mem64(Mem),
    Mex64(Mex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm8R8 {
    Reg8Reg8(Reg8, Reg8),
    Rex8Rex8(Rex8, Rex8),
    Mem8Reg8(Mem, Reg8),
    Mex8Rex8(Mex, Rex8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm16R16 {
    Reg16Reg16(Reg16, Reg16),
    Rex16Rex16(Rex16, Rex16),
    Mem16Reg16(Mem, Reg16),
    Mex16Rex16(Mex, Rex16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm32R32 {
    Reg32Reg32(Reg32, Reg32),
    Rex32Rex32(Rex32, Rex32),
    Mem32Reg32(Mem, Reg32),
    Mex32Rex32(Mex, Rex32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rm64R64 {
    Rex64Rex64(Rex64, Rex64),
    Mex64Rex64(Mex, Rex64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R8Rm8 {
    Reg8Reg8(Reg8, Reg8),
    Rex8Rex8(Rex8, Rex8),
    Reg8Mem8(Reg8, Mem),
    Rex8Mex8(Reg8, Mex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R16Rm16 {
    Reg16Reg16(Reg16, Reg16),
    Rex16Rex16(Rex16, Rex16),
    Reg16Mem16(Reg16, Mem),
    Rex16Mex16(Reg16, Mex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R32Rm32 {
    Reg32Reg32(Reg32, Reg32),
    Rex32Rex32(Rex32, Rex32),
    Reg32Mem32(Reg32, Mem),
    Rex32Mex32(Reg32, Mex),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R64Rm64 {
    Rex64Rex64(Rex64, Rex64),
    Rex64Mex64(Reg64, Mex),
}
