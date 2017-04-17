use core::fmt::{Display, Formatter, Error};

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
    R8l {
        Al => "al",
        Bl => "bl",
        Cl => "cl",
        Dl => "dl",
        Ah => "ah",
        Bh => "bh",
        Ch => "ch",
        Dh => "dh",
    },

    R8 {
        Al   => "al",
        Bl   => "bl",
        Cl   => "cl",
        Dl   => "dl",
        Sil  => "sil",
        Dil  => "dil",
        Bpl  => "bpl",
        Spl  => "spl",
        R8l  => "r8l",
        R9l  => "r9l",
        R10l => "r10l",
        R11l => "r11l",
        R12l => "r12l",
        R13l => "r13l",
        R14l => "r14l",
        R15l => "r15l",
    },

    R16l {
        Ax => "ax",
        Bx => "bx",
        Cx => "cx",
        Dx => "dx",
        Si => "si",
        Di => "di",
        Bp => "bp",
        Sp => "sp",
    },

    R16 {
        Ax   => "ax",
        Bx   => "bx",
        Cx   => "cx",
        Dx   => "dx",
        Si   => "si",
        Di   => "di",
        Bp   => "bp",
        Sp   => "sp",
        R8w  => "r8w",
        R9w  => "r9w",
        R10w => "r10w",
        R11w => "r11w",
        R12w => "r12w",
        R13w => "r13w",
        R14w => "r14w",
        R15w => "r15w",
    },

    R32l {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
        Esp => "esp",
    },

    R32 {
        Eax  => "eax",
        Ebx  => "ebx",
        Ecx  => "ecx",
        Edx  => "edx",
        Esi  => "esi",
        Edi  => "edi",
        Ebp  => "ebp",
        Esp  => "esp",
        R8d  => "r8d",
        R9d  => "r9d",
        R10d => "r10d",
        R11d => "r11d",
        R12d => "r12d",
        R13d => "r13d",
        R14d => "r14d",
        R15d => "r15d",
    },

    R64l {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
        Rsp => "rsp",
    },

    R64 {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
        Rsp => "rsp",
        R8  => "r8",
        R9  => "r9",
        R10 => "r10",
        R11 => "r11",
        R12 => "r12",
        R13 => "r13",
        R14 => "r14",
        R15 => "r15",
    },

    Sreg {
        Cs => "cs",
        Ds => "ds",
        Ss => "ss",
        Es => "es",
        Fs => "fs",
        Gs => "gs",
    },

    Sti {
        St0 => "st0",
        St1 => "st1",
        St2 => "st2",
        St3 => "st3",
        St4 => "st4",
        St5 => "st5",
        St6 => "st6",
        St7 => "st7",
    },

    Cr {
        Cr0 => "cr0",
        Cr2 => "cr2",
        Cr3 => "cr3",
        Cr4 => "cr4",
        Cr8 => "cr8",
    },

    Dr {
        Dr0 => "dr0",
        Dr1 => "dr1",
        Dr2 => "dr2",
        Dr3 => "dr3",
        Dr4 => "dr4",
        Dr5 => "dr5",
        Dr6 => "dr6",
        Dr7 => "dr7",
    },
}
