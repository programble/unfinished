use mnemonic::instruction::*;
use mnemonic::operand::*;

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
    R8L {
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
    R8 {
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
    R16 {
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
    R32L {
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
    R32 {
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
    R64L {
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
    R64 {
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

impl_display_reg!(
    Index32L {
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
    Index32 {
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
    Index64L {
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
    Index64 {
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

impl_display_reg!(
    Scale {
        X1 => "1",
        X2 => "2",
        X4 => "4",
        X8 => "8",
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

impl<Base, Index> Display for Offset<Base, Index>
where Base: Display + Copy, Index: Display + Copy {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Offset::Base(b)                   => write!(f, "{}", b),
            Offset::BaseDisp(b, d)            => write!(f, "{} + {}", b, d),
            Offset::BaseIndex(b, i, s)        => write!(f, "{} + {} * {}", b, i, s),
            Offset::BaseIndexDisp(b, i, s, d) => write!(f, "{} + {} * {} + {}", b, i, s, d),
            Offset::Index(i, s)               => write!(f, "{} * {}", i, s),
            Offset::IndexDisp(i, s, d)        => write!(f, "{} * {} + {:#010x}", i, s, d),
            Offset::Disp(d)                   => write!(f, "{:#010x}", d),
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

impl<B32, I32, B64, I64> Display for Mem<B32, I32, B64, I64>
where Offset<B32, I32>: Display, Offset<B64, I64>: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Mem::Offset32(None, ref offset)       => write!(f, "[{}]", offset),
            Mem::Offset32(Some(sreg), ref offset) => write!(f, "[{}:{}]", sreg, offset),
            Mem::Offset64(None, ref offset)       => write!(f, "[{}]", offset),
            Mem::Offset64(Some(sreg), ref offset) => write!(f, "[{}:{}]", sreg, offset),
        }
    }
}

impl Display for Rm8L {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm8L::R8L(r) => write!(f, "{}", r),
            Rm8L::M8L(m) => write!(f, "byte {}", m),
        }
    }
}

impl Display for Rm8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm8::R8(r) => write!(f, "{}", r),
            Rm8::M8(m) => write!(f, "byte {}", m),
        }
    }
}

impl Display for Rm16 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm16::R16(r) => write!(f, "{}", r),
            Rm16::M16(m) => write!(f, "word {}", m),
        }
    }
}

impl Display for Rm32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm32::R32(r) => write!(f, "{}", r),
            Rm32::M32(m) => write!(f, "dword {}", m),
        }
    }
}

impl Display for Rm64 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Rm64::R64(r) => write!(f, "{}", r),
            Rm64::M64(m) => write!(f, "qword {}", m),
        }
    }
}

impl Display for Adc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Adc::AlImm8(imm)        => write!(f, "adc al, {}", imm),
            Adc::AxImm16(imm)       => write!(f, "adc ax, {}", imm),
            Adc::EaxImm32(imm)      => write!(f, "adc eax, {}", imm),
            Adc::RaxImm32(imm)      => write!(f, "adc rax, {}", imm),
            Adc::Rm8LImm8(rm, imm)  => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm8Imm8(rm, imm)   => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm16Imm16(rm, imm) => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm32Imm32(rm, imm) => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm64Imm32(rm, imm) => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm16Imm8(rm, imm)  => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm32Imm8(rm, imm)  => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm64Imm8(rm, imm)  => write!(f, "adc {}, {}", rm, imm),
            Adc::Rm8LR8L(rm, r)     => write!(f, "adc {}, {}", rm, r),
            Adc::Rm8R8(rm, r)       => write!(f, "adc {}, {}", rm, r),
            Adc::Rm16R16(rm, r)     => write!(f, "adc {}, {}", rm, r),
            Adc::Rm32R32(rm, r)     => write!(f, "adc {}, {}", rm, r),
            Adc::Rm64R64(rm, r)     => write!(f, "adc {}, {}", rm, r),
            Adc::R8LRm8L(r, rm)     => write!(f, "adc {}, {}", r, rm),
            Adc::R8Rm8(r, rm)       => write!(f, "adc {}, {}", r, rm),
            Adc::R16Rm16(r, rm)     => write!(f, "adc {}, {}", r, rm),
            Adc::R32Rm32(r, rm)     => write!(f, "adc {}, {}", r, rm),
            Adc::R64Rm64(r, rm)     => write!(f, "adc {}, {}", r, rm),
        }
    }
}

