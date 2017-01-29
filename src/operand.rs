pub type Imm8 = u8;
pub type Imm16 = u16;
pub type Imm32 = u32;
pub type Imm64 = u64;

pub enum R8 {
    Al, Bl, Cl, Dl,
    Ah, Bh, Ch, Dh,
    Dil, Sil, Bpl, Spl,
    R8l, R9l, R10l, R11l, R12l, R13l, R14l, R15l,
}

pub enum R16 {
    Ax, Bx, Cx, Dx, Di, Si, Bp, Sp,
    R8w, R9w, R10w, R11w, R12w, R13w, R14w, R15w,
}

pub enum R32 {
    Eax, Ebx, Ecx, Edx, Edi, Esi, Ebp, Esp,
    R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d,
}

pub enum R64 {
    Rax, Rbx, Rcx, Rdx, Rdi, Rsi, Rbp, Rsp,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

pub enum Rm8 {
    R8(R8),
    // TODO
}

pub enum Rm16 {
    R16(R16),
    // TODO
}

pub enum Rm32 {
    R32(R32),
    // TODO
}

pub enum Rm64 {
    R64(R64),
    // TODO
}
