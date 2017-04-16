/// 8-bit general-purpose registers without REX prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R8l {
    Al, Bl, Cl, Dl, Ah, Bh, Ch, Dh
}

/// 8-bit general-purpose registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R8 {
    Al, Bl, Cl, Dl, Sil, Dil, Bpl, Spl, R8l, R9l, R10l, R11l, R12l, R13l, R14l, R15l
}

/// 16-bit general-purpose registers without REX prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R16l {
    Ax, Bx, Cx, Dx, Si, Di, Bp, Sp
}

/// 16-bit general-purpose registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R16 {
    Ax, Bx, Cx, Dx, Si, Di, Bp, Sp, R8w, R9w, R10w, R11w, R12w, R13w, R14w, R15w
}

/// 32-bit general-purpose registers without REX prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R32l {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, Esp
}

/// 32-bit general-purpose registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R32 {
    Eax, Ebx, Ecx, Edx, Esi, Edi, Ebp, Esp, R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d
}

/// 64-bit general-purpose registers without REX prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R64l {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp
}

/// 64-bit general-purpose registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum R64 {
    Rax, Rbx, Rcx, Rdx, Rsi, Rdi, Rbp, Rsp, R8, R9, R10, R11, R12, R13, R14, R15
}

/// Segment registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sreg {
    Cs, Ds, Ss, Es, Fs, Gs
}

/// FPU register-stack indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sti {
    St0, St1, St2, St3, St4, St5, St6, St7
}

/// Control registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cr {
    Cr0, Cr2, Cr3, Cr4, Cr8
}

/// Debug registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dr {
    Dr0, Dr1, Dr2, Dr3, Dr4, Dr5, Dr6, Dr7
}