impl Display for Adcx {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Adcx::R32Rm32(r, rm) => write!(f, "adcx {}, {}", r, rm),
            Adcx::R64Rm64(r, rm) => write!(f, "adcx {}, {}", r, rm),
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
            Add::Rm8LImm8(rm, imm)  => write!(f, "add {}, {}", rm, imm),
            Add::Rm8Imm8(rm, imm)   => write!(f, "add {}, {}", rm, imm),
            Add::Rm16Imm16(rm, imm) => write!(f, "add {}, {}", rm, imm),
            Add::Rm32Imm32(rm, imm) => write!(f, "add {}, {}", rm, imm),
            Add::Rm64Imm32(rm, imm) => write!(f, "add {}, {}", rm, imm),
            Add::Rm16Imm8(rm, imm)  => write!(f, "add {}, {}", rm, imm),
            Add::Rm32Imm8(rm, imm)  => write!(f, "add {}, {}", rm, imm),
            Add::Rm64Imm8(rm, imm)  => write!(f, "add {}, {}", rm, imm),
            Add::Rm8LR8L(rm, r)     => write!(f, "add {}, {}", rm, r),
            Add::Rm8R8(rm, r)       => write!(f, "add {}, {}", rm, r),
            Add::Rm16R16(rm, r)     => write!(f, "add {}, {}", rm, r),
            Add::Rm32R32(rm, r)     => write!(f, "add {}, {}", rm, r),
            Add::Rm64R64(rm, r)     => write!(f, "add {}, {}", rm, r),
            Add::R8LRm8L(r, rm)     => write!(f, "add {}, {}", r, rm),
            Add::R8Rm8(r, rm)       => write!(f, "add {}, {}", r, rm),
            Add::R16Rm16(r, rm)     => write!(f, "add {}, {}", r, rm),
            Add::R32Rm32(r, rm)     => write!(f, "add {}, {}", r, rm),
            Add::R64Rm64(r, rm)     => write!(f, "add {}, {}", r, rm),
        }
    }
}

impl Display for Adox {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Adox::R32Rm32(r, rm) => write!(f, "adox {}, {}", r, rm),
            Adox::R64Rm64(r, rm) => write!(f, "adox {}, {}", r, rm),
        }
    }
}

impl Display for And {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            And::AlImm8(imm)        => write!(f, "and al, {}", imm),
            And::AxImm16(imm)       => write!(f, "and ax, {}", imm),
            And::EaxImm32(imm)      => write!(f, "and eax, {}", imm),
            And::RaxImm32(imm)      => write!(f, "and rax, {}", imm),
            And::Rm8LImm8(rm, imm)  => write!(f, "and {}, {}", rm, imm),
            And::Rm8Imm8(rm, imm)   => write!(f, "and {}, {}", rm, imm),
            And::Rm16Imm16(rm, imm) => write!(f, "and {}, {}", rm, imm),
            And::Rm32Imm32(rm, imm) => write!(f, "and {}, {}", rm, imm),
            And::Rm64Imm32(rm, imm) => write!(f, "and {}, {}", rm, imm),
            And::Rm16Imm8(rm, imm)  => write!(f, "and {}, {}", rm, imm),
            And::Rm32Imm8(rm, imm)  => write!(f, "and {}, {}", rm, imm),
            And::Rm64Imm8(rm, imm)  => write!(f, "and {}, {}", rm, imm),
            And::Rm8LR8L(rm, r)     => write!(f, "and {}, {}", rm, r),
            And::Rm8R8(rm, r)       => write!(f, "and {}, {}", rm, r),
            And::Rm16R16(rm, r)     => write!(f, "and {}, {}", rm, r),
            And::Rm32R32(rm, r)     => write!(f, "and {}, {}", rm, r),
            And::Rm64R64(rm, r)     => write!(f, "and {}, {}", rm, r),
            And::R8LRm8L(r, rm)     => write!(f, "and {}, {}", r, rm),
            And::R8Rm8(r, rm)       => write!(f, "and {}, {}", r, rm),
            And::R16Rm16(r, rm)     => write!(f, "and {}, {}", r, rm),
            And::R32Rm32(r, rm)     => write!(f, "and {}, {}", r, rm),
            And::R64Rm64(r, rm)     => write!(f, "and {}, {}", r, rm),
        }
    }
}

