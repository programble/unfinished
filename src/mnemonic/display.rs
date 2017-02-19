use mnemonic::instruction::*;
use mnemonic::operand::*;
use mnemonic::prefix::*;

use core::fmt::{Display, Formatter, Error};

impl Display for Imm8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#04x}", self.0)
    }
}

impl Display for Imm16 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#06x}", self.0)
    }
}

impl Display for Imm32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#010x}", self.0)
    }
}

impl Display for Imm64 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#018x}", self.0)
    }
}

macro_rules! impl_display_reg {
    ($reg:ident { $($var:ident => $str:expr,)+ }) => {
        impl Display for $reg {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                f.write_str(
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

impl Display for Disp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Disp::Disp8(disp) => write!(f, "{:#04x}", disp),
            Disp::Disp32(disp) => write!(f, "{:#010x}", disp),
        }
    }
}

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Scale::X1 => f.write_str("1"),
            Scale::X2 => f.write_str("2"),
            Scale::X4 => f.write_str("4"),
            Scale::X8 => f.write_str("8"),
        }
    }
}

impl_display_reg!(
    IndexReg32 {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
    }
);

impl_display_reg!(
    IndexRex32 {
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
    }
);

impl_display_reg!(
    IndexReg64 {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
    }
);

impl_display_reg!(
    IndexRex64 {
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
    }
);

impl<Base, Index> Display for Offset<Base, Index>
where Base: Display + Copy, Index: Display + Copy {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Offset::Disp(d)                   => write!(f, "{:#010x}", d),
            Offset::Index(i, s)               => write!(f, "{} * {}", i, s),
            Offset::IndexDisp(i, s, d)        => write!(f, "{} * {} + {:#010x}", i, s, d),
            Offset::Base(b)                   => write!(f, "{}", b),
            Offset::BaseDisp(b, d)            => write!(f, "{} + {}", b, d),
            Offset::BaseIndex(b, i, s)        => write!(f, "{} + {} * {}", b, i, s),
            Offset::BaseIndexDisp(b, i, s, d) => write!(f, "{} + {} * {} + {}", b, i, s, d),
            Offset::RipDisp(d)                => write!(f, "rip + {:#010x}", d),
        }
    }
}

impl_display_reg!(
    Sreg {
        Cs => "cs",
        Ds => "ds",
        Ss => "ss",
        Es => "es",
        Fs => "fs",
        Gs => "gs",
    }
);

impl<Base32, Index32, Base64, Index64> Display for Memory<Base32, Index32, Base64, Index64>
where Offset<Base32, Index32>: Display, Offset<Base64, Index64>: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Memory::Offset32(None, ref offset)       => write!(f, "[{}]", offset),
            Memory::Offset32(Some(sreg), ref offset) => write!(f, "[{}:{}]", sreg, offset),
            Memory::Offset64(None, ref offset)       => write!(f, "[{}]", offset),
            Memory::Offset64(Some(sreg), ref offset) => write!(f, "[{}:{}]", sreg, offset),
        }
    }
}

impl Display for Rm8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm8::Reg8(reg)     => write!(f, "{}", reg),
            Rm8::Rex8(rex)     => write!(f, "{}", rex),
            Rm8::Mem8(ref mem) => write!(f, "byte {}", mem),
            Rm8::Mex8(ref mex) => write!(f, "byte {}", mex),
        }
    }
}

impl Display for Rm16 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm16::Reg16(reg)     => write!(f, "{}", reg),
            Rm16::Rex16(rex)     => write!(f, "{}", rex),
            Rm16::Mem16(ref mem) => write!(f, "word {}", mem),
            Rm16::Mex16(ref mex) => write!(f, "word {}", mex),
        }
    }
}

impl Display for Rm32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm32::Reg32(reg)     => write!(f, "{}", reg),
            Rm32::Rex32(rex)     => write!(f, "{}", rex),
            Rm32::Mem32(ref mem) => write!(f, "dword {}", mem),
            Rm32::Mex32(ref mex) => write!(f, "dword {}", mex),
        }
    }
}

impl Display for Rm64 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm64::Rex64(rex)     => write!(f, "{}", rex),
            Rm64::Mem64(ref mem) => write!(f, "qword {}", mem),
            Rm64::Mex64(ref mex) => write!(f, "qword {}", mex),
        }
    }
}

macro_rules! impl_display_rmrm {
    ($rmrm:ident { $($var:ident),+ }) => {
        impl Display for $rmrm {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match *self {
                    $($rmrm::$var(ref a, ref b) => write!(f, "{}, {}", a, b)),+
                }
            }
        }
    }
}

