use mnemonic::instruction::*;
use mnemonic::operand::*;
use mnemonic::prefix::*;

use core::fmt::{Display, Formatter, Error};

macro_rules! impl_display_reg {
    ($reg:ident { $($var:ident => $str:expr,)+ }) => {
        impl Display for $reg {
            fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
                fmt.write_str(
                    match *self {
                        $($reg::$var => $str),+
                    }
                )
            }
        }
    }
}

impl_display_reg!(
    Reg8 {
        Al => "al",
        Bl => "bl",
        Cl => "cl",
        Dl => "dl",
        Ah => "ah",
        Bh => "bh",
        Ch => "ch",
        Dh => "dh",
    }
);

impl_display_reg!(
    Rex8 {
        Al => "al",
        Bl => "bl",
        Cl => "cl",
        Dl => "dl",
        Sil => "sil",
        Dil => "dil",
        Bpl => "bpl",
        Spl => "spl",
        R8l => "r8l",
        R9l => "r9l",
        R10l => "r10l",
        R11l => "r11l",
        R12l => "r12l",
        R13l => "r13l",
        R14l => "r14l",
        R15l => "r15l",
    }
);

impl_display_reg!(
    Reg16 {
        Ax => "ax",
        Bx => "bx",
        Cx => "cx",
        Dx => "dx",
        Si => "si",
        Di => "di",
        Bp => "bp",
        Sp => "sp",
    }
);

impl_display_reg!(
    Rex16 {
        Ax => "ax",
        Bx => "bx",
        Cx => "cx",
        Dx => "dx",
        Si => "si",
        Di => "di",
        Bp => "bp",
        Sp => "sp",
        R8w => "r8w",
        R9w => "r9w",
        R10w => "r10w",
        R11w => "r11w",
        R12w => "r12w",
        R13w => "r13w",
        R14w => "r14w",
        R15w => "r15w",
    }
);

impl_display_reg!(
    Reg32 {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
        Esp => "esp",
    }
);

impl_display_reg!(
    Rex32 {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
        Esp => "esp",
        R8d => "r8d",
        R9d => "r9d",
        R10d => "r10d",
        R11d => "r11d",
        R12d => "r12d",
        R13d => "r13d",
        R14d => "r14d",
        R15d => "r15d",
    }
);

impl_display_reg!(
    Reg64 {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
        Rsp => "rsp",
    }
);

impl_display_reg!(
    Rex64 {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
        Rsp => "rsp",
        R8 => "r8",
        R9 => "r9",
        R10 => "r10",
        R11 => "r11",
        R12 => "r12",
        R13 => "r13",
        R14 => "r14",
        R15 => "r15",
    }
);