impl Display for Bsf {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Bsf::R16Rm16(r, rm) => write!(f, "bsf {}, {}", r, rm),
            Bsf::R32Rm32(r, rm) => write!(f, "bsf {}, {}", r, rm),
            Bsf::R64Rm64(r, rm) => write!(f, "bsf {}, {}", r, rm),
        }
    }
}

impl Display for Bsr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Bsr::R16Rm16(r, rm) => write!(f, "bsr {}, {}", r, rm),
            Bsr::R32Rm32(r, rm) => write!(f, "bsr {}, {}", r, rm),
            Bsr::R64Rm64(r, rm) => write!(f, "bsr {}, {}", r, rm),
        }
    }
}

impl Display for Bswap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Bswap::R32(r) => write!(f, "bswap {}", r),
            Bswap::R64(r) => write!(f, "bswap {}", r),
        }
    }
}

impl Display for Bt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Bt::Rm16R16(rm, r)    => write!(f, "bt {}, {}", rm, r),
            Bt::Rm32R32(rm, r)    => write!(f, "bt {}, {}", rm, r),
            Bt::Rm64R64(rm, r)    => write!(f, "bt {}, {}", rm, r),
            Bt::Rm16Imm8(rm, imm) => write!(f, "bt {}, {}", rm, imm),
            Bt::Rm32Imm8(rm, imm) => write!(f, "bt {}, {}", rm, imm),
            Bt::Rm64Imm8(rm, imm) => write!(f, "bt {}, {}", rm, imm),
        }
    }
}

impl Display for Btc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Btc::Rm16R16(rm, r)    => write!(f, "btc {}, {}", rm, r),
            Btc::Rm32R32(rm, r)    => write!(f, "btc {}, {}", rm, r),
            Btc::Rm64R64(rm, r)    => write!(f, "btc {}, {}", rm, r),
            Btc::Rm16Imm8(rm, imm) => write!(f, "btc {}, {}", rm, imm),
            Btc::Rm32Imm8(rm, imm) => write!(f, "btc {}, {}", rm, imm),
            Btc::Rm64Imm8(rm, imm) => write!(f, "btc {}, {}", rm, imm),
        }
    }
}

impl Display for Btr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Btr::Rm16R16(rm, r)    => write!(f, "btr {}, {}", rm, r),
            Btr::Rm32R32(rm, r)    => write!(f, "btr {}, {}", rm, r),
            Btr::Rm64R64(rm, r)    => write!(f, "btr {}, {}", rm, r),
            Btr::Rm16Imm8(rm, imm) => write!(f, "btr {}, {}", rm, imm),
            Btr::Rm32Imm8(rm, imm) => write!(f, "btr {}, {}", rm, imm),
            Btr::Rm64Imm8(rm, imm) => write!(f, "btr {}, {}", rm, imm),
        }
    }
}

impl Display for Bts {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Bts::Rm16R16(rm, r)    => write!(f, "bts {}, {}", rm, r),
            Bts::Rm32R32(rm, r)    => write!(f, "bts {}, {}", rm, r),
            Bts::Rm64R64(rm, r)    => write!(f, "bts {}, {}", rm, r),
            Bts::Rm16Imm8(rm, imm) => write!(f, "bts {}, {}", rm, imm),
            Bts::Rm32Imm8(rm, imm) => write!(f, "bts {}, {}", rm, imm),
            Bts::Rm64Imm8(rm, imm) => write!(f, "bts {}, {}", rm, imm),
        }
    }
}