impl_display_rmrm!(Rm8R8 { Reg8Reg8, Rex8Rex8, Mem8Reg8, Mex8Rex8 });
impl_display_rmrm!(Rm16R16 { Reg16Reg16, Rex16Rex16, Mem16Reg16, Mex16Rex16 });
impl_display_rmrm!(Rm32R32 { Reg32Reg32, Rex32Rex32, Mem32Reg32, Mex32Rex32 });
impl_display_rmrm!(Rm64R64 { Rex64Rex64, Mex64Rex64 });

impl_display_rmrm!(R8Rm8 { Reg8Reg8, Rex8Rex8, Reg8Mem8, Rex8Mex8 });
impl_display_rmrm!(R16Rm16 { Reg16Reg16, Rex16Rex16, Reg16Mem16, Rex16Mex16 });
impl_display_rmrm!(R32Rm32 { Reg32Reg32, Rex32Rex32, Reg32Mem32, Rex32Mex32 });
impl_display_rmrm!(R64Rm64 { Rex64Rex64, Rex64Mex64 });

impl<I> Display for Lock<I> where I: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "lock {}", self.0)
    }
}

impl<I> Display for Repne<I> where I: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "repne {}", self.0)
    }
}

impl<I> Display for Rep<I> where I: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "rep {}", self.0)
    }
}

impl Display for Adc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Adc::AlImm8(imm)        => write!(f, "adc al, {}", imm),
            Adc::AxImm16(imm)       => write!(f, "adc ax, {}", imm),
            Adc::EaxImm32(imm)      => write!(f, "adc eax, {}", imm),
            Adc::RaxImm32(imm)      => write!(f, "adc rax, {}", imm),
            Adc::Rm8Imm8(rm, imm)   => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm16Imm16(rm, imm) => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm32Imm32(rm, imm) => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm64Imm32(rm, imm) => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm16Imm8(rm, imm)  => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm32Imm8(rm, imm)  => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm64Imm8(rm, imm)  => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm8R8(rmr)         => write!(f, "adc {}", rmr),
            Adc::Rm16R16(rmr)       => write!(f, "adc {}", rmr),
            Adc::Rm32R32(rmr)       => write!(f, "adc {}", rmr),
            Adc::Rm64R64(rmr)       => write!(f, "adc {}", rmr),
            Adc::R8Rm8(rrm)         => write!(f, "adc {}", rrm),
            Adc::R16Rm16(rrm)       => write!(f, "adc {}", rrm),
            Adc::R32Rm32(rrm)       => write!(f, "adc {}", rrm),
            Adc::R64Rm64(rrm)       => write!(f, "adc {}", rrm),
        }
    }
}

impl Display for Adcx {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Adcx::R32Rm32(rrm) => write!(f, "adcx {}", rrm),
            Adcx::R64Rm64(rrm) => write!(f, "adcx {}", rrm),
        }
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Add::AlImm8(imm)        => write!(f, "add al, {}", imm),
            Add::AxImm16(imm)       => write!(f, "add ax, {}", imm),
            Add::EaxImm32(imm)      => write!(f, "add eax, {}", imm),
            Add::RaxImm32(imm)      => write!(f, "add rax, {}", imm),
            Add::Rm8Imm8(rm, imm)   => write!(f, "add {}, {}", rm, imm),
            Add::Rm16Imm16(rm, imm) => write!(f, "add {}, {}", rm, imm),
            Add::Rm32Imm32(rm, imm) => write!(f, "add {}, {}", rm, imm),
            Add::Rm64Imm32(rm, imm) => write!(f, "add {}, {}", rm, imm),
            Add::Rm16Imm8(rm, imm)  => write!(f, "add {}, {}", rm, imm),
            Add::Rm32Imm8(rm, imm)  => write!(f, "add {}, {}", rm, imm),
            Add::Rm64Imm8(rm, imm)  => write!(f, "add {}, {}", rm, imm),
            Add::Rm8R8(rmr)         => write!(f, "add {}", rmr),
            Add::Rm16R16(rmr)       => write!(f, "add {}", rmr),
            Add::Rm32R32(rmr)       => write!(f, "add {}", rmr),
            Add::Rm64R64(rmr)       => write!(f, "add {}", rmr),
            Add::R8Rm8(rrm)         => write!(f, "add {}", rrm),
            Add::R16Rm16(rrm)       => write!(f, "add {}", rrm),
            Add::R32Rm32(rrm)       => write!(f, "add {}", rrm),
            Add::R64Rm64(rrm)       => write!(f, "add {}", rrm),
        }
    }
}
