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

impl Display for Rel32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#010x}", self.0)
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
    Cc {
        A   => "a",
        Ae  => "ae",
        B   => "b",
        Be  => "be",
        C   => "c",
        E   => "e",
        G   => "g",
        Ge  => "ge",
        L   => "l",
        Le  => "le",
        Na  => "na",
        Nae => "nae",
        Nb  => "nb",
        Nbe => "nbe",
        Nc  => "nc",
        Ne  => "ne",
        Ng  => "ng",
        Nge => "nge",
        Nl  => "nl",
        Nle => "nle",
        No  => "no",
        Np  => "np",
        Ns  => "ns",
        Nz  => "nz",
        O   => "o",
        P   => "p",
        Pe  => "pe",
        Po  => "po",
        S   => "s",
        Z   => "z",
    }
);

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

macro_rules! impl_display_arith {
    ($m:ident, $s:expr) => {
        impl Display for $m {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match *self {
                    $m::AlImm8(imm)        => write!(f, concat!($s, " al, {}"), imm),
                    $m::AxImm16(imm)       => write!(f, concat!($s, " ax, {}"), imm),
                    $m::EaxImm32(imm)      => write!(f, concat!($s, " eax, {}"), imm),
                    $m::RaxImm32(imm)      => write!(f, concat!($s, " rax, {}"), imm),
                    $m::Rm8LImm8(rm, imm)  => write!(f, concat!($s, " {}, {}"), rm, imm),
                    $m::Rm8Imm8(rm, imm)   => write!(f, concat!($s, " {}, {}"), rm, imm),
                    $m::Rm16Imm16(rm, imm) => write!(f, concat!($s, " {}, {}"), rm, imm),
                    $m::Rm32Imm32(rm, imm) => write!(f, concat!($s, " {}, {}"), rm, imm),
                    $m::Rm64Imm32(rm, imm) => write!(f, concat!($s, " {}, {}"), rm, imm),
                    $m::Rm16Imm8(rm, imm)  => write!(f, concat!($s, " {}, {}"), rm, imm),
                    $m::Rm32Imm8(rm, imm)  => write!(f, concat!($s, " {}, {}"), rm, imm),
                    $m::Rm64Imm8(rm, imm)  => write!(f, concat!($s, " {}, {}"), rm, imm),
                    $m::Rm8LR8L(rm, r)     => write!(f, concat!($s, " {}, {}"), rm, r),
                    $m::Rm8R8(rm, r)       => write!(f, concat!($s, " {}, {}"), rm, r),
                    $m::Rm16R16(rm, r)     => write!(f, concat!($s, " {}, {}"), rm, r),
                    $m::Rm32R32(rm, r)     => write!(f, concat!($s, " {}, {}"), rm, r),
                    $m::Rm64R64(rm, r)     => write!(f, concat!($s, " {}, {}"), rm, r),
                    $m::R8LRm8L(r, rm)     => write!(f, concat!($s, " {}, {}"), r, rm),
                    $m::R8Rm8(r, rm)       => write!(f, concat!($s, " {}, {}"), r, rm),
                    $m::R16Rm16(r, rm)     => write!(f, concat!($s, " {}, {}"), r, rm),
                    $m::R32Rm32(r, rm)     => write!(f, concat!($s, " {}, {}"), r, rm),
                    $m::R64Rm64(r, rm)     => write!(f, concat!($s, " {}, {}"), r, rm),
                }
            }
        }
    }
}

impl_display_arith!(Adc, "adc");

impl Display for Adcx {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Adcx::R32Rm32(r, rm) => write!(f, "adcx {}, {}", r, rm),
            Adcx::R64Rm64(r, rm) => write!(f, "adcx {}, {}", r, rm),
        }
    }
}

impl_display_arith!(Add, "add");

impl Display for Adox {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Adox::R32Rm32(r, rm) => write!(f, "adox {}, {}", r, rm),
            Adox::R64Rm64(r, rm) => write!(f, "adox {}, {}", r, rm),
        }
    }
}

impl_display_arith!(And, "and");

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

impl Display for Call {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Call::Rel32(rel) => write!(f, "call rel {}", rel),
            Call::Rm64(rm)   => write!(f, "call near {}", rm),
            Call::M1616(m)   => write!(f, "call far {}", m),
            Call::M1632(m)   => write!(f, "call far {}", m),
            Call::M1664(m)   => write!(f, "call far {}", m),
        }
    }
}

impl Display for Cbw {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("cbw")
    }
}

impl Display for Cwde {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("cwde")
    }
}

impl Display for Cdqe {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("cdqe")
    }
}

impl Display for Clac {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("clac")
    }
}

impl Display for Clc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("clc")
    }
}

impl Display for Cld {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("cld")
    }
}

impl Display for Clflush {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Clflush::M8(m) => write!(f, "clflush {}", m),
        }
    }
}

impl Display for Clflushopt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Clflushopt::M8(m) => write!(f, "clflushopt {}", m),
        }
    }
}

impl Display for Cli {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("cli")
    }
}

impl Display for Clts {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("clts")
    }
}

impl Display for Cmc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("cmc")
    }
}

impl Display for Cmov {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Cmov::CcR16Rm16(cc, r, rm) => write!(f, "cmov{} {}, {}", cc, r, rm),
            Cmov::CcR32Rm32(cc, r, rm) => write!(f, "cmov{} {}, {}", cc, r, rm),
            Cmov::CcR64Rm64(cc, r, rm) => write!(f, "cmov{} {}, {}", cc, r, rm),
        }
    }
}

impl_display_arith!(Cmp, "cmp");

impl Display for Cmps {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Cmps::B => f.write_str("cmpsb"),
            Cmps::W => f.write_str("cmpsw"),
            Cmps::D => f.write_str("cmpsd"),
            Cmps::Q => f.write_str("cmpsq"),
        }
    }
}

impl Display for Cmpxchg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Cmpxchg::Rm8LR8L(rm, r) => write!(f, "cmpxchg {}, {}", rm, r),
            Cmpxchg::Rm8R8(rm, r)   => write!(f, "cmpxchg {}, {}", rm, r),
            Cmpxchg::Rm16R16(rm, r) => write!(f, "cmpxchg {}, {}", rm, r),
            Cmpxchg::Rm32R32(rm, r) => write!(f, "cmpxchg {}, {}", rm, r),
            Cmpxchg::Rm64R64(rm, r) => write!(f, "cmpxchg {}, {}", rm, r),
        }
    }
}
